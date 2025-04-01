// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

import "./ITransparentUpgradeableProxy.sol";

interface IProxyAdmin {
    function getProxyImplementation(ITransparentUpgradeableProxy proxy) external view returns (address);

    function getProxyAdmin(ITransparentUpgradeableProxy proxy) external view returns (address);

    function changeProxyAdmin(ITransparentUpgradeableProxy proxy, address newAdmin) external;

    function upgrade(ITransparentUpgradeableProxy proxy, address implementation) external;

    function upgradeAndCall(ITransparentUpgradeableProxy proxy, address implementation, bytes memory data)
        external
        payable;
}
