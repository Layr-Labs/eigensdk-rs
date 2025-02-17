use alloy::transports::http::reqwest::{self, Url};
use mime_sniffer::MimeTypeSniffer;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

/// OperatorMetadata is the metadata operator uploads while registering
/// itself to EigenLayer.
#[derive(Deserialize, Serialize)]
pub struct OperatorMetadata {
    /// Name of the operator
    name: String,

    /// Description of the operator. There is a 200-character limit
    description: String,

    /// Logo of the operator. This should be a link to a image file
    /// which is publicly accessible
    logo: String,

    /// Website of the operator
    website: Option<String>,

    /// Twitter handle of the operator (optional)
    twitter: Option<String>,
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum OperatorMetadataError {
    #[error("Name cannot be empty")]
    NameEmpty,
    #[error("Name has to be less than 500 characters long")]
    NameTooLong,
    #[error("Name contains invalid characters")]
    NameInvalid,

    #[error("Description cannot be empty")]
    DescriptionEmpty,
    #[error("Description has to be less than 500 characters long")]
    DescriptionTooLong,
    #[error("Description contains invalid characters")]
    DescriptionInvalid,

    #[error("Logo cannot be empty")]
    LogoUrlEmpty,
    #[error("Logo url is invalid")]
    LogoUrlInvalid,
    #[error("Logo url has an invalid image extension")]
    LogoUrlInvalidImageExtension,
    #[error("Logo has an unsupported mime type")]
    LogoUrlInvalidMimeType,

    #[error("Website url is invalid")]
    WebsiteUrlInvalid,
    #[error("Website url is longer than 1024 characters")]
    WebsiteUrlTooLong,
    #[error("Website url points to local server")]
    WebsiteUrlPointsToLocalServer,

    #[error("Twitter url is invalid. It must be of the format https://twitter.com/<username> or https://x.com/<username>")]
    TwitterUrlInvalid,
    #[error("Twitter url is longer than 1024 characters")]
    TwitterUrlTooLong,
    #[error("Twitter url points to local server")]
    TwitterUrlPointsToLocalServer,
}

impl OperatorMetadata {
    pub async fn validate(&self) -> Result<(), OperatorMetadataError> {
        // Alias the error types for brevity
        use OperatorMetadataError::*;

        let text_regex = regex::Regex::new(r#"^[a-zA-Z0-9 +.,;:?!'’"“”\-_/()\[\]~&#$—%]+$"#)
            .expect("regex is valid");

        // name must be non-empty, less than 500 characters, and match the regex
        if self.name.is_empty() {
            return Err(NameEmpty);
        }
        if self.name.len() > 500 {
            return Err(NameTooLong);
        }
        if !text_regex.is_match(&self.name) {
            return Err(NameInvalid);
        }

        // description must be non-empty, less than 500 characters, and match the regex
        if self.description.is_empty() {
            return Err(DescriptionEmpty);
        }
        if self.description.len() > 500 {
            return Err(DescriptionTooLong);
        }
        if !text_regex.is_match(&self.description) {
            return Err(DescriptionInvalid);
        }

        // logo must be non-empty, must be a valid URL, end in .png,
        // and the server must return the content with a "image/png" mime type
        if self.logo.is_empty() {
            return Err(LogoUrlEmpty);
        }
        let Ok(url) = Url::parse(&self.logo) else {
            return Err(LogoUrlInvalid);
        };
        let path = PathBuf::from(url.path());
        if path.extension().map(|ext| ext != "png").unwrap_or(true) {
            return Err(LogoUrlInvalidImageExtension);
        }
        // Check the server returns content with a "image/png" mime type
        let response = reqwest::get(&self.logo).await.unwrap();
        let body = response.bytes().await.unwrap();

        if body.sniff_mime_type() != Some("image/png") {
            return Err(LogoUrlInvalidMimeType);
        }

        // website, if non-empty, must have less than 1024 characters,
        // not point to localhost or 127.0.0.1, and must be a valid URL that matches the regex
        if self.website.as_ref().is_some_and(|s| !s.is_empty()) {
            let website = self.website.as_ref().unwrap();
            if website.len() > 1024 {
                return Err(WebsiteUrlTooLong);
            }
            let url = Url::parse(website).ok().ok_or(WebsiteUrlInvalid)?;

            let host = url.host_str().ok_or(WebsiteUrlInvalid)?;
            if url.scheme().is_empty() || host.is_empty() {
                return Err(WebsiteUrlInvalid);
            }
            if host == "localhost" || host == "127.0.0.1" {
                return Err(WebsiteUrlPointsToLocalServer);
            }

            let website_regex =
                regex::Regex::new(r#"^(https?)://[^\s/$.?#].[^\s]*$"#).expect("regex is valid");

            if !website_regex.is_match(website) {
                return Err(WebsiteUrlInvalid);
            }
        }

        // twitter, if non-empty, must have less than 1024 characters,
        // not point to localhost or 127.0.0.1, and must be a valid URL
        // Also matches: `^(?:https?://)?(?:www\.)?(?:twitter\.com/\w+|x\.com/\w+)(?:/?|$)`
        if self.twitter.as_ref().is_some_and(|s| !s.is_empty()) {
            let twitter = self.twitter.as_ref().unwrap();
            if twitter.len() > 1024 {
                return Err(TwitterUrlTooLong);
            }
            let url = Url::parse(twitter).ok().ok_or(TwitterUrlInvalid)?;

            let host = url.host_str().ok_or(TwitterUrlInvalid)?;
            if url.scheme().is_empty() || host.is_empty() {
                return Err(TwitterUrlInvalid);
            }
            if host == "localhost" || host == "127.0.0.1" {
                return Err(TwitterUrlPointsToLocalServer);
            }

            let twitter_regex = regex::Regex::new(
                r#"^(?:https?://)?(?:www\.)?(?:twitter\.com/\w+|x\.com/\w+)(?:/?|$)"#,
            )
            .expect("regex is valid");

            if !twitter_regex.is_match(twitter) {
                return Err(TwitterUrlInvalid);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{operator::OperatorMetadata, operator_metadata::OperatorMetadataError};

    fn get_default_metadata() -> OperatorMetadata {
        OperatorMetadata {
            name: "Ethereum Utopia".to_string(),
            description: "Rust operator is good operator".to_string(),
            logo: "https://goerli-operator-metadata.s3.amazonaws.com/eigenlayer.png".to_string(),
            website: Some("https://test.com".to_string()),
            twitter: Some("https://twitter.com/test".to_string()),
        }
    }

    #[tokio::test]
    async fn test_valid_metadata() {
        let metadata = get_default_metadata();
        metadata.validate().await.unwrap();
    }

    #[tokio::test]
    async fn test_twitter_url_with_ending_slash() {
        let mut metadata = get_default_metadata();
        metadata.twitter = Some("https://twitter.com/test/".to_string());
        metadata.validate().await.unwrap();
    }

    #[tokio::test]
    async fn test_twitter_x_url() {
        let mut metadata = get_default_metadata();
        metadata.twitter = Some("https://x.com/test".to_string());
        metadata.validate().await.unwrap();
    }

    #[tokio::test]
    async fn test_empty_website_and_twitter() {
        let mut metadata = get_default_metadata();
        metadata.website = None;
        metadata.twitter = None;
        metadata.validate().await.unwrap();
    }

    #[tokio::test]
    async fn test_invalid_no_name() {
        let mut metadata = get_default_metadata();
        metadata.name = "".to_string();
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::NameEmpty);
    }

    #[tokio::test]
    async fn test_invalid_name_too_long() {
        let mut metadata = get_default_metadata();
        metadata.name = "0".repeat(501);
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::NameTooLong);
    }

    #[tokio::test]
    async fn test_invalid_name_has_js_script() {
        let mut metadata = get_default_metadata();
        metadata.name = "<script> alert('test') </script>".to_string();
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::NameInvalid);
    }

    #[tokio::test]
    async fn test_invalid_no_description() {
        let mut metadata = get_default_metadata();
        metadata.description = "".to_string();
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::DescriptionEmpty);
    }

    #[tokio::test]
    async fn test_invalid_description_too_long() {
        let mut metadata = get_default_metadata();
        metadata.description = "0".repeat(501);
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::DescriptionTooLong);
    }

