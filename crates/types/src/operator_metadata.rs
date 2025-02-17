use serde::{Deserialize, Serialize};

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
