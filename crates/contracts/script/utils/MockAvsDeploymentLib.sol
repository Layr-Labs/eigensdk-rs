// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Script} from "forge-std/Script.sol";
import {Vm} from "forge-std/Vm.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {IAVSDirectory} from "@eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {MockAvsServiceManager} from "../../src/MockAvsServiceManager.sol";
import {IDelegationManager} from "@eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {IRewardsCoordinator} from "@eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {Quorum} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";
import {UpgradeableProxyLib} from "./UpgradeableProxyLib.sol";
import {CoreDeploymentLib} from "./CoreDeploymentLib.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {BLSApkRegistry} from "@eigenlayer-middleware/src/BLSApkRegistry.sol";
import {IndexRegistry} from "@eigenlayer-middleware/src/IndexRegistry.sol";
import {StakeRegistry} from "@eigenlayer-middleware/src/StakeRegistry.sol";
import {IRegistryCoordinator} from "@eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";
import {IServiceManager} from "@eigenlayer-middleware/src/interfaces/IServiceManager.sol";
import {IStrategy} from "@eigenlayer/contracts/interfaces/IStrategyManager.sol";
import {IAllocationManager} from "@eigenlayer/contracts/interfaces/IAllocationManager.sol";

import {RegistryCoordinator, IBLSApkRegistry, IIndexRegistry} from "@eigenlayer-middleware/src/RegistryCoordinator.sol";
import {IStakeRegistry, StakeType} from "@eigenlayer-middleware/src/interfaces/IStakeRegistry.sol";
import {IAVSDirectory} from "@eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {PauserRegistry, IPauserRegistry} from "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import {OperatorStateRetriever} from "@eigenlayer-middleware/src/OperatorStateRetriever.sol";
import {Vm} from "forge-std/Vm.sol";
import {IPauserRegistry} from "@eigenlayer/contracts/interfaces/IPauserRegistry.sol";

