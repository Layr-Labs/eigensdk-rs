// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {M2CoreDeploymentLib} from "./utils/M2CoreDeploymentLib.sol";
import {M2UpgradeableProxyLib} from "./utils/M2UpgradeableProxyLib.sol";

contract M2DeployEigenlayerCore is Script {
    using M2CoreDeploymentLib for *;
    using M2UpgradeableProxyLib for address;

    address internal deployer;
    address internal proxyAdmin;
    M2CoreDeploymentLib.DeploymentData internal deploymentData;
    M2CoreDeploymentLib.DeploymentConfigData internal configData;

    function setUp() public virtual {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(deployer, "Deployer");
    }

    function run() external {
        vm.startBroadcast(deployer);
        proxyAdmin = M2UpgradeableProxyLib.deployProxyAdmin();
        deploymentData = M2CoreDeploymentLib.deployContracts(deployer, proxyAdmin, configData);
        vm.stopBroadcast();
        string memory deploymentPath = "script/deployments/core/m2/";
        M2CoreDeploymentLib.writeDeploymentJson(deploymentPath, block.chainid, deploymentData);
    }
}
