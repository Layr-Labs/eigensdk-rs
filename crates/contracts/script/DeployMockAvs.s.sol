// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "./DeployMockAvsRegistries.s.sol";
import "forge-std/console.sol";

// forge script script/DeployMockAvs.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --etherscan-api-key $ETHERSCAN_API_KEY --broadcast --verify
contract DeployMockAvs is DeployMockAvsRegistries {
    MockAvsServiceManager public mockAvsServiceManager;
    MockAvsServiceManager public mockAvsServiceManagerImplementation;

    function run() public virtual {
        // The ContractsRegistry contract should always be deployed at this address on anvil
        // it's the address of the contract created at nonce 0 by the first anvil account (0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266)
        ContractsRegistry contractsRegistry = ContractsRegistry(
            0x5FbDB2315678afecb367f032d93F642f64180aa3
        );
        EigenlayerContracts
            memory eigenlayerContracts = _loadEigenlayerDeployedContracts();
        MockAvsOpsAddresses memory addressConfig = _loadAvsOpsAddresses(
            "ops_addresses"
        );

        vm.startBroadcast();

        // Deploy proxy admin for ability to upgrade proxy contracts
        // Note: can't deploy ProxyAdmin in setUp function, b/c its owner is not set correctly if so.
        //       not sure why...
        emptyContract = new EmptyContract();
        mockAvsProxyAdmin = new ProxyAdmin();
        mockAvsServiceManager = MockAvsServiceManager(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(mockAvsProxyAdmin),
                    ""
                )
            )
        );
        MockAvsContracts
            memory mockAvsContracts = _deploymockAvsRegistryContracts(
                eigenlayerContracts,
                addressConfig,
                mockAvsServiceManager,
                mockAvsServiceManagerImplementation
            );
        mockAvsServiceManagerImplementation = new MockAvsServiceManager(
            registryCoordinator,
            eigenlayerContracts.avsDirectory,
            eigenlayerContracts.rewardsCoordinator
        );

        mockAvsProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(
                payable(address(mockAvsServiceManager))
            ),
            address(mockAvsServiceManagerImplementation),
            abi.encodeWithSelector(
                mockAvsServiceManager.initialize.selector,
                addressConfig.communityMultisig
            )
        );
        require(
            Ownable(address(mockAvsServiceManager)).owner() != address(0),
            "Owner uninitialized"
        );

        if (block.chainid == 31337 || block.chainid == 1337) {
            _writeContractsToRegistry(
                contractsRegistry,
                eigenlayerContracts,
                mockAvsContracts
            );
        }
        vm.stopBroadcast();
    }
}
