// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

interface ITimelock {
    function delay() external view returns (uint256);

    function queuedTransactions(bytes32) external view returns (bool);

    function queueTransaction(address target, uint256 value, string memory signature, bytes memory data, uint256 eta)
        external
        returns (bytes32);

    function executeTransaction(address target, uint256 value, string memory signature, bytes memory data, uint256 eta)
        external
        payable
        returns (bytes memory);

    function cancelTransaction(address target, uint256 value, string memory signature, bytes memory data, uint256 eta)
        external;
}
