use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    #[serde(rename = "SUBMITTED")]
    Submitted,
    #[serde(rename = "PENDING_AML_SCREENING")]
    PendingScreening,
    #[serde(rename = "PENDING_AUTHORIZATION")]
    PendingAuthorization,
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "PENDING_SIGNATURE")]
    PendingSignature,
    #[serde(rename = "PENDING_3RD_PARTY_MANUAL_APPROVAL")]
    PendingEmailApproval,
    #[serde(rename = "PENDING_3RD_PARTY")]
    Pending3rdParity,
    #[serde(rename = "BROADCASTING")]
    Broadcasting,
    #[serde(rename = "CONFIRMING")]
    Confirming,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "CANCELLING")]
    Cancelling,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "BLOCKED")]
    Blocked,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "FAILED")]
    Failed,
}
