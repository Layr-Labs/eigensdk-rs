// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {ProxyAdmin} from "@m2-openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@m2-openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {UpgradeableBeacon} from "@m2-openzeppelin/contracts/proxy/beacon/UpgradeableBeacon.sol";
import {console2} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";
import {ISlasher} from "@m2-eigenlayer/contracts/interfaces/ISlasher.sol";
// Start of Selection
import {IBeacon} from "@openzeppelin/contracts/proxy/beacon/IBeacon.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {DelegationManager} from "@m2-eigenlayer/contracts/core/DelegationManager.sol";
import {StrategyManager} from "@m2-eigenlayer/contracts/core/StrategyManager.sol";
import {AVSDirectory} from "@m2-eigenlayer/contracts/core/AVSDirectory.sol";
import {EigenPodManager} from "@m2-eigenlayer/contracts/pods/EigenPodManager.sol";
import {RewardsCoordinator} from "@m2-eigenlayer/contracts/core/RewardsCoordinator.sol";
import {StrategyBase} from "@m2-eigenlayer/contracts/strategies/StrategyBase.sol";
import {EigenPod} from "@m2-eigenlayer/contracts/pods/EigenPod.sol";
import {IETHPOSDeposit} from "@m2-eigenlayer/contracts/interfaces/IETHPOSDeposit.sol";
import {StrategyBaseTVLLimits} from "@m2-eigenlayer/contracts/strategies/StrategyBaseTVLLimits.sol";
import {PauserRegistry} from "@m2-eigenlayer/contracts/permissions/PauserRegistry.sol";
import {IStrategy} from "@m2-eigenlayer/contracts/interfaces/IStrategy.sol";
import {IERC20} from "@m2-openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ISignatureUtils} from "@m2-eigenlayer/contracts/interfaces/ISignatureUtils.sol";
import {IDelegationManager} from "@m2-eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {IStrategyManager} from "@m2-eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {IEigenPodManager} from "@m2-eigenlayer/contracts/interfaces/IEigenPodManager.sol";
import {IAVSDirectory} from "@m2-eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {IPauserRegistry} from "@m2-eigenlayer/contracts/interfaces/IPauserRegistry.sol";
import {Strings} from "@m2-openzeppelin/contracts/utils/Strings.sol";
import {StrategyFactory} from "@m2-eigenlayer/contracts/strategies/StrategyFactory.sol";
import {M2UpgradeableProxyLib} from "./M2UpgradeableProxyLib.sol";

