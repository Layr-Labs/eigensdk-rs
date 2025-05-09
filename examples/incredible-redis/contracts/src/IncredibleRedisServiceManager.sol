// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@eigenlayer/contracts/libraries/BytesLib.sol";
import "./IIncredibleRedisTaskManager.sol";
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
 * @title Primary entrypoint for procuring services from IncredibleRedis.
 * @author Layr Labs, Inc.
 */
contract IncredibleRedisServiceManager is ServiceManagerBase {
    using BytesLib for bytes;

    IIncredibleRedisTaskManager public immutable incredibleRedisTaskManager;

    /// @notice when applied to a function, ensures that the function is only callable by the `registryCoordinator`.
    modifier onlyIncredibleRedisTaskManager() {
        require(
            msg.sender == address(incredibleRedisTaskManager),
            "onlyIncredibleRedisTaskManager: not from credible Dot Product task manager"
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
        IIncredibleRedisTaskManager _incredibleRedisTaskManager
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
        incredibleRedisTaskManager = _incredibleRedisTaskManager;
    }

    function initialize(address initialOwner, address rewardsInitiator) external initializer {
        __ServiceManagerBase_init(initialOwner, rewardsInitiator);
    }
}
