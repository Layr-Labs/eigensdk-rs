// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

import "../utils/ZeusScript.sol";

/**
 * @title MultisigBuilder
 * @dev Abstract contract for building arbitrary multisig scripts.
 */
abstract contract MultisigBuilder is ZeusScript {
    bool private hasPranked;

    modifier prank(address caller) {
        _startPrank(caller);
        _;
        _stopPrank();
    }

    /**
     * @notice Constructs a SafeTx object for a Gnosis Safe to ingest. Emits via `ZeusMultisigExecute`
     */
    function execute() public {
        _runAsMultisig();
        require(hasPranked, "MultisigBuilder.execute: did not use prank helpers");
    }

    /**
     * @dev Implement the most high-level call performed by the target multisig for this script.
     * Note: This function should be written as if the target multisig is performing the call directly.
     * Note: This function MUST be written by using the `prank` modifier or `_startPrank`/`_stopPrank`
     * helper methods.
     */
    function _runAsMultisig() internal virtual;

    function _startPrank(address caller) internal {
        require(!hasPranked, "MultisigBuilder._startPrank: called twice in txn");
        hasPranked = true;

        emit ZeusRequireMultisig(caller, Encode.Operation.Call);
        vm.startPrank(caller);
    }

    function _stopPrank() internal {
        require(hasPranked, "MultisigBuilder._stopPrank: _startPrank not called");
        vm.stopPrank();
    }

    /// @dev Only meant for use with tests. Please ensure you know what you are doing if you call this!
    function _unsafeResetHasPranked() internal {
        hasPranked = false;
    }
}
