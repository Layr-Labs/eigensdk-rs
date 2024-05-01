use eigensdk_crypto_bls::attestation::Signature;

pub type TaskIndex = u32;

pub type TaskResponseDigest = [u8; 32];

#[derive(Debug, Clone)]
pub struct SignedTaskResponseDigest {
    pub task_response_digest: TaskResponseDigest,

    pub bls_signature: Signature,

    pub operator_id: [u8; 32],
}
