// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Script} from "forge-std/Script.sol";

import {M2CoreDeploymentLib} from "./utils/M2CoreDeploymentLib.sol";
import {M2MockAvsDeploymentLib} from "./utils/M2MockAvsDeploymentLib.sol";
import {WriteToContractsRegistryLib} from "./utils/WriteToContractsRegistryLib.sol";

contract ContractsRegistry is Script {
    address private deployer;
    address public CONTRACT_REGISTRY;
    M2CoreDeploymentLib.DeploymentData coreConfigData;
    M2MockAvsDeploymentLib.DeploymentData avsConfigData;

    function setUp() public {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");
        CONTRACT_REGISTRY = vm.envAddress("CONTRACTS_REGISTRY_ADDR");

        coreConfigData = M2CoreDeploymentLib.readDeploymentJson("script/deployments/core/", "31337.json");
        avsConfigData = M2MockAvsDeploymentLib.readDeploymentJson("script/deployments/mock-avs/", block.chainid);
    }

    function run() external {
        vm.startBroadcast(deployer);
        if (block.chainid == 31337 || block.chainid == 1337) {
            WriteToContractsRegistryLib.writeCoreContractsToRegistry(CONTRACT_REGISTRY, coreConfigData);
            WriteToContractsRegistryLib.writeMockAvsContractsToRegistry(CONTRACT_REGISTRY, avsConfigData);
        }
        vm.stopBroadcast();
    }
}