library M2CoreDeploymentLib {
    using stdJson for *;
    using Strings for *;
    using M2UpgradeableProxyLib for address;

    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    struct StrategyManagerConfig {
        uint256 initPausedStatus;
        uint256 initWithdrawalDelayBlocks;
    }

    struct SlasherConfig {
        uint256 initPausedStatus;
    }

    struct DelegationManagerConfig {
        uint256 initPausedStatus;
        uint256 withdrawalDelayBlocks;
    }

    struct EigenPodManagerConfig {
        uint256 initPausedStatus;
    }

    struct RewardsCoordinatorConfig {
        uint256 initPausedStatus;
        uint256 maxRewardsDuration;
        uint256 maxRetroactiveLength;
        uint256 maxFutureLength;
        uint256 genesisRewardsTimestamp;
        address updater;
        uint256 activationDelay;
        uint256 calculationIntervalSeconds;
        uint256 globalOperatorCommissionBips;
    }

    struct StrategyFactoryConfig {
        uint256 initPausedStatus;
    }

    struct DeploymentData {
        address delegationManager;
        address avsDirectory;
        address strategyManager;
        address eigenPodManager;
        address rewardsCoordinator;
        address eigenPodBeacon;
        address pauserRegistry;
        address strategyFactory;
        address strategyBeacon;
    }

    function deployContracts(address deployer, address proxyAdmin, DeploymentConfigData memory configData)
        internal
        returns (DeploymentData memory)
    {
        DeploymentData memory result;

        result.delegationManager = M2UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.avsDirectory = M2UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.strategyManager = M2UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.eigenPodManager = M2UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.rewardsCoordinator = M2UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.eigenPodBeacon = M2UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.pauserRegistry = M2UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.strategyFactory = M2UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);

        // Deploy the implementation contracts, using the proxy contracts as inputs
        address delegationManagerImpl = address(
            new DelegationManager(
                IStrategyManager(result.strategyManager), ISlasher(address(0)), IEigenPodManager(result.eigenPodManager)
            )
        );
        address avsDirectoryImpl = address(new AVSDirectory(IDelegationManager(result.delegationManager)));

        address strategyManagerImpl = address(
            new StrategyManager(
                IDelegationManager(result.delegationManager),
                IEigenPodManager(result.eigenPodManager),
                ISlasher(address(0))
            )
        );

        address strategyFactoryImpl = address(new StrategyFactory(IStrategyManager(result.strategyManager)));

        address ethPOSDeposit;
        if (block.chainid == 1) {
            ethPOSDeposit = 0x00000000219ab540356cBB839Cbe05303d7705Fa;
        } else {
            // For non-mainnet chains, you might want to deploy a mock or read from a config
            // This assumes you have a similar config setup as in M2_Deploy_From_Scratch.s.sol
            /// TODO: Handle Eth pos
        }
        IBeacon ibeacon = IBeacon(result.eigenPodBeacon);
        address eigenPodManagerImpl = address(
            new EigenPodManager(
                IETHPOSDeposit(ethPOSDeposit),
                IBeacon(result.eigenPodBeacon),
                IStrategyManager(result.strategyManager),
                ISlasher(address(0)),
                IDelegationManager(result.delegationManager)
            )
        );

        /// TODO: Get actual values
        uint32 CALCULATION_INTERVAL_SECONDS = 1 days;
        uint32 MAX_REWARDS_DURATION = 1 days;
        uint32 MAX_RETROACTIVE_LENGTH = 3 days;
        uint32 MAX_FUTURE_LENGTH = 1 days;
        uint32 GENESIS_REWARDS_TIMESTAMP = 10 days;
        address rewardsCoordinatorImpl = address(
            new RewardsCoordinator(
                IDelegationManager(result.delegationManager),
                IStrategyManager(result.strategyManager),
                CALCULATION_INTERVAL_SECONDS,
                MAX_REWARDS_DURATION,
                MAX_RETROACTIVE_LENGTH,
                MAX_FUTURE_LENGTH,
                GENESIS_REWARDS_TIMESTAMP
            )
        );

        uint64 GENESIS_TIME = 1_564_000;

        address eigenPodImpl =
            address(new EigenPod(IETHPOSDeposit(ethPOSDeposit), IEigenPodManager(result.eigenPodManager), GENESIS_TIME));
        address eigenPodBeaconImpl = address(new UpgradeableBeacon(eigenPodImpl));
        address baseStrategyImpl = address(new StrategyBase(IStrategyManager(result.strategyManager)));
        /// TODO: PauserRegistry isn't upgradeable
        address pauserRegistryImpl = address(
            new PauserRegistry(
                new address[](0), // Empty array for pausers
                proxyAdmin // ProxyAdmin as the unpauser
            )
        );

        // Deploy and configure the strategy beacon
        result.strategyBeacon = address(new UpgradeableBeacon(baseStrategyImpl));

        // Upgrade contracts
        /// TODO: Get from config
        bytes memory upgradeCall = abi.encodeCall(
            DelegationManager.initialize,
            (
                proxyAdmin, // initialOwner
                IPauserRegistry(result.pauserRegistry), // _pauserRegistry
                configData.delegationManager.initPausedStatus, // initialPausedStatus
                configData.delegationManager.withdrawalDelayBlocks, // _minWithdrawalDelayBlocks
                new IStrategy[](0), // _strategies (empty array for now)
                new uint256[](0) // _withdrawalDelayBlocks (empty array for now)
            )
        );
        M2UpgradeableProxyLib.upgradeAndCall(result.delegationManager, delegationManagerImpl, upgradeCall);

        // Upgrade StrategyManager contract
        upgradeCall = abi.encodeCall(
            StrategyManager.initialize,
            (
                proxyAdmin, // initialOwner
                result.strategyFactory, // initialStrategyWhitelister
                IPauserRegistry(result.pauserRegistry), // _pauserRegistry
                configData.strategyManager.initPausedStatus // initialPausedStatus
            )
        );
        M2UpgradeableProxyLib.upgradeAndCall(result.strategyManager, strategyManagerImpl, upgradeCall);

        // Upgrade StrategyFactory contract
        upgradeCall = abi.encodeCall(
            StrategyFactory.initialize,
            (
                proxyAdmin, // initialOwner
                IPauserRegistry(result.pauserRegistry), // _pauserRegistry
                configData.strategyFactory.initPausedStatus, // initialPausedStatus
                IBeacon(result.strategyBeacon)
            )
        );
        M2UpgradeableProxyLib.upgradeAndCall(result.strategyFactory, strategyFactoryImpl, upgradeCall);

        // Upgrade EigenPodManager contract
        upgradeCall = abi.encodeCall(
            EigenPodManager.initialize,
            (
                proxyAdmin, // initialOwner
                IPauserRegistry(result.pauserRegistry), // _pauserRegistry
                configData.eigenPodManager.initPausedStatus // initialPausedStatus
            )
        );
        M2UpgradeableProxyLib.upgradeAndCall(result.eigenPodManager, eigenPodManagerImpl, upgradeCall);

        // Upgrade AVSDirectory contract
        upgradeCall = abi.encodeCall(
            AVSDirectory.initialize,
            (
                proxyAdmin, // initialOwner
                IPauserRegistry(result.pauserRegistry), // _pauserRegistry
                0 // TODO: AVS Missing configinitialPausedStatus
            )
        );
        M2UpgradeableProxyLib.upgradeAndCall(result.avsDirectory, avsDirectoryImpl, upgradeCall);

        // Upgrade RewardsCoordinator contract
        upgradeCall = abi.encodeCall(
            RewardsCoordinator.initialize,
            (
                deployer, // initialOwner
                IPauserRegistry(result.pauserRegistry), // _pauserRegistry
                configData.rewardsCoordinator.initPausedStatus, // initialPausedStatus
                /// TODO: is there a setter and is this expected?
                deployer, // rewards updater
                uint32(configData.rewardsCoordinator.activationDelay), // _activationDelay
                uint16(configData.rewardsCoordinator.globalOperatorCommissionBips) // _globalCommissionBips
            )
        );
        M2UpgradeableProxyLib.upgradeAndCall(result.rewardsCoordinator, rewardsCoordinatorImpl, upgradeCall);

        // Upgrade EigenPod contract
        upgradeCall = abi.encodeCall(
            EigenPod.initialize,
            // TODO: Double check this
            (address(result.eigenPodManager)) // _podOwner
        );
        M2UpgradeableProxyLib.upgradeAndCall(result.eigenPodBeacon, eigenPodImpl, upgradeCall);

        return result;
    }

    struct DeploymentConfigData {
        StrategyManagerConfig strategyManager;
        DelegationManagerConfig delegationManager;
        SlasherConfig slasher;
        EigenPodManagerConfig eigenPodManager;
        RewardsCoordinatorConfig rewardsCoordinator;
        StrategyFactoryConfig strategyFactory;
    }
    // StrategyConfig[] strategies;

    function readDeploymentConfigValues(string memory directoryPath, string memory fileName)
        internal
        returns (DeploymentConfigData memory)
    {
        string memory pathToFile = string.concat(directoryPath, fileName);

        require(vm.exists(pathToFile), "Deployment file does not exist");

        string memory json = vm.readFile(pathToFile);

        DeploymentConfigData memory data;

        // StrategyManager start
        data.strategyManager.initPausedStatus = json.readUint(".strategyManager.init_paused_status");
        data.strategyManager.initWithdrawalDelayBlocks =
            uint32(json.readUint(".strategyManager.init_withdrawal_delay_blocks"));
        // slasher config end

        // DelegationManager config start
        data.delegationManager.initPausedStatus = json.readUint(".delegation.init_paused_status");
        data.delegationManager.withdrawalDelayBlocks = json.readUint(".delegation.init_withdrawal_delay_blocks");
        // DelegationManager config end

        // Slasher config start
        data.slasher.initPausedStatus = json.readUint(".slasher.init_paused_status");

        // Slasher config end

        // EigenPodManager config start
        data.eigenPodManager.initPausedStatus = json.readUint(".eigenPodManager.init_paused_status");
        // EigenPodManager config end

        // RewardsCoordinator config start
        data.rewardsCoordinator.initPausedStatus = json.readUint(".rewardsCoordinator.init_paused_status");
        data.rewardsCoordinator.maxRewardsDuration = json.readUint(".rewardsCoordinator.MAX_REWARDS_DURATION");
        data.rewardsCoordinator.maxRetroactiveLength = json.readUint(".rewardsCoordinator.MAX_RETROACTIVE_LENGTH");
        data.rewardsCoordinator.maxFutureLength = json.readUint(".rewardsCoordinator.MAX_FUTURE_LENGTH");
        data.rewardsCoordinator.genesisRewardsTimestamp = json.readUint(".rewardsCoordinator.GENESIS_REWARDS_TIMESTAMP");
        data.rewardsCoordinator.updater = json.readAddress(".rewardsCoordinator.rewards_updater_address");
        data.rewardsCoordinator.activationDelay = json.readUint(".rewardsCoordinator.activation_delay");
        data.rewardsCoordinator.calculationIntervalSeconds =
            json.readUint(".rewardsCoordinator.calculation_interval_seconds");
        data.rewardsCoordinator.globalOperatorCommissionBips =
            json.readUint(".rewardsCoordinator.global_operator_commission_bips");
        // RewardsCoordinator config end

        return data;
    }

    function readDeploymentConfigValues(string memory directoryPath, uint256 chainId)
        internal
        returns (DeploymentConfigData memory)
    {
        return readDeploymentConfigValues(directoryPath, string.concat(vm.toString(chainId), ".json"));
    }

    function readDeploymentJson(string memory directoryPath, uint256 chainId) public returns (DeploymentData memory) {
        return readDeploymentJson(directoryPath, string.concat(vm.toString(chainId), ".json"));
    }

    function readDeploymentJson(string memory path, string memory fileName) internal returns (DeploymentData memory) {
        string memory pathToFile = string.concat(path, fileName);

        require(vm.exists(pathToFile), "Deployment file does not exist");

        string memory json = vm.readFile(pathToFile);

        DeploymentData memory data;
        data.strategyFactory = json.readAddress(".addresses.strategyFactory");
        data.strategyManager = json.readAddress(".addresses.strategyManager");
        data.eigenPodManager = json.readAddress(".addresses.eigenPodManager");
        data.delegationManager = json.readAddress(".addresses.delegation");
        data.avsDirectory = json.readAddress(".addresses.avsDirectory");
        data.rewardsCoordinator = json.readAddress(".addresses.rewardsCoordinator");

        return data;
    }

    /// TODO: Need to be able to read json from eigenlayer-contracts repo for holesky/mainnet and output the json here
    function writeDeploymentJson(DeploymentData memory data) internal {
        writeDeploymentJson("deployments/core/m2/", block.chainid, data);
    }

    function writeDeploymentJson(string memory path, uint256 chainId, DeploymentData memory data) internal {
        address proxyAdmin = address(M2UpgradeableProxyLib.getProxyAdmin(data.strategyManager));

        string memory deploymentData = _generateDeploymentJson(data, proxyAdmin);

        string memory fileName = string.concat(path, vm.toString(chainId), ".json");
        if (!vm.exists(path)) {
            vm.createDir(path, true);
        }

        vm.writeFile(fileName, deploymentData);
        console2.log("Deployment artifacts written to:", fileName);
    }

    function _generateDeploymentJson(DeploymentData memory data, address proxyAdmin)
        private
        view
        returns (string memory)
    {
        return string.concat(
            '{"lastUpdate":{"timestamp":"',
            vm.toString(block.timestamp),
            '","block_number":"',
            vm.toString(block.number),
            '"},"addresses":',
            _generateContractsJson(data, proxyAdmin),
            "}"
        );
    }

    function _generateContractsJson(DeploymentData memory data, address proxyAdmin)
        private
        view
        returns (string memory)
    {
        /// TODO: namespace contracts -> {avs, core}
        return string.concat(
            '{"proxyAdmin":"',
            proxyAdmin.toHexString(),
            '","delegation":"',
            data.delegationManager.toHexString(),
            '","delegationManagerImpl":"',
            data.delegationManager.getImplementation().toHexString(),
            '","avsDirectory":"',
            data.avsDirectory.toHexString(),
            '","avsDirectoryImpl":"',
            data.avsDirectory.getImplementation().toHexString(),
            '","strategyManager":"',
            data.strategyManager.toHexString(),
            '","strategyManagerImpl":"',
            data.strategyManager.getImplementation().toHexString(),
            '","eigenPodManager":"',
            data.eigenPodManager.toHexString(),
            '","eigenPodManagerImpl":"',
            data.eigenPodManager.getImplementation().toHexString(),
            '","strategyFactory":"',
            data.strategyFactory.toHexString(),
            '","rewardsCoordinator":"',
            data.rewardsCoordinator.toHexString(),
            '","strategyBeacon":"',
            data.strategyBeacon.toHexString(),
            '"}'
        );
    }
}
