use alloy::primitives::{aliases::U192, Address, FixedBytes, U256};
use eigen_crypto_bls::{convert_to_g1_point, error::BlsError, BlsG1Point, BlsKeyPair};
use eigen_utils::common::get_url_content;
use ethers::utils::keccak256;
use num_bigint::BigUint;
use thiserror::Error;
use url::Url;

pub use crate::operator_metadata::OperatorMetadata;
use crate::operator_metadata::OperatorMetadataError;
pub use crate::operator_pubkeys::OperatorPubKeys;

#[derive(Debug, Error)]
pub enum OperatorTypesError {
    #[error("Operator id from pub key conversion failed")]
    OperatorIdFromPubKey(#[from] BlsError),
    #[error("Invalid Metadata URL")]
    InvalidMetadataUrl,
    #[error("Metadata Not Found")]
    MetadataNotFound,
    #[error("Metadata Parse Error")]
    MetadataParseError,
    #[error("Metadata Validation Error")]
    MetadataValidationError(#[from] OperatorMetadataError),
    #[error("Invalid Address")]
    InvalidAddress,
    #[error("Invalid Delegation Approver Address")]
    InvalidDelegationApproverAddress,
}

const MAX_NUMBER_OF_QUORUMS: u8 = 192;

pub type OperatorId = FixedBytes<32>;

pub fn bitmap_to_quorum_ids(quorum_bitmaps: U256) -> Vec<u8> {
    let bytes = quorum_bitmaps.to_be_bytes::<32>();

    let mut quorum_ids: Vec<u8> = Vec::with_capacity(usize::from(MAX_NUMBER_OF_QUORUMS));

    for i in 0..MAX_NUMBER_OF_QUORUMS {
        let bitmap = BigUint::from_bytes_be(&bytes);
        if bitmap.bit(u64::from(i)) {
            quorum_ids.push(i);
        }
    }
    quorum_ids
}

pub fn bitmap_to_quorum_ids_from_u192(quorum_bitmaps: U192) -> Vec<u8> {
    let bytes = quorum_bitmaps.to_be_bytes::<24>();

    let mut quorum_ids: Vec<u8> = Vec::with_capacity(usize::from(MAX_NUMBER_OF_QUORUMS));

    for i in 0..MAX_NUMBER_OF_QUORUMS {
        let bitmap = BigUint::from_bytes_be(&bytes);
        if bitmap.bit(u64::from(i)) {
            quorum_ids.push(i);
        }
    }
    quorum_ids
}

/// Operator represents EigenLayer's view of an operator
pub struct Operator {
    /// Operator Address
    pub address: Address,

    /// https://github.com/Layr-Labs/eigenlayer-contracts/blob/delegation-redesign/src/contracts/interfaces/IDelegationManager.sol#L18
    pub delegation_approver_address: Address,

    /// MetadataUrl URL where operator metadata is stored
    pub metadata_url: String,

    /// `allocation_delay` is the delay in seconds where an operator is allowed to change allocation
    /// This can only be set once by the operator. Once set this can't be changed
    pub allocation_delay: u32,
}

impl Operator {
    pub fn new(
        address: &str,
        delegation_approver_address: &str,
        metadata_url: String,
        allocation_delay: u32,
    ) -> Result<Self, OperatorTypesError> {
        let address = Address::try_from(address.as_bytes())
            .map_err(|_| OperatorTypesError::InvalidAddress)?;
        let delegation_approver_address = Address::try_from(delegation_approver_address.as_bytes())
            .map_err(|_| OperatorTypesError::InvalidDelegationApproverAddress)?;

        Ok(Self {
            address,
            delegation_approver_address,
            metadata_url,
            allocation_delay,
        })
    }

    pub async fn validate(&self) -> Result<(), OperatorTypesError> {
        // Check for valid URL in metadata_url
        Url::parse(&self.metadata_url).map_err(|_| OperatorTypesError::InvalidMetadataUrl)?;

        // Check if metadata is valid
        let body = get_url_content(&self.metadata_url)
            .await
            .map_err(|_| OperatorTypesError::MetadataNotFound)?;

        let operator_metadata: OperatorMetadata =
            serde_json::from_str(&body).map_err(|_| OperatorTypesError::MetadataParseError)?;

        operator_metadata
            .validate()
            .await
            .map_err(OperatorTypesError::MetadataValidationError)
    }

    pub fn operator_id_from_key_pair(
        keypair: BlsKeyPair,
    ) -> Result<OperatorId, OperatorTypesError> {
        operator_id_from_g1_pub_key(keypair.public_key())
    }
}

pub type Socket = String;

pub type QuorumNum = u8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorInfo {
    pub pub_keys: Option<OperatorPubKeys>,
}

pub fn operator_id_from_g1_pub_key(pub_key: BlsG1Point) -> Result<OperatorId, OperatorTypesError> {
    let x: [u8; 32] = convert_to_g1_point(pub_key.g1())?.X.to_be_bytes();
    let y: [u8; 32] = convert_to_g1_point(pub_key.g1())?.Y.to_be_bytes();
    let mut bytes = [0_u8; 64];
    bytes[..32].copy_from_slice(&x);
    bytes[32..].copy_from_slice(&y);
    Ok(keccak256(bytes).into())
}

pub type QuorumThresholdPercentage = u8;

pub type QuorumThresholdPercentages = Vec<QuorumThresholdPercentage>;
