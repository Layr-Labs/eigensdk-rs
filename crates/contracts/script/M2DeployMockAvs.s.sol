// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {IStrategy} from "@m2-eigenlayer/contracts/interfaces/IStrategy.sol";
import {M2MockAvsDeploymentLib} from "./utils/M2MockAvsDeploymentLib.sol";
import {M2CoreDeploymentLib} from "./utils/M2CoreDeploymentLib.sol";
import {FundOperator} from "./utils/FundOperator.sol";
import {StrategyFactory} from "@m2-eigenlayer/contracts/strategies/StrategyFactory.sol";
import {MockERC20} from "../src/MockERC20.sol";
import {Vm} from "forge-std/Vm.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";
import {StrategyManager} from "@m2-eigenlayer/contracts/core/StrategyManager.sol";
import {IDelegationManager} from "@m2-eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {DelegationManager} from "@m2-eigenlayer/contracts/core/DelegationManager.sol";

import "forge-std/StdCheats.sol";

// forge script script/DeployMockAvs.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $ETHERSCAN_API_KEY --broadcast --verify
contract M2DeployMockAvs {
    Vm internal constant _VM = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    address internal _deployer;
    address internal _proxyAdmin;
    M2CoreDeploymentLib.DeploymentData internal _configData;
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
        _configData = M2CoreDeploymentLib.readDeploymentJson("script/deployments/core/", block.chainid);
        erc20Mock = new MockERC20();
        M2MockAvsDeploymentLib.MockAvsSetupConfig memory avsconfig =
            M2MockAvsDeploymentLib.readMockAvsConfigJson("mock_avs_config");
        FundOperator.fundOperator(address(erc20Mock), avsconfig.operator_addr, 10e18);
        _mockAvsStrategy = IStrategy(StrategyFactory(_configData.strategyFactory).deployNewStrategy(erc20Mock));
        M2MockAvsDeploymentLib.DeploymentData memory depData = M2MockAvsDeploymentLib.deployContracts(
            _proxyAdmin, _configData, address(_mockAvsStrategy), avsconfig, msg.sender
        );

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

        _VM.stopBroadcast();

        IDelegationManager.OperatorDetails memory operatorDetails = IDelegationManager.OperatorDetails({
            __deprecated_earningsReceiver: address(0),
            delegationApprover: address(0),
            stakerOptOutWindowBlocks: uint32(0)
        });

        for (uint256 i = 0; i < numberOfOperators; i++) {
            string memory metadataURI = string.concat("https://coolstuff.com/operator/", _VM.toString(i));
        
            uint256 privateKey = _VM.deriveKey(mnemonic, uint32(i));
            _VM.startBroadcast(privateKey);
            console2.log("delegationnn");
            console2.log(_configData.delegationManager);
            DelegationManager(_configData.delegationManager).registerAsOperator(
                operatorDetails,metadataURI
            );
            StrategyManager(_configData.strategyManager).depositIntoStrategy(
                _mockAvsStrategy, erc20Mock, operatorTokenAmounts[i]
            );
            _VM.stopBroadcast();
        }

        depData.token = address(erc20Mock);
        depData.strategy = address(_mockAvsStrategy);

        M2MockAvsDeploymentLib.writeDeploymentJson(depData);
    }
}
