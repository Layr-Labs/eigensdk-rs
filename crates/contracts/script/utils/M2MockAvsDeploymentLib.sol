// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {ProxyAdmin} from "@m2-openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@m2-openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {Script} from "forge-std/Script.sol";
import {Vm} from "forge-std/Vm.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {console2} from "forge-std/console2.sol";
import {IAVSDirectory} from "@m2-eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {MockAvsServiceManager} from "../../src/MockAvsServiceManager.sol";
import {IDelegationManager} from "@m2-eigenlayer/contracts/interfaces/IDelegationManager.sol";
import {IRewardsCoordinator} from "@m2-eigenlayer/contracts/interfaces/IRewardsCoordinator.sol";
import {Quorum} from "@m2-eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";
import {UpgradeableProxyLib} from "./UpgradeableProxyLib.sol";
import {M2CoreDeploymentLib} from "./M2CoreDeploymentLib.sol";
import {Strings} from "@m2-openzeppelin/contracts/utils/Strings.sol";
import {BLSApkRegistry} from "@m2-eigenlayer-middleware/src/BLSApkRegistry.sol";
import {IndexRegistry} from "@m2-eigenlayer-middleware/src/IndexRegistry.sol";
import {SocketRegistry,ISocketRegistry} from "@m2-eigenlayer-middleware/src/SocketRegistry.sol";
import {StakeRegistry} from "@m2-eigenlayer-middleware/src/StakeRegistry.sol";
import {IRegistryCoordinator} from "@m2-eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";
import {IServiceManager} from "@m2-eigenlayer-middleware/src/interfaces/IServiceManager.sol";
import {IStrategy} from "@m2-eigenlayer/contracts/interfaces/IStrategyManager.sol";

import {
    RegistryCoordinator, IBLSApkRegistry, IIndexRegistry
} from "@m2-eigenlayer-middleware/src/RegistryCoordinator.sol";
import {IStakeRegistry} from "@m2-eigenlayer-middleware/src/interfaces/IStakeRegistry.sol";
import {IAVSDirectory} from "@m2-eigenlayer/contracts/interfaces/IAVSDirectory.sol";
import {PauserRegistry, IPauserRegistry} from "@m2-eigenlayer/contracts/permissions/PauserRegistry.sol";
import {OperatorStateRetriever} from "@m2-eigenlayer-middleware/src/OperatorStateRetriever.sol";
import {Vm} from "forge-std/Vm.sol";
import {IPauserRegistry} from "@m2-eigenlayer/contracts/interfaces/IPauserRegistry.sol";

