use eigensdk_crypto_bls::attestation::Signature;
use ethers::types::Sign;

pub type TaskIndex = u32;

pub type TaskResponseDigest = [u8; 32];

#[derive(Debug, Clone)]
pub struct SignedTaskResponseDigest {
    task_response_digest: TaskResponseDigest,

    bls_signature: Signature,

    operator_id: [u8; 32],
}
