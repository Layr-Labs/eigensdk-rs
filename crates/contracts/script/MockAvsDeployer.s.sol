// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";

import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

import "@eigenlayer/contracts/permissions/PauserRegistry.sol";

import {IDelegationManager} from "@eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {IAVSDirectory} from "@eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {IStrategyManager, IStrategy} from "@eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {ISlasher} from "@eigenlayer/contracts/interfaces/ISlasher.sol";
import {StrategyBaseTVLLimits} from "@eigenlayer/contracts/strategies/StrategyBaseTVLLimits.sol";
import "@eigenlayer/test/mocks/EmptyContract.sol";

import "@eigenlayer-middleware/src/RegistryCoordinator.sol" as regcoord;
import {IBLSApkRegistry, IIndexRegistry, IStakeRegistry} from "@eigenlayer-middleware/src/RegistryCoordinator.sol";
import {BLSApkRegistry} from "@eigenlayer-middleware/src/BLSApkRegistry.sol";
import {IndexRegistry} from "@eigenlayer-middleware/src/IndexRegistry.sol";
import {StakeRegistry} from "@eigenlayer-middleware/src/StakeRegistry.sol";
import "@eigenlayer-middleware/src/OperatorStateRetriever.sol";

import {MockAvsServiceManager, IServiceManager} from "../src/MockAvsServiceManager.sol";
import "../src/MockERC20.sol";

import "forge-std/Test.sol";
import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console.sol";
import {StrategyFactory} from "@eigenlayer/contracts/strategies/StrategyFactory.sol";

import {ContractsRegistry} from "../src/ContractsRegistry.sol";
import {IncredibleSquaringDeploymentLib} from "../script/utils/IncredibleSquaringDeploymentLib.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";

import {FundOperator} from "./utils/FundOperator.sol";
// # To deploy and verify our contract
// forge script script/IncredibleSquaringDeployer.s.sol:IncredibleSquaringDeployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv

contract IncredibleSquaringDeployer is Script {
    // DEPLOYMENT CONSTANTS
    address public CONTRACTS_REGISTRY_ADDR;
    ContractsRegistry contractsRegistry;

    StrategyBaseTVLLimits public erc20MockStrategy;

    address public rewardscoordinator;

    ProxyAdmin public incredibleSquaringProxyAdmin;
    PauserRegistry public incredibleSquaringPauserReg;



   


    MockAvsServiceManager public incredibleSquaringServiceManager;
    IServiceManager public incredibleSquaringServiceManagerImplementation;


    CoreDeploymentLib.DeploymentData internal configData;
    IStrategy incredibleSquaringStrategy;
    address private deployer;
    MockERC20 public erc20Mock;
    IncredibleSquaringDeploymentLib.DeploymentData incrediblSquaringDeployment;

    using UpgradeableProxyLib for address;

    address proxyAdmin;

    function setUp() public virtual {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");
    }

    function run() external {
        // Eigenlayer contracts
        vm.startBroadcast(deployer);
        IncredibleSquaringDeploymentLib.IncredibleSquaringSetupConfig memory isConfig =
            IncredibleSquaringDeploymentLib.readIncredibleSquaringConfigJson("incredible_squaring_config");

        // AGGREGATOR_ADDR = vm.envAddress("AGGREGATOR_ADDR");
        // TASK_GENERATOR_ADDR = vm.envAddress("TASK_GENERATOR_ADDR");
        // CONTRACTS_REGISTRY_ADDR = vm.envAddress("CONTRACTS_REGISTRY_ADDR");
        // OPERATOR_ADDR = vm.envAddress("OPERATOR_ADDR");
        // contractsRegistry = ContractsRegistry(CONTRACTS_REGISTRY_ADDR);

        configData = CoreDeploymentLib.readDeploymentJson("script/deployments/core/", block.chainid);

        erc20Mock = new MockERC20();
        FundOperator.fund_operator(address(erc20Mock), isConfig.operator_addr, 10e18);

        incredibleSquaringStrategy = IStrategy(StrategyFactory(configData.strategyFactory).deployNewStrategy(erc20Mock));
        rewardscoordinator = configData.rewardsCoordinator;

        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
        require(address(incredibleSquaringStrategy) != address(0));
        require(address(incredibleSquaringStrategy) != address(0));
        incrediblSquaringDeployment = IncredibleSquaringDeploymentLib.deployContracts(
            proxyAdmin, configData, address(incredibleSquaringStrategy), isConfig, msg.sender
        );

        IncredibleSquaringDeploymentLib.writeDeploymentJson(incrediblSquaringDeployment);

        vm.stopBroadcast();
    }
}