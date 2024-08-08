//! register operator in quorum with avs registry coordinator
use alloy_primitives::U256;
use alloy_primitives::{Bytes, FixedBytes};
use alloy_signer_local::PrivateKeySigner;
use ark_bn254::{Fq, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AffineRepr, CurveGroup, Group};
use ark_ff::BigInteger256;
use ark_ff::{BigInt, BigInteger, PrimeField, UniformRand};
use ark_serialize::CanonicalSerialize;
use ark_std::str::FromStr;
use ark_std::{ops::Mul, test_rng};
use eigen_client_avsregistry::writer::AvsRegistryChainWriter;
use eigen_client_elcontracts::{writer::ELChainWriter,reader::ELChainReader};
use eigen_crypto_bls::{cids_multiplication, PrivateKey};
use eigen_crypto_bls::BlsKeyPair;
use eigen_types::operator::Operator;

// use eigen_crypto_bls::BlsKeypair;
use alloy_provider::Provider;
use eigen_testing_utils::m2_holesky_constants::{
    AVS_DIRECTORY_ADDRESS, BLS_APK_REGISTRY, DELEGATION_MANAGER_ADDRESS, OPERATOR_STATE_RETRIEVER,
    REGISTRY_COORDINATOR, SERVICE_MANAGER_ADDRESS, SLASHER_ADDRESS, STAKE_REGISTRY, STRATEGY_MANAGER_ADDRESS,
};
use eigen_utils::binding::DelegationManager;
use eigen_utils::get_provider;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let holesky_provider = "https://ethereum-holesky.blockpi.network/v1/rpc/public";
    let pvt_key = "bead471191bea97fc3aeac36c9d74c895e8a6242602e144e43152f96219e96e8";
    let avs_registry_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
        holesky_provider.to_string(),
        pvt_key.to_string(),
        REGISTRY_COORDINATOR,
        OPERATOR_STATE_RETRIEVER,
    )
    .await
    .expect("avs writer build fail ");
    let mut rng = ark_std::test_rng();
    let sk = Fr::rand(&mut rng);

    println!("private key {:?}", sk);
    let bls_priv_key_str =
        ("12248929636257230549931416853095037629726205319386239410403476017439825112537")
            .as_bytes();

    println!("bls pvt key{:?}", bls_priv_key_str);
    let a = sk_to_pk_g1(bls_priv_key_str);
    println!("bls pub key {:?}", a);

    let _sk = Fr::from_str(
        "12248929636257230549931416853095037629726205319386239410403476017439825112537",
    )
    .unwrap();
    println!("{:?}", _sk);
    let pk = G1Projective::from(G1Affine::generator()) * _sk;
    println!("{:?}", pk.into_affine());
    let mut compressed_bytes = Vec::new();
    pk.serialize_uncompressed(&mut compressed_bytes).unwrap();
    println!("uncompressed public key {:?}", compressed_bytes);
    let bls_key_pair = BlsKeyPair::new(
        "12248929636257230549931416853095037629726205319386239410403476017439825112537".to_string(),
    );
    let public_key = bls_key_pair.public_key().g1();
    println!("public key {:?}", public_key);

    let digest_hash: FixedBytes<32> = FixedBytes::from([
        0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
        0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
        0x02, 0x02,
    ]);
    let provider = get_provider(&holesky_provider.to_string());
    let sig_expiry: U256 =U256::from(1723123419);
    println!("sig expiry {:?}",sig_expiry);
    let quorum_nums = Bytes::from([0x01]);
    println!("quorum nums : {:?}", quorum_nums);

    let delegation_contract = DelegationManager::new(DELEGATION_MANAGER_ADDRESS,provider);
    let el_chain_reader = ELChainReader::new(
        SLASHER_ADDRESS,
        DELEGATION_MANAGER_ADDRESS,
        AVS_DIRECTORY_ADDRESS,
        "https://ethereum-holesky.blockpi.network/v1/rpc/public".to_string(),
    );
    // let el_writer = ELChainWriter::new(DELEGATION_MANAGER_ADDRESS,STRATEGY_MANAGER_ADDRESS,el_chain_reader,"https://ethereum-holesky.blockpi.network/v1/rpc/public".to_string(),"bead471191bea97fc3aeac36c9d74c895e8a6242602e144e43152f96219e96e8".to_string());
    // let wallet = PrivateKeySigner::from_str("bead471191bea97fc3aeac36c9d74c895e8a6242602e144e43152f96219e96e8").expect("no key ");
    // let operator_details  = Operator::new(wallet.address(),wallet.address(),wallet.address(),3,Some("eigensdk-rs".to_string()));
    // let s = el_writer.register_as_operator(operator_details).await;
    
    let tx_hash = avs_registry_writer
        .register_operator_in_quorum_with_avs_registry_coordinator(
            bls_key_pair,
            digest_hash,
            sig_expiry,
            quorum_nums,
            "65.109.158.181:33078;31078".to_string(),
        )
        .await
        .unwrap();
    println!("tx hash :{:?}", tx_hash);
    Ok(())
}

/// operator private key  bead471191bea97fc3aeac36c9d74c895e8a6242602e144e43152f96219e96e8
/// operator address 0x4c234bf6518786b81e1175579432a8aeff1d85e8
/// operator salt [118 15 193 89 169 241 163 168 115 153 188 76 159 191 30 1 234 156 242 212 86 80 46 245 170 155 235 28 91 213 201 93]
/// expiry 2114246
///

pub fn sk_to_pk_g1(sk: &[u8]) -> Vec<u8> {
    let _sk = Fr::from_be_bytes_mod_order(sk);
    let mut compressed_bytes = Vec::new();
    let pk = G1Projective::from(G1Affine::generator()) * _sk;
    pk.serialize_uncompressed(&mut compressed_bytes).unwrap();
    compressed_bytes
}

pub fn deserialize_montgomery_elements(data: &[Fr], buffer: &mut Vec<u8>) {
    let mut temp_buffer: Vec<u8> = data
        .iter()
        .rev()
        .flat_map(|elem| elem.into_bigint().to_bytes_le())
        .collect();

    temp_buffer.reverse();
    buffer.extend(temp_buffer);
}
// #[tokio::main]
// async fn main() {}

// go sdk
// [0 157 80 130 136 151 254 32 130 117 217 137 171 221 202 215 98 191 27 177 160 137 213 173 64 202 93 199 142 32 250 172 37 108 121 246 129 127 215 159 58 72 152 228 27 82 18 204 174 102 213 233 68 28 156 118 242 57 162 150 111 36 186 94]

// eigen-rs
// [175, 255, 132, 86, 4, 113, 62, 12, 203, 17, 78, 245, 196, 106, 40, 241, 248, 145, 73, 113, 143, 179, 216, 16, 90, 214, 138, 39, 114, 66, 129, 27, 255, 35, 38, 149, 177, 184, 23, 21, 164, 213, 207, 165, 96, 129, 59, 179, 65, 56, 240, 99, 164, 181, 22, 203, 31, 135, 46, 55, 186, 5, 17, 174]
