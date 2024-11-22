// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Vm} from "forge-std/Vm.sol";
import {ContractsRegistry} from "../../src/ContractsRegistry.sol";
import {CoreDeploymentLib} from "./CoreDeploymentLib.sol";
import {MockAvsDeploymentLib} from "./MockAvsDeploymentLib.sol";

library WriteToContractsRegistryLib {
    function writeCoreContractsToRegistry(
        address contracts_registry_addr,
        CoreDeploymentLib.DeploymentData memory deploymentdata
    ) internal {
        ContractsRegistry contractsRegistry = ContractsRegistry(contracts_registry_addr);
        contractsRegistry.registerContract("delegationManager", address(deploymentdata.delegationManager));
        contractsRegistry.registerContract("strategyManager", address(deploymentdata.strategyManager));
        contractsRegistry.registerContract("avsDirectory", address(deploymentdata.avsDirectory));
    }

    function writeMockAvsContractsToRegistry(
        address contracts_registry_addr,
        MockAvsDeploymentLib.DeploymentData memory deploymentdata
    ) internal {
        ContractsRegistry contractsRegistry = ContractsRegistry(contracts_registry_addr);

        contractsRegistry.registerContract("erc20MockStrategy", address(deploymentdata.strategy));
        contractsRegistry.registerContract("mockAvsServiceManager", deploymentdata.mockAvsServiceManager);
        contractsRegistry.registerContract("mockAvsRegistryCoordinator", address(deploymentdata.registryCoordinator));
        contractsRegistry.registerContract(
            "mockAvsOperatorStateRetriever", address(deploymentdata.operatorStateRetriever)
        );
    }
}
