// // SPDX-License-Identifier: UNLICENSED
// pragma solidity ^0.8.20;

// import {IStrategy} from "@eigenlayer/contracts/interfaces/IStrategy.sol";
// import {MockAvsDeploymentLib} from "./utils/MockAvsDeploymentLib.sol";
// import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
// import {FundOperator} from "./utils/FundOperator.sol";
// import {StrategyFactory} from "@eigenlayer/contracts/strategies/StrategyFactory.sol";
// import {MockERC20} from "../src/MockERC20.sol";
// import {Vm} from "forge-std/Vm.sol";
// import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";

// // forge script script/DeployMockAvs.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $ETHERSCAN_API_KEY --broadcast --verify
// contract DeployMockAvs {
//     Vm internal constant _VM =
//         Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

//     address internal _deployer;
//     address internal _proxyAdmin;
//     CoreDeploymentLib.DeploymentData internal _configData;
//     MockERC20 public erc20Mock;
//     MockERC20 public erc20MockRewards;
//     IStrategy _mockAvsStrategy;

//     function setUp() public virtual {
//         _deployer = _VM.rememberKey(_VM.envUint("PRIVATE_KEY"));
//         _VM.label(_deployer, "Deployer");
//     }

//     function run() public virtual {
//         _VM.startBroadcast(_deployer);
//         _proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
//         _configData = CoreDeploymentLib.readDeploymentJson(
//             "script/deployments/core/",
//             block.chainid
//         );
//         erc20Mock = new MockERC20();
//         erc20MockRewards = new MockERC20();
//         MockAvsDeploymentLib.MockAvsSetupConfig
//             memory avsconfig = MockAvsDeploymentLib.readMockAvsConfigJson(
//                 "mock_avs_config"
//             );
//         FundOperator.fundOperator(
//             address(erc20Mock),
//             avsconfig.operatorAddr,
//             10e18
//         );
//         _mockAvsStrategy = IStrategy(
//             StrategyFactory(_configData.strategyFactory).deployNewStrategy(
//                 erc20Mock
//             )
//         );
//         MockAvsDeploymentLib.DeploymentData
//             memory depData = MockAvsDeploymentLib.deployContracts(
//                 _proxyAdmin,
//                 _configData,
//                 address(_mockAvsStrategy),
//                 avsconfig,
//                 msg.sender
//             );
//         IStrategy(
//             StrategyFactory(_configData.strategyFactory).deployNewStrategy(
//                 erc20MockRewards
//             )
//         );

//         depData.tokenRewards = address(erc20MockRewards);
//         depData.token = address(erc20Mock);

//         MockAvsDeploymentLib.writeDeploymentJson(depData);

//         _VM.stopBroadcast();
//     }
// }
