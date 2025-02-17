use alloy::primitives::{aliases::U192, Address, FixedBytes, U256};
use eigen_crypto_bls::{convert_to_g1_point, error::BlsError, BlsG1Point, BlsKeyPair};
use eigen_utils::common::get_url_content;
use ethers::utils::keccak256;
use num_bigint::BigUint;
use thiserror::Error;

pub use crate::operator_metadata::OperatorMetadata;
use crate::operator_metadata::OperatorMetadataError;
pub use crate::operator_pubkeys::OperatorPubKeys;

#[derive(Debug, Error)]
pub enum OperatorTypesError {
    #[error("Operator id from pub key conversion failed")]
    OperatorIdFromPubKey(#[from] BlsError),
    #[error("Metadata Not Found")]
    MetadataNotFound,
    #[error("Metadata Parse Error")]
    MetadataParseError,
    #[error("Metadata Validation Error")]
    MetadataValidationError(#[from] OperatorMetadataError),
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

pub struct Operator {
    pub address: Address,
    pub staker_opt_out_window_blocks: u32,
    pub delegation_approver_address: Address,
    pub metadata_url: String,

    /// `allocation_delay` is the delay in seconds where an operator is allowed to change allocation
    /// This can only be set once by the operator. Once set this can't be changed
    pub allocation_delay: u32,
}

impl Operator {
    pub async fn validate(&self) -> Result<(), OperatorTypesError> {
        //if !utils.IsValidEthereumAddress(o.Address) {
        //    return ErrInvalidOperatorAddress
        //}

        if !self.address.is_zero() {
            return Err(OperatorTypesError::OperatorIdFromPubKey(
                BlsError::InvalidPublicKey,
            ));
        }

        //if o.DelegationApproverAddress != ZeroAddress && !utils.IsValidEthereumAddress(o.DelegationApproverAddress) {
        //    return ErrInvalidDelegationApproverAddress
        //}
        if !self.delegation_approver_address.is_zero() {
            return Err(OperatorTypesError::OperatorIdFromPubKey(
                BlsError::InvalidPublicKey,
            ));
        }

        // Validate metadata
        /*
        body, err := utils.ReadPublicURL(o.MetadataUrl)
        if err != nil {
            return utils.WrapError(ErrReadingMetadataUrlResponse, err)
        }

        operatorMetadata := OperatorMetadata{}
        err = json.Unmarshal(body, &operatorMetadata)
        if err != nil {
            return ErrUnmarshalOperatorMetadata
        }

        return operatorMetadata.Validate()
        */

        // Check for valid URL
        /*
        err := utils.CheckIfUrlIsValid(o.MetadataUrl)
        if err != nil {
            return utils.WrapError(ErrInvalidMetadataUrl, err)
        }
        */

        let body = get_url_content(&self.metadata_url)
            .await
            .map_err(|_| OperatorTypesError::MetadataNotFound)?;
        let operator_metadata: OperatorMetadata =
            serde_json::from_str(&body).map_err(|_| OperatorTypesError::MetadataParseError)?;

        operator_metadata.validate()?;

        Ok(())
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
    Ok(FixedBytes::from(keccak256(bytes)))
}

pub type QuorumThresholdPercentage = u8;

pub type QuorumThresholdPercentages = Vec<QuorumThresholdPercentage>;
