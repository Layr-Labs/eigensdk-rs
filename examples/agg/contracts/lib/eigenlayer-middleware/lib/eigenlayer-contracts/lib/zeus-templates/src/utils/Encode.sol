// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

// import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
// import "@openzeppelin/contracts/proxy/beacon/UpgradeableBeacon.sol";

import "../interfaces/IMultiSend.sol";
import "../interfaces/ISafe.sol";
import "../interfaces/IProxyAdmin.sol";
import "../interfaces/IUpgradeableBeacon.sol";

struct MultisigCall {
    address to;
    uint256 value;
    bytes data;
}

library Encode {
    /// Used for state vars:
    ///
    /// uint idx;
    /// mapping(uint => MultisigCall[]) map;
    bytes32 internal constant IDX_SLOT = keccak256("IDX_SLOT");
    bytes32 internal constant MAP_SLOT = keccak256("MAP_SLOT");

    /// Used to describe calls from a Gnosis Safe
    enum Operation {
        Call,
        DelegateCall
    }

    /// Constants for Safe.execTransaction inputs we don't usee
    uint256 constant SAFE_TX_GAS = 0;
    uint256 constant BASE_GAS = 0;
    uint256 constant GAS_PRICE = 0;
    address constant GAS_TOKEN = address(uint160(0));
    address payable constant REFUND_RECEIVER = payable(address(uint160(0)));

    /// Dummy types and variables to facilitate syntax, e.g: `Encode.proxyAdmin.upgrade(...)`
    enum EncProxyAdmin {
        A
    }
    enum EncUpgradeableBeacon {
        A
    }
    enum EncGnosisSafe {
        A
    }

    EncProxyAdmin internal constant proxyAdmin = EncProxyAdmin.A;
    EncUpgradeableBeacon internal constant upgradeableBeacon = EncUpgradeableBeacon.A;
    EncGnosisSafe internal constant gnosisSafe = EncGnosisSafe.A;

    /// @dev Creates a new, clean `MultisigCall[] storage` pointer, guaranteeing
    /// any previous pointers will not be overwritten.
    /// Since we're in a library, we have to use assembly+slot pointers, but the
    /// high level version of this function equates to:
    ///
    /// uint _idx = storage.idx;
    /// storage.idx++;
    /// MultisigCall[] storage calls = storage.map[_idx];
    /// return calls;
    function newMultisigCalls() internal returns (MultisigCall[] storage) {
        bytes32 _IDX_SLOT = IDX_SLOT;
        bytes32 _MAP_SLOT = MAP_SLOT;

        uint256 idx;
        assembly {
            idx := sload(_IDX_SLOT)
            sstore(_IDX_SLOT, add(1, idx))
        }

        // fn pointer indirection fools the compiler into letting us have
        // an uninitialized storage pointer
        function() pure returns (mapping(uint => MultisigCall[]) storage) fn;
        function() pure returns (uint) fn2 = func;
        assembly {
            fn := fn2
        }
        mapping(uint256 => MultisigCall[]) storage map = fn();
        assembly {
            map.slot := _MAP_SLOT
        }

        return map[idx];
    }

    function func() internal pure returns (uint256) {
        return 0;
    }

    /// @dev Appends a call to a list of `MultisigCalls`, returning the original storage pointer
    /// to facilitate call chaining syntax, e.g:
    ///
    /// calls
    ///   .append(...)
    ///   .append(...)
    ///   .append(...);
    function append(MultisigCall[] storage calls, address to, bytes memory data)
        internal
        returns (MultisigCall[] storage)
    {
        calls.push(MultisigCall({to: to, value: 0, data: data}));

        return calls;
    }

    /// @dev Encodes a call to `ProxyAdmin.upgrade(proxy, impl)`
    function upgrade(EncProxyAdmin, address proxy, address impl) internal pure returns (bytes memory) {
        return abi.encodeCall(IProxyAdmin.upgrade, (ITransparentUpgradeableProxy(proxy), impl));
    }

    /// @dev Encodes a call to `UpgradeableBeacon.upgradeTo(newImpl)`
    function upgradeTo(EncUpgradeableBeacon, address newImpl) internal pure returns (bytes memory) {
        return abi.encodeCall(IUpgradeableBeacon.upgradeTo, (newImpl));
    }

    /// @dev Encodes a call to `MultiSend.multiSend(data)`
    function multiSend(MultisigCall[] memory calls) internal pure returns (bytes memory) {
        bytes memory packedCalls = new bytes(0);

        for (uint256 i = 0; i < calls.length; i++) {
            packedCalls = abi.encodePacked(
                packedCalls,
                abi.encodePacked(uint8(0), calls[i].to, calls[i].value, uint256(calls[i].data.length), calls[i].data)
            );
        }

        return abi.encodeCall(IMultiSend.multiSend, packedCalls);
    }

    /// @dev Encodes a call to `Safe.execTransaction`
    function execTransaction(EncGnosisSafe, address from, address to, Operation op, bytes memory data)
        internal
        pure
        returns (bytes memory)
    {
        return _encExecTranasction({from: from, to: to, op: op, value: 0, data: data});
    }

    function _encExecTranasction(address from, address to, Operation op, uint256 value, bytes memory data)
        private
        pure
        returns (bytes memory)
    {
        bytes1 v = bytes1(uint8(1));
        bytes32 r = bytes32(uint256(uint160(from)));
        bytes32 s;
        bytes memory sig = abi.encodePacked(r, s, v);

        return abi.encodeCall(
            ISafe.execTransaction,
            (to, value, data, uint8(op), SAFE_TX_GAS, BASE_GAS, GAS_PRICE, GAS_TOKEN, REFUND_RECEIVER, sig)
        );
    }
}
