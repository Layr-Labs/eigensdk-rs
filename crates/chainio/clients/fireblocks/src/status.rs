use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    #[serde(rename = "APPROVED")]
    Approved,
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

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Approved => "Approved",
            Status::Submitted => "SUBMITTED",
            Status::PendingScreening => "PENDING_AML_SCREENING",
            Status::PendingAuthorization => "PENDING_AUTHORIZATION",
            Status::Queued => "QUEUED",
            Status::PendingSignature => "PENDING_SIGNATURE",
            Status::PendingEmailApproval => "PENDING_3RD_PARTY_MANUAL_APPROVAL",
            Status::Pending3rdParity => "PENDING_3RD_PARTY",
            Status::Broadcasting => "BROADCASTING",
            Status::Confirming => "CONFIRMING",
            Status::Completed => "COMPLETED",
            Status::Cancelling => "CANCELLING",
            Status::Cancelled => "CANCELLED",
            Status::Blocked => "BLOCKED",
            Status::Rejected => "REJECTED",
            Status::Failed => "FAILED",
        }
    }
}
