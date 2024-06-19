// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/interfaces/IERC20.sol";
import {IStrategyManager, IStrategy} from "eigenlayer-contracts/src/contracts/interfaces/IStrategyManager.sol";

import {ConfigsReadWriter} from "./ConfigsReadWriter.sol";
import "forge-std/StdJson.sol";

struct TokenAndStrategyContracts {
    IERC20 token;
    IStrategy strategy;
}

// TODO: support more than one token/strategy pair (dee deployTokensAndStrategies script)
contract TokenAndStrategyContractsParser is ConfigsReadWriter {
    function _loadTokenAndStrategyContracts()
        internal
        view
        returns (TokenAndStrategyContracts memory)
    {
        // Token and Strategy contracts
        string memory tokenAndStrategyConfigFile = readOutput(
            "token_and_strategy_deployment_output"
        );

        bytes memory tokensAndStrategiesConfigsRaw = stdJson.parseRaw(
            tokenAndStrategyConfigFile,
            ".addresses"
        );
        TokenAndStrategyContracts memory tokensAndStrategiesContracts = abi
            .decode(tokensAndStrategiesConfigsRaw, (TokenAndStrategyContracts));

        return tokensAndStrategiesContracts;
    }
}
