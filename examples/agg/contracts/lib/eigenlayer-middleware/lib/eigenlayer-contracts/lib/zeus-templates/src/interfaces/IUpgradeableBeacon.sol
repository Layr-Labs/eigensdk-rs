// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

interface IUpgradeableBeacon {
    function implementation() external view returns (address);

    function upgradeTo(address newImplementation) external;
}
