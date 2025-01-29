// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {IAVSDirectory} from "@m2-eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {IRewardsCoordinator} from "@m2-eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {IBLSSignatureChecker} from "@m2-eigenlayer-middleware/src/interfaces/IBLSSignatureChecker.sol";
import {IRegistryCoordinator} from "@m2-eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";
import {ServiceManagerBase} from "@m2-eigenlayer-middleware/src/ServiceManagerBase.sol";
import {BLSSignatureChecker} from "@m2-eigenlayer-middleware/src/BLSSignatureChecker.sol";
import {IStakeRegistry} from "@m2-eigenlayer-middleware/src/StakeRegistry.sol";

contract MockAvsServiceManager is ServiceManagerBase {
    constructor(
        IAVSDirectory _avsDirectory,
        IRegistryCoordinator _registryCoordinator,
        IStakeRegistry _stakeRegistry,
        address rewards_coordinator
    )
        ServiceManagerBase(_avsDirectory, IRewardsCoordinator(rewards_coordinator), _registryCoordinator, _stakeRegistry)
    {
    }

    function initialize(address _initialOwner) external initializer {
        // TODO: setting _rewardsInitializer to be _initialOwner for now.
        // TODO: setting _slasher to be _initialOwner for now.
        __ServiceManagerBase_init(_initialOwner, _initialOwner);
    }
}
