// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {IStrategy} from "@eigenlayer/contracts/interfaces/IStrategy.sol";
import {MockAvsDeploymentLib} from "./utils/MockAvsDeploymentLib.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {FundOperator} from "./utils/FundOperator.sol";
import {StrategyFactory} from "@eigenlayer/contracts/strategies/StrategyFactory.sol";
import {MockERC20} from "../src/MockERC20.sol";
import {MockAvsServiceManager} from "../src/MockAvsServiceManager.sol";
import {AllocationManager} from "@eigenlayer/contracts/core/AllocationManager.sol";
import {Vm} from "forge-std/Vm.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";
import {StrategyManager} from "@eigenlayer/contracts/core/StrategyManager.sol";
import {DelegationManager} from "@eigenlayer/contracts/core/DelegationManager.sol";
import "forge-std/StdCheats.sol";

// // forge script script/DeployMockAvs.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $ETHERSCAN_API_KEY --broadcast --verify
contract DeployMockAvs {
    Vm internal constant _VM = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    address internal _deployer;
    address internal _proxyAdmin;
    CoreDeploymentLib.DeploymentData internal _configData;
    MockERC20 public erc20Mock;
    MockERC20 public erc20MockRewards;
    IStrategy _mockAvsStrategy;

    function setUp() public virtual {
        _deployer = _VM.rememberKey(_VM.envUint("PRIVATE_KEY"));
        _VM.label(_deployer, "Deployer");
    }

    function run() public virtual {
        _VM.startBroadcast(_deployer);
        _proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
        _configData = CoreDeploymentLib.readDeploymentJson("script/deployments/core/", block.chainid);
        erc20Mock = new MockERC20();
        erc20MockRewards = new MockERC20();
        MockAvsDeploymentLib.MockAvsSetupConfig memory avsconfig =
            MockAvsDeploymentLib.readMockAvsConfigJson("mock_avs_config");
        FundOperator.fundOperator(address(erc20Mock), avsconfig.operatorAddr, 10e18);
        _mockAvsStrategy = IStrategy(StrategyFactory(_configData.strategyFactory).deployNewStrategy(erc20Mock));
        MockAvsDeploymentLib.DeploymentData memory depData = MockAvsDeploymentLib.deployContracts(
            _proxyAdmin, _configData, address(_mockAvsStrategy), avsconfig, msg.sender
        );
        StrategyFactory(_configData.strategyFactory).deployNewStrategy(erc20MockRewards);

        // Register operators with EigenLayer
        uint256 numberOfOperators = 10;
        string memory mnemonic = "test test test test test test test test test test test junk";
        address[] memory operators = new address[](numberOfOperators);
        uint256[] memory operatorTokenAmounts = new uint256[](numberOfOperators);
        for (uint256 i; i < numberOfOperators; i++) {
            uint256 privateKey = _VM.deriveKey(mnemonic, uint32(i));
            address operator = _VM.rememberKey(privateKey);
            operators[i] = operator;
            operatorTokenAmounts[i] = 10 ether;
            FundOperator.fundOperator(address(erc20Mock), operator, 10e18);
        }

        MockAvsServiceManager(depData.mockAvsServiceManager).setAppointee(
            _deployer, _configData.allocationManager, AllocationManager.updateAVSMetadataURI.selector
        );
        AllocationManager(_configData.allocationManager).updateAVSMetadataURI(
            depData.mockAvsServiceManager, "https://coolstuff.com/avs"
        );

        _VM.stopBroadcast();

        for (uint256 i = 0; i < numberOfOperators; i++) {
            address delegationApprover = address(0); // anyone can delegate to this operator
            uint32 allocationDelayBlocks = 1;
            string memory metadataURI = string.concat("https://coolstuff.com/operator/", _VM.toString(i));
            uint256 privateKey = _VM.deriveKey(mnemonic, uint32(i));
            _VM.startBroadcast(privateKey);
            DelegationManager(_configData.delegationManager).registerAsOperator(
                delegationApprover, allocationDelayBlocks, metadataURI
            );
            StrategyManager(_configData.strategyManager).depositIntoStrategy(
                _mockAvsStrategy, erc20Mock, operatorTokenAmounts[i]
            );
            _VM.stopBroadcast();
        }

        depData.tokenRewards = address(erc20MockRewards);
        depData.token = address(erc20Mock);

        MockAvsDeploymentLib.writeDeploymentJson(depData);
    }
}
