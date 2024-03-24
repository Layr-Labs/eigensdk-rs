use thiserror::Error;

/// Error returned by AvsRegistry
#[derive(Debug, Error)]
pub enum AvsRegistryError {
    /// Could not get bls apk registry addresss
    #[error("Failed to get bls apk registry address")]
    GetBlsApkRegistry,

    /// Failed to get quorum count
    #[error("Faield to get quorum count")]
    GetQuorumCount,

    /// Failed to ger operator state
    #[error("Failed to get operator state")]
    GetOperatorState,

    /// Failed to get operator state with registry coordinator and operator id
    #[error("Failed to get oeprator state with registry coordinator and operator id ")]
    GetOperatorStateWithRegistryCoordinatorAndOperatorId,

    /// Failed to get current block number
    #[error("Failed to get current block number")]
    GetBlockNumber,

    /// block number overflow
    #[error("block number overflowed")]
    BlockNumberOverflow,

    /// Operator stake in quourm at block
    #[error("failed to get operator stake in quorum at block number")]
    GetOperatorStakeInQuorumAtBlockNumber,

    /// Operator stake in quorums at block operator id
    #[error("Failed to get operator stake in quorums at block operator id ")]
    GetOperatorStakeInQuorumAtBlockOperatorId,

    /// Operator stake in quourm at block
    #[error("failed to get operator stake in quorum at current block number")]
    GetOperatorStakeInQuorumAtCurrentBlockNumber,

    /// Current Quorum bitmap
    #[error("Failed to get current quorum bitmap")]
    GetCurrentQuorumBitmap,

    /// Current stake
    #[error("Failed to get current stake")]
    GetCurrentStake,

    /// Check Signature indices result
    #[error("Check Signature indices result")]
    CheckSignatureIndices,

    /// Get Operator Id
    #[error("Get Operator Id")]
    GetOperatorId,

    /// Get Operator from Id
    #[error("Get Operator from Id")]
    GetOperatorFromId,

    /// Get Operator Status
    #[error("Get Operator Status")]
    GetOperatorStatus,

    /// Eth logs query
    #[error("Failed to get eth_logs")]
    GetEthLogs,

    /// Failed to decode event
    #[error("Failed to decode New PubKey Registration Filter")]
    DecodeEventNewPubkeyRegistrationFilter,
}
