// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "forge-std/console.sol";
import {IStrategy} from "@eigenlayer/contracts/interfaces/IStrategy.sol";
import {MockAvsDeploymentLib} from "./utils/MockAvsDeploymentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {FundOperator} from "./utils/FundOperator.sol";

import {StrategyFactory} from "@eigenlayer/contracts/strategies/StrategyFactory.sol";

import {MockERC20} from "../src/MockERC20.sol";
import {Vm} from "forge-std/Vm.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";

// forge script script/DeployMockAvs.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $ETHERSCAN_API_KEY --broadcast --verify
contract DeployMockAvs {
    Vm internal constant vm =
        Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    address internal deployer;
    address internal proxyAdmin;
    CoreDeploymentLib.DeploymentData internal configData;
    MockERC20 public erc20Mock;
    IStrategy mockAvsStrategy;

    function setUp() public virtual {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");
    }

    function run() public virtual {
        vm.startBroadcast(deployer);
        proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
        configData = CoreDeploymentLib.readDeploymentJson(
            "script/deployments/core/",
            block.chainid
        );
        erc20Mock = new MockERC20();
        MockAvsDeploymentLib.MockAvsSetupConfig
            memory avsconfig = MockAvsDeploymentLib.readMockAvsConfigJson(
                "mock_avs_config"
            );
        FundOperator.fund_operator(
            address(erc20Mock),
            avsconfig.operator_addr,
            10e18
        );
        mockAvsStrategy = IStrategy(
            StrategyFactory(configData.strategyFactory).deployNewStrategy(
                erc20Mock
            )
        );
        MockAvsDeploymentLib.DeploymentData
            memory depData = MockAvsDeploymentLib.deployContracts(
                proxyAdmin,
                configData,
                address(mockAvsStrategy),
                avsconfig,
                msg.sender
            );

        MockAvsDeploymentLib.writeDeploymentJson(depData);

        vm.stopBroadcast();
    }
}
