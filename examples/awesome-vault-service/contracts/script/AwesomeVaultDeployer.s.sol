// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";

import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@eigenlayer/contracts/permissions/PauserRegistry.sol";

import {IDelegationManager} from "@eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {IAVSDirectory} from "@eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {IStrategyManager, IStrategy} from "@eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {StrategyBaseTVLLimits} from "@eigenlayer/contracts/strategies/StrategyBaseTVLLimits.sol";
import "@eigenlayer/test/mocks/EmptyContract.sol";

import "@eigenlayer-middleware/src/RegistryCoordinator.sol" as regcoord;
import {
    IBLSApkRegistry,
    IIndexRegistry,
    IStakeRegistry
} from "@eigenlayer-middleware/src/RegistryCoordinator.sol";
import {BLSApkRegistry} from "@eigenlayer-middleware/src/BLSApkRegistry.sol";
import {IndexRegistry} from "@eigenlayer-middleware/src/IndexRegistry.sol";
import {StakeRegistry} from "@eigenlayer-middleware/src/StakeRegistry.sol";
import "@eigenlayer-middleware/src/OperatorStateRetriever.sol";

import {
    IncredibleRedisServiceManager,
    IServiceManager
} from "../src/IncredibleRedisServiceManager.sol";
import {IncredibleRedisTaskManager} from "../src/IncredibleRedisTaskManager.sol";
import {IIncredibleRedisTaskManager} from "../src/IIncredibleRedisTaskManager.sol";
import "../src/MockERC20.sol";

import "forge-std/Test.sol";
import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console.sol";
import {StrategyFactory} from "@eigenlayer/contracts/strategies/StrategyFactory.sol";

import {ContractsRegistry} from "../src/ContractsRegistry.sol";
import {IncredibleRedisDeploymentLib} from "../script/utils/IncredibleRedisDeploymentLib.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";

import {FundOperator} from "./utils/FundOperator.sol";
// # To deploy and verify our contract
// forge script script/AwesomeVaultDeployer.s.sol:AwesomeVaultDeployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv

contract AwesomeVaultDeployer is Script {
    // DEPLOYMENT CONSTANTS
    uint256 public constant QUORUM_THRESHOLD_PERCENTAGE = 100;
    uint32 public constant TASK_RESPONSE_WINDOW_BLOCK = 30;
    uint32 public constant TASK_DURATION_BLOCKS = 0;
    address public AGGREGATOR_ADDR;
    address public TASK_GENERATOR_ADDR;
    address public CONTRACTS_REGISTRY_ADDR;
    address public OPERATOR_ADDR;
    address public OPERATOR_2_ADDR;
    ContractsRegistry contractsRegistry;

    StrategyBaseTVLLimits public erc20MockStrategy;

    address public rewardscoordinator;

    ProxyAdmin public awesomeVaultProxyAdmin;
    PauserRegistry public awesomeVaultPauserReg;

    regcoord.RegistryCoordinator public registryCoordinator;
    regcoord.IRegistryCoordinator public registryCoordinatorImplementation;

    IBLSApkRegistry public blsApkRegistry;
    IBLSApkRegistry public blsApkRegistryImplementation;

    IIndexRegistry public indexRegistry;
    IIndexRegistry public indexRegistryImplementation;

    IStakeRegistry public stakeRegistry;
    IStakeRegistry public stakeRegistryImplementation;

    OperatorStateRetriever public operatorStateRetriever;

    AwesomeVaultServiceManager public awesomeVaultServiceManager;
    IServiceManager public awesomeVaultServiceManagerImplementation;

    AwesomeVaultTaskManager public awesomeVaultTaskManager;
    IAwesomeVaultTaskManager public awesomeVaultTaskManagerImplementation;
    CoreDeploymentLib.DeploymentData internal configData;
    IStrategy awesomeVaultStrategy;
    address private deployer;
    MockERC20 public erc20Mock;
    AwesomeVaultDeploymentLib.DeploymentData awesomeVaultDeployment;

    using UpgradeableProxyLib for address;

    address proxyAdmin;

    function setUp() public virtual {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");
    }

    function run() external {
        // Eigenlayer contracts
        vm.startBroadcast(deployer);
        AwesomeVaultDeploymentLib.AwesomeVaultSetupConfig memory avsConfig =
        AwesomeVaultDeploymentLib.readAwesomeVaultConfigJson(
            "config/avs/awesome_vault_config"
        );
        configData = CoreDeploymentLib.readDeploymentJson("script/deployments/core/", block.chainid);

        erc20Mock = new MockERC20();
        console.log(address(erc20Mock));
        FundOperator.fund_operator(address(erc20Mock), idpConfig.operator_addr, 15_000e18);
        FundOperator.fund_operator(address(erc20Mock), idpConfig.operator_2_addr, 30_000e18);
        console.log(idpConfig.operator_2_addr);
        (bool s,) = idpConfig.operator_2_addr.call{value: 0.1 ether}("");
        require(s);
        incredibleRedisStrategy =
            IStrategy(StrategyFactory(configData.strategyFactory).deployNewStrategy(erc20Mock));
        rewardscoordinator = configData.rewardsCoordinator;

        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
        require(address(incredibleRedisStrategy) != address(0));
        incredibleRedisDeployment = IncredibleRedisDeploymentLib.deployContracts(
            proxyAdmin, configData, address(incredibleRedisStrategy), idpConfig, msg.sender
        );
        console.log("instantSlasher", incredibleRedisDeployment.slasher);

        FundOperator.fund_operator(
            address(erc20Mock), incredibleRedisDeployment.incredibleRedisServiceManager, 1e18
        );
        incredibleRedisDeployment.token = address(erc20Mock);

        IncredibleRedisDeploymentLib.writeDeploymentJson(incredibleRedisDeployment);

        vm.stopBroadcast();
    }
}
