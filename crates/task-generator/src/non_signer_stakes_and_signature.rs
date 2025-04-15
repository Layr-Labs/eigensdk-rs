/// This is the documentation for the `NonSignerStakesAndSignature` struct.
#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
#[derive(Debug, Clone)]
pub struct NonSignerStakesAndSignature<G1Point, G2Point>
where
    G1Point: alloy::sol_types::SolType + Clone,
    G2Point: alloy::sol_types::SolType + Clone,
{
    #[allow(missing_docs)]
    pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
    #[allow(missing_docs)]
    pub nonSignerPubkeys: alloy::sol_types::private::Vec<G1Point>,
    #[allow(missing_docs)]
    pub quorumApks: alloy::sol_types::private::Vec<G1Point>,
    #[allow(missing_docs)]
    pub apkG2: G2Point, // as alloy::sol_types::SolType,
    #[allow(missing_docs)]
    pub sigma: G1Point, // as alloy::sol_types::SolType,
    #[allow(missing_docs)]
    pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
    #[allow(missing_docs)]
    pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
    #[allow(missing_docs)]
    pub nonSignerStakeIndices: alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
}
