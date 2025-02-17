use serde::{Deserialize, Serialize};
use thiserror::Error;

/// OperatorMetadata is the metadata operator uploads while registering
/// itself to EigenLayer.
#[derive(Deserialize, Serialize)]
pub struct OperatorMetadata {
    /// Name of the operator
    name: String,

    /// Website of the operator
    website: String,

    /// Description of the operator. There is a 200-character limit
    description: String,

    /// Logo of the operator. This should be a link to a image file
    /// which is publicly accessible
    logo: String,

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
}

impl OperatorMetadata {
    pub fn validate(&self) -> Result<(), OperatorMetadataError> {
        // name is between 1-500 characters and matches the regex:
        // `^[a-zA-Z0-9 +.,;:?!'’"“”\-_/()\[\]~&#$—%]+$`
        let text_regex = regex::Regex::new(r#"^[a-zA-Z0-9 +.,;:?!'’"“”\-_/()\[\]~&#$—%]+$"#)
            .expect("regex is valid");
        if self.name.is_empty() {
            return Err(OperatorMetadataError::NameEmpty);
        }
        if self.name.len() > 500 {
            return Err(OperatorMetadataError::NameTooLong);
        }
        if text_regex.is_match(&self.name) {
            return Err(OperatorMetadataError::NameInvalid);
        }
        // same for description
        if self.description.is_empty() {
            return Err(OperatorMetadataError::DescriptionEmpty);
        }
        if self.description.len() > 500 {
            return Err(OperatorMetadataError::DescriptionTooLong);
        }
        if text_regex.is_match(&self.description) {
            return Err(OperatorMetadataError::DescriptionInvalid);
        }

        // logo must be non-empty, must be a valid URL, end in .png,
        // and the server must return the content with a "image/png" mime type

        // website, if non-empty, must have less than 1024 characters,
        // not point to localhost or 127.0.0.1, and must be a valid URL
        // Also matches: `^(https?)://[^\s/$.?#].[^\s]*$`

        // twitter, if non-empty, must have less than 1024 characters,
        // not point to localhost or 127.0.0.1, and must be a valid URL
        // Also matches: `^(?:https?://)?(?:www\.)?(?:twitter\.com/\w+|x\.com/\w+)(?:/?|$)`

        Ok(())
    }
}
