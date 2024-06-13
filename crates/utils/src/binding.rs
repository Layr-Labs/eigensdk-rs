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