    #[tokio::test]
    async fn test_invalid_description_has_js_script() {
        let mut metadata = get_default_metadata();
        metadata.description = "<script> alert('test') </script>".to_string();
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::DescriptionInvalid);
    }

    #[tokio::test]
    async fn test_invalid_logo_url_empty() {
        let mut metadata = get_default_metadata();
        metadata.logo = "".to_string();
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::LogoUrlEmpty);
    }

    #[tokio::test]
    async fn test_invalid_logo_wrong_image_format() {
        let mut metadata = get_default_metadata();
        metadata.logo = "https://test.com/test.svg".to_string();
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::LogoUrlInvalidImageExtension);
    }

    #[tokio::test]
    async fn test_invalid_logo_invalid_mime_type() {
        let mut metadata = get_default_metadata();
        metadata.logo = "https://goerli-operator-metadata.s3.amazonaws.com/cat.png".to_string();
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::LogoUrlInvalidMimeType);
    }

    #[tokio::test]
    async fn test_invalid_website_url_1() {
        let mut metadata = get_default_metadata();
        metadata.website = Some("https".to_string());
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::WebsiteUrlInvalid);
    }

    #[tokio::test]
    async fn test_invalid_website_url_2() {
        let mut metadata = get_default_metadata();
        metadata.website = Some("https:/test.com".to_string());
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::WebsiteUrlInvalid);
    }

    #[tokio::test]
    async fn test_invalid_website_url_3() {
        let mut metadata = get_default_metadata();
        metadata.website = Some("ps://test.com".to_string());
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::WebsiteUrlInvalid);
    }

    #[tokio::test]
    async fn test_invalid_twitter_url_1() {
        let mut metadata = get_default_metadata();
        metadata.twitter = Some("http".to_string());
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::TwitterUrlInvalid);
    }

    #[tokio::test]
    async fn test_invalid_twitter_url_2() {
        let mut metadata = get_default_metadata();
        metadata.twitter = Some("ht://twitter.com/test".to_string());
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::TwitterUrlInvalid);
    }

    #[tokio::test]
    async fn test_invalid_twitter_url_3() {
        let mut metadata = get_default_metadata();
        metadata.twitter = Some("https:/twitt".to_string());
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::TwitterUrlInvalid);
    }

    #[tokio::test]
    async fn test_invalid_twitter_url_4() {
        let mut metadata = get_default_metadata();
        metadata.twitter = Some("https://facebook.com/test".to_string());
        let err = metadata.validate().await.unwrap_err();
        assert_eq!(err, OperatorMetadataError::TwitterUrlInvalid);
    }
}
