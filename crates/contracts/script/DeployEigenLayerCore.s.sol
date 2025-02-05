// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

import {Script} from "forge-std/Script.sol";
import {CoreDeploymentLib} from "./utils/CoreDeploymentLib.sol";
import {UpgradeableProxyLib} from "./utils/UpgradeableProxyLib.sol";

contract DeployEigenlayerCore is Script {
    using CoreDeploymentLib for *;
    using UpgradeableProxyLib for address;

    address internal _deployer;
    address internal _proxyAdmin;
    CoreDeploymentLib.DeploymentData internal _deploymentData;
    CoreDeploymentLib.DeploymentConfigData internal _configData;

    function setUp() public virtual {
        _deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        vm.label(_deployer, "Deployer");
    }

    function run() external {
        vm.startBroadcast(_deployer);
        _proxyAdmin = UpgradeableProxyLib.deployProxyAdmin();
        _deploymentData = CoreDeploymentLib.deployContracts(_deployer, _proxyAdmin, _configData);
        vm.stopBroadcast();
        string memory deploymentPath = "script/deployments/core/";
        CoreDeploymentLib.writeDeploymentJson(deploymentPath, block.chainid, _deploymentData);
    }
}
