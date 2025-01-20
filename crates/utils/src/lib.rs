/// forge bind --alloy --skip-build --bindings-path ../utils/src/core --overwrite --root ./lib/eigenlayer-middleware/lib/eigenlayer-contracts  --module  --select DelegationManager --select RewardsCoordinator --select StrategyManager --select IEigenPod --select EigenPod --select EigenPodManager --select Strategy --select AVSDirectory --select AllocationManager --select PermissionController --select ERC20 --select Slasher
pub mod core;

/// forge bind --alloy --skip-build --bindings-path ../utils/src/middleware --overwrite --root ./lib/eigenlayer-middleware  --module  --select RegistryCoordinator --select IndexRegistry --select OperatorStateRetriever --select StakeRegistry --select BLSApkRegistry --select IBLSSignatureChecker --select ServiceManagerBase --select IERC20
pub mod middleware;

/// forge bind --alloy --skip-build --bindings-path ../utils/src/sdk --overwrite -C src/contracts --module  --select MockAvsServiceManager --select ContractsRegistry
pub mod sdk;
