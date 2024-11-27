// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {IAVSDirectory} from "eigenlayer-contracts/src/contracts/interfaces/IAVSDirectory.sol";
import {IRewardsCoordinator} from "eigenlayer-contracts/src/contracts/interfaces/IRewardsCoordinator.sol";

import {IRegistryCoordinator} from "eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";
import {IBLSSignatureChecker} from "eigenlayer-middleware/src/interfaces/IBLSSignatureChecker.sol";
import {ServiceManagerBase} from "eigenlayer-middleware/src/ServiceManagerBase.sol";
import {BLSSignatureChecker} from "eigenlayer-middleware/src/BLSSignatureChecker.sol";

contract MockAvsServiceManager is ServiceManagerBase, BLSSignatureChecker {
    constructor(
        IRegistryCoordinator _registryCoordinator,
        IAVSDirectory _avsDirectory,
        IRewardsCoordinator _rewardsCoordinator
    )
        ServiceManagerBase(
            _avsDirectory,
            _rewardsCoordinator,
            _registryCoordinator,
            _registryCoordinator.stakeRegistry()
        )
        BLSSignatureChecker(_registryCoordinator)
    {}

    function initialize(address _initialOwner) external initializer {
        // TODO: setting _rewardsInitializer to be _initialOwner for now.
        __ServiceManagerBase_init(_initialOwner, _initialOwner);
    }
}
