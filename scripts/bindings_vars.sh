### SDK bindings ###
SDK_CONTRACTS="MockAvsServiceManager ContractsRegistry MockERC20"
SDK_CONTRACTS_LOCATION=crates/contracts
SDK_BINDINGS_PATH=crates/utils/src/sdk

### Middleware bindings ###
MIDDLEWARE_CONTRACTS="RegistryCoordinator IndexRegistry OperatorStateRetriever StakeRegistry BLSApkRegistry IBLSSignatureChecker ServiceManagerBase IERC20 ECDSAStakeRegistry ECDSAServiceManagerBase"
MIDDLEWARE_CONTRACTS_LOCATION=$SDK_CONTRACTS_LOCATION/lib/eigenlayer-middleware
MIDDLEWARE_BINDINGS_PATH=crates/utils/src/middleware

### Core bindings ###
CORE_CONTRACTS="DelegationManager IRewardsCoordinator RewardsCoordinator  StrategyManager IEigenPod EigenPod IEigenPodManager EigenPodManager IStrategy AVSDirectory AllocationManager PermissionController ERC20 Slasher ISlasher"
CORE_CONTRACTS_LOCATION=$MIDDLEWARE_CONTRACTS_LOCATION/lib/eigenlayer-contracts
CORE_BINDINGS_PATH=crates/utils/src/core
