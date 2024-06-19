// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {RegistryCoordinator} from "eigenlayer-middleware/src/RegistryCoordinator.sol";
import {OperatorStateRetriever} from "eigenlayer-middleware/src/OperatorStateRetriever.sol";

import "../../src/MockAvsServiceManager.sol";

import {ConfigsReadWriter} from "./ConfigsReadWriter.sol";
import "forge-std/StdJson.sol";

struct MockAvsContracts {
    MockAvsServiceManager mockAvsServiceManager;
    RegistryCoordinator registryCoordinator;
    OperatorStateRetriever operatorStateRetriever;
}

contract MockAvsContractsParser is ConfigsReadWriter {
    function _loadMockAvsDeployedContracts()
        internal
        view
        returns (MockAvsContracts memory)
    {
        // Eigenlayer contracts
        string memory mockAvsDeployedContracts = readOutput(
            "mockavs_deployment_output"
        );
        MockAvsServiceManager mockAvsServiceManager = MockAvsServiceManager(
            stdJson.readAddress(
                mockAvsDeployedContracts,
                ".addresses.mockAvsServiceManager"
            )
        );
        require(
            address(mockAvsServiceManager) != address(0),
            "MockAvsContractsParser: mockAvsServiceManager address is 0"
        );
        RegistryCoordinator registryCoordinator = RegistryCoordinator(
            stdJson.readAddress(
                mockAvsDeployedContracts,
                ".addresses.registryCoordinator"
            )
        );
        require(
            address(registryCoordinator) != address(0),
            "MockAvsContractsParser: registryCoordinator address is 0"
        );
        OperatorStateRetriever operatorStateRetriever = OperatorStateRetriever(
            stdJson.readAddress(
                mockAvsDeployedContracts,
                ".addresses.operatorStateRetriever"
            )
        );
        require(
            address(operatorStateRetriever) != address(0),
            "MockAvsContractsParser: operatorStateRetriever address is 0"
        );

        return
            MockAvsContracts(
                mockAvsServiceManager,
                registryCoordinator,
                operatorStateRetriever
            );
    }
}
