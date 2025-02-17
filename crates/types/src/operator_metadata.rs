use std::path::{Path, PathBuf};

use alloy::transports::http::reqwest::{self, Url};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Error)]
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
    pub fn validate(&self) -> Result<(), OperatorMetadataError> {
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
        if path.extension().map(|ext| ext == "png").unwrap_or(false) {
            return Err(LogoUrlInvalidImageExtension);
        }
        // TODO: check if the server returns the content with a "image/png" mime type

        // website, if non-empty, must have less than 1024 characters,
        // not point to localhost or 127.0.0.1, and must be a valid URL that matches the regex
        if !self.website.as_ref().is_none_or(|s| s.is_empty()) {
            let website = self.website.as_ref().unwrap();
            if website.len() > 1024 {
                return Err(WebsiteUrlTooLong);
            }
            let url = Url::parse(&website).ok().ok_or(WebsiteUrlInvalid)?;

            let host = url.host_str().ok_or(WebsiteUrlInvalid)?;
            if url.scheme().is_empty() || host.is_empty() {
                return Err(WebsiteUrlInvalid);
            }
            if host == "localhost" || host == "127.0.0.1" {
                return Err(WebsiteUrlPointsToLocalServer);
            }

            let website_regex =
                regex::Regex::new(r#"^(https?)://[^\s/$.?#].[^\s]*$"#).expect("regex is valid");

            if !website_regex.is_match(&website) {
                return Err(WebsiteUrlInvalid);
            }
        }

        // twitter, if non-empty, must have less than 1024 characters,
        // not point to localhost or 127.0.0.1, and must be a valid URL
        // Also matches: `^(?:https?://)?(?:www\.)?(?:twitter\.com/\w+|x\.com/\w+)(?:/?|$)`
        if !self.twitter.as_ref().is_none_or(|s| s.is_empty()) {
            let twitter = self.twitter.as_ref().unwrap();
            if twitter.len() > 1024 {
                return Err(TwitterUrlTooLong);
            }
            let url = Url::parse(&twitter).ok().ok_or(TwitterUrlInvalid)?;

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

            if !twitter_regex.is_match(&twitter) {
                return Err(TwitterUrlInvalid);
            }
        }

        Ok(())
    }
}