library M2MockAvsDeploymentLib {
   using stdJson for *;
    using Strings for *;
    using UpgradeableProxyLib for address;

    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    struct DeploymentData {
        address mockAvsServiceManager;
        address registryCoordinator;
        address operatorStateRetriever;
        address blsapkRegistry;
        address indexRegistry;
        address stakeRegistry;
        address socketRegistry;
        address strategy;
        address token;
    }

    struct MockAvsSetupConfig {
        uint256 numQuorums;
        uint256[] operatorParams;
        address operator_addr;
        address contracts_registry_addr;
    }

    function deployContracts(
        address proxyAdmin,
        M2CoreDeploymentLib.DeploymentData memory core,
        address strategy,
        MockAvsSetupConfig memory isConfig,
        address admin
    ) internal returns (DeploymentData memory) {
        /// read EL deployment address
        M2CoreDeploymentLib.DeploymentData memory coredata =
            readCoreDeploymentJson("script/deployments/core/", block.chainid);
        address avsdirectory = coredata.avsDirectory;

        DeploymentData memory result;

        // First, deploy upgradeable proxy contracts that will point to the implementations.
        result.mockAvsServiceManager = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.stakeRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.registryCoordinator = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.blsapkRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.indexRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.strategy = strategy;
        result.socketRegistry = UpgradeableProxyLib.setUpEmptyProxy(proxyAdmin);
        result.operatorStateRetriever = address(new OperatorStateRetriever());
        // Deploy the implementation contracts, using the proxy contracts as inputs
        address stakeRegistryImpl = address(
            new StakeRegistry(
                IRegistryCoordinator(result.registryCoordinator), IDelegationManager(core.delegationManager)
            )
        );
        UpgradeableProxyLib.upgrade(result.stakeRegistry, stakeRegistryImpl);

        address blsApkRegistryImpl = address(new BLSApkRegistry(IRegistryCoordinator(result.registryCoordinator)));
        UpgradeableProxyLib.upgrade(result.blsapkRegistry, blsApkRegistryImpl);

        address indexRegistryimpl = address(new IndexRegistry(IRegistryCoordinator(result.registryCoordinator)));
        UpgradeableProxyLib.upgrade(result.indexRegistry, indexRegistryimpl);

        address[] memory pausers = new address[](2);
        pausers[0] = admin;
        pausers[1] = admin;
        PauserRegistry pausercontract = new PauserRegistry(pausers, admin);

        IStrategy[1] memory deployedStrategyArray = [IStrategy(strategy)];
        uint256 numStrategies = deployedStrategyArray.length;

        uint256 numQuorums = isConfig.numQuorums;
        IRegistryCoordinator.OperatorSetParam[] memory quorumsOperatorSetParams =
            new IRegistryCoordinator.OperatorSetParam[](numQuorums);
        uint256[] memory operator_params = isConfig.operatorParams;

        for (uint256 i = 0; i < numQuorums; i++) {
            quorumsOperatorSetParams[i] = IRegistryCoordinator.OperatorSetParam({
                maxOperatorCount: uint32(operator_params[i]),
                kickBIPsOfOperatorStake: uint16(operator_params[i + 1]),
                kickBIPsOfTotalStake: uint16(operator_params[i + 2])
            });
        }
        // set to 0 for every quorum
        uint96[] memory quorumsMinimumStake = new uint96[](numQuorums);
        IStakeRegistry.StrategyParams[][] memory quorumsStrategyParams =
            new IStakeRegistry.StrategyParams[][](numQuorums);
        for (uint256 i = 0; i < numQuorums; i++) {
            quorumsStrategyParams[i] = new IStakeRegistry.StrategyParams[](numStrategies);
            for (uint256 j = 0; j < numStrategies; j++) {
                quorumsStrategyParams[i][j] = IStakeRegistry.StrategyParams({
                    strategy: deployedStrategyArray[j],
                    // setting this to 1 ether since the divisor is also 1 ether
                    // therefore this allows an operator to register with even just 1 token
                    // see https://github.com/Layr-Labs/eigenlayer-middleware/blob/m2-mainnet/src/StakeRegistry.sol#L484
                    //    weight += uint96(sharesAmount * strategyAndMultiplier.multiplier / WEIGHTING_DIVISOR);
                    multiplier: 1 ether
                });
            }
        }

        bytes memory upgradeCall = abi.encodeCall(
            RegistryCoordinator.initialize,
            (
                admin,
                admin,
                admin,
                pausercontract,
                0,
                quorumsOperatorSetParams,
                quorumsMinimumStake,
                quorumsStrategyParams
            )
        );
        console2.log("servvv");
        console2.log(result.mockAvsServiceManager);
        console2.log("stakerrr");
        console2.log(result.stakeRegistry);
        console2.log("blspapp");
        console2.log(result.blsapkRegistry);
        console2.log("indexreg");
        console2.log(result.indexRegistry);
        console2.log("socckk");
        console2.log(result.socketRegistry);
        address registryCoordinatorImpl = address(
            new RegistryCoordinator(
                IServiceManager(result.mockAvsServiceManager),
                IStakeRegistry(result.stakeRegistry),
                IBLSApkRegistry(result.blsapkRegistry),
                IIndexRegistry(result.indexRegistry),
                ISocketRegistry(result.socketRegistry)
            )
        );

        address socketRegistryImpl = address(new SocketRegistry(IRegistryCoordinator(result.registryCoordinator)));
        UpgradeableProxyLib.upgrade(result.socketRegistry, socketRegistryImpl);
        UpgradeableProxyLib.upgradeAndCall(result.registryCoordinator, registryCoordinatorImpl, upgradeCall);
        MockAvsServiceManager mockAvsServiceManagerImpl = new MockAvsServiceManager(
            (IAVSDirectory(avsdirectory)),
            IRegistryCoordinator(result.registryCoordinator),
            IStakeRegistry(result.stakeRegistry),
            core.rewardsCoordinator
        );
        UpgradeableProxyLib.upgrade(
            result.mockAvsServiceManager, address(mockAvsServiceManagerImpl)
        );
        MockAvsServiceManager(result.mockAvsServiceManager).initialize(admin);
        verify_deployment(result);

        return result;
    }

    function readDeploymentJson(uint256 chainId) internal returns (DeploymentData memory) {
        return readDeploymentJson("script/deployments/mock-avs/", chainId);
    }

    function readMockAvsConfigJson(string memory directoryPath)
        internal
        returns (MockAvsSetupConfig memory)
    {
        string memory fileName = string.concat(directoryPath, ".json");
        require(vm.exists(fileName), "Deployment file does not exist");
        string memory json = vm.readFile(fileName);

        MockAvsSetupConfig memory data;
        data.numQuorums = json.readUint(".num_quorums");
        data.operatorParams = json.readUintArray(".operator_params");
        data.contracts_registry_addr = json.readAddress(".contracts_registry_addr");
        data.operator_addr = json.readAddress(".operator_addr");
        return data;
    }

    function readDeploymentJson(string memory directoryPath, uint256 chainId)
        internal
        returns (DeploymentData memory)
    {
        string memory fileName = string.concat(directoryPath, vm.toString(chainId), ".json");

        require(vm.exists(fileName), "Deployment file does not exist");

        string memory json = vm.readFile(fileName);

        DeploymentData memory data;
        data.mockAvsServiceManager = json.readAddress(".addresses.mockAvsServiceManager");
        data.registryCoordinator = json.readAddress(".addresses.registryCoordinator");
        data.operatorStateRetriever = json.readAddress(".addresses.operatorStateRetriever");
        data.stakeRegistry = json.readAddress(".addresses.stakeRegistry");
        data.strategy = json.readAddress(".addresses.strategy");
        data.token = json.readAddress(".addresses.token");

        return data;
    }

    /// write to default output path
    function writeDeploymentJson(DeploymentData memory data) internal {
        writeDeploymentJson("script/deployments/mock-avs/", block.chainid, data);
    }

    function writeDeploymentJson(string memory outputPath, uint256 chainId, DeploymentData memory data) internal {
        address proxyAdmin = address(UpgradeableProxyLib.getProxyAdmin(data.mockAvsServiceManager));

        string memory deploymentData = _generateDeploymentJson(data, proxyAdmin);

        string memory fileName = string.concat(outputPath, vm.toString(chainId), ".json");
        if (!vm.exists(outputPath)) {
            vm.createDir(outputPath, true);
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
        return string.concat(
            '{"proxyAdmin":"',
            proxyAdmin.toHexString(),
            '","mockAvsServiceManager":"',
            data.mockAvsServiceManager.toHexString(),
            '","mockAvsServiceManagerImpl":"',
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
            '"}'
        );
    }

    function readCoreDeploymentJson(string memory directoryPath, uint256 chainId)
        internal
        returns (M2CoreDeploymentLib.DeploymentData memory)
    {
        return readCoreDeploymentJson(directoryPath, string.concat(vm.toString(chainId), ".json"));
    }

    function readCoreDeploymentJson(string memory path, string memory fileName)
        internal
        returns (M2CoreDeploymentLib.DeploymentData memory)
    {
        string memory pathToFile = string.concat(path, fileName);

        require(vm.exists(pathToFile), "Deployment file does not exist");

        string memory json = vm.readFile(pathToFile);

        M2CoreDeploymentLib.DeploymentData memory data;
        data.strategyFactory = json.readAddress(".addresses.strategyFactory");
        data.strategyManager = json.readAddress(".addresses.strategyManager");
        data.eigenPodManager = json.readAddress(".addresses.eigenPodManager");
        data.delegationManager = json.readAddress(".addresses.delegation");
        data.avsDirectory = json.readAddress(".addresses.avsDirectory");

        return data;
    }

    function verify_deployment(DeploymentData memory result) internal view {
        IBLSApkRegistry blsapkregistry = IRegistryCoordinator(result.registryCoordinator).blsApkRegistry();
        require(address(blsapkregistry) != address(0));
        IStakeRegistry stakeregistry = IRegistryCoordinator(result.registryCoordinator).stakeRegistry();
        require(address(stakeregistry) != address(0));
        IDelegationManager delegationmanager = IStakeRegistry(address(stakeregistry)).delegation();
        require(address(delegationmanager) != address(0));
    }
}
