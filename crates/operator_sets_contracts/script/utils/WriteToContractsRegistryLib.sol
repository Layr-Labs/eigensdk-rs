// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Vm} from "forge-std/Vm.sol";
import {ContractsRegistry} from "../../src/ContractsRegistry.sol";
import {M2CoreDeploymentLib} from "./M2CoreDeploymentLib.sol";
import {M2MockAvsDeploymentLib} from "./M2MockAvsDeploymentLib.sol";

library WriteToContractsRegistryLib {
    function writeCoreContractsToRegistry(
        address contractsRegistryAddr,
        M2CoreDeploymentLib.DeploymentData memory deploymentdata
    ) internal {
        ContractsRegistry contractsRegistry = ContractsRegistry(contractsRegistryAddr);
        contractsRegistry.registerContract("delegationManager", address(deploymentdata.delegationManager));
        contractsRegistry.registerContract("strategyManager", address(deploymentdata.strategyManager));
        contractsRegistry.registerContract("avsDirectory", address(deploymentdata.avsDirectory));
        contractsRegistry.registerContract("rewardsCoordinator", address(deploymentdata.rewardsCoordinator));
        contractsRegistry.registerContract("pauserRegistry", address(deploymentdata.pauserRegistry));
    }

    function writeMockAvsContractsToRegistry(
        address contractsRegistryAddr,
        M2MockAvsDeploymentLib.DeploymentData memory deploymentdata
    ) internal {
        ContractsRegistry contractsRegistry = ContractsRegistry(contractsRegistryAddr);

        contractsRegistry.registerContract("erc20MockStrategy", address(deploymentdata.strategy));
        contractsRegistry.registerContract("mockAvsServiceManager", deploymentdata.mockAvsServiceManager);
        contractsRegistry.registerContract("mockAvsRegistryCoordinator", address(deploymentdata.registryCoordinator));
        contractsRegistry.registerContract(
            "mockAvsOperatorStateRetriever", address(deploymentdata.operatorStateRetriever)
        );
    }
}
