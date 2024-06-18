// //! register operator in quorum with avs registry coordinator
// use alloy_primitives::{FixedBytes,Bytes};
// use eigen_client_avsregistry::writer::AvsRegistryChainWriter;
// use eigen_client_elcontracts::reader::ELChainReader;
// use eigen_testing_utils::m2_holesky_constants::{
//     AVS_DIRECTORY_ADDRESS, BLS_APK_REGISTRY, DELEGATION_MANAGER_ADDRESS, OPERATOR_STATE_RETRIEVER,
//     REGISTRY_COORDINATOR, SERVICE_MANAGER_ADDRESS, SLASHER_ADDRESS, STAKE_REGISTRY,
// };
// use std::str::FromStr;
// use eigen_chainio_utils::convert_to_bn254_g1_point;
// use ark_bn254::{Fr,G1Affine,Fq};
// use alloy_provider::Provider;
// use alloy_primitives::{U256};
// use eigen_utils::get_provider;
// use eigen_crypto_bls::attestation::{KeyPair};
// use eyre::Result;
// use ark_ff::{fields::PrimeField};
// use ark_ff::{BigInt, BigInteger};
// #[tokio::main]
// async fn main() -> Result<()> {
//     let holesky_provider = "https://holesky.drpc.org";
//     let pvt_key = "160443ef7d1ada994b300f7d2bf88db16217db6f825708e4b23f69aa028d7c8c";
//     let el_chain_reader: ELChainReader = ELChainReader::new(
//         SLASHER_ADDRESS,
//         DELEGATION_MANAGER_ADDRESS,
//         AVS_DIRECTORY_ADDRESS,
//         holesky_provider.to_string(),
//     );
//     let avs_registry_writer = AvsRegistryChainWriter::new(
//         SERVICE_MANAGER_ADDRESS,
//         REGISTRY_COORDINATOR,
//         OPERATOR_STATE_RETRIEVER,
//         STAKE_REGISTRY,
//         BLS_APK_REGISTRY,
//         el_chain_reader,
//         holesky_provider.to_string(),
//         pvt_key.to_string(),
//     )
//     .await;

//     let key_pair :KeyPair = KeyPair::from_string(pvt_key.to_string())?;
//     let digest_hash :FixedBytes<32>= FixedBytes::from([0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02,0x02]) ;
//     let provider = get_provider(&holesky_provider.to_string());
//     let current_block_number = provider.get_block_number().await?;
//     let sig_expiry : U256 = U256::from(current_block_number + 20);
//     let quorum_nums = Bytes::from([0x01]);
//     println!("quorum nums : {:?}",quorum_nums);
//     let tx_hash = avs_registry_writer.register_operator_in_quorum_with_avs_registry_coordinator(key_pair,digest_hash,U256::from(1718697416),quorum_nums,"65.109.158.181:33078;31078".to_string()).await.unwrap();
//     println!("tx hash :{:?}",tx_hash);
//     Ok(())
// }

// pub fn deserialize_montgomery_elements(data: &[Fr], buffer: &mut Vec<u8>) {
//     let mut temp_buffer: Vec<u8> = data
//         .iter()
//         .rev()
//         .flat_map(|elem| elem.into_bigint().to_bytes_le())
//         .collect();

//     temp_buffer.reverse();
//     buffer.extend(temp_buffer);
// }

#[tokio::main]
async fn main() {}
