// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "forge-std/StdJson.sol";
import {console} from "forge-std/console.sol";
import "./parsers/TokensAndStrategiesContractsParser.sol";
import "./parsers/EigenlayerContractsParser.sol";
import {ConfigsReadWriter} from "./parsers/ConfigsReadWriter.sol";
import {ContractsRegistry} from "../src/ContractsRegistry.sol";

contract UpdateOperators is ConfigsReadWriter, EigenlayerContractsParser, TokenAndStrategyContractsParser {
    using Strings for uint256;

    string internal mnemonic;
    uint256 internal numberOfOperators;
    ContractsRegistry contractsRegistry = ContractsRegistry(0x5FbDB2315678afecb367f032d93F642f64180aa3);

    function setUp() public {
        numberOfOperators = 10;
        if (block.chainid == 31337 || block.chainid == 1337) {
            mnemonic = "test test test test test test test test test test test junk";
        } else {
            mnemonic = vm.envString("MNEMONIC");
        }
    }

    function run() external {
        EigenlayerContracts memory eigenlayerContracts = _loadEigenlayerDeployedContracts();
        TokenAndStrategyContracts memory tokenAndStrategy = _loadTokenAndStrategyContracts();
        
        address[] memory operators = new address[](numberOfOperators);
        uint256[] memory operatorsETHAmount = new uint256[](numberOfOperators);
        uint256[] memory operatorTokenAmounts = new uint256[](numberOfOperators);
        for (uint256 i; i < numberOfOperators; i++) {
            (address operator,) = deriveRememberKey(mnemonic, uint32(i));
            operators[i] = operator;
            operatorsETHAmount[i] = 5 ether;
            operatorTokenAmounts[i] = bound(uint256(keccak256(bytes.concat(bytes32(uint256(i))))), 1 ether, 10 ether);
        }
        vm.startBroadcast();

        // Allocate eth to operators
        _allocateEthOrErc20(address(0), operators, operatorsETHAmount);

        // Allocate tokens to operators
        _allocateEthOrErc20(address(tokenAndStrategy.token), operators, operatorTokenAmounts);

        vm.stopBroadcast();

        // Register operators with EigenLayer
        for (uint256 i = 0; i < numberOfOperators; i++) {
            address delegationApprover = address(0); // anyone can delegate to this operator
            uint32 stakerOptOutWindowBlocks = 120;
            (, uint256 privateKey) = deriveRememberKey(mnemonic, uint32(i));
            vm.startBroadcast(privateKey);
            contractsRegistry.store_test("test_modify_operator_details", int256(i), block.number, block.timestamp);
            eigenlayerContracts.delegationManager.modifyOperatorDetails(
                IDelegationManager.OperatorDetails(operators[i], delegationApprover, stakerOptOutWindowBlocks)
            );
            vm.stopBroadcast();
        }
    }

    // setting token=address(0) will allocate eth
    function _allocateEthOrErc20(address token, address[] memory tos, uint256[] memory amounts) internal {
        for (uint256 i = 0; i < tos.length; i++) {
            if (token == address(0)) {
                payable(tos[i]).transfer(amounts[i]);
            } else {
                IERC20(token).transfer(tos[i], amounts[i]);
            }
        }
    }
}
