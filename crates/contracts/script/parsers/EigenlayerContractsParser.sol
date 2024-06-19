// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "eigenlayer-contracts/src/contracts/permissions/PauserRegistry.sol";

import {IAVSDirectory} from "eigenlayer-contracts/src/contracts/interfaces/IAVSDirectory.sol";
import {IDelegationManager} from "eigenlayer-contracts/src/contracts/interfaces/IDelegationManager.sol";
import {IStrategyManager, IStrategy} from "eigenlayer-contracts/src/contracts/interfaces/IStrategyManager.sol";
import {ISlasher} from "eigenlayer-contracts/src/contracts/interfaces/ISlasher.sol";
import {StrategyBaseTVLLimits} from "eigenlayer-contracts/src/contracts/strategies/StrategyBaseTVLLimits.sol";
import {IRewardsCoordinator} from "eigenlayer-contracts/src/contracts/interfaces/IRewardsCoordinator.sol";

import {ConfigsReadWriter} from "./ConfigsReadWriter.sol";
import "forge-std/StdJson.sol";

struct EigenlayerContracts {
    ProxyAdmin eigenlayerProxyAdmin;
    PauserRegistry eigenlayerPauserReg;
    IStrategyManager strategyManager;
    IDelegationManager delegationManager;
    ISlasher slasher;
    IAVSDirectory avsDirectory;
    IRewardsCoordinator rewardsCoordinator;
    StrategyBaseTVLLimits baseStrategyImplementation;
}

contract EigenlayerContractsParser is ConfigsReadWriter {
    function _loadEigenlayerDeployedContracts()
        internal
        view
        returns (EigenlayerContracts memory)
    {
        // Eigenlayer contracts
        string memory eigenlayerDeployedContracts = readOutput(
            "eigenlayer_deployment_output"
        );
        ProxyAdmin eigenlayerProxyAdmin = ProxyAdmin(
            stdJson.readAddress(
                eigenlayerDeployedContracts,
                ".addresses.eigenLayerProxyAdmin"
            )
        );
        PauserRegistry eigenlayerPauserReg = PauserRegistry(
            stdJson.readAddress(
                eigenlayerDeployedContracts,
                ".addresses.eigenLayerPauserReg"
            )
        );
        IStrategyManager strategyManager = IStrategyManager(
            stdJson.readAddress(
                eigenlayerDeployedContracts,
                ".addresses.strategyManager"
            )
        );
        IDelegationManager delegationManager = IDelegationManager(
            stdJson.readAddress(
                eigenlayerDeployedContracts,
                ".addresses.delegation"
            )
        );
        IAVSDirectory avsDirectory = IAVSDirectory(
            stdJson.readAddress(
                eigenlayerDeployedContracts,
                ".addresses.avsDirectory"
            )
        );
        ISlasher slasher = ISlasher(
            stdJson.readAddress(
                eigenlayerDeployedContracts,
                ".addresses.slasher"
            )
        );
        StrategyBaseTVLLimits baseStrategyImplementation = StrategyBaseTVLLimits(
                stdJson.readAddress(
                    eigenlayerDeployedContracts,
                    ".addresses.baseStrategyImplementation"
                )
            );
        // TODO: Update this to read from the eigenlayerDeployedContracts
        // right now M2_Deploy_from_scratch.s.sol deployment script doesnt deploy rewardsCoordinator
        IRewardsCoordinator rewardsCoordinator = IRewardsCoordinator(
            address(0x0)
        );
        // IRewardsCoordinator rewardsCoordinator = IRewardsCoordinator(
        //     stdJson.readAddress(
        //         eigenlayerDeployedContracts,
        //         ".addresses.rewardsCoordinator"
        //     )
        // );
        return
            EigenlayerContracts(
                eigenlayerProxyAdmin,
                eigenlayerPauserReg,
                strategyManager,
                delegationManager,
                slasher,
                avsDirectory,
                rewardsCoordinator,
                baseStrategyImplementation
            );
    }
}
