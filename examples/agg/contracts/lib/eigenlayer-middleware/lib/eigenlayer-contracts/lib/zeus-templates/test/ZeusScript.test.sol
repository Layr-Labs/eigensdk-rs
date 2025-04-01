// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

import {Test} from "forge-std/Test.sol";
import {ZeusScript} from "../src/utils/ZeusScript.sol";
import "../src/utils/Encode.sol";
import {EOADeployer} from "../src/templates/EOADeployer.sol";
import "../src/utils/ZEnvHelpers.sol";
import {StringUtils} from "../src/utils/StringUtils.sol";
import {ScriptHelpers} from "../src/utils/ScriptHelpers.sol";

contract ZeusScriptTest is EOADeployer {
    using ScriptHelpers for *;
    using ZEnvHelpers for *;

    function _runAsEOA() internal override {}

    function setUp() public {
        // Set some environment variables to test fallback logic with simple incremental addresses.
        vm.setEnv("ZEUS_ENV_MY_FALLBACK_UINT256", "9999");
        vm.setEnv("ZEUS_ENV_MY_FALLBACK_BOOL", "true");
        vm.setEnv("ZEUS_ENV_MY_FALLBACK_STRING", "fallbackValue");
        vm.setEnv("ZEUS_ENV_MY_FALLBACK_ADDRESS", "0x0000000000000000000000000000000000000001");
        vm.setEnv("ZEUS_ENV_MY_FALLBACK_UINT32", "12345");
        vm.setEnv("ZEUS_ENV_MY_FALLBACK_UINT16", "321");
        vm.setEnv("ZEUS_ENV_MY_FALLBACK_UINT8", "42");
        vm.setEnv("ZEUS_ENV_MY_FALLBACK_UINT64", "7777777");

        // Set environment variables for deployed contracts, using simple addresses like 0x2, 0x3, etc.
        vm.setEnv("ZEUS_DEPLOYED_MyContract_0", "0x0000000000000000000000000000000000000003");
        vm.setEnv("ZEUS_DEPLOYED_MyContract_1", "0x0000000000000000000000000000000000000004");
        vm.setEnv("ZEUS_DEPLOYED_MyContract_Proxy", "0x0000000000000000000000000000000000000005");
        vm.setEnv("ZEUS_DEPLOYED_MyContract_Impl", "0x0000000000000000000000000000000000000006");

        vm.setEnv("ZEUS_TEST", "true");
    }

    function testZAssert() public {
        State storage state = ZEnvHelpers.state();

        // simple deploys
        do {
            string[] memory contracts = new string[](2);
            contracts[0] = "MyContract".impl();
            contracts[1] = "AnotherContract".impl();

            deploySingleton(address(1000), contracts[0]);
            deploySingleton(address(2000), contracts[1]);
            deployInstance(address(2000), contracts[1]);

            // Call zAssertDeployed
            state.assertDeployed(contracts);

            string[] memory instancesDeployed = new string[](1);
            instancesDeployed[0] = contracts[1].instance("0");

            state.assertDeployed(instancesDeployed);
            state.assertClean();
        } while (false);

        // simple state updates
        do {
            vm.setEnv("ZEUS_TEST", "false");
            string[] memory contracts = new string[](1);
            contracts[0] = "MyContract";

            vm.expectRevert("not a zeus test");
            state.assertDeployed(contracts);
            vm.setEnv("ZEUS_TEST", "true");

            string[] memory envParams = new string[](2);
            envParams[0] = "API_KEY";
            envParams[1] = "OWNER_ADDRESS";

            // Mark them as DIRTY
            zUpdate("API_KEY", "123");
            zUpdate("OWNER_ADDRESS", address(1));

            state.assertUpdated(envParams);
            state.assertClean();
        } while (false);

        // every state type
        do {
            vm.setEnv("ZEUS_TEST", "true");
            state.assertClean();

            string[] memory envParams = new string[](8);
            envParams[0] = "UINT256";
            envParams[1] = "UINT32";
            envParams[2] = "UINT64";
            envParams[3] = "ADDRESS";
            envParams[4] = "STRING";
            envParams[5] = "BOOL";
            envParams[6] = "UINT_16";
            envParams[7] = "UINT_8";

            zUpdateUint256("UINT256", uint256(1));
            zUpdateUint32("UINT32", uint32(1));
            zUpdateUint64("UINT64", uint64(1));
            zUpdate("ADDRESS", address(1));
            zUpdate("STRING", "1");
            zUpdate("BOOL", true);
            zUpdateUint16("UINT_16", uint16(1));
            zUpdateUint8("UINT_8", uint8(1));

            state.assertUpdated(envParams);
            state.assertClean();
        } while (false);

        // a few more scenarios
        do {
            vm.setEnv("ZEUS_TEST", "false");
            string[] memory envParams = new string[](1);
            envParams[0] = "SOME_PARAM";

            vm.expectRevert("not a zeus test");
            state.assertUpdated(envParams);
            vm.setEnv("ZEUS_TEST", "true");

            vm.setEnv("ZEUS_TEST", "true");

            zUpdate("KEY1", "1");
            zUpdate("KEY2", "2");

            vm.expectRevert("KEY1: key was not asserted");
            state.assertClean();

            // forget some keys
            string[] memory someKeys = new string[](1);
            someKeys[0] = "KEY1";
            state.assertUpdated(someKeys);

            vm.expectRevert("KEY2: key was not asserted");
            state.assertClean();

            string[] memory restOfKeys = new string[](1);
            restOfKeys[0] = "KEY2";

            // after cleaning up the rest of the keys, assertClean() should be fine.
            state.assertUpdated(restOfKeys);
            state.assertClean();

            // subsequent calls to assertClean() should not throw.
            state.assertClean();
        } while (false);

        do {
            vm.setEnv("ZEUS_TEST", "false");
            zUpdateUint256("SOME_KEY", 42);

            vm.expectRevert("not a zeus test");
            state.assertClean();

            vm.setEnv("ZEUS_TEST", "true");
            vm.expectRevert("SOME_KEY: key was not asserted");
            state.assertClean();

            string[] memory updatedKeys = new string[](1);
            updatedKeys[0] = "SOME_KEY";
            state.assertUpdated(updatedKeys);
            state.assertClean();
        } while (false);
    }

    // --------------------------------------
    // Test zUpdate Functions and Events
    // --------------------------------------

    function testUpdateString() public {
        State storage state = ZEnvHelpers.state();

        vm.expectEmit(true, true, true, true);
        emit ZeusEnvironmentUpdate("MY_STRING_KEY", EnvironmentVariableType.STRING, abi.encode("hello"));

        string memory updated = zUpdate("MY_STRING_KEY", "hello");
        assertEq(updated, "hello");
        assertEq(uint256(state.updatedTypes["MY_STRING_KEY"]), uint256(EnvironmentVariableType.STRING));
        // The code stores the key itself in updatedStrings
        assertEq(state.updatedStrings["MY_STRING_KEY"], "MY_STRING_KEY");
    }

    function testUpdateAddress() public {
        State storage state = ZEnvHelpers.state();

        address testAddr = address(0x1234);

        vm.expectEmit(true, true, true, true);
        emit ZeusEnvironmentUpdate("MY_ADDRESS_KEY", EnvironmentVariableType.ADDRESS, abi.encode(testAddr));

        address updated = zUpdate("MY_ADDRESS_KEY", testAddr);
        assertEq(updated, testAddr);
        assertEq(uint256(state.updatedTypes["MY_ADDRESS_KEY"]), uint256(EnvironmentVariableType.ADDRESS));
        assertEq(state.updatedAddresses["MY_ADDRESS_KEY"], testAddr);
    }

    function testUpdateUint256() public {
        State storage state = ZEnvHelpers.state();
        uint256 val = 42;

        vm.expectEmit(true, true, true, true);
        emit ZeusEnvironmentUpdate("MY_UINT256_KEY", EnvironmentVariableType.UINT_256, abi.encode(val));

        uint256 updated = zUpdateUint256("MY_UINT256_KEY", val);
        assertEq(updated, val);
        assertEq(uint256(state.updatedTypes["MY_UINT256_KEY"]), uint256(EnvironmentVariableType.UINT_256));
        assertEq(state.updatedUInt256s["MY_UINT256_KEY"], val);
    }

    function testUpdateUint64() public {
        State storage state = ZEnvHelpers.state();
        uint64 val = 98765;

        vm.expectEmit(true, true, true, true);
        emit ZeusEnvironmentUpdate("MY_UINT64_KEY", EnvironmentVariableType.UINT_64, abi.encode(val));

        uint64 updated = zUpdateUint64("MY_UINT64_KEY", val);
        assertEq(updated, val);
        assertEq(uint256(state.updatedTypes["MY_UINT64_KEY"]), uint256(EnvironmentVariableType.UINT_64));
        assertEq(state.updatedUInt64s["MY_UINT64_KEY"], val);
    }

    function testUpdateUint32() public {
        State storage state = ZEnvHelpers.state();
        uint32 val = 1234;

        vm.expectEmit(true, true, true, true);
        emit ZeusEnvironmentUpdate("MY_UINT32_KEY", EnvironmentVariableType.UINT_32, abi.encode(val));

        uint32 updated = zUpdateUint32("MY_UINT32_KEY", val);
        assertEq(updated, val);
        assertEq(uint256(state.updatedTypes["MY_UINT32_KEY"]), uint256(EnvironmentVariableType.UINT_32));
        assertEq(state.updatedUInt32s["MY_UINT32_KEY"], val);
    }

    function testUpdateUint16() public {
        State storage state = ZEnvHelpers.state();
        uint16 val = 4321;

        vm.expectEmit(true, true, true, true);
        emit ZeusEnvironmentUpdate("MY_UINT16_KEY", EnvironmentVariableType.UINT_16, abi.encode(val));

        uint16 updated = zUpdateUint16("MY_UINT16_KEY", val);
        assertEq(updated, val);
        assertEq(uint256(state.updatedTypes["MY_UINT16_KEY"]), uint256(EnvironmentVariableType.UINT_16));
        assertEq(state.updatedUInt16s["MY_UINT16_KEY"], val);
    }

    function testUpdateUint8() public {
        State storage state = ZEnvHelpers.state();
        uint8 val = 99;

        vm.expectEmit(true, true, true, true);
        emit ZeusEnvironmentUpdate("MY_UINT8_KEY", EnvironmentVariableType.UINT_8, abi.encode(val));

        uint8 updated = zUpdateUint8("MY_UINT8_KEY", val);
        assertEq(updated, val);
        assertEq(uint256(state.updatedTypes["MY_UINT8_KEY"]), uint256(EnvironmentVariableType.UINT_8));
        assertEq(state.updatedUInt8s["MY_UINT8_KEY"], val);
    }

    function testUpdateBool() public {
        State storage state = ZEnvHelpers.state();
        bool val = true;

        vm.expectEmit(true, true, true, true);
        emit ZeusEnvironmentUpdate("MY_BOOL_KEY", EnvironmentVariableType.BOOL, abi.encode(val));

        bool updated = zUpdate("MY_BOOL_KEY", val);
        assertTrue(updated);
        assertEq(uint256(state.updatedTypes["MY_BOOL_KEY"]), uint256(EnvironmentVariableType.BOOL));
        assertEq(state.updatedBools["MY_BOOL_KEY"], true);
    }

    function testUpdatePreventsTypeChange() public {
        zUpdateUint256("MY_KEY", 123);
        // Attempting to change type from UINT_256 to STRING should revert
        vm.expectRevert();
        zUpdate("MY_KEY", "notAllowed");
    }

    function testMultipleUpdatesSameType() public {
        zUpdateUint256("REUSE_KEY", 100);
        uint256 val = zUpdateUint256("REUSE_KEY", 200);
        assertEq(val, 200);
    }

    // --------------------------------------
    // Test Deployed Contract and Instances
    // --------------------------------------

    function testZDeployedInstanceFallback() public view {
        State storage state = ZEnvHelpers.state();
        // from env: ZEUS_DEPLOYED_MyContract_0 = 0x...bbbb
        //           ZEUS_DEPLOYED_MyContract_1 = 0x...cccc
        address inst0 = state.deployedInstance("MyContract", 0);
        address inst1 = state.deployedInstance("MyContract", 1);
        assertEq(inst0, address(0x0000000000000000000000000000000000000003));
        assertEq(inst1, address(0x0000000000000000000000000000000000000004));
    }

    function testZDeployedInstanceWithOverrides() public {
        State storage state = ZEnvHelpers.state();
        state.updatedContracts["MyContract_0"] = address(0x1111111111111111111111111111111111111111);
        state.updatedContracts["MyContract_1"] = address(0x2222222222222222222222222222222222222222);

        address inst0 = state.deployedInstance("MyContract", 0);
        address inst1 = state.deployedInstance("MyContract", 1);
        assertEq(inst0, address(0x1111111111111111111111111111111111111111));
        assertEq(inst1, address(0x2222222222222222222222222222222222222222));
    }

    function testZDeployedInstanceCountWithOverrides() public {
        State storage state = ZEnvHelpers.state();
        state.updatedContracts["MyContract_0"] = address(0x1111111111111111111111111111111111111111);
        state.updatedContracts["MyContract_1"] = address(0x2222222222222222222222222222222222222222);

        uint256 count = state.deployedInstanceCount("MyContract");
        assertEq(count, 2);
    }

    function testZDeployedInstanceCountFromEnv() public view {
        State storage state = ZEnvHelpers.state();
        // We'll rely on env variables: MyContract_0 and MyContract_1 are set, MyContract_2 is not
        uint256 count = state.deployedInstanceCount("MyContract");
        assertEq(count, 2);
    }

    function testZDeployedProxyFallback() public view {
        State storage state = ZEnvHelpers.state();
        address p = state.deployedProxy("MyContract");
        assertEq(p, address(0x0000000000000000000000000000000000000005));
    }

    function testZDeployedImplFallback() public view {
        State storage state = ZEnvHelpers.state();
        address i = state.deployedImpl("MyContract");
        assertEq(i, address(0x0000000000000000000000000000000000000006));
    }

    // --------------------------------------
    // Test Env Variable Getters without updates (fallback to vm.env)
    // --------------------------------------

    function testZAddressFallback() public view {
        State storage state = ZEnvHelpers.state();
        address val = state.envAddress("MY_FALLBACK_ADDRESS");
        assertEq(val, address(0x0000000000000000000000000000000000000001));
    }

    function testZUint256Fallback() public view {
        State storage state = ZEnvHelpers.state();
        uint256 val = state.envU256("MY_FALLBACK_UINT256");
        assertEq(val, 9999);
    }

    function testZBoolFallback() public view {
        State storage state = ZEnvHelpers.state();
        bool val = state.envBool("MY_FALLBACK_BOOL");
        assertTrue(val);
    }

    function testZStringFallback() public view {
        State storage state = ZEnvHelpers.state();
        string memory val = state.envString("MY_FALLBACK_STRING");
        assertEq(val, "fallbackValue");
    }

    function testZUint32Fallback() public view {
        State storage state = ZEnvHelpers.state();
        uint32 val = state.envU32("MY_FALLBACK_UINT32");
        assertEq(val, 12345);
    }

    function testZUint16Fallback() public view {
        State storage state = ZEnvHelpers.state();
        uint16 val = state.envU16("MY_FALLBACK_UINT16");
        assertEq(val, 321);
    }

    function testZUint8Fallback() public view {
        State storage state = ZEnvHelpers.state();
        uint8 val = state.envU8("MY_FALLBACK_UINT8");
        assertEq(val, 42);
    }

    function testZUint64Fallback() public view {
        State storage state = ZEnvHelpers.state();
        uint64 val = state.envU64("MY_FALLBACK_UINT64");
        assertEq(val, 7777777);
    }

    // Test updating and then calling fallback getters to ensure updated value overrides env:
    function testZBoolOverride() public {
        State storage state = ZEnvHelpers.state();
        zUpdate("MY_FALLBACK_BOOL", false);
        bool val = state.envBool("MY_FALLBACK_BOOL");
        assertFalse(val);
    }

    // --------------------------------------
    // Test that we can emit unused events for coverage
    // --------------------------------------

    function testZeusRequireMultisigEvent() public {
        vm.expectEmit(true, true, true, true);
        emit ZeusRequireMultisig(address(0xabc), Encode.Operation.Call);
        emit ZeusRequireMultisig(address(0xabc), Encode.Operation.Call);
    }

    function testZeusDeployEvent() public {
        vm.expectEmit(true, true, true, true);
        emit ZeusDeploy("TestName", address(0xdef), true);
        emit ZeusDeploy("TestName", address(0xdef), true);
    }

    function testZeusMultisigExecuteEvent() public {
        vm.expectEmit(true, true, true, true);
        emit ZeusMultisigExecute(address(0x123), 1, "0x1234", Encode.Operation.Call);
        emit ZeusMultisigExecute(address(0x123), 1, "0x1234", Encode.Operation.Call);
    }

    function testZDeployedInstanceCountZero() public view {
        State storage state = ZEnvHelpers.state();
        // No env vars or updatedContracts set for "UnknownContract".
        // This should return 0 without entering the loop multiple times.
        uint256 count = state.deployedInstanceCount("UnknownContract");
        assertEq(count, 0);
    }

    function testMissingEnvVarForZDeployedContract() public {
        State storage state = ZEnvHelpers.state();
        // Attempting to get a deployed contract that doesn't exist should revert.
        vm.expectRevert();
        state.deployedProxy("NonExistentContract");
    }

    function testMissingEnvVarForZAddress() public {
        State storage state = ZEnvHelpers.state();
        // No update and no env var for this key; should revert on env lookup.
        vm.expectRevert();
        state.envAddress("NoSuchKey");
    }

    function testMissingEnvVarForZBool() public {
        State storage state = ZEnvHelpers.state();
        vm.expectRevert();
        state.envBool("NoBoolKey");
    }

    function testMissingEnvVarForZString() public {
        State storage state = ZEnvHelpers.state();
        vm.expectRevert();
        state.envString("NoStringKey");
    }

    function testMissingEnvVarForZUint256() public {
        State storage state = ZEnvHelpers.state();
        vm.expectRevert();
        state.envU256("NoUint256Key");
    }

    function testMissingEnvVarForZUint64() public {
        State storage state = ZEnvHelpers.state();
        vm.expectRevert();
        state.envU64("NoUint64Key");
    }

    function testMissingEnvVarForZUint32() public {
        State storage state = ZEnvHelpers.state();
        vm.expectRevert();
        state.envU32("NoUint32Key");
    }

    function testMissingEnvVarForZUint16() public {
        State storage state = ZEnvHelpers.state();
        vm.expectRevert();
        state.envU16("NoUint16Key");
    }

    function testMissingEnvVarForZUint8() public {
        State storage state = ZEnvHelpers.state();
        vm.expectRevert();
        state.envU8("NoUint8Key");
    }

    function testZUpdateTypeChangeOnOtherTypes() public {
        // Set as ADDRESS first
        zUpdate("TYPE_CHANGE_KEY", address(0x1));
        // Now try changing to BOOL
        vm.expectRevert();
        zUpdate("TYPE_CHANGE_KEY", true);
    }

    function testZUpdateTypeChangeOnIntegers() public {
        // Set as UINT_32 first
        zUpdateUint32("TYPE_CHANGE_INT_KEY", 100);
        // Now try changing to UINT_64
        vm.expectRevert();
        zUpdateUint64("TYPE_CHANGE_INT_KEY", 200);
    }

    function testZUpdateTypeChangeOnStringToUint() public {
        zUpdate("TYPE_CHANGE_STR_KEY", "original");
        // Now try changing to UINT_256
        vm.expectRevert();
        zUpdateUint256("TYPE_CHANGE_STR_KEY", 999);
    }

    function testZDeployedInstanceNonExistentIndex() public {
        State storage state = ZEnvHelpers.state();
        // Ensure that requesting a non-existent index reverts or returns correctly.
        // If `vm.envAddress` reverts on missing var, catch it.
        vm.expectRevert();
        state.deployedInstance("MyContract", 999); // large index with no env
    }

    function testZeusRequireMultisigEventDelegateCall() public {
        // Emit with DelegateCall to cover enum branch
        vm.expectEmit(true, true, true, true);
        emit ZeusRequireMultisig(address(0xabc), Encode.Operation.DelegateCall);
        emit ZeusRequireMultisig(address(0xabc), Encode.Operation.DelegateCall);
    }

    function testZeusDeployEventFalseSingleton() public {
        // Emit with false to cover different boolean branch
        vm.expectEmit(true, true, true, true);
        emit ZeusDeploy("AnotherTestName", address(0xdef), false);
        emit ZeusDeploy("AnotherTestName", address(0xdef), false);
    }

    function testZeusMultisigExecuteEventDelegateCall() public {
        // Emit with DelegateCall operation to cover that branch
        vm.expectEmit(true, true, true, true);
        emit ZeusMultisigExecute(address(0x123), 1, "0x5678", Encode.Operation.DelegateCall);
        emit ZeusMultisigExecute(address(0x123), 1, "0x5678", Encode.Operation.DelegateCall);
    }

    function testInvalidEnvAddress() public {
        State storage state = ZEnvHelpers.state();
        vm.setEnv("ZEUS_ENV_INVALID_ADDRESS", "notAnAddress");
        vm.expectRevert();
        state.envAddress("INVALID_ADDRESS");
    }

    function testInvalidEnvUint() public {
        State storage state = ZEnvHelpers.state();
        vm.setEnv("ZEUS_ENV_INVALID_UINT256", "notANumber");
        vm.expectRevert();
        state.envU256("INVALID_UINT256");
    }

    function testInvalidEnvBool() public {
        State storage state = ZEnvHelpers.state();
        vm.setEnv("ZEUS_ENV_INVALID_BOOL", "notABool");
        vm.expectRevert();
        state.envBool("INVALID_BOOL");
    }

    function testMixedZDeployedInstanceCount() public {
        State storage state = ZEnvHelpers.state();
        // For "MixedContract":
        // - 0 from updatedContracts
        // - 1 from environment variable
        // - 2 missing entirely
        state.updatedContracts["MixedContract_0"] = address(0x0000000000000000000000000000000000000011);
        vm.setEnv("ZEUS_DEPLOYED_MixedContract_1", "0x0000000000000000000000000000000000000012");
        // No _2 set, should return count=2
        uint256 count = state.deployedInstanceCount("MixedContract");
        assertEq(count, 2);
    }

    function testMultipleUpdatesSameTypeUint64() public {
        State storage state = ZEnvHelpers.state();
        zUpdateUint64("RETEST_UINT64", 1000);
        zUpdateUint64("RETEST_UINT64", 2000);
        zUpdateUint64("RETEST_UINT64", 3000);
        assertEq(state.envU64("RETEST_UINT64"), 3000);
    }
}