library MockAvsDeploymentLib {
    using stdJson for *;
    using Strings for *;
    using UpgradeableProxyLib for address;

    Vm internal constant VM =
        Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    struct DeploymentData {
        address mockAvsServiceManager;
        address registryCoordinator;
        address operatorStateRetriever;
        address blsapkRegistry;
        address indexRegistry;
        address stakeRegistry;
        address strategy;
        address token;
        address tokenRewards;
    }

    struct MockAvsSetupConfig {
        uint256 numQuorums;
        uint256[] operatorParams;
        address operatorAddr;
        address contractsRegistryAddr;
    }

    function deployContracts(
        address proxyAdmin,
        CoreDeploymentLib.DeploymentData memory core,
        address strategy,
        MockAvsSetupConfig memory isConfig,
        address admin
    ) internal returns (DeploymentData memory) {
        /// read EL deployment address
        CoreDeploymentLib.DeploymentData
            memory coredata = readCoreDeploymentJson(
                "script/deployments/core/",
                block.chainid
            );

        DeploymentData memory result;

        // First, deploy upgradeable proxy contracts that will point to the implementations.
        result.mockAvsServiceManager = UpgradeableProxyLib.setUpEmptyProxy(
            proxyAdmin
        );
        result.stakeRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.registryCoordinator = UpgradeableProxyLib.setUpEmptyProxy(
            proxyAdmin
        );
        result.blsapkRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.indexRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        OperatorStateRetriever operatorStateRetriever = new OperatorStateRetriever();
        result.strategy = strategy;
        result.operatorStateRetriever = address(operatorStateRetriever);
        // Deploy the implementation contracts, using the proxy contracts as inputs
        address stakeRegistryImpl = address(
            new StakeRegistry(
                IRegistryCoordinator(result.registryCoordinator),
                IDelegationManager(core.delegationManager),
                IAVSDirectory(core.avsDirectory),
                MockAvsServiceManager(result.mockAvsServiceManager)
            )
        );

        address blsApkRegistryImpl = address(
            new BLSApkRegistry(IRegistryCoordinator(result.registryCoordinator))
        );
        address indexRegistryimpl = address(
            new IndexRegistry(IRegistryCoordinator(result.registryCoordinator))
        );
        address registryCoordinatorImpl = address(
            new RegistryCoordinator(
                MockAvsServiceManager(result.mockAvsServiceManager),
                IStakeRegistry(result.stakeRegistry),
                IBLSApkRegistry(result.blsapkRegistry),
                IIndexRegistry(result.indexRegistry),
                IAVSDirectory(core.avsDirectory),
                IPauserRegistry(core.pauserRegistry)
            )
        );

        address[] memory pausers = new address[](2);
        pausers[0] = admin;
        pausers[1] = admin;

        IStrategy[1] memory deployedStrategyArray = [IStrategy(strategy)];
        uint256 numStrategies = deployedStrategyArray.length;

        uint256 numQuorums = isConfig.numQuorums;
        IRegistryCoordinator.OperatorSetParam[]
            memory quorumsOperatorSetParams = new IRegistryCoordinator.OperatorSetParam[](
                numQuorums
            );
        uint256[] memory operatorParams = isConfig.operatorParams;

        for (uint256 i = 0; i < numQuorums; i++) {
            quorumsOperatorSetParams[i] = IRegistryCoordinator
                .OperatorSetParam({
                    maxOperatorCount: uint32(operatorParams[i]),
                    kickBIPsOfOperatorStake: uint16(operatorParams[i + 1]),
                    kickBIPsOfTotalStake: uint16(operatorParams[i + 2])
                });
        }
        // set to 0 for every quorum
        uint96[] memory quorumsMinimumStake = new uint96[](numQuorums);
        IStakeRegistry.StrategyParams[][]
            memory quorumsStrategyParams = new IStakeRegistry.StrategyParams[][](
                numQuorums
            );
        StakeType[] memory stakeTypes = new StakeType[](numQuorums);
        uint32[] memory lookAheadPeriods = new uint32[](numQuorums);
        for (uint256 i = 0; i < numQuorums; i++) {
            quorumsStrategyParams[i] = new IStakeRegistry.StrategyParams[](
                numStrategies
            );
            for (uint256 j = 0; j < numStrategies; j++) {
                quorumsStrategyParams[i][j] = IStakeRegistry.StrategyParams({
                    strategy: deployedStrategyArray[j],
                    // setting this to 1 ether since the divisor is also 1 ether
                    // therefore this allows an operator to register with even just 1 token
                    // see https://github.com/Layr-Labs/eigenlayer-middleware/blob/m2-mainnet/src/StakeRegistry.sol#L484
                    //    weight += uint96(sharesAmount * strategyAndMultiplier.multiplier / WEIGHTING_DIVISOR);
                    multiplier: 1 ether
                });
                stakeTypes[i] = StakeType.TOTAL_SLASHABLE;
                lookAheadPeriods[i] = 1;
            }
        }

        bytes memory upgradeCall = abi.encodeCall(
            RegistryCoordinator.initialize,
            (
                admin,
                admin,
                admin,
                0,
                quorumsOperatorSetParams,
                quorumsMinimumStake,
                quorumsStrategyParams,
                stakeTypes,
                lookAheadPeriods
            )
        );

        UpgradeableProxyLib.upgrade(result.stakeRegistry, stakeRegistryImpl);
        UpgradeableProxyLib.upgrade(result.blsapkRegistry, blsApkRegistryImpl);
        UpgradeableProxyLib.upgrade(result.indexRegistry, indexRegistryimpl);
        UpgradeableProxyLib.upgradeAndCall(
            result.registryCoordinator,
            registryCoordinatorImpl,
            upgradeCall
        );
        MockAvsServiceManager mockAvsServiceManagerImpl = new MockAvsServiceManager(
                IRegistryCoordinator(result.registryCoordinator),
                IAVSDirectory(coredata.avsDirectory),
                IRewardsCoordinator(coredata.rewardsCoordinator),
                IAllocationManager(coredata.allocationManager)
            );
        bytes memory mockavsmanagerupgradecall = abi.encodeCall(
            MockAvsServiceManager.initialize,
            admin
        );
        UpgradeableProxyLib.upgradeAndCall(
            result.mockAvsServiceManager,
            address(mockAvsServiceManagerImpl),
            mockavsmanagerupgradecall
        );

        verify_deployment(result);

        return result;
    }

    function readDeploymentJson(
        uint256 chainId
    ) internal returns (DeploymentData memory) {
        return readDeploymentJson("script/deployments/mock-avs/", chainId);
    }

    function readMockAvsConfigJson(
        string memory directoryPath
    ) internal returns (MockAvsSetupConfig memory) {
        string memory fileName = string.concat(directoryPath, ".json");
        require(VM.exists(fileName), "Deployment file does not exist");
        string memory json = VM.readFile(fileName);

        MockAvsSetupConfig memory data;
        data.contractsRegistryAddr = json.readAddress(
            ".contracts_registry_addr"
        );
        data.operatorAddr = json.readAddress(".operatorAddr");
        data.numQuorums = json.readUint(".num_quorums");
        data.operatorParams = json.readUintArray(".operatorParams");
        return data;
    }

    function readDeploymentJson(
        string memory directoryPath,
        uint256 chainId
    ) internal returns (DeploymentData memory) {
        string memory fileName = string.concat(
            directoryPath,
            VM.toString(chainId),
            ".json"
        );

        require(VM.exists(fileName), "Deployment file does not exist");

        string memory json = VM.readFile(fileName);

        DeploymentData memory data;
        data.mockAvsServiceManager = json.readAddress(
            ".addresses.MockAvsServiceManager"
        );
        data.registryCoordinator = json.readAddress(
            ".addresses.registryCoordinator"
        );
        data.operatorStateRetriever = json.readAddress(
            ".addresses.operatorStateRetriever"
        );
        data.stakeRegistry = json.readAddress(".addresses.stakeRegistry");
        data.strategy = json.readAddress(".addresses.strategy");
        data.token = json.readAddress(".addresses.token");

        return data;
    }

    /// write to default output path
    function writeDeploymentJson(DeploymentData memory data) internal {
        writeDeploymentJson(
            "script/deployments/mock-avs/",
            block.chainid,
            data
        );
    }

    function writeDeploymentJson(
        string memory outputPath,
        uint256 chainId,
        DeploymentData memory data
    ) internal {
        address proxyAdmin = address(
            UpgradeableProxyLib.getProxyAdmin(data.mockAvsServiceManager)
        );

        string memory deploymentData = _generateDeploymentJson(
            data,
            proxyAdmin
        );

        string memory fileName = string.concat(
            outputPath,
            VM.toString(chainId),
            ".json"
        );
        if (!VM.exists(outputPath)) {
            VM.createDir(outputPath, true);
        }

        VM.writeFile(fileName, deploymentData);
    }

    function _generateDeploymentJson(
        DeploymentData memory data,
        address proxyAdmin
    ) private view returns (string memory) {
        return
            string.concat(
                '{"lastUpdate":{"timestamp":"',
                VM.toString(block.timestamp),
                '","block_number":"',
                VM.toString(block.number),
                '"},"addresses":',
                _generateContractsJson(data, proxyAdmin),
                "}"
            );
    }

    function _generateContractsJson(
        DeploymentData memory data,
        address proxyAdmin
    ) private view returns (string memory) {
        return
            string.concat(
                '{"proxyAdmin":"',
                proxyAdmin.toHexString(),
                '","MockAvsServiceManager":"',
                data.mockAvsServiceManager.toHexString(),
                '","MockAvsServiceManagerImpl":"',
                data.mockAvsServiceManager.getImplementation().toHexString(),
                '","registryCoordinator":"',
                data.registryCoordinator.toHexString(),
                '","blsapkRegistry":"',
                data.blsapkRegistry.toHexString(),
                '","indexRegistry":"',
                data.indexRegistry.toHexString(),
                '","stakeRegistry":"',
                data.stakeRegistry.toHexString(),
                '","operatorStateRetriever":"',
                data.operatorStateRetriever.toHexString(),
                '","strategy":"',
                data.strategy.toHexString(),
                '","token":"',
                data.token.toHexString(),
                '","tokenRewards":"',
                data.tokenRewards.toHexString(),
                '"}'
            );
    }

    function readCoreDeploymentJson(
        string memory directoryPath,
        uint256 chainId
    ) internal returns (CoreDeploymentLib.DeploymentData memory) {
        return
            readCoreDeploymentJson(
                directoryPath,
                string.concat(VM.toString(chainId), ".json")
            );
    }

    function readCoreDeploymentJson(
        string memory path,
        string memory fileName
    ) internal returns (CoreDeploymentLib.DeploymentData memory) {
        string memory pathToFile = string.concat(path, fileName);

        require(VM.exists(pathToFile), "Deployment file does not exist");

        string memory json = VM.readFile(pathToFile);

        CoreDeploymentLib.DeploymentData memory data;
        data.strategyFactory = json.readAddress(".addresses.strategyFactory");
        data.strategyManager = json.readAddress(".addresses.strategyManager");
        data.eigenPodManager = json.readAddress(".addresses.eigenPodManager");
        data.delegationManager = json.readAddress(".addresses.delegation");
        data.avsDirectory = json.readAddress(".addresses.avsDirectory");
        data.allocationManager = json.readAddress(".addresses.allocationManager");

        return data;
    }

    function verify_deployment(DeploymentData memory result) internal view {
        IBLSApkRegistry blsapkregistry = IRegistryCoordinator(
            result.registryCoordinator
        ).blsApkRegistry();
        require(address(blsapkregistry) != address(0));
        IStakeRegistry stakeregistry = IRegistryCoordinator(
            result.registryCoordinator
        ).stakeRegistry();
        require(address(stakeregistry) != address(0));
        IDelegationManager delegationmanager = IStakeRegistry(
            address(stakeregistry)
        ).delegation();
        require(address(delegationmanager) != address(0));
    }
}
