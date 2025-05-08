// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@eigenlayer/contracts/libraries/BytesLib.sol";
import "./IIncredibleDotProductTaskManager.sol";
import "@eigenlayer-middleware/src/ServiceManagerBase.sol";
import {
    IAllocationManager,
    IAllocationManagerTypes
} from "@eigenlayer/contracts/interfaces/IAllocationManager.sol";
// import {IAVSRegistrar} from "@eigenlayer/contracts/interfaces/IAVSRegistrar.sol";
import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {ISlashingRegistryCoordinator} from
    "@eigenlayer-middleware/src/interfaces/ISlashingRegistryCoordinator.sol";

/**
 * @title Primary entrypoint for procuring services from IncredibleDotProduct.
 * @author Layr Labs, Inc.
 */
contract IncredibleDotProductServiceManager is ServiceManagerBase {
    using BytesLib for bytes;

    IIncredibleDotProductTaskManager public immutable incredibleDotProductTaskManager;

    /// @notice when applied to a function, ensures that the function is only callable by the `registryCoordinator`.
    modifier onlyIncredibleDotProductTaskManager() {
        require(
            msg.sender == address(incredibleDotProductTaskManager),
            "onlyIncredibleDotProductTaskManager: not from credible Dot Product task manager"
        );
        _;
    }

    constructor(
        IAVSDirectory _avsDirectory,
        ISlashingRegistryCoordinator _registryCoordinator,
        IStakeRegistry _stakeRegistry,
        address rewards_coordinator,
        IAllocationManager allocationManager,
        IPermissionController _permissionController,
        IIncredibleDotProductTaskManager _incredibleDotProductTaskManager
    )
        ServiceManagerBase(
            _avsDirectory,
            IRewardsCoordinator(rewards_coordinator),
            _registryCoordinator,
            _stakeRegistry,
            _permissionController,
            allocationManager
        )
    {
        incredibleDotProductTaskManager = _incredibleDotProductTaskManager;
    }

    function initialize(address initialOwner, address rewardsInitiator) external initializer {
        __ServiceManagerBase_init(initialOwner, rewardsInitiator);
    }
}
