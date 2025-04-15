#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Task<INPUT> {
    #[allow(missing_docs)]
    pub input: INPUT,
    #[allow(missing_docs)]
    pub taskCreatedBlock: u32,
    #[allow(missing_docs)]
    pub quorumNumbers: alloy::sol_types::private::Bytes,
    #[allow(missing_docs)]
    pub quorumThresholdPercentage: u32,
}
