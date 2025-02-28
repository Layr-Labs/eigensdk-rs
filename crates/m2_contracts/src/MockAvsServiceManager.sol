// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {
    ServiceManagerBase,
    IAVSDirectory,
    IRewardsCoordinator,
    IRegistryCoordinator,
    IStakeRegistry
} from "@eigenlayer-middleware/src/ServiceManagerBase.sol";
import {BLSSignatureChecker} from "@eigenlayer-middleware/src/BLSSignatureChecker.sol";

contract MockAvsServiceManager is ServiceManagerBase, BLSSignatureChecker {
    constructor(
        IAVSDirectory _avsDirectory,
        IRegistryCoordinator _registryCoordinator,
        IStakeRegistry _stakeRegistry,
        address rewards_coordinator
    )
        ServiceManagerBase(_avsDirectory, IRewardsCoordinator(rewards_coordinator), _registryCoordinator, _stakeRegistry)
        BLSSignatureChecker(_registryCoordinator)
    {}

    function initialize(address _initialOwner) external initializer {
        __ServiceManagerBase_init(_initialOwner, _initialOwner);
    }
}
