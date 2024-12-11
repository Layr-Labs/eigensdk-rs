/**

Generated by the following Solidity interface...
```solidity
interface SignatureCheckerLib {
    error InvalidSignature();

    function isValidSignature(address signer, bytes32 digestHash, bytes memory signature) external view;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "isValidSignature",
    "inputs": [
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "digestHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod SignatureCheckerLib {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608080604052346019576103aa908161001e823930815050f35b5f80fdfe60806040526004361015610011575f80fd5b5f3560e01c63238a4d1e14610024575f80fd5b60603660031901126100c0576004356001600160a01b03811681036100c0576044359067ffffffffffffffff82116100c057366023830112156100c057816004013590610070826100fa565b9161007e60405193846100c4565b80835236602482860101116100c0576020815f9260246100aa9701838701378401015260243590610116565b156100b157005b638baa579f60e01b5f5260045ffd5b5f80fd5b90601f8019910116810190811067ffffffffffffffff8211176100e657604052565b634e487b7160e01b5f52604160045260245ffd5b67ffffffffffffffff81116100e657601f01601f191660200190565b9190916101238284610245565b60058110156102315715908161021b575b50610213575f9260206101936084869560405193849181830196630b135d3f60e11b88526024840152604060448401528051918291826064860152018484015e87838284010152601f801991011681010301601f1981018352826100c4565b51915afa3d1561020c573d6101a7816100fa565b906101b560405192836100c4565b81523d5f602083013e5b81610200575b816101ce575090565b90506020818051810103126100c057602001516001600160e01b03198116908190036100c057630b135d3f60e11b1490565b805160201491506101c5565b60606101bf565b505050600190565b6001600160a01b0383811691161490505f610134565b634e487b7160e01b5f52602160045260245ffd5b81516041810361027157509061026d91602082015190606060408401519301515f1a906102c7565b9091565b6040036102be5760406020830151920151918260ff1c91601b83018093116102aa5761026d936001600160ff1b03169260ff16906102c7565b634e487b7160e01b5f52601160045260245ffd5b50505f90600290565b907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116103695760ff1690601b8214158061035e575b610353576020935f93608093604051938452868401526040830152606082015282805260015afa15610348575f516001600160a01b0381161561034057905f90565b505f90600190565b6040513d5f823e3d90fd5b505050505f90600490565b50601c8214156102fe565b505050505f9060039056fea2646970667358221220bf23de668d96ef4941de2afe11243daa890303c71c9dc6927503e10bacc0aaaf64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`\x19Wa\x03\xAA\x90\x81a\0\x1E\x8290\x81PP\xF3[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1Cc#\x8AM\x1E\x14a\0$W_\x80\xFD[``6`\x03\x19\x01\x12a\0\xC0W`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\0\xC0W`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\0\xC0W6`#\x83\x01\x12\x15a\0\xC0W\x81`\x04\x015\x90a\0p\x82a\0\xFAV[\x91a\0~`@Q\x93\x84a\0\xC4V[\x80\x83R6`$\x82\x86\x01\x01\x11a\0\xC0W` \x81_\x92`$a\0\xAA\x97\x01\x83\x87\x017\x84\x01\x01R`$5\x90a\x01\x16V[\x15a\0\xB1W\0[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[_\x80\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\0\xE6W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xE6W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x91\x90\x91a\x01#\x82\x84a\x02EV[`\x05\x81\x10\x15a\x021W\x15\x90\x81a\x02\x1BW[Pa\x02\x13W_\x92` a\x01\x93`\x84\x86\x95`@Q\x93\x84\x91\x81\x83\x01\x96c\x0B\x13]?`\xE1\x1B\x88R`$\x84\x01R`@`D\x84\x01R\x80Q\x91\x82\x91\x82`d\x86\x01R\x01\x84\x84\x01^\x87\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01`\x1F\x19\x81\x01\x83R\x82a\0\xC4V[Q\x91Z\xFA=\x15a\x02\x0CW=a\x01\xA7\x81a\0\xFAV[\x90a\x01\xB5`@Q\x92\x83a\0\xC4V[\x81R=_` \x83\x01>[\x81a\x02\0W[\x81a\x01\xCEWP\x90V[\x90P` \x81\x80Q\x81\x01\x03\x12a\0\xC0W` \x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x90\x81\x90\x03a\0\xC0Wc\x0B\x13]?`\xE1\x1B\x14\x90V[\x80Q` \x14\x91Pa\x01\xC5V[``a\x01\xBFV[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_a\x014V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x81Q`A\x81\x03a\x02qWP\x90a\x02m\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a\x02\xC7V[\x90\x91V[`@\x03a\x02\xBEW`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a\x02\xAAWa\x02m\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90a\x02\xC7V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[PP_\x90`\x02\x90V[\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a\x03iW`\xFF\x16\x90`\x1B\x82\x14\x15\x80a\x03^W[a\x03SW` \x93_\x93`\x80\x93`@Q\x93\x84R\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x03HW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x03@W\x90_\x90V[P_\x90`\x01\x90V[`@Q=_\x82>=\x90\xFD[PPPP_\x90`\x04\x90V[P`\x1C\x82\x14\x15a\x02\xFEV[PPPP_\x90`\x03\x90V\xFE\xA2dipfsX\"\x12 \xBF#\xDEf\x8D\x96\xEFIA\xDE*\xFE\x11$=\xAA\x89\x03\x03\xC7\x1C\x9D\xC6\x92u\x03\xE1\x0B\xAC\xC0\xAA\xAFdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c63238a4d1e14610024575f80fd5b60603660031901126100c0576004356001600160a01b03811681036100c0576044359067ffffffffffffffff82116100c057366023830112156100c057816004013590610070826100fa565b9161007e60405193846100c4565b80835236602482860101116100c0576020815f9260246100aa9701838701378401015260243590610116565b156100b157005b638baa579f60e01b5f5260045ffd5b5f80fd5b90601f8019910116810190811067ffffffffffffffff8211176100e657604052565b634e487b7160e01b5f52604160045260245ffd5b67ffffffffffffffff81116100e657601f01601f191660200190565b9190916101238284610245565b60058110156102315715908161021b575b50610213575f9260206101936084869560405193849181830196630b135d3f60e11b88526024840152604060448401528051918291826064860152018484015e87838284010152601f801991011681010301601f1981018352826100c4565b51915afa3d1561020c573d6101a7816100fa565b906101b560405192836100c4565b81523d5f602083013e5b81610200575b816101ce575090565b90506020818051810103126100c057602001516001600160e01b03198116908190036100c057630b135d3f60e11b1490565b805160201491506101c5565b60606101bf565b505050600190565b6001600160a01b0383811691161490505f610134565b634e487b7160e01b5f52602160045260245ffd5b81516041810361027157509061026d91602082015190606060408401519301515f1a906102c7565b9091565b6040036102be5760406020830151920151918260ff1c91601b83018093116102aa5761026d936001600160ff1b03169260ff16906102c7565b634e487b7160e01b5f52601160045260245ffd5b50505f90600290565b907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116103695760ff1690601b8214158061035e575b610353576020935f93608093604051938452868401526040830152606082015282805260015afa15610348575f516001600160a01b0381161561034057905f90565b505f90600190565b6040513d5f823e3d90fd5b505050505f90600490565b50601c8214156102fe565b505050505f9060039056fea2646970667358221220bf23de668d96ef4941de2afe11243daa890303c71c9dc6927503e10bacc0aaaf64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1Cc#\x8AM\x1E\x14a\0$W_\x80\xFD[``6`\x03\x19\x01\x12a\0\xC0W`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\0\xC0W`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\0\xC0W6`#\x83\x01\x12\x15a\0\xC0W\x81`\x04\x015\x90a\0p\x82a\0\xFAV[\x91a\0~`@Q\x93\x84a\0\xC4V[\x80\x83R6`$\x82\x86\x01\x01\x11a\0\xC0W` \x81_\x92`$a\0\xAA\x97\x01\x83\x87\x017\x84\x01\x01R`$5\x90a\x01\x16V[\x15a\0\xB1W\0[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[_\x80\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\0\xE6W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xE6W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x91\x90\x91a\x01#\x82\x84a\x02EV[`\x05\x81\x10\x15a\x021W\x15\x90\x81a\x02\x1BW[Pa\x02\x13W_\x92` a\x01\x93`\x84\x86\x95`@Q\x93\x84\x91\x81\x83\x01\x96c\x0B\x13]?`\xE1\x1B\x88R`$\x84\x01R`@`D\x84\x01R\x80Q\x91\x82\x91\x82`d\x86\x01R\x01\x84\x84\x01^\x87\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01`\x1F\x19\x81\x01\x83R\x82a\0\xC4V[Q\x91Z\xFA=\x15a\x02\x0CW=a\x01\xA7\x81a\0\xFAV[\x90a\x01\xB5`@Q\x92\x83a\0\xC4V[\x81R=_` \x83\x01>[\x81a\x02\0W[\x81a\x01\xCEWP\x90V[\x90P` \x81\x80Q\x81\x01\x03\x12a\0\xC0W` \x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x90\x81\x90\x03a\0\xC0Wc\x0B\x13]?`\xE1\x1B\x14\x90V[\x80Q` \x14\x91Pa\x01\xC5V[``a\x01\xBFV[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_a\x014V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x81Q`A\x81\x03a\x02qWP\x90a\x02m\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a\x02\xC7V[\x90\x91V[`@\x03a\x02\xBEW`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a\x02\xAAWa\x02m\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90a\x02\xC7V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[PP_\x90`\x02\x90V[\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a\x03iW`\xFF\x16\x90`\x1B\x82\x14\x15\x80a\x03^W[a\x03SW` \x93_\x93`\x80\x93`@Q\x93\x84R\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x03HW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x03@W\x90_\x90V[P_\x90`\x01\x90V[`@Q=_\x82>=\x90\xFD[PPPP_\x90`\x04\x90V[P`\x1C\x82\x14\x15a\x02\xFEV[PPPP_\x90`\x03\x90V\xFE\xA2dipfsX\"\x12 \xBF#\xDEf\x8D\x96\xEFIA\xDE*\xFE\x11$=\xAA\x89\x03\x03\xC7\x1C\x9D\xC6\x92u\x03\xE1\x0B\xAC\xC0\xAA\xAFdsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
```solidity
error InvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Function with signature `isValidSignature(address,bytes32,bytes)` and selector `0x238a4d1e`.
```solidity
function isValidSignature(address signer, bytes32 digestHash, bytes memory signature) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureCall {
        pub signer: alloy::sol_types::private::Address,
        pub digestHash: alloy::sol_types::private::FixedBytes<32>,
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`isValidSignature(address,bytes32,bytes)`](isValidSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<isValidSignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureCall) -> Self {
                    (value.signer, value.digestHash, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        signer: tuple.0,
                        digestHash: tuple.1,
                        signature: tuple.2,
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
            impl ::core::convert::From<isValidSignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidSignatureCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidSignatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidSignature(address,bytes32,bytes)";
            const SELECTOR: [u8; 4] = [35u8, 138u8, 77u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.digestHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
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
    ///Container for all the [`SignatureCheckerLib`](self) function calls.
    pub enum SignatureCheckerLibCalls {
        isValidSignature(isValidSignatureCall),
    }
    #[automatically_derived]
    impl SignatureCheckerLibCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[35u8, 138u8, 77u8, 30u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SignatureCheckerLibCalls {
        const NAME: &'static str = "SignatureCheckerLibCalls";
        const MIN_DATA_LENGTH: usize = 128usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::isValidSignature(_) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SignatureCheckerLibCalls>] = &[
                {
                    fn isValidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SignatureCheckerLibCalls> {
                        <isValidSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SignatureCheckerLibCalls::isValidSignature)
                    }
                    isValidSignature
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
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SignatureCheckerLib`](self) custom errors.
    pub enum SignatureCheckerLibErrors {
        InvalidSignature(InvalidSignature),
    }
    #[automatically_derived]
    impl SignatureCheckerLibErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[139u8, 170u8, 87u8, 159u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SignatureCheckerLibErrors {
        const NAME: &'static str = "SignatureCheckerLibErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<SignatureCheckerLibErrors>] = &[
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SignatureCheckerLibErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SignatureCheckerLibErrors::InvalidSignature)
                    }
                    InvalidSignature
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
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SignatureCheckerLib`](self) contract instance.

See the [wrapper's documentation](`SignatureCheckerLibInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SignatureCheckerLibInstance<T, P, N> {
        SignatureCheckerLibInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<SignatureCheckerLibInstance<T, P, N>>,
    > {
        SignatureCheckerLibInstance::<T, P, N>::deploy(provider)
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
        SignatureCheckerLibInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`SignatureCheckerLib`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SignatureCheckerLib`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SignatureCheckerLibInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SignatureCheckerLibInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SignatureCheckerLibInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SignatureCheckerLibInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SignatureCheckerLib`](self) contract instance.

See the [wrapper's documentation](`SignatureCheckerLibInstance`) for more details.*/
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
        ) -> alloy_contract::Result<SignatureCheckerLibInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> SignatureCheckerLibInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SignatureCheckerLibInstance<T, P, N> {
            SignatureCheckerLibInstance {
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
    > SignatureCheckerLibInstance<T, P, N> {
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
        ///Creates a new call builder for the [`isValidSignature`] function.
        pub fn isValidSignature(
            &self,
            signer: alloy::sol_types::private::Address,
            digestHash: alloy::sol_types::private::FixedBytes<32>,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidSignatureCall, N> {
            self.call_builder(
                &isValidSignatureCall {
                    signer,
                    digestHash,
                    signature,
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
    > SignatureCheckerLibInstance<T, P, N> {
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
