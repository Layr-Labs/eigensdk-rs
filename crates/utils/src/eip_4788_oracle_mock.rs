/**

Generated by the following Solidity interface...
```solidity
interface EIP_4788_Oracle_Mock {
    fallback() external;

    function setBlockRoot(uint64 timestamp, bytes32 blockRoot) external;
    function timestampToBlockRoot(uint256 timestamp) external view returns (bytes32);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "fallback",
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setBlockRoot",
    "inputs": [
      {
        "name": "timestamp",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "blockRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "timestampToBlockRoot",
    "inputs": [
      {
        "name": "timestamp",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod EIP_4788_Oracle_Mock {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b5061028e8061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610034575f3560e01c8063643599f2146101a4578063acd414a8146101df575b6020361461009d5760405162461bcd60e51b815260206004820152602b60248201527f343738384f7261636c654d6f636b2e66616c6c6261636b3a206d616c666f726d60448201526a6564206d73672e6461746160a81b60648201526084015b60405180910390fd5b5f6100a8368261020b565b9050805f036101095760405162461bcd60e51b815260206004820152602760248201527f343738384f7261636c654d6f636b2e66616c6c6261636b3a2074696d6573746160448201526606d7020697320360cc1b6064820152608401610094565b5f818152602081905260408120549081900361019c5760405162461bcd60e51b815260206004820152604660248201527f343738384f7261636c654d6f636b2e66616c6c6261636b3a206e6f20626c6f6360448201527f6b20726f6f7420666f756e642e2044494420594f5520555345204348454154536064820152652e574152503f60d01b608482015260a401610094565b805f5260205ff35b6101cd6101b236600461020b565b67ffffffffffffffff165f9081526020819052604090205490565b60405190815260200160405180910390f35b6102096101ed366004610222565b67ffffffffffffffff9091165f90815260208190526040902055565b005b5f6020828403121561021b575f5ffd5b5035919050565b5f5f60408385031215610233575f5ffd5b823567ffffffffffffffff8116811461024a575f5ffd5b94602093909301359350505056fea2646970667358221220e3b4eaf33a45b229cee6df28c29bb7c42a1a424389218afa08d0fbdc8cb5dd5e64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\x02\x8E\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cd5\x99\xF2\x14a\x01\xA4W\x80c\xAC\xD4\x14\xA8\x14a\x01\xDFW[` 6\x14a\0\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7F4788OracleMock.fallback: malform`D\x82\x01Rjed msg.data`\xA8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_a\0\xA86\x82a\x02\x0BV[\x90P\x80_\x03a\x01\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7F4788OracleMock.fallback: timesta`D\x82\x01Rf\x06\xD7\x02\x06\x972\x03`\xCC\x1B`d\x82\x01R`\x84\x01a\0\x94V[_\x81\x81R` \x81\x90R`@\x81 T\x90\x81\x90\x03a\x01\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7F4788OracleMock.fallback: no bloc`D\x82\x01R\x7Fk root found. DID YOU USE CHEATS`d\x82\x01Re.WARP?`\xD0\x1B`\x84\x82\x01R`\xA4\x01a\0\x94V[\x80_R` _\xF3[a\x01\xCDa\x01\xB26`\x04a\x02\x0BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x02\ta\x01\xED6`\x04a\x02\"V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16_\x90\x81R` \x81\x90R`@\x90 UV[\0[_` \x82\x84\x03\x12\x15a\x02\x1BW__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x023W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02JW__\xFD[\x94` \x93\x90\x93\x015\x93PPPV\xFE\xA2dipfsX\"\x12 \xE3\xB4\xEA\xF3:E\xB2)\xCE\xE6\xDF(\xC2\x9B\xB7\xC4*\x1ABC\x89!\x8A\xFA\x08\xD0\xFB\xDC\x8C\xB5\xDD^dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610034575f3560e01c8063643599f2146101a4578063acd414a8146101df575b6020361461009d5760405162461bcd60e51b815260206004820152602b60248201527f343738384f7261636c654d6f636b2e66616c6c6261636b3a206d616c666f726d60448201526a6564206d73672e6461746160a81b60648201526084015b60405180910390fd5b5f6100a8368261020b565b9050805f036101095760405162461bcd60e51b815260206004820152602760248201527f343738384f7261636c654d6f636b2e66616c6c6261636b3a2074696d6573746160448201526606d7020697320360cc1b6064820152608401610094565b5f818152602081905260408120549081900361019c5760405162461bcd60e51b815260206004820152604660248201527f343738384f7261636c654d6f636b2e66616c6c6261636b3a206e6f20626c6f6360448201527f6b20726f6f7420666f756e642e2044494420594f5520555345204348454154536064820152652e574152503f60d01b608482015260a401610094565b805f5260205ff35b6101cd6101b236600461020b565b67ffffffffffffffff165f9081526020819052604090205490565b60405190815260200160405180910390f35b6102096101ed366004610222565b67ffffffffffffffff9091165f90815260208190526040902055565b005b5f6020828403121561021b575f5ffd5b5035919050565b5f5f60408385031215610233575f5ffd5b823567ffffffffffffffff8116811461024a575f5ffd5b94602093909301359350505056fea2646970667358221220e3b4eaf33a45b229cee6df28c29bb7c42a1a424389218afa08d0fbdc8cb5dd5e64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cd5\x99\xF2\x14a\x01\xA4W\x80c\xAC\xD4\x14\xA8\x14a\x01\xDFW[` 6\x14a\0\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7F4788OracleMock.fallback: malform`D\x82\x01Rjed msg.data`\xA8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_a\0\xA86\x82a\x02\x0BV[\x90P\x80_\x03a\x01\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7F4788OracleMock.fallback: timesta`D\x82\x01Rf\x06\xD7\x02\x06\x972\x03`\xCC\x1B`d\x82\x01R`\x84\x01a\0\x94V[_\x81\x81R` \x81\x90R`@\x81 T\x90\x81\x90\x03a\x01\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7F4788OracleMock.fallback: no bloc`D\x82\x01R\x7Fk root found. DID YOU USE CHEATS`d\x82\x01Re.WARP?`\xD0\x1B`\x84\x82\x01R`\xA4\x01a\0\x94V[\x80_R` _\xF3[a\x01\xCDa\x01\xB26`\x04a\x02\x0BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x02\ta\x01\xED6`\x04a\x02\"V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16_\x90\x81R` \x81\x90R`@\x90 UV[\0[_` \x82\x84\x03\x12\x15a\x02\x1BW__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x023W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02JW__\xFD[\x94` \x93\x90\x93\x015\x93PPPV\xFE\xA2dipfsX\"\x12 \xE3\xB4\xEA\xF3:E\xB2)\xCE\xE6\xDF(\xC2\x9B\xB7\xC4*\x1ABC\x89!\x8A\xFA\x08\xD0\xFB\xDC\x8C\xB5\xDD^dsolcC\0\x08\x1B\x003",
    );
    /**Function with signature `setBlockRoot(uint64,bytes32)` and selector `0xacd414a8`.
```solidity
function setBlockRoot(uint64 timestamp, bytes32 blockRoot) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBlockRootCall {
        pub timestamp: u64,
        pub blockRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`setBlockRoot(uint64,bytes32)`](setBlockRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBlockRootReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setBlockRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: setBlockRootCall) -> Self {
                    (value.timestamp, value.blockRoot)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBlockRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        timestamp: tuple.0,
                        blockRoot: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setBlockRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setBlockRootReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBlockRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBlockRootCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBlockRootReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setBlockRoot(uint64,bytes32)";
            const SELECTOR: [u8; 4] = [172u8, 212u8, 20u8, 168u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestamp),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockRoot),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `timestampToBlockRoot(uint256)` and selector `0x643599f2`.
```solidity
function timestampToBlockRoot(uint256 timestamp) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct timestampToBlockRootCall {
        pub timestamp: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`timestampToBlockRoot(uint256)`](timestampToBlockRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct timestampToBlockRootReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<timestampToBlockRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: timestampToBlockRootCall) -> Self {
                    (value.timestamp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for timestampToBlockRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { timestamp: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<timestampToBlockRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: timestampToBlockRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for timestampToBlockRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for timestampToBlockRootCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = timestampToBlockRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "timestampToBlockRoot(uint256)";
            const SELECTOR: [u8; 4] = [100u8, 53u8, 153u8, 242u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestamp),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`EIP_4788_Oracle_Mock`](self) function calls.
    pub enum EIP_4788_Oracle_MockCalls {
        setBlockRoot(setBlockRootCall),
        timestampToBlockRoot(timestampToBlockRootCall),
    }
    #[automatically_derived]
    impl EIP_4788_Oracle_MockCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [100u8, 53u8, 153u8, 242u8],
            [172u8, 212u8, 20u8, 168u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EIP_4788_Oracle_MockCalls {
        const NAME: &'static str = "EIP_4788_Oracle_MockCalls";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 2usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::setBlockRoot(_) => {
                    <setBlockRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::timestampToBlockRoot(_) => {
                    <timestampToBlockRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<EIP_4788_Oracle_MockCalls>] = &[
                {
                    fn timestampToBlockRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EIP_4788_Oracle_MockCalls> {
                        <timestampToBlockRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EIP_4788_Oracle_MockCalls::timestampToBlockRoot)
                    }
                    timestampToBlockRoot
                },
                {
                    fn setBlockRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EIP_4788_Oracle_MockCalls> {
                        <setBlockRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EIP_4788_Oracle_MockCalls::setBlockRoot)
                    }
                    setBlockRoot
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::setBlockRoot(inner) => {
                    <setBlockRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::timestampToBlockRoot(inner) => {
                    <timestampToBlockRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::setBlockRoot(inner) => {
                    <setBlockRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::timestampToBlockRoot(inner) => {
                    <timestampToBlockRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`EIP_4788_Oracle_Mock`](self) contract instance.

See the [wrapper's documentation](`EIP_4788_Oracle_MockInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EIP_4788_Oracle_MockInstance<T, P, N> {
        EIP_4788_Oracle_MockInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<EIP_4788_Oracle_MockInstance<T, P, N>>,
    > {
        EIP_4788_Oracle_MockInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        EIP_4788_Oracle_MockInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`EIP_4788_Oracle_Mock`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EIP_4788_Oracle_Mock`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EIP_4788_Oracle_MockInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EIP_4788_Oracle_MockInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EIP_4788_Oracle_MockInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EIP_4788_Oracle_MockInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EIP_4788_Oracle_Mock`](self) contract instance.

See the [wrapper's documentation](`EIP_4788_Oracle_MockInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<EIP_4788_Oracle_MockInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> EIP_4788_Oracle_MockInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EIP_4788_Oracle_MockInstance<T, P, N> {
            EIP_4788_Oracle_MockInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EIP_4788_Oracle_MockInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`setBlockRoot`] function.
        pub fn setBlockRoot(
            &self,
            timestamp: u64,
            blockRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, setBlockRootCall, N> {
            self.call_builder(
                &setBlockRootCall {
                    timestamp,
                    blockRoot,
                },
            )
        }
        ///Creates a new call builder for the [`timestampToBlockRoot`] function.
        pub fn timestampToBlockRoot(
            &self,
            timestamp: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, timestampToBlockRootCall, N> {
            self.call_builder(
                &timestampToBlockRootCall {
                    timestamp,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EIP_4788_Oracle_MockInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}