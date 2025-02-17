// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {MockERC20} from "../../src/MockERC20.sol";

library FundOperator {
    function fundOperator(address erc20, address operator, uint256 amount) internal {
        MockERC20 erc20Contract = MockERC20(erc20);

        erc20Contract.mint(operator, amount);
    }
}
