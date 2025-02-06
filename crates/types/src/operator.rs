use alloy::primitives::{aliases::U192, Address, FixedBytes, U256};
use eigen_crypto_bls::{convert_to_g1_point, error::BlsError, BlsG1Point, BlsG2Point, BlsKeyPair};
use eigen_utils::common::get_url_content;
use ethers::{types::U64, utils::keccak256};
use num_bigint::BigUint;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OperatorTypesError {
    #[error("Operator id from pub key conversion failed")]
    OperatorIdFromPubKey(#[from] BlsError),
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
    pub fn validate(&self) -> Result<(), OperatorTypesError> {
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

            let body = get_url_content(metadata_url)?;
            let operator_metadata: OperatorMetadata = serde_json::from_str(&body)?;
            operator_metadata.validate()?;
        }

        Ok(())
    }

    //OperatorPubkeys.ToContractPubkeys
    pub fn to_contract_public_keys(&self) -> (BlsG1Point, BlsG2Point) {
        // from Go code:
        /*
        func (op OperatorPubkeys) ToContractPubkeys() (apkreg.BN254G1Point, apkreg.BN254G2Point) {
            return apkreg.BN254G1Point{
                    X: op.G1Pubkey.X.BigInt(new(big.Int)),
                    Y: op.G1Pubkey.Y.BigInt(new(big.Int)),
                }, apkreg.BN254G2Point{
                    X: [2]*big.Int{op.G2Pubkey.X.A0.BigInt(new(big.Int)), op.G2Pubkey.X.A1.BigInt(new(big.Int))},
                    Y: [2]*big.Int{op.G2Pubkey.Y.A0.BigInt(new(big.Int)), op.G2Pubkey.Y.A1.BigInt(new(big.Int))},
                }
        }
        */
        todo!()
    }

    pub fn operator_id_from_contract_g1_pubkey(pubkey: BlsG1Point) -> OperatorId {
        //func OperatorIdFromContractG1Pubkey(pubkey apkreg.BN254G1Point) OperatorId {
        //    return OperatorIdFromG1Pubkey(G1PubkeyFromContractG1Pubkey(pubkey))
        //}
        //OperatorId::from(G1PubkeyFromContractG1Pubkey(pubkey))
        todo!()
    }

    pub fn operator_id_from_key_pair(keypair: BlsKeyPair) -> OperatorId {
        //func OperatorIdFromKeyPair(keyPair *bls.KeyPair) OperatorId {
        //    return OperatorIdFromG1Pubkey(keyPair.GetPubKeyG1())
        //}
        //OperatorId::from(G1PubkeyFromContractG1Pubkey(keypair.get_pub_key_g1()))
        todo!()
    }

    pub fn g1_pubkey_from_contract_g1_pubkey(pubkey: BlsG1Point) -> BlsG1Point {
        //func G1PubkeyFromContractG1Pubkey(pubkey apkreg.BN254G1Point) *bls.G1Point {
        //    return bls.NewG1Point(pubkey.X, pubkey.Y)
        //}
        //bls.NewG1Point(pubkey.X, pubkey.Y)
        todo!()
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

pub fn operator_id_from_g1_pub_key(pub_key: BlsG1Point) -> Result<[u8; 32], OperatorTypesError> {
    let x: [u8; 32] = (convert_to_g1_point(pub_key.g1())?.X).to_be_bytes();
    let y: [u8; 32] = convert_to_g1_point(pub_key.g1())?.Y.to_be_bytes();
    let bytes: Vec<u8> = x.iter().cloned().chain(y.iter().cloned()).collect();
    Ok(keccak256(bytes))
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
