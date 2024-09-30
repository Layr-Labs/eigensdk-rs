use alloy_sol_types::sol;

// https://github.com/Layr-Labs/eigenlayer-middleware/blob/m2-mainnet/src/StakeRegistry.sol
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    StakeRegistry,
    "json/StakeRegistry.json"
);

// https://github.com/Layr-Labs/eigenlayer-middleware/blob/m2-mainnet/src/RegistryCoordinator.sol
sol!(
    #[allow(clippy::too_many_arguments)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    RegistryCoordinator,
    "json/RegistryCoordinator.json"
);

// https://github.com/Layr-Labs/eigenlayer-middleware/blob/m2-mainnet/src/OperatorStateRetriever.sol
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    OperatorStateRetriever,
    "json/OperatorStateRetriever.json"
);

// https://github.com/Layr-Labs/eigenlayer-middleware/blob/m2-mainnet/src/BLSApkRegistry.sol
sol!(
    #[allow(missing_docs)]
    #[derive(Debug)]
    #[sol(rpc)]
    BLSApkRegistry,
    "json/BLSApkRegistry.json"
);

// https://github.com/Layr-Labs/eigenlayer-middleware/blob/m2-mainnet/src/ServiceManagerBase.sol
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ServiceManagerBase,
    "json/ServiceManagerBase.json"
);

// https://github.com/Layr-Labs/eigenlayer-contracts/blob/mainnet/src/contracts/core/AVSDirectory.sol
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    AVSDirectory,
    "json/AVSDirectory.json"
);

// https://github.com/Layr-Labs/eigenlayer-contracts/blob/mainnet/src/contracts/core/DelegationManager.sol
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DelegationManager,
    "json/DelegationManager.json"
);

// https://github.com/Layr-Labs/eigenlayer-contracts/blob/mainnet/src/contracts/interfaces/ISlasher.sol
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ISlasher,
    "json/ISlasher.json"
);

// https://github.com/Layr-Labs/eigenlayer-contracts/blob/mainnet/src/contracts/interfaces/IStrategy.sol
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IStrategy,
    "json/IStrategy.json"
);

// standard
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IERC20,
    "json/IERC20.json"
);

// https://github.com/Layr-Labs/eigenlayer-contracts/blob/mainnet/src/contracts/core/StrategyManager.sol
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    StrategyManager,
    "json/StrategyManager.json"
);

// https://github.com/Layr-Labs/eigenlayer-contracts/blob/mainnet/src/contracts/interfaces/IAVSDirectory.sol
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IAVSDirectory,
    "json/IAVSDirectory.json"
);

// https://github.com/Layr-Labs/eigenlayer-middleware/blob/mainnet/src/unaudited/ECDSAStakeRegistry.sol
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ECDSAStakeRegistry,
    "json/ECDSAStakeRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IBLSSignatureChecker,
    "json/IBLSSignatureChecker.json"
);

// Anvil utilities

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ContractsRegistry,
    "json/ContractsRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    mockAvsServiceManager,
    "json/MockAvsServiceManager.json"
);
