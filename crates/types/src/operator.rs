use alloy::primitives::{aliases::U192, Address, FixedBytes, U256};
use ark_ff::PrimeField;
use eigen_crypto_bls::{convert_to_g1_point, error::BlsError, BlsG1Point, BlsG2Point, BlsKeyPair};
use eigen_utils::{
    common::get_url_content,
    slashing::middleware::blsapkregistry::BN254::{G1Point, G2Point},
};
use ethers::{types::U64, utils::keccak256};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OperatorTypesError {
    #[error("Operator id from pub key conversion failed")]
    OperatorIdFromPubKey(#[from] BlsError),
    #[error("Metadata Not Found")]
    MetadataNotFound,
    #[error("Metadata Error")]
    MetadataError,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorPubKeys {
    pub g1_pub_key: BlsG1Point,
    pub g2_pub_key: BlsG2Point,
}

impl From<BlsKeyPair> for OperatorPubKeys {
    fn from(keypair: BlsKeyPair) -> Self {
        Self {
            g1_pub_key: keypair.public_key(),
            g2_pub_key: keypair.public_key_g2(),
        }
    }
}

impl OperatorPubKeys {
    pub fn to_contract_public_keys(&self) -> (G1Point, G2Point) {
        let x = self.g1_pub_key.g1().x;
        let y = self.g1_pub_key.g1().y;
        let g1 = G1Point {
            X: U256::from_limbs(x.into_bigint().0),
            Y: U256::from_limbs(y.into_bigint().0),
        };

        let x2 = self.g2_pub_key.g2().x;
        let y2 = self.g2_pub_key.g2().y;
        let g2 = G2Point {
            X: [
                U256::from_limbs(x2.c0.into_bigint().0),
                U256::from_limbs(x2.c1.into_bigint().0),
            ],
            Y: [
                U256::from_limbs(y2.c0.into_bigint().0),
                U256::from_limbs(y2.c1.into_bigint().0),
            ],
        };
        (g1, g2)
    }
}

#[derive(Deserialize, Serialize)]
pub struct OperatorMetadata {
    #[serde(rename = "quorumBitmap")]
    quorum_bitmap: U256,
    //    address operator;
    //    bytes32 operatorId;
    //    BN254.G1Point pubkey;
    //    uint96[] stakes; // in every quorum for simplicity
}

pub struct Operator {
    pub address: Address,
    pub staker_opt_out_window_blocks: u32,
    pub delegation_approver_address: Address,
    pub metadata_url: Option<String>,

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

        if let Some(metadata_url) = &self.metadata_url {
            // Check for valid URL
            /*
            err := utils.CheckIfUrlIsValid(o.MetadataUrl)
            if err != nil {
                return utils.WrapError(ErrInvalidMetadataUrl, err)
            }
            */

            let body = get_url_content(metadata_url)
                .await
                .map_err(|_| OperatorTypesError::MetadataNotFound)?;
            let operator_metadata: OperatorMetadata =
                serde_json::from_str(&body).map_err(|_| OperatorTypesError::MetadataError)?;

            //operator_metadata.validate()?;
        }

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorAvsState {
    pub operator_id: [u8; 32],
    pub operator_info: OperatorInfo,
    pub stake_per_quorum: HashMap<QuorumNum, U256>,
    pub block_num: U64,
}

pub fn operator_id_from_g1_pub_key(pub_key: BlsG1Point) -> Result<OperatorId, OperatorTypesError> {
    let x: [u8; 32] = convert_to_g1_point(pub_key.g1())?.X.to_be_bytes();
    let y: [u8; 32] = convert_to_g1_point(pub_key.g1())?.Y.to_be_bytes();
    let mut bytes = [0_u8; 64];
    bytes[..32].copy_from_slice(&x);
    bytes[32..].copy_from_slice(&y);
    Ok(FixedBytes::from(keccak256(bytes)))
}

#[derive(Debug, PartialEq, Eq)]
pub struct QuorumAvsState {
    pub quorum_num: u8,
    pub total_stake: U256,
    pub agg_pub_key_g1: BlsG1Point,
    pub block_num: u32,
}

pub type QuorumThresholdPercentage = u8;

pub type QuorumThresholdPercentages = Vec<QuorumThresholdPercentage>;
