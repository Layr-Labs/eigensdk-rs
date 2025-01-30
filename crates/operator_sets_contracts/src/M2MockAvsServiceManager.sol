// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {
    ServiceManagerBase,
    IAVSDirectory,
    IRewardsCoordinator,
    IRegistryCoordinator,IStakeRegistry
} from "@m2-eigenlayer-middleware/src/ServiceManagerBase.sol";

contract M2MockAvsServiceManager is ServiceManagerBase {
    constructor(
        IAVSDirectory _avsDirectory,
        IRegistryCoordinator _registryCoordinator,
        IStakeRegistry _stakeRegistry,
        address rewards_coordinator
    )
        ServiceManagerBase(_avsDirectory, IRewardsCoordinator(rewards_coordinator), _registryCoordinator, _stakeRegistry)
    {}

    function initialize(address _initialOwner) external initializer {
        __ServiceManagerBase_init(_initialOwner, _initialOwner);
    }
}
