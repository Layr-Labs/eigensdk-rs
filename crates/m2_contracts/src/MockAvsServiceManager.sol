// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {IAVSDirectory} from "@eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {IBLSSignatureChecker} from "@eigenlayer-middleware/src/interfaces/IBLSSignatureChecker.sol";
import {IRegistryCoordinator} from "@eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";
import {ServiceManagerBase} from "@eigenlayer-middleware/src/ServiceManagerBase.sol";
import {BLSSignatureChecker} from "@eigenlayer-middleware/src/BLSSignatureChecker.sol";
import {IStakeRegistry} from "@eigenlayer-middleware/src/StakeRegistry.sol";
import {ISlashingRegistryCoordinator} from "@eigenlayer-middleware/src/interfaces/ISlashingRegistryCoordinator.sol";
import {IPermissionController} from "@eigenlayer/contracts/permissions/PermissionController.sol";
import {IAllocationManager} from "@eigenlayer/contracts/interfaces/IAllocationManager.sol";

contract MockAvsServiceManager is ServiceManagerBase {
    constructor(
        IAVSDirectory _avsDirectory,
        ISlashingRegistryCoordinator _slashingRegCoordinator,
        IStakeRegistry _stakeRegistry,
        address rewards_coordinator,
        IPermissionController _permissionController,
        IAllocationManager _allocationManager
    )
        ServiceManagerBase(
            _avsDirectory,
            IRewardsCoordinator(rewards_coordinator),
            _slashingRegCoordinator,
            _stakeRegistry,
            _permissionController,
            _allocationManager
        )
    {}

    function initialize(address _initialOwner) external initializer {
        // TODO: setting _rewardsInitializer to be _initialOwner for now.
        // TODO: setting _slasher to be _initialOwner for now.
        __ServiceManagerBase_init(_initialOwner, _initialOwner);
    }
}
