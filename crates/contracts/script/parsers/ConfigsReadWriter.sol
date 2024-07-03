// SPDX-License-Identifier: BUSL-1.1
pragma solidity =0.8.12;

import "eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";
import "eigenlayer-contracts/src/contracts/strategies/StrategyBase.sol";

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract ConfigsReadWriter is Script {
    // Forge scripts best practice: https://book.getfoundry.sh/tutorials/best-practices#scripts
    // inputFileName should not contain the .json extension, we add it automatically
    function readInput(string memory inputFileName) internal view returns (string memory) {
        string memory inputDir = string.concat(vm.projectRoot(), "/script/input/");
        string memory chainDir = string.concat(vm.toString(block.chainid), "/");
        string memory file = string.concat(inputFileName, ".json");
        return vm.readFile(string.concat(inputDir, chainDir, file));
    }

    function readOutput(string memory outputFileName) internal view returns (string memory) {
        string memory inputDir = string.concat(vm.projectRoot(), "/script/output/");
        string memory chainDir = string.concat(vm.toString(block.chainid), "/");
        string memory file = string.concat(outputFileName, ".json");
        return vm.readFile(string.concat(inputDir, chainDir, file));
    }

    function writeOutput(string memory outputJson, string memory outputFileName) internal {
        string memory outputDir = string.concat(vm.projectRoot(), "/script/output/");
        string memory chainDir = string.concat(vm.toString(block.chainid), "/");
        string memory outputFilePath = string.concat(outputDir, chainDir, outputFileName, ".json");
        vm.writeJson(outputJson, outputFilePath);
    }
}
