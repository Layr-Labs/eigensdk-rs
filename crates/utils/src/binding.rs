use alloy_sol_types::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    StakeRegistry,
    "../../crates/contracts/bindings/utils/json/StakeRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    RegistryCoordinator,
    "../../crates/contracts/bindings/utils/json/RegistryCoordinator.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    OperatorStateRetriever,
    "../../crates/contracts/bindings/utils/json/OperatorStateRetriever.json"
);

sol!(
    #[allow(missing_docs)]
    #[derive(Debug)]
    #[sol(rpc)]
    BLSApkRegistry,
    "../../crates/contracts/bindings/utils/json/BLSApkRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ServiceManagerBase,
    "../../crates/contracts/bindings/utils/json/ServiceManagerBase.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    AVSDirectory,
    "../../crates/contracts/bindings/utils/json/AVSDirectory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DelegationManager,
    "../../crates/contracts/bindings/utils/json/DelegationManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ISlasher,
    "../../crates/contracts/bindings/utils/json/ISlasher.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IStrategy,
    "../../crates/contracts/bindings/utils/json/IStrategy.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IERC20,
    "../../crates/contracts/bindings/utils/json/IERC20.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    StrategyManager,
    "../../crates/contracts/bindings/utils/json/StrategyManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IAVSDirectory,
    "../../crates/contracts/bindings/utils/json/IAVSDirectory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ECDSAStakeRegistry,
    "../../crates/contracts/bindings/utils/json/ECDSAStakeRegistry.json"
);

