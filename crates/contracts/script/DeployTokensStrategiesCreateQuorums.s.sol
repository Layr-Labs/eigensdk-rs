// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "eigenlayer-contracts/src/contracts/permissions/PauserRegistry.sol";

import {IStrategyManager, IStrategy} from "eigenlayer-contracts/src/contracts/interfaces/IStrategyManager.sol";
import {StrategyBaseTVLLimits} from "eigenlayer-contracts/src/contracts/strategies/StrategyBaseTVLLimits.sol";

import "eigenlayer-middleware/src/interfaces/IStakeRegistry.sol";
import "eigenlayer-middleware/src/RegistryCoordinator.sol" as regcoord;

import {MockERC20, IERC20} from "../src/MockERC20.sol";
import "./parsers/EigenlayerContractsParser.sol";
import "./parsers/MockAvsContractsParser.sol";
import {ContractsRegistry} from "../src/ContractsRegistry.sol";

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract DeployTokensStrategiesCreateQuorums is
    Script,
    EigenlayerContractsParser,
    MockAvsContractsParser
{
    uint MINT_AMOUNT = 5_000 ether;

    function run() external {
        // hardcoded as first contract deployed by anvil's 0th account
        // (generated from mnemonic "test test test test test test test test test test test junk")
        // 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 (sk = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80)
        ContractsRegistry contractsRegistry = ContractsRegistry(
            0x5FbDB2315678afecb367f032d93F642f64180aa3
        );
        EigenlayerContracts
            memory eigenlayerContracts = _loadEigenlayerDeployedContracts();
        MockAvsContracts
            memory mockAvsContracts = _loadMockAvsDeployedContracts();
        IStrategy strat;
        IERC20 mockToken;
        vm.startBroadcast();
        if (block.chainid == 31337 || block.chainid == 1337) {
            (mockToken, strat) = _deployErc20AndStrategyAndWhitelistStrategy(
                eigenlayerContracts.eigenlayerProxyAdmin,
                eigenlayerContracts.eigenlayerPauserReg,
                eigenlayerContracts.baseStrategyImplementation,
                eigenlayerContracts.strategyManager
            );
            contractsRegistry.registerContract(
                "erc20MockStrategy",
                address(strat)
            );
        } else if (block.chainid == 17000) {
            strat = IStrategy(0x5C8b55722f421556a2AAfb7A3EA63d4c3e514312); /// Whitelisted stETH strat
            mockToken = IERC20(0x3F1c547b21f65e10480dE3ad8E19fAAC46C95034); /// stETH
        } else {
            revert("Configure Token and Strategy for Chain");
        }
        _createQuorum(mockAvsContracts.registryCoordinator, strat);
        vm.stopBroadcast();
    }

    function _deployErc20AndStrategyAndWhitelistStrategy(
        ProxyAdmin eigenLayerProxyAdmin,
        PauserRegistry eigenLayerPauserReg,
        StrategyBaseTVLLimits baseStrategyImplementation,
        IStrategyManager strategyManager
    ) internal returns (IERC20, IStrategy) {
        MockERC20 mockERC20 = new MockERC20();
        mockERC20.mint(tx.origin, MINT_AMOUNT);
        // TODO(samlaf): any reason why we are using the strategybase with tvl limits instead of just using strategybase?
        // the maxPerDeposit and maxDeposits below are just arbitrary values.
        StrategyBaseTVLLimits erc20MockStrategy = StrategyBaseTVLLimits(
            address(
                new TransparentUpgradeableProxy(
                    address(baseStrategyImplementation),
                    address(eigenLayerProxyAdmin),
                    abi.encodeWithSelector(
                        StrategyBaseTVLLimits.initialize.selector,
                        1_000 ether, // maxPerDeposit
                        1_000 ether, // maxDeposits
                        IERC20(mockERC20),
                        eigenLayerPauserReg
                    )
                )
            )
        );
        IStrategy[] memory strats = new IStrategy[](1);
        strats[0] = erc20MockStrategy;
        bool[] memory thirdPartyTransfersForbiddenValues = new bool[](1);
        thirdPartyTransfersForbiddenValues[0] = false;
        strategyManager.addStrategiesToDepositWhitelist(
            strats,
            thirdPartyTransfersForbiddenValues
        );

        // WRITE JSON DATA
        // TODO: support more than one token/strategy pair
        string memory parent_object = "parent object";
        string memory deployed_addresses = "addresses";
        vm.serializeAddress(
            deployed_addresses,
            "erc20mock",
            address(mockERC20)
        );
        string memory deployed_addresses_output = vm.serializeAddress(
            deployed_addresses,
            "erc20mockstrategy",
            address(erc20MockStrategy)
        );
        string memory finalJson = vm.serializeString(
            parent_object,
            deployed_addresses,
            deployed_addresses_output
        );
        writeOutput(finalJson, "token_and_strategy_deployment_output");

        return (IERC20(mockERC20), erc20MockStrategy);
    }

    function _createQuorum(
        regcoord.RegistryCoordinator mockAvsRegCoord,
        IStrategy strat
    ) internal {
        // for each quorum to setup, we need to define
        // quorumsOperatorSetParams, quorumsMinimumStake, and quorumsStrategyParams
        regcoord.RegistryCoordinator.OperatorSetParam
            memory quorumOperatorSetParams = regcoord
                .IRegistryCoordinator
                .OperatorSetParam({
                    // hardcoded for now
                    maxOperatorCount: 10000,
                    kickBIPsOfOperatorStake: 15000,
                    kickBIPsOfTotalStake: 100
                });
        uint96 quorumMinimumStake = 0;
        IStakeRegistry.StrategyParams[]
            memory quorumStrategyParams = new IStakeRegistry.StrategyParams[](
                1
            );
        quorumStrategyParams[0] = IStakeRegistry.StrategyParams({
            strategy: strat,
            // setting this to 1 ether since the divisor is also 1 ether
            // therefore this allows an operator to register with even just 1 token
            // see https://github.com/Layr-Labs/eigenlayer-middleware/blob/m2-mainnet/src/StakeRegistry.sol#L484
            //    weight += uint96(sharesAmount * strategyAndMultiplier.multiplier / WEIGHTING_DIVISOR);
            multiplier: 1 ether
        });

        regcoord.RegistryCoordinator(address(mockAvsRegCoord)).createQuorum(
            quorumOperatorSetParams,
            quorumMinimumStake,
            quorumStrategyParams
        );
    }
}
