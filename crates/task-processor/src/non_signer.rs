use eigen_utils::slashing::middleware::blsapkregistry::BN254::{G1Point, G2Point};

#[derive(Clone)]
pub struct NonSignerStakesAndSignature {
    #[allow(missing_docs)]
    pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
    #[allow(missing_docs)]
    pub nonSignerPubkeys: Vec<G1Point>,
    #[allow(missing_docs)]
    pub quorumApks: Vec<G1Point>,
    #[allow(missing_docs)]
    pub apkG2: G2Point,
    #[allow(missing_docs)]
    pub sigma: G1Point,
    #[allow(missing_docs)]
    pub quorumApkIndices: Vec<u32>,
    #[allow(missing_docs)]
    pub totalStakeIndices: Vec<u32>,
    #[allow(missing_docs)]
    pub nonSignerStakeIndices: Vec<Vec<u32>>,
}
