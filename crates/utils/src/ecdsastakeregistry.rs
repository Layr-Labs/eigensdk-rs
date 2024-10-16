///Module containing a contract's types and functions.
/**

```solidity
library ISignatureUtils {
    struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
}
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod ISignatureUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct SignatureWithSaltAndExpiry {
        pub signature: alloy::sol_types::private::Bytes,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureWithSaltAndExpiry> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureWithSaltAndExpiry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signature: tuple.0,
                    salt: tuple.1,
                    expiry: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignatureWithSaltAndExpiry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SignatureWithSaltAndExpiry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SignatureWithSaltAndExpiry {
            const NAME: &'static str = "SignatureWithSaltAndExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithSaltAndExpiry(bytes signature,bytes32 salt,uint256 expiry)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiry)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignatureWithSaltAndExpiry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiry,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiry,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

    See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsInstance<T, P, N> {
        ISignatureUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtils`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ISignatureUtils`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISignatureUtilsInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

        See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
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
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsInstance<T, P, N> {
            ISignatureUtilsInstance {
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
        > ISignatureUtilsInstance<T, P, N>
    {
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
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISignatureUtilsInstance<T, P, N>
    {
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
/**

Generated by the following Solidity interface...
```solidity
library ISignatureUtils {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

interface ECDSAStakeRegistry {
    struct Quorum {
        StrategyParams[] strategies;
    }
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }

    error InsufficientSignedStake();
    error InsufficientWeight();
    error InvalidLength();
    error InvalidQuorum();
    error InvalidReferenceBlock();
    error InvalidSignature();
    error InvalidSignedWeight();
    error InvalidThreshold();
    error LengthMismatch();
    error MustUpdateAllOperators();
    error NotSorted();
    error OperatorAlreadyRegistered();
    error OperatorNotRegistered();

    event Initialized(uint8 version);
    event MinimumWeightUpdated(uint256 _old, uint256 _new);
    event OperatorDeregistered(address indexed _operator, address indexed _avs);
    event OperatorRegistered(address indexed _operator, address indexed _avs);
    event OperatorWeightUpdated(address indexed _operator, uint256 oldWeight, uint256 newWeight);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event QuorumUpdated(Quorum _old, Quorum _new);
    event SigningKeyUpdate(address indexed operator, uint256 indexed updateBlock, address indexed newSigningKey, address oldSigningKey);
    event ThresholdWeightUpdated(uint256 _thresholdWeight);
    event TotalWeightUpdated(uint256 oldTotalWeight, uint256 newTotalWeight);
    event UpdateMinimumWeight(uint256 oldMinimumWeight, uint256 newMinimumWeight);

    constructor(address _delegationManager);

    function deregisterOperator() external;
    function getLastCheckpointOperatorWeight(address _operator) external view returns (uint256);
    function getLastCheckpointThresholdWeight() external view returns (uint256);
    function getLastCheckpointThresholdWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
    function getLastCheckpointTotalWeight() external view returns (uint256);
    function getLastCheckpointTotalWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
    function getLastestOperatorSigningKey(address _operator) external view returns (address);
    function getOperatorSigningKeyAtBlock(address _operator, uint256 _blockNumber) external view returns (address);
    function getOperatorWeight(address _operator) external view returns (uint256);
    function getOperatorWeightAtBlock(address _operator, uint32 _blockNumber) external view returns (uint256);
    function initialize(address _serviceManager, uint256 _thresholdWeight, Quorum memory _quorum) external;
    function isValidSignature(bytes32 _dataHash, bytes memory _signatureData) external view returns (bytes4);
    function minimumWeight() external view returns (uint256);
    function operatorRegistered(address _operator) external view returns (bool);
    function owner() external view returns (address);
    function quorum() external view returns (Quorum memory);
    function registerOperatorWithSignature(ISignatureUtils.SignatureWithSaltAndExpiry memory _operatorSignature, address _signingKey) external;
    function renounceOwnership() external;
    function transferOwnership(address newOwner) external;
    function updateMinimumWeight(uint256 _newMinimumWeight, address[] memory _operators) external;
    function updateOperatorSigningKey(address _newSigningKey) external;
    function updateOperators(address[] memory _operators) external;
    function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory) external;
    function updateQuorumConfig(Quorum memory _quorum, address[] memory _operators) external;
    function updateStakeThreshold(uint256 _thresholdWeight) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_delegationManager",
        "type": "address",
        "internalType": "contract IDelegationManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperator",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getLastCheckpointOperatorWeight",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointThresholdWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointThresholdWeightAtBlock",
    "inputs": [
      {
        "name": "_blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointTotalWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointTotalWeightAtBlock",
    "inputs": [
      {
        "name": "_blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastestOperatorSigningKey",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorSigningKeyAtBlock",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_blockNumber",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorWeight",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorWeightAtBlock",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_serviceManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_quorum",
        "type": "tuple",
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isValidSignature",
    "inputs": [
      {
        "name": "_dataHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_signatureData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "minimumWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorRegistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quorum",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registerOperatorWithSignature",
    "inputs": [
      {
        "name": "_operatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithSaltAndExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "_signingKey",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateMinimumWeight",
    "inputs": [
      {
        "name": "_newMinimumWeight",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorSigningKey",
    "inputs": [
      {
        "name": "_newSigningKey",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperators",
    "inputs": [
      {
        "name": "_operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorsForQuorum",
    "inputs": [
      {
        "name": "operatorsPerQuorum",
        "type": "address[][]",
        "internalType": "address[][]"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateQuorumConfig",
    "inputs": [
      {
        "name": "_quorum",
        "type": "tuple",
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      },
      {
        "name": "_operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateStakeThreshold",
    "inputs": [
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MinimumWeightUpdated",
    "inputs": [
      {
        "name": "_old",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "_new",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorDeregistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "_avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRegistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "_avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorWeightUpdated",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumUpdated",
    "inputs": [
      {
        "name": "_old",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      },
      {
        "name": "_new",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SigningKeyUpdate",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "updateBlock",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "newSigningKey",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldSigningKey",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ThresholdWeightUpdated",
    "inputs": [
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TotalWeightUpdated",
    "inputs": [
      {
        "name": "oldTotalWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newTotalWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdateMinimumWeight",
    "inputs": [
      {
        "name": "oldMinimumWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newMinimumWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "InsufficientSignedStake",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientWeight",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidQuorum",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidReferenceBlock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignedWeight",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MustUpdateAllOperators",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorAlreadyRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegistered",
    "inputs": []
  }
]
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod ECDSAStakeRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523480156200001157600080fd5b506040516200299538038062002995833981016040819052620000349162000046565b6001600160a01b031660805262000078565b6000602082840312156200005957600080fd5b81516001600160a01b03811681146200007157600080fd5b9392505050565b60805161290162000094600039600061071901526129016000f3fe608060405234801561001057600080fd5b50600436106101725760003560e01c8063696255be116100de57806398ec1ac911610097578063cdcd358111610071578063cdcd358114610315578063dec5d1f614610328578063ec7fbb311461033b578063f2fde38b1461037757600080fd5b806398ec1ac9146102e7578063ab118995146102fa578063b933fa741461030d57600080fd5b8063696255be1461028d578063715018a6146102a0578063743c31f4146102a8578063857dc190146102bb5780638da5cb5b146102c3578063955f2d90146102d457600080fd5b80633b242e4a116101305780633b242e4a1461020e5780633d5611f61461022157806340bf2fb7146102345780635140a5481461023c5780635e1042e81461024f5780635ef533291461027a57600080fd5b8062cf2ab5146101775780630dba33941461018c5780631626ba7e146101b25780631703a018146101de5780631e4cd85e146101f3578063314f3a4914610206575b600080fd5b61018a610185366004611dee565b61038a565b005b61019f61019a366004611e3c565b610396565b6040519081526020015b60405180910390f35b6101c56101c0366004611ed1565b6103b2565b6040516001600160e01b031990911681526020016101a9565b6101e66103f0565b6040516101a99190611f7a565b61019f610201366004611e3c565b610483565b61019f610499565b61019f61021c366004611f8d565b6104aa565b61018a61022f366004611faa565b6104cb565b60675461019f565b61018a61024a366004612057565b6104da565b61026261025d366004612120565b6104fd565b6040516001600160a01b0390911681526020016101a9565b61018a61028836600461214c565b610526565b61018a61029b366004612165565b610537565b61018a610551565b61018a6102b6366004611f8d565b610565565b61018a61059f565b6033546001600160a01b0316610262565b61019f6102e23660046121a1565b6105a8565b61019f6102f5366004611f8d565b6105d3565b61018a6103083660046122b0565b61083a565b61019f610956565b610262610323366004611f8d565b610962565b61018a610336366004612308565b610983565b610367610349366004611f8d565b6001600160a01b03166000908152606e602052604090205460ff1690565b60405190151581526020016101a9565b61018a610385366004611f8d565b610994565b61039381610a0a565b50565b60006103ac606b63ffffffff80851690610a6116565b92915050565b600080600080848060200190518101906103cc9190612458565b9250925092506103de86848484610b70565b50630b135d3f60e11b95945050505050565b6040805160208101909152606081526040805160668054602081810284018501855283018181529293919284929091849160009085015b8282101561047657600084815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b031681830152825260019092019101610427565b5050505081525050905090565b60006103ac606c63ffffffff80851690610a6116565b60006104a5606b610c2f565b905090565b6001600160a01b0381166000908152606d602052604081206103ac90610c2f565b6104d6338383610c8b565b5050565b6104d6826000815181106104f0576104f061252c565b6020026020010151610dc3565b6001600160a01b0382166000908152606a6020526040812061051f9083610a61565b9392505050565b61052e610de6565b61039381610e40565b61053f610de6565b61054882610e83565b6104d681610a0a565b610559610de6565b6105636000610ec9565b565b336000908152606e602052604090205460ff16610595576040516325ec6c1f60e01b815260040160405180910390fd5b6103933382610f1b565b61056333610fce565b6001600160a01b0382166000908152606d6020526040812061051f9063ffffffff80851690610a6116565b6000806066600001805480602002602001604051908101604052809291908181526020016000905b8282101561064a57600084815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b0316818301528252600190920191016105fb565b50505050905060008082516001600160401b0381111561066c5761066c611cb2565b604051908082528060200260200182016040528015610695578160200160208202803683370190505b50905060005b83518110156106fe578381815181106106b6576106b661252c565b6020026020010151600001518282815181106106d4576106d461252c565b6001600160a01b0390921660209283029190910190910152806106f681612558565b91505061069b565b50604051639004134760e01b81526000906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906390041347906107509089908690600401612573565b600060405180830381865afa15801561076d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261079591908101906125cf565b905060005b845181101561080c578481815181106107b5576107b561252c565b6020026020010151602001516001600160601b03168282815181106107dc576107dc61252c565b60200260200101516107ee919061265f565b6107f8908561267e565b93508061080481612558565b91505061079a565b5061081961271084612696565b9250606754831061082e575090949350505050565b50600095945050505050565b600054610100900460ff161580801561085a5750600054600160ff909116105b806108745750303b158015610874575060005460ff166001145b6108dc5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b6000805460ff1916600117905580156108ff576000805461ff0019166101001790555b61090a8484846110f1565b8015610950576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b60006104a5606c610c2f565b6001600160a01b0381166000908152606a602052604081206103ac90610c2f565b61098b610de6565b61054882611152565b61099c610de6565b6001600160a01b038116610a015760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016108d3565b61039381610ec9565b6000805b8251811015610a5757610a39838281518110610a2c57610a2c61252c565b60200260200101516112b1565b610a4390836126b8565b915080610a4f81612558565b915050610a0e565b50610950816113d4565b6000438210610ab25760405162461bcd60e51b815260206004820181905260248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e656460448201526064016108d3565b825460005b81811015610b17576000610acb8284611440565b905084866000018281548110610ae357610ae361252c565b60009182526020909120015463ffffffff161115610b0357809250610b11565b610b0e81600161267e565b91505b50610ab7565b8115610b5b5784610b296001846126f9565b81548110610b3957610b3961252c565b60009182526020909120015464010000000090046001600160e01b0316610b5e565b60005b6001600160e01b031695945050505050565b600083519050600080600080610b8785885161145b565b60005b85811015610c1957888181518110610ba457610ba461252c565b60200260200101519450610bb8858861149c565b9250610bc484866114ef565b610be8838b8a8481518110610bdb57610bdb61252c565b6020026020010151611521565b8493506000610bf78689611552565b9050610c03818461267e565b9250508080610c1190612558565b915050610b8a565b50610c2481876115a5565b505050505050505050565b80546000908015610c785782610c466001836126f9565b81548110610c5657610c5661252c565b60009182526020909120015464010000000090046001600160e01b0316610c7b565b60005b6001600160e01b03169392505050565b6001600160a01b0383166000908152606e602052604090205460ff1615610cc5576040516342ee68b560e01b815260040160405180910390fd5b60658054906000610cd583612558565b90915550506001600160a01b0383166000908152606e60205260408120805460ff19166001179055610d06846112b1565b9050610d11816113d4565b5050610d1d8483610f1b565b606854604051639926ee7d60e01b81526001600160a01b0390911690639926ee7d90610d4f908790879060040161273c565b600060405180830381600087803b158015610d6957600080fd5b505af1158015610d7d573d6000803e3d6000fd5b50506068546040516001600160a01b03918216935090871691507fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c190600090a350505050565b60655481511461038a5760405163169efb5b60e11b815260040160405180910390fd5b6033546001600160a01b031633146105635760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016108d3565b610e4b606c82611601565b50506040518181527f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b9060200160405180910390a150565b606780549082905560408051828152602081018490527f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f91015b60405180910390a15050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6001600160a01b0382166000908152606a60205260408120610f3c90610c2f565b9050806001600160a01b0316826001600160a01b03161415610f5d57505050565b6001600160a01b038381166000908152606a60205260409020610f81918416611601565b50506040516001600160a01b0382811682528084169143918616907fd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea13150029060200160405180910390a4505050565b6001600160a01b0381166000908152606e602052604090205460ff16611007576040516325ec6c1f60e01b815260040160405180910390fd5b6065805490600061101783612787565b90915550506001600160a01b0381166000908152606e60205260408120805460ff19169055611045826112b1565b9050611050816113d4565b50506068546040516351b27a6d60e11b81526001600160a01b0384811660048301529091169063a364f4da90602401600060405180830381600087803b15801561109957600080fd5b505af11580156110ad573d6000803e3d6000fd5b50506068546040516001600160a01b03918216935090851691507f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed58090600090a35050565b600054610100900460ff166111185760405162461bcd60e51b81526004016108d39061279e565b606880546001600160a01b0319166001600160a01b03851617905561113c82610e40565b61114581611152565b61114d61172c565b505050565b61115b8161175b565b6111785760405163d173577960e01b815260040160405180910390fd5b60408051606680546020818102840185018552830181815260009484928491879085015b828210156111eb57600084815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b03168183015282526001909201910161119c565b5050509152509091506066905060006112048282611c84565b505060005b82515181101561127f5782518051606691908390811061122b5761122b61252c565b6020908102919091018101518254600181018455600093845292829020815191909201516001600160601b0316600160a01b026001600160a01b03909116179101558061127781612558565b915050611209565b507f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e8183604051610ebd9291906127e9565b6001600160a01b0381166000908152606d602052604081208190819081906112d890610c2f565b6001600160a01b0386166000908152606e602052604090205490915060ff1661133d576113058184612817565b9250826113155750909392505050565b6001600160a01b0385166000908152606d6020526040812061133691611601565b5050611387565b611346856105d3565b91506113528183612817565b9250826113625750909392505050565b6001600160a01b0385166000908152606d602052604090206113849083611601565b50505b60408051828152602081018490526001600160a01b038716917f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594910160405180910390a250909392505050565b6000806113e1606b610c2f565b915060006113ef84846126b8565b91508190506113ff606b82611601565b505060408051848152602081018490527f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b910160405180910390a150915091565b600061144f6002848418612696565b61051f9084841661267e565b80821461147e576040516001621398b960e31b0319815260040160405180910390fd5b816104d65760405163251f56a160e21b815260040160405180910390fd5b6000438263ffffffff16106114c45760405163e64f180f60e01b815260040160405180910390fd5b6001600160a01b0383166000908152606a6020526040902061051f9063ffffffff80851690610a6116565b806001600160a01b0316826001600160a01b0316106104d65760405163ba50f91160e01b815260040160405180910390fd5b6115356001600160a01b038416838361182b565b61114d57604051638baa579f60e01b815260040160405180910390fd5b6000438263ffffffff161061157a5760405163e64f180f60e01b815260040160405180910390fd5b6001600160a01b0383166000908152606d6020526040902061051f9063ffffffff80851690610a6116565b60006115b082611977565b9050808311156115d357604051634b05a0f760e11b815260040160405180910390fd5b60006115de836119b3565b9050838111156109505760405163e121632f60e01b815260040160405180910390fd5b815460009081908161161286610c2f565b90506000821180156116505750438661162c6001856126f9565b8154811061163c5761163c61252c565b60009182526020909120015463ffffffff16145b156116b05761165e856119ef565b8661166a6001856126f9565b8154811061167a5761167a61252c565b9060005260206000200160000160046101000a8154816001600160e01b0302191690836001600160e01b0316021790555061171e565b8560000160405180604001604052806116c843611a5c565b63ffffffff1681526020016116dc886119ef565b6001600160e01b0390811690915282546001810184556000938452602093849020835194909301519091166401000000000263ffffffff909316929092179101555b9250839150505b9250929050565b600054610100900460ff166117535760405162461bcd60e51b81526004016108d39061279e565b610563611ac1565b8051600090818080805b8451811015611809578481815181106117805761178061252c565b6020026020010151600001519250826001600160a01b0316846001600160a01b0316106117c05760405163ba50f91160e01b815260040160405180910390fd5b8293508481815181106117d5576117d561252c565b6020026020010151602001516001600160601b0316826117f5919061267e565b91508061180181612558565b915050611765565b50612710811461181f5750600095945050505050565b50600195945050505050565b600080600061183a8585611af1565b9092509050600081600481111561185357611853612856565b1480156118715750856001600160a01b0316826001600160a01b0316145b156118815760019250505061051f565b600080876001600160a01b0316631626ba7e60e01b88886040516024016118a992919061286c565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516118e79190612885565b600060405180830381855afa9150503d8060008114611922576040519150601f19603f3d011682016040523d82523d6000602084013e611927565b606091505b509150915081801561193a575080516020145b801561196b57508051630b135d3f60e11b9061195f90830160209081019084016128a1565b6001600160e01b031916145b98975050505050505050565b6000438263ffffffff161061199f5760405163e64f180f60e01b815260040160405180910390fd5b6103ac606b63ffffffff80851690610a6116565b6000438263ffffffff16106119db5760405163e64f180f60e01b815260040160405180910390fd5b6103ac606c63ffffffff80851690610a6116565b60006001600160e01b03821115611a585760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b60648201526084016108d3565b5090565b600063ffffffff821115611a585760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b60648201526084016108d3565b600054610100900460ff16611ae85760405162461bcd60e51b81526004016108d39061279e565b61056333610ec9565b600080825160411415611b285760208301516040840151606085015160001a611b1c87828585611b5e565b94509450505050611725565b825160401415611b525760208301516040840151611b47868383611c4b565b935093505050611725565b50600090506002611725565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0831115611b955750600090506003611c42565b8460ff16601b14158015611bad57508460ff16601c14155b15611bbe5750600090506004611c42565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015611c12573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b038116611c3b57600060019250925050611c42565b9150600090505b94509492505050565b6000806001600160ff1b03831681611c6860ff86901c601b61267e565b9050611c7687828885611b5e565b935093505050935093915050565b508054600082559060005260206000209081019061039391905b80821115611a585760008155600101611c9e565b634e487b7160e01b600052604160045260246000fd5b604051602081016001600160401b0381118282101715611cea57611cea611cb2565b60405290565b604080519081016001600160401b0381118282101715611cea57611cea611cb2565b604051601f8201601f191681016001600160401b0381118282101715611d3a57611d3a611cb2565b604052919050565b60006001600160401b03821115611d5b57611d5b611cb2565b5060051b60200190565b6001600160a01b038116811461039357600080fd5b600082601f830112611d8b57600080fd5b81356020611da0611d9b83611d42565b611d12565b82815260059290921b84018101918181019086841115611dbf57600080fd5b8286015b84811015611de3578035611dd681611d65565b8352918301918301611dc3565b509695505050505050565b600060208284031215611e0057600080fd5b81356001600160401b03811115611e1657600080fd5b611e2284828501611d7a565b949350505050565b63ffffffff8116811461039357600080fd5b600060208284031215611e4e57600080fd5b813561051f81611e2a565b60006001600160401b03821115611e7257611e72611cb2565b50601f01601f191660200190565b600082601f830112611e9157600080fd5b8135611e9f611d9b82611e59565b818152846020838601011115611eb457600080fd5b816020850160208301376000918101602001919091529392505050565b60008060408385031215611ee457600080fd5b8235915060208301356001600160401b03811115611f0157600080fd5b611f0d85828601611e80565b9150509250929050565b8051602080845281518482018190526000926040919083019082870190855b81811015611f6d57835180516001600160a01b031684528601516001600160601b0316868401529285019291840191600101611f36565b5090979650505050505050565b60208152600061051f6020830184611f17565b600060208284031215611f9f57600080fd5b813561051f81611d65565b60008060408385031215611fbd57600080fd5b82356001600160401b0380821115611fd457600080fd5b9084019060608287031215611fe857600080fd5b60405160608101818110838211171561200357612003611cb2565b60405282358281111561201557600080fd5b61202188828601611e80565b8252506020830135602082015260408301356040820152809450505050602083013561204c81611d65565b809150509250929050565b6000806040838503121561206a57600080fd5b82356001600160401b038082111561208157600080fd5b818501915085601f83011261209557600080fd5b813560206120a5611d9b83611d42565b82815260059290921b840181019181810190898411156120c457600080fd5b8286015b848110156120fc578035868111156120e05760008081fd5b6120ee8c86838b0101611d7a565b8452509183019183016120c8565b509650508601359250508082111561211357600080fd5b50611f0d85828601611e80565b6000806040838503121561213357600080fd5b823561213e81611d65565b946020939093013593505050565b60006020828403121561215e57600080fd5b5035919050565b6000806040838503121561217857600080fd5b8235915060208301356001600160401b0381111561219557600080fd5b611f0d85828601611d7a565b600080604083850312156121b457600080fd5b82356121bf81611d65565b9150602083013561204c81611e2a565b600060208083850312156121e257600080fd5b6121ea611cc8565b915082356001600160401b0381111561220257600080fd5b8301601f8101851361221357600080fd5b8035612221611d9b82611d42565b81815260069190911b8201830190838101908783111561224057600080fd5b928401925b828410156122a3576040848903121561225e5760008081fd5b612266611cf0565b843561227181611d65565b8152848601356001600160601b038116811461228d5760008081fd5b8187015282526040939093019290840190612245565b8552509295945050505050565b6000806000606084860312156122c557600080fd5b83356122d081611d65565b92506020840135915060408401356001600160401b038111156122f257600080fd5b6122fe868287016121cf565b9150509250925092565b6000806040838503121561231b57600080fd5b82356001600160401b038082111561233257600080fd5b61233e868387016121cf565b9350602085013591508082111561235457600080fd5b50611f0d85828601611d7a565b60005b8381101561237c578181015183820152602001612364565b838111156109505750506000910152565b600082601f83011261239e57600080fd5b815160206123ae611d9b83611d42565b82815260059290921b840181019181810190868411156123cd57600080fd5b8286015b84811015611de35780516001600160401b038111156123f05760008081fd5b8701603f810189136124025760008081fd5b848101516040612414611d9b83611e59565b8281528b828486010111156124295760008081fd5b61243883898301848701612361565b86525050509183019183016123d1565b805161245381611e2a565b919050565b60008060006060848603121561246d57600080fd5b83516001600160401b038082111561248457600080fd5b818601915086601f83011261249857600080fd5b815160206124a8611d9b83611d42565b82815260059290921b8401810191818101908a8411156124c757600080fd5b948201945b838610156124ee5785516124df81611d65565b825294820194908201906124cc565b9189015191975090935050508082111561250757600080fd5b506125148682870161238d565b92505061252360408501612448565b90509250925092565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600060001982141561256c5761256c612542565b5060010190565b6001600160a01b038381168252604060208084018290528451918401829052600092858201929091906060860190855b818110156125c15785518516835294830194918301916001016125a3565b509098975050505050505050565b600060208083850312156125e257600080fd5b82516001600160401b038111156125f857600080fd5b8301601f8101851361260957600080fd5b8051612617611d9b82611d42565b81815260059190911b8201830190838101908783111561263657600080fd5b928401925b828410156126545783518252928401929084019061263b565b979650505050505050565b600081600019048311821515161561267957612679612542565b500290565b6000821982111561269157612691612542565b500190565b6000826126b357634e487b7160e01b600052601260045260246000fd5b500490565b600080821280156001600160ff1b03849003851316156126da576126da612542565b600160ff1b83900384128116156126f3576126f3612542565b50500190565b60008282101561270b5761270b612542565b500390565b60008151808452612728816020860160208601612361565b601f01601f19169290920160200192915050565b60018060a01b038316815260406020820152600082516060604084015261276660a0840182612710565b90506020840151606084015260408401516080840152809150509392505050565b60008161279657612796612542565b506000190190565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b6040815260006127fc6040830185611f17565b828103602084015261280e8185611f17565b95945050505050565b60008083128015600160ff1b85018412161561283557612835612542565b6001600160ff1b038401831381161561285057612850612542565b50500390565b634e487b7160e01b600052602160045260246000fd5b828152604060208201526000611e226040830184612710565b60008251612897818460208701612361565b9190910192915050565b6000602082840312156128b357600080fd5b81516001600160e01b03198116811461051f57600080fdfea264697066735822122041a1848da3a333d4821839e44ee9cccc30bf436bcb37f47a0d3f0d7c8247493364736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0)\x958\x03\x80b\0)\x95\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0FV[`\x01`\x01`\xA0\x1B\x03\x16`\x80Rb\0\0xV[`\0` \x82\x84\x03\x12\x15b\0\0YW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0qW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa)\x01b\0\0\x94`\09`\0a\x07\x19\x01Ra)\x01`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01rW`\x005`\xE0\x1C\x80cibU\xBE\x11a\0\xDEW\x80c\x98\xEC\x1A\xC9\x11a\0\x97W\x80c\xCD\xCD5\x81\x11a\0qW\x80c\xCD\xCD5\x81\x14a\x03\x15W\x80c\xDE\xC5\xD1\xF6\x14a\x03(W\x80c\xEC\x7F\xBB1\x14a\x03;W\x80c\xF2\xFD\xE3\x8B\x14a\x03wW`\0\x80\xFD[\x80c\x98\xEC\x1A\xC9\x14a\x02\xE7W\x80c\xAB\x11\x89\x95\x14a\x02\xFAW\x80c\xB93\xFAt\x14a\x03\rW`\0\x80\xFD[\x80cibU\xBE\x14a\x02\x8DW\x80cqP\x18\xA6\x14a\x02\xA0W\x80ct<1\xF4\x14a\x02\xA8W\x80c\x85}\xC1\x90\x14a\x02\xBBW\x80c\x8D\xA5\xCB[\x14a\x02\xC3W\x80c\x95_-\x90\x14a\x02\xD4W`\0\x80\xFD[\x80c;$.J\x11a\x010W\x80c;$.J\x14a\x02\x0EW\x80c=V\x11\xF6\x14a\x02!W\x80c@\xBF/\xB7\x14a\x024W\x80cQ@\xA5H\x14a\x02<W\x80c^\x10B\xE8\x14a\x02OW\x80c^\xF53)\x14a\x02zW`\0\x80\xFD[\x80b\xCF*\xB5\x14a\x01wW\x80c\r\xBA3\x94\x14a\x01\x8CW\x80c\x16&\xBA~\x14a\x01\xB2W\x80c\x17\x03\xA0\x18\x14a\x01\xDEW\x80c\x1EL\xD8^\x14a\x01\xF3W\x80c1O:I\x14a\x02\x06W[`\0\x80\xFD[a\x01\x8Aa\x01\x856`\x04a\x1D\xEEV[a\x03\x8AV[\0[a\x01\x9Fa\x01\x9A6`\x04a\x1E<V[a\x03\x96V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xC5a\x01\xC06`\x04a\x1E\xD1V[a\x03\xB2V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x01\xA9V[a\x01\xE6a\x03\xF0V[`@Qa\x01\xA9\x91\x90a\x1FzV[a\x01\x9Fa\x02\x016`\x04a\x1E<V[a\x04\x83V[a\x01\x9Fa\x04\x99V[a\x01\x9Fa\x02\x1C6`\x04a\x1F\x8DV[a\x04\xAAV[a\x01\x8Aa\x02/6`\x04a\x1F\xAAV[a\x04\xCBV[`gTa\x01\x9FV[a\x01\x8Aa\x02J6`\x04a WV[a\x04\xDAV[a\x02ba\x02]6`\x04a! V[a\x04\xFDV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xA9V[a\x01\x8Aa\x02\x886`\x04a!LV[a\x05&V[a\x01\x8Aa\x02\x9B6`\x04a!eV[a\x057V[a\x01\x8Aa\x05QV[a\x01\x8Aa\x02\xB66`\x04a\x1F\x8DV[a\x05eV[a\x01\x8Aa\x05\x9FV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02bV[a\x01\x9Fa\x02\xE26`\x04a!\xA1V[a\x05\xA8V[a\x01\x9Fa\x02\xF56`\x04a\x1F\x8DV[a\x05\xD3V[a\x01\x8Aa\x03\x086`\x04a\"\xB0V[a\x08:V[a\x01\x9Fa\tVV[a\x02ba\x03#6`\x04a\x1F\x8DV[a\tbV[a\x01\x8Aa\x0366`\x04a#\x08V[a\t\x83V[a\x03ga\x03I6`\x04a\x1F\x8DV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`n` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xA9V[a\x01\x8Aa\x03\x856`\x04a\x1F\x8DV[a\t\x94V[a\x03\x93\x81a\n\nV[PV[`\0a\x03\xAC`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[\x92\x91PPV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03\xCC\x91\x90a$XV[\x92P\x92P\x92Pa\x03\xDE\x86\x84\x84\x84a\x0BpV[Pc\x0B\x13]?`\xE1\x1B\x95\x94PPPPPV[`@\x80Q` \x81\x01\x90\x91R``\x81R`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R\x92\x93\x91\x92\x84\x92\x90\x91\x84\x91`\0\x90\x85\x01[\x82\x82\x10\x15a\x04vW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x04'V[PPPP\x81RPP\x90P\x90V[`\0a\x03\xAC`lc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[`\0a\x04\xA5`ka\x0C/V[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`m` R`@\x81 a\x03\xAC\x90a\x0C/V[a\x04\xD63\x83\x83a\x0C\x8BV[PPV[a\x04\xD6\x82`\0\x81Q\x81\x10a\x04\xF0Wa\x04\xF0a%,V[` \x02` \x01\x01Qa\r\xC3V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`j` R`@\x81 a\x05\x1F\x90\x83a\naV[\x93\x92PPPV[a\x05.a\r\xE6V[a\x03\x93\x81a\x0E@V[a\x05?a\r\xE6V[a\x05H\x82a\x0E\x83V[a\x04\xD6\x81a\n\nV[a\x05Ya\r\xE6V[a\x05c`\0a\x0E\xC9V[V[3`\0\x90\x81R`n` R`@\x90 T`\xFF\x16a\x05\x95W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x933\x82a\x0F\x1BV[a\x05c3a\x0F\xCEV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`m` R`@\x81 a\x05\x1F\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[`\0\x80`f`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06JW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x05\xFBV[PPPP\x90P`\0\x80\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06lWa\x06la\x1C\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\x95W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x06\xFEW\x83\x81\x81Q\x81\x10a\x06\xB6Wa\x06\xB6a%,V[` \x02` \x01\x01Q`\0\x01Q\x82\x82\x81Q\x81\x10a\x06\xD4Wa\x06\xD4a%,V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x06\xF6\x81a%XV[\x91PPa\x06\x9BV[P`@Qc\x90\x04\x13G`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x90\x04\x13G\x90a\x07P\x90\x89\x90\x86\x90`\x04\x01a%sV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\x95\x91\x90\x81\x01\x90a%\xCFV[\x90P`\0[\x84Q\x81\x10\x15a\x08\x0CW\x84\x81\x81Q\x81\x10a\x07\xB5Wa\x07\xB5a%,V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x07\xDCWa\x07\xDCa%,V[` \x02` \x01\x01Qa\x07\xEE\x91\x90a&_V[a\x07\xF8\x90\x85a&~V[\x93P\x80a\x08\x04\x81a%XV[\x91PPa\x07\x9AV[Pa\x08\x19a'\x10\x84a&\x96V[\x92P`gT\x83\x10a\x08.WP\x90\x94\x93PPPPV[P`\0\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x08ZWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x08tWP0;\x15\x80\x15a\x08tWP`\0T`\xFF\x16`\x01\x14[a\x08\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x08\xFFW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\t\n\x84\x84\x84a\x10\xF1V[\x80\x15a\tPW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[`\0a\x04\xA5`la\x0C/V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`j` R`@\x81 a\x03\xAC\x90a\x0C/V[a\t\x8Ba\r\xE6V[a\x05H\x82a\x11RV[a\t\x9Ca\r\xE6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\n\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\xD3V[a\x03\x93\x81a\x0E\xC9V[`\0\x80[\x82Q\x81\x10\x15a\nWWa\n9\x83\x82\x81Q\x81\x10a\n,Wa\n,a%,V[` \x02` \x01\x01Qa\x12\xB1V[a\nC\x90\x83a&\xB8V[\x91P\x80a\nO\x81a%XV[\x91PPa\n\x0EV[Pa\tP\x81a\x13\xD4V[`\0C\x82\x10a\n\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R`d\x01a\x08\xD3V[\x82T`\0[\x81\x81\x10\x15a\x0B\x17W`\0a\n\xCB\x82\x84a\x14@V[\x90P\x84\x86`\0\x01\x82\x81T\x81\x10a\n\xE3Wa\n\xE3a%,V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\x03W\x80\x92Pa\x0B\x11V[a\x0B\x0E\x81`\x01a&~V[\x91P[Pa\n\xB7V[\x81\x15a\x0B[W\x84a\x0B)`\x01\x84a&\xF9V[\x81T\x81\x10a\x0B9Wa\x0B9a%,V[`\0\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0B^V[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x95\x94PPPPPV[`\0\x83Q\x90P`\0\x80`\0\x80a\x0B\x87\x85\x88Qa\x14[V[`\0[\x85\x81\x10\x15a\x0C\x19W\x88\x81\x81Q\x81\x10a\x0B\xA4Wa\x0B\xA4a%,V[` \x02` \x01\x01Q\x94Pa\x0B\xB8\x85\x88a\x14\x9CV[\x92Pa\x0B\xC4\x84\x86a\x14\xEFV[a\x0B\xE8\x83\x8B\x8A\x84\x81Q\x81\x10a\x0B\xDBWa\x0B\xDBa%,V[` \x02` \x01\x01Qa\x15!V[\x84\x93P`\0a\x0B\xF7\x86\x89a\x15RV[\x90Pa\x0C\x03\x81\x84a&~V[\x92PP\x80\x80a\x0C\x11\x90a%XV[\x91PPa\x0B\x8AV[Pa\x0C$\x81\x87a\x15\xA5V[PPPPPPPPPV[\x80T`\0\x90\x80\x15a\x0CxW\x82a\x0CF`\x01\x83a&\xF9V[\x81T\x81\x10a\x0CVWa\x0CVa%,V[`\0\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0C{V[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`n` R`@\x90 T`\xFF\x16\x15a\x0C\xC5W`@QcB\xEEh\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90`\0a\x0C\xD5\x83a%XV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`n` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\r\x06\x84a\x12\xB1V[\x90Pa\r\x11\x81a\x13\xD4V[PPa\r\x1D\x84\x83a\x0F\x1BV[`hT`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x99&\xEE}\x90a\rO\x90\x87\x90\x87\x90`\x04\x01a'<V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\riW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r}W=`\0\x80>=`\0\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x87\x16\x91P\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90`\0\x90\xA3PPPPV[`eT\x81Q\x14a\x03\x8AW`@Qc\x16\x9E\xFB[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08\xD3V[a\x0EK`l\x82a\x16\x01V[PP`@Q\x81\x81R\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`g\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8F\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`j` R`@\x81 a\x0F<\x90a\x0C/V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0F]WPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`j` R`@\x90 a\x0F\x81\x91\x84\x16a\x16\x01V[PP`@Q`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82R\x80\x84\x16\x91C\x91\x86\x16\x90\x7F\xD0a\x16\x82R\xF4As6X\xF0\x9EM\x8F[-\x99\x8E\xD4\xEF$\xA2\xBB\xFDl\xEC\xA5.\xA11P\x02\x90` \x01`@Q\x80\x91\x03\x90\xA4PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`n` R`@\x90 T`\xFF\x16a\x10\x07W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90`\0a\x10\x17\x83a'\x87V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`n` R`@\x81 \x80T`\xFF\x19\x16\x90Ua\x10E\x82a\x12\xB1V[\x90Pa\x10P\x81a\x13\xD4V[PP`hT`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xA3d\xF4\xDA\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x99W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xADW=`\0\x80>=`\0\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x85\x16\x91P\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x11\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xD3\x90a'\x9EV[`h\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua\x11<\x82a\x0E@V[a\x11E\x81a\x11RV[a\x11Ma\x17,V[PPPV[a\x11[\x81a\x17[V[a\x11xW`@Qc\xD1sWy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R`\0\x94\x84\x92\x84\x91\x87\x90\x85\x01[\x82\x82\x10\x15a\x11\xEBW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x11\x9CV[PPP\x91RP\x90\x91P`f\x90P`\0a\x12\x04\x82\x82a\x1C\x84V[PP`\0[\x82QQ\x81\x10\x15a\x12\x7FW\x82Q\x80Q`f\x91\x90\x83\x90\x81\x10a\x12+Wa\x12+a%,V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x92\x82\x90 \x81Q\x91\x90\x92\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x91\x01U\x80a\x12w\x81a%XV[\x91PPa\x12\tV[P\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x81\x83`@Qa\x0E\xBD\x92\x91\x90a'\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`m` R`@\x81 \x81\x90\x81\x90\x81\x90a\x12\xD8\x90a\x0C/V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`n` R`@\x90 T\x90\x91P`\xFF\x16a\x13=Wa\x13\x05\x81\x84a(\x17V[\x92P\x82a\x13\x15WP\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`m` R`@\x81 a\x136\x91a\x16\x01V[PPa\x13\x87V[a\x13F\x85a\x05\xD3V[\x91Pa\x13R\x81\x83a(\x17V[\x92P\x82a\x13bWP\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`m` R`@\x90 a\x13\x84\x90\x83a\x16\x01V[PP[`@\x80Q\x82\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16\x91\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x91\x01`@Q\x80\x91\x03\x90\xA2P\x90\x93\x92PPPV[`\0\x80a\x13\xE1`ka\x0C/V[\x91P`\0a\x13\xEF\x84\x84a&\xB8V[\x91P\x81\x90Pa\x13\xFF`k\x82a\x16\x01V[PP`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B\x91\x01`@Q\x80\x91\x03\x90\xA1P\x91P\x91V[`\0a\x14O`\x02\x84\x84\x18a&\x96V[a\x05\x1F\x90\x84\x84\x16a&~V[\x80\x82\x14a\x14~W`@Q`\x01b\x13\x98\xB9`\xE3\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81a\x04\xD6W`@Qc%\x1FV\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x14\xC4W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`j` R`@\x90 a\x05\x1F\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x10a\x04\xD6W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x155`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a\x18+V[a\x11MW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x15zW`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`m` R`@\x90 a\x05\x1F\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[`\0a\x15\xB0\x82a\x19wV[\x90P\x80\x83\x11\x15a\x15\xD3W`@QcK\x05\xA0\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x15\xDE\x83a\x19\xB3V[\x90P\x83\x81\x11\x15a\tPW`@Qc\xE1!c/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T`\0\x90\x81\x90\x81a\x16\x12\x86a\x0C/V[\x90P`\0\x82\x11\x80\x15a\x16PWPC\x86a\x16,`\x01\x85a&\xF9V[\x81T\x81\x10a\x16<Wa\x16<a%,V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x16\xB0Wa\x16^\x85a\x19\xEFV[\x86a\x16j`\x01\x85a&\xF9V[\x81T\x81\x10a\x16zWa\x16za%,V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xE0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xE0\x1B\x03\x16\x02\x17\x90UPa\x17\x1EV[\x85`\0\x01`@Q\x80`@\x01`@R\x80a\x16\xC8Ca\x1A\\V[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x16\xDC\x88a\x19\xEFV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[\x92P\x83\x91PP[\x92P\x92\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x17SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xD3\x90a'\x9EV[a\x05ca\x1A\xC1V[\x80Q`\0\x90\x81\x80\x80\x80[\x84Q\x81\x10\x15a\x18\tW\x84\x81\x81Q\x81\x10a\x17\x80Wa\x17\x80a%,V[` \x02` \x01\x01Q`\0\x01Q\x92P\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x17\xC0W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x93P\x84\x81\x81Q\x81\x10a\x17\xD5Wa\x17\xD5a%,V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82a\x17\xF5\x91\x90a&~V[\x91P\x80a\x18\x01\x81a%XV[\x91PPa\x17eV[Pa'\x10\x81\x14a\x18\x1FWP`\0\x95\x94PPPPPV[P`\x01\x95\x94PPPPPV[`\0\x80`\0a\x18:\x85\x85a\x1A\xF1V[\x90\x92P\x90P`\0\x81`\x04\x81\x11\x15a\x18SWa\x18Sa(VV[\x14\x80\x15a\x18qWP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x18\x81W`\x01\x92PPPa\x05\x1FV[`\0\x80\x87`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01a\x18\xA9\x92\x91\x90a(lV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x18\xE7\x91\x90a(\x85V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x19\"W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19'V[``\x91P[P\x91P\x91P\x81\x80\x15a\x19:WP\x80Q` \x14[\x80\x15a\x19kWP\x80Qc\x0B\x13]?`\xE1\x1B\x90a\x19_\x90\x83\x01` \x90\x81\x01\x90\x84\x01a(\xA1V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14[\x98\x97PPPPPPPPV[`\0C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x19\x9FW`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xAC`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[`\0C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x19\xDBW`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xAC`lc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x1AXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x08\xD3V[P\x90V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x1AXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\xD3V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xD3\x90a'\x9EV[a\x05c3a\x0E\xC9V[`\0\x80\x82Q`A\x14\x15a\x1B(W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x1B\x1C\x87\x82\x85\x85a\x1B^V[\x94P\x94PPPPa\x17%V[\x82Q`@\x14\x15a\x1BRW` \x83\x01Q`@\x84\x01Qa\x1BG\x86\x83\x83a\x1CKV[\x93P\x93PPPa\x17%V[P`\0\x90P`\x02a\x17%V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1B\x95WP`\0\x90P`\x03a\x1CBV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a\x1B\xADWP\x84`\xFF\x16`\x1C\x14\x15[\x15a\x1B\xBEWP`\0\x90P`\x04a\x1CBV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1C\x12W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C;W`\0`\x01\x92P\x92PPa\x1CBV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a\x1Ch`\xFF\x86\x90\x1C`\x1Ba&~V[\x90Pa\x1Cv\x87\x82\x88\x85a\x1B^V[\x93P\x93PPP\x93P\x93\x91PPV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x03\x93\x91\x90[\x80\x82\x11\x15a\x1AXW`\0\x81U`\x01\x01a\x1C\x9EV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xEAWa\x1C\xEAa\x1C\xB2V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xEAWa\x1C\xEAa\x1C\xB2V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D:Wa\x1D:a\x1C\xB2V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1D[Wa\x1D[a\x1C\xB2V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x93W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x1D\x8BW`\0\x80\xFD[\x815` a\x1D\xA0a\x1D\x9B\x83a\x1DBV[a\x1D\x12V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x1D\xBFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1D\xE3W\x805a\x1D\xD6\x81a\x1DeV[\x83R\x91\x83\x01\x91\x83\x01a\x1D\xC3V[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1E\0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\x16W`\0\x80\xFD[a\x1E\"\x84\x82\x85\x01a\x1DzV[\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\x93W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1ENW`\0\x80\xFD[\x815a\x05\x1F\x81a\x1E*V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1ErWa\x1Era\x1C\xB2V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1E\x91W`\0\x80\xFD[\x815a\x1E\x9Fa\x1D\x9B\x82a\x1EYV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1E\xB4W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1E\xE4W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\x01W`\0\x80\xFD[a\x1F\r\x85\x82\x86\x01a\x1E\x80V[\x91PP\x92P\x92\x90PV[\x80Q` \x80\x84R\x81Q\x84\x82\x01\x81\x90R`\0\x92`@\x91\x90\x83\x01\x90\x82\x87\x01\x90\x85[\x81\x81\x10\x15a\x1FmW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x86\x01Q`\x01`\x01``\x1B\x03\x16\x86\x84\x01R\x92\x85\x01\x92\x91\x84\x01\x91`\x01\x01a\x1F6V[P\x90\x97\x96PPPPPPPV[` \x81R`\0a\x05\x1F` \x83\x01\x84a\x1F\x17V[`\0` \x82\x84\x03\x12\x15a\x1F\x9FW`\0\x80\xFD[\x815a\x05\x1F\x81a\x1DeV[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\xBDW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1F\xD4W`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x1F\xE8W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a \x03Wa \x03a\x1C\xB2V[`@R\x825\x82\x81\x11\x15a \x15W`\0\x80\xFD[a !\x88\x82\x86\x01a\x1E\x80V[\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x94PPPP` \x83\x015a L\x81a\x1DeV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a jW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a \x81W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a \x95W`\0\x80\xFD[\x815` a \xA5a\x1D\x9B\x83a\x1DBV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a \xC4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xFCW\x805\x86\x81\x11\x15a \xE0W`\0\x80\x81\xFD[a \xEE\x8C\x86\x83\x8B\x01\x01a\x1DzV[\x84RP\x91\x83\x01\x91\x83\x01a \xC8V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a!\x13W`\0\x80\xFD[Pa\x1F\r\x85\x82\x86\x01a\x1E\x80V[`\0\x80`@\x83\x85\x03\x12\x15a!3W`\0\x80\xFD[\x825a!>\x81a\x1DeV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a!^W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a!xW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x95W`\0\x80\xFD[a\x1F\r\x85\x82\x86\x01a\x1DzV[`\0\x80`@\x83\x85\x03\x12\x15a!\xB4W`\0\x80\xFD[\x825a!\xBF\x81a\x1DeV[\x91P` \x83\x015a L\x81a\x1E*V[`\0` \x80\x83\x85\x03\x12\x15a!\xE2W`\0\x80\xFD[a!\xEAa\x1C\xC8V[\x91P\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x02W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\"\x13W`\0\x80\xFD[\x805a\"!a\x1D\x9B\x82a\x1DBV[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\"@W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\"\xA3W`@\x84\x89\x03\x12\x15a\"^W`\0\x80\x81\xFD[a\"fa\x1C\xF0V[\x845a\"q\x81a\x1DeV[\x81R\x84\x86\x015`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\"\x8DW`\0\x80\x81\xFD[\x81\x87\x01R\x82R`@\x93\x90\x93\x01\x92\x90\x84\x01\x90a\"EV[\x85RP\x92\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"\xC5W`\0\x80\xFD[\x835a\"\xD0\x81a\x1DeV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xF2W`\0\x80\xFD[a\"\xFE\x86\x82\x87\x01a!\xCFV[\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a#\x1BW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a#2W`\0\x80\xFD[a#>\x86\x83\x87\x01a!\xCFV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a#TW`\0\x80\xFD[Pa\x1F\r\x85\x82\x86\x01a\x1DzV[`\0[\x83\x81\x10\x15a#|W\x81\x81\x01Q\x83\x82\x01R` \x01a#dV[\x83\x81\x11\x15a\tPWPP`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a#\x9EW`\0\x80\xFD[\x81Q` a#\xAEa\x1D\x9B\x83a\x1DBV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a#\xCDW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1D\xE3W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xF0W`\0\x80\x81\xFD[\x87\x01`?\x81\x01\x89\x13a$\x02W`\0\x80\x81\xFD[\x84\x81\x01Q`@a$\x14a\x1D\x9B\x83a\x1EYV[\x82\x81R\x8B\x82\x84\x86\x01\x01\x11\x15a$)W`\0\x80\x81\xFD[a$8\x83\x89\x83\x01\x84\x87\x01a#aV[\x86RPPP\x91\x83\x01\x91\x83\x01a#\xD1V[\x80Qa$S\x81a\x1E*V[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a$mW`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a$\x84W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a$\x98W`\0\x80\xFD[\x81Q` a$\xA8a\x1D\x9B\x83a\x1DBV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a$\xC7W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a$\xEEW\x85Qa$\xDF\x81a\x1DeV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a$\xCCV[\x91\x89\x01Q\x91\x97P\x90\x93PPP\x80\x82\x11\x15a%\x07W`\0\x80\xFD[Pa%\x14\x86\x82\x87\x01a#\x8DV[\x92PPa%#`@\x85\x01a$HV[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a%lWa%la%BV[P`\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`@` \x80\x84\x01\x82\x90R\x84Q\x91\x84\x01\x82\x90R`\0\x92\x85\x82\x01\x92\x90\x91\x90``\x86\x01\x90\x85[\x81\x81\x10\x15a%\xC1W\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a%\xA3V[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a%\xE2W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xF8W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a&\tW`\0\x80\xFD[\x80Qa&\x17a\x1D\x9B\x82a\x1DBV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a&6W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a&TW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a&;V[\x97\x96PPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a&yWa&ya%BV[P\x02\x90V[`\0\x82\x19\x82\x11\x15a&\x91Wa&\x91a%BV[P\x01\x90V[`\0\x82a&\xB3WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80\x82\x12\x80\x15`\x01`\x01`\xFF\x1B\x03\x84\x90\x03\x85\x13\x16\x15a&\xDAWa&\xDAa%BV[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15a&\xF3Wa&\xF3a%BV[PP\x01\x90V[`\0\x82\x82\x10\x15a'\x0BWa'\x0Ba%BV[P\x03\x90V[`\0\x81Q\x80\x84Ra'(\x81` \x86\x01` \x86\x01a#aV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra'f`\xA0\x84\x01\x82a'\x10V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[`\0\x81a'\x96Wa'\x96a%BV[P`\0\x19\x01\x90V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R`\0a'\xFC`@\x83\x01\x85a\x1F\x17V[\x82\x81\x03` \x84\x01Ra(\x0E\x81\x85a\x1F\x17V[\x95\x94PPPPPV[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a(5Wa(5a%BV[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a(PWa(Pa%BV[PP\x03\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x82\x81R`@` \x82\x01R`\0a\x1E\"`@\x83\x01\x84a'\x10V[`\0\x82Qa(\x97\x81\x84` \x87\x01a#aV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a(\xB3W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x05\x1FW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 A\xA1\x84\x8D\xA3\xA33\xD4\x82\x189\xE4N\xE9\xCC\xCC0\xBFCk\xCB7\xF4z\r?\r|\x82GI3dsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106101725760003560e01c8063696255be116100de57806398ec1ac911610097578063cdcd358111610071578063cdcd358114610315578063dec5d1f614610328578063ec7fbb311461033b578063f2fde38b1461037757600080fd5b806398ec1ac9146102e7578063ab118995146102fa578063b933fa741461030d57600080fd5b8063696255be1461028d578063715018a6146102a0578063743c31f4146102a8578063857dc190146102bb5780638da5cb5b146102c3578063955f2d90146102d457600080fd5b80633b242e4a116101305780633b242e4a1461020e5780633d5611f61461022157806340bf2fb7146102345780635140a5481461023c5780635e1042e81461024f5780635ef533291461027a57600080fd5b8062cf2ab5146101775780630dba33941461018c5780631626ba7e146101b25780631703a018146101de5780631e4cd85e146101f3578063314f3a4914610206575b600080fd5b61018a610185366004611dee565b61038a565b005b61019f61019a366004611e3c565b610396565b6040519081526020015b60405180910390f35b6101c56101c0366004611ed1565b6103b2565b6040516001600160e01b031990911681526020016101a9565b6101e66103f0565b6040516101a99190611f7a565b61019f610201366004611e3c565b610483565b61019f610499565b61019f61021c366004611f8d565b6104aa565b61018a61022f366004611faa565b6104cb565b60675461019f565b61018a61024a366004612057565b6104da565b61026261025d366004612120565b6104fd565b6040516001600160a01b0390911681526020016101a9565b61018a61028836600461214c565b610526565b61018a61029b366004612165565b610537565b61018a610551565b61018a6102b6366004611f8d565b610565565b61018a61059f565b6033546001600160a01b0316610262565b61019f6102e23660046121a1565b6105a8565b61019f6102f5366004611f8d565b6105d3565b61018a6103083660046122b0565b61083a565b61019f610956565b610262610323366004611f8d565b610962565b61018a610336366004612308565b610983565b610367610349366004611f8d565b6001600160a01b03166000908152606e602052604090205460ff1690565b60405190151581526020016101a9565b61018a610385366004611f8d565b610994565b61039381610a0a565b50565b60006103ac606b63ffffffff80851690610a6116565b92915050565b600080600080848060200190518101906103cc9190612458565b9250925092506103de86848484610b70565b50630b135d3f60e11b95945050505050565b6040805160208101909152606081526040805160668054602081810284018501855283018181529293919284929091849160009085015b8282101561047657600084815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b031681830152825260019092019101610427565b5050505081525050905090565b60006103ac606c63ffffffff80851690610a6116565b60006104a5606b610c2f565b905090565b6001600160a01b0381166000908152606d602052604081206103ac90610c2f565b6104d6338383610c8b565b5050565b6104d6826000815181106104f0576104f061252c565b6020026020010151610dc3565b6001600160a01b0382166000908152606a6020526040812061051f9083610a61565b9392505050565b61052e610de6565b61039381610e40565b61053f610de6565b61054882610e83565b6104d681610a0a565b610559610de6565b6105636000610ec9565b565b336000908152606e602052604090205460ff16610595576040516325ec6c1f60e01b815260040160405180910390fd5b6103933382610f1b565b61056333610fce565b6001600160a01b0382166000908152606d6020526040812061051f9063ffffffff80851690610a6116565b6000806066600001805480602002602001604051908101604052809291908181526020016000905b8282101561064a57600084815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b0316818301528252600190920191016105fb565b50505050905060008082516001600160401b0381111561066c5761066c611cb2565b604051908082528060200260200182016040528015610695578160200160208202803683370190505b50905060005b83518110156106fe578381815181106106b6576106b661252c565b6020026020010151600001518282815181106106d4576106d461252c565b6001600160a01b0390921660209283029190910190910152806106f681612558565b91505061069b565b50604051639004134760e01b81526000906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906390041347906107509089908690600401612573565b600060405180830381865afa15801561076d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261079591908101906125cf565b905060005b845181101561080c578481815181106107b5576107b561252c565b6020026020010151602001516001600160601b03168282815181106107dc576107dc61252c565b60200260200101516107ee919061265f565b6107f8908561267e565b93508061080481612558565b91505061079a565b5061081961271084612696565b9250606754831061082e575090949350505050565b50600095945050505050565b600054610100900460ff161580801561085a5750600054600160ff909116105b806108745750303b158015610874575060005460ff166001145b6108dc5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b6000805460ff1916600117905580156108ff576000805461ff0019166101001790555b61090a8484846110f1565b8015610950576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b60006104a5606c610c2f565b6001600160a01b0381166000908152606a602052604081206103ac90610c2f565b61098b610de6565b61054882611152565b61099c610de6565b6001600160a01b038116610a015760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016108d3565b61039381610ec9565b6000805b8251811015610a5757610a39838281518110610a2c57610a2c61252c565b60200260200101516112b1565b610a4390836126b8565b915080610a4f81612558565b915050610a0e565b50610950816113d4565b6000438210610ab25760405162461bcd60e51b815260206004820181905260248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e656460448201526064016108d3565b825460005b81811015610b17576000610acb8284611440565b905084866000018281548110610ae357610ae361252c565b60009182526020909120015463ffffffff161115610b0357809250610b11565b610b0e81600161267e565b91505b50610ab7565b8115610b5b5784610b296001846126f9565b81548110610b3957610b3961252c565b60009182526020909120015464010000000090046001600160e01b0316610b5e565b60005b6001600160e01b031695945050505050565b600083519050600080600080610b8785885161145b565b60005b85811015610c1957888181518110610ba457610ba461252c565b60200260200101519450610bb8858861149c565b9250610bc484866114ef565b610be8838b8a8481518110610bdb57610bdb61252c565b6020026020010151611521565b8493506000610bf78689611552565b9050610c03818461267e565b9250508080610c1190612558565b915050610b8a565b50610c2481876115a5565b505050505050505050565b80546000908015610c785782610c466001836126f9565b81548110610c5657610c5661252c565b60009182526020909120015464010000000090046001600160e01b0316610c7b565b60005b6001600160e01b03169392505050565b6001600160a01b0383166000908152606e602052604090205460ff1615610cc5576040516342ee68b560e01b815260040160405180910390fd5b60658054906000610cd583612558565b90915550506001600160a01b0383166000908152606e60205260408120805460ff19166001179055610d06846112b1565b9050610d11816113d4565b5050610d1d8483610f1b565b606854604051639926ee7d60e01b81526001600160a01b0390911690639926ee7d90610d4f908790879060040161273c565b600060405180830381600087803b158015610d6957600080fd5b505af1158015610d7d573d6000803e3d6000fd5b50506068546040516001600160a01b03918216935090871691507fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c190600090a350505050565b60655481511461038a5760405163169efb5b60e11b815260040160405180910390fd5b6033546001600160a01b031633146105635760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016108d3565b610e4b606c82611601565b50506040518181527f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b9060200160405180910390a150565b606780549082905560408051828152602081018490527f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f91015b60405180910390a15050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6001600160a01b0382166000908152606a60205260408120610f3c90610c2f565b9050806001600160a01b0316826001600160a01b03161415610f5d57505050565b6001600160a01b038381166000908152606a60205260409020610f81918416611601565b50506040516001600160a01b0382811682528084169143918616907fd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea13150029060200160405180910390a4505050565b6001600160a01b0381166000908152606e602052604090205460ff16611007576040516325ec6c1f60e01b815260040160405180910390fd5b6065805490600061101783612787565b90915550506001600160a01b0381166000908152606e60205260408120805460ff19169055611045826112b1565b9050611050816113d4565b50506068546040516351b27a6d60e11b81526001600160a01b0384811660048301529091169063a364f4da90602401600060405180830381600087803b15801561109957600080fd5b505af11580156110ad573d6000803e3d6000fd5b50506068546040516001600160a01b03918216935090851691507f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed58090600090a35050565b600054610100900460ff166111185760405162461bcd60e51b81526004016108d39061279e565b606880546001600160a01b0319166001600160a01b03851617905561113c82610e40565b61114581611152565b61114d61172c565b505050565b61115b8161175b565b6111785760405163d173577960e01b815260040160405180910390fd5b60408051606680546020818102840185018552830181815260009484928491879085015b828210156111eb57600084815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b03168183015282526001909201910161119c565b5050509152509091506066905060006112048282611c84565b505060005b82515181101561127f5782518051606691908390811061122b5761122b61252c565b6020908102919091018101518254600181018455600093845292829020815191909201516001600160601b0316600160a01b026001600160a01b03909116179101558061127781612558565b915050611209565b507f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e8183604051610ebd9291906127e9565b6001600160a01b0381166000908152606d602052604081208190819081906112d890610c2f565b6001600160a01b0386166000908152606e602052604090205490915060ff1661133d576113058184612817565b9250826113155750909392505050565b6001600160a01b0385166000908152606d6020526040812061133691611601565b5050611387565b611346856105d3565b91506113528183612817565b9250826113625750909392505050565b6001600160a01b0385166000908152606d602052604090206113849083611601565b50505b60408051828152602081018490526001600160a01b038716917f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594910160405180910390a250909392505050565b6000806113e1606b610c2f565b915060006113ef84846126b8565b91508190506113ff606b82611601565b505060408051848152602081018490527f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b910160405180910390a150915091565b600061144f6002848418612696565b61051f9084841661267e565b80821461147e576040516001621398b960e31b0319815260040160405180910390fd5b816104d65760405163251f56a160e21b815260040160405180910390fd5b6000438263ffffffff16106114c45760405163e64f180f60e01b815260040160405180910390fd5b6001600160a01b0383166000908152606a6020526040902061051f9063ffffffff80851690610a6116565b806001600160a01b0316826001600160a01b0316106104d65760405163ba50f91160e01b815260040160405180910390fd5b6115356001600160a01b038416838361182b565b61114d57604051638baa579f60e01b815260040160405180910390fd5b6000438263ffffffff161061157a5760405163e64f180f60e01b815260040160405180910390fd5b6001600160a01b0383166000908152606d6020526040902061051f9063ffffffff80851690610a6116565b60006115b082611977565b9050808311156115d357604051634b05a0f760e11b815260040160405180910390fd5b60006115de836119b3565b9050838111156109505760405163e121632f60e01b815260040160405180910390fd5b815460009081908161161286610c2f565b90506000821180156116505750438661162c6001856126f9565b8154811061163c5761163c61252c565b60009182526020909120015463ffffffff16145b156116b05761165e856119ef565b8661166a6001856126f9565b8154811061167a5761167a61252c565b9060005260206000200160000160046101000a8154816001600160e01b0302191690836001600160e01b0316021790555061171e565b8560000160405180604001604052806116c843611a5c565b63ffffffff1681526020016116dc886119ef565b6001600160e01b0390811690915282546001810184556000938452602093849020835194909301519091166401000000000263ffffffff909316929092179101555b9250839150505b9250929050565b600054610100900460ff166117535760405162461bcd60e51b81526004016108d39061279e565b610563611ac1565b8051600090818080805b8451811015611809578481815181106117805761178061252c565b6020026020010151600001519250826001600160a01b0316846001600160a01b0316106117c05760405163ba50f91160e01b815260040160405180910390fd5b8293508481815181106117d5576117d561252c565b6020026020010151602001516001600160601b0316826117f5919061267e565b91508061180181612558565b915050611765565b50612710811461181f5750600095945050505050565b50600195945050505050565b600080600061183a8585611af1565b9092509050600081600481111561185357611853612856565b1480156118715750856001600160a01b0316826001600160a01b0316145b156118815760019250505061051f565b600080876001600160a01b0316631626ba7e60e01b88886040516024016118a992919061286c565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516118e79190612885565b600060405180830381855afa9150503d8060008114611922576040519150601f19603f3d011682016040523d82523d6000602084013e611927565b606091505b509150915081801561193a575080516020145b801561196b57508051630b135d3f60e11b9061195f90830160209081019084016128a1565b6001600160e01b031916145b98975050505050505050565b6000438263ffffffff161061199f5760405163e64f180f60e01b815260040160405180910390fd5b6103ac606b63ffffffff80851690610a6116565b6000438263ffffffff16106119db5760405163e64f180f60e01b815260040160405180910390fd5b6103ac606c63ffffffff80851690610a6116565b60006001600160e01b03821115611a585760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b60648201526084016108d3565b5090565b600063ffffffff821115611a585760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b60648201526084016108d3565b600054610100900460ff16611ae85760405162461bcd60e51b81526004016108d39061279e565b61056333610ec9565b600080825160411415611b285760208301516040840151606085015160001a611b1c87828585611b5e565b94509450505050611725565b825160401415611b525760208301516040840151611b47868383611c4b565b935093505050611725565b50600090506002611725565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0831115611b955750600090506003611c42565b8460ff16601b14158015611bad57508460ff16601c14155b15611bbe5750600090506004611c42565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015611c12573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b038116611c3b57600060019250925050611c42565b9150600090505b94509492505050565b6000806001600160ff1b03831681611c6860ff86901c601b61267e565b9050611c7687828885611b5e565b935093505050935093915050565b508054600082559060005260206000209081019061039391905b80821115611a585760008155600101611c9e565b634e487b7160e01b600052604160045260246000fd5b604051602081016001600160401b0381118282101715611cea57611cea611cb2565b60405290565b604080519081016001600160401b0381118282101715611cea57611cea611cb2565b604051601f8201601f191681016001600160401b0381118282101715611d3a57611d3a611cb2565b604052919050565b60006001600160401b03821115611d5b57611d5b611cb2565b5060051b60200190565b6001600160a01b038116811461039357600080fd5b600082601f830112611d8b57600080fd5b81356020611da0611d9b83611d42565b611d12565b82815260059290921b84018101918181019086841115611dbf57600080fd5b8286015b84811015611de3578035611dd681611d65565b8352918301918301611dc3565b509695505050505050565b600060208284031215611e0057600080fd5b81356001600160401b03811115611e1657600080fd5b611e2284828501611d7a565b949350505050565b63ffffffff8116811461039357600080fd5b600060208284031215611e4e57600080fd5b813561051f81611e2a565b60006001600160401b03821115611e7257611e72611cb2565b50601f01601f191660200190565b600082601f830112611e9157600080fd5b8135611e9f611d9b82611e59565b818152846020838601011115611eb457600080fd5b816020850160208301376000918101602001919091529392505050565b60008060408385031215611ee457600080fd5b8235915060208301356001600160401b03811115611f0157600080fd5b611f0d85828601611e80565b9150509250929050565b8051602080845281518482018190526000926040919083019082870190855b81811015611f6d57835180516001600160a01b031684528601516001600160601b0316868401529285019291840191600101611f36565b5090979650505050505050565b60208152600061051f6020830184611f17565b600060208284031215611f9f57600080fd5b813561051f81611d65565b60008060408385031215611fbd57600080fd5b82356001600160401b0380821115611fd457600080fd5b9084019060608287031215611fe857600080fd5b60405160608101818110838211171561200357612003611cb2565b60405282358281111561201557600080fd5b61202188828601611e80565b8252506020830135602082015260408301356040820152809450505050602083013561204c81611d65565b809150509250929050565b6000806040838503121561206a57600080fd5b82356001600160401b038082111561208157600080fd5b818501915085601f83011261209557600080fd5b813560206120a5611d9b83611d42565b82815260059290921b840181019181810190898411156120c457600080fd5b8286015b848110156120fc578035868111156120e05760008081fd5b6120ee8c86838b0101611d7a565b8452509183019183016120c8565b509650508601359250508082111561211357600080fd5b50611f0d85828601611e80565b6000806040838503121561213357600080fd5b823561213e81611d65565b946020939093013593505050565b60006020828403121561215e57600080fd5b5035919050565b6000806040838503121561217857600080fd5b8235915060208301356001600160401b0381111561219557600080fd5b611f0d85828601611d7a565b600080604083850312156121b457600080fd5b82356121bf81611d65565b9150602083013561204c81611e2a565b600060208083850312156121e257600080fd5b6121ea611cc8565b915082356001600160401b0381111561220257600080fd5b8301601f8101851361221357600080fd5b8035612221611d9b82611d42565b81815260069190911b8201830190838101908783111561224057600080fd5b928401925b828410156122a3576040848903121561225e5760008081fd5b612266611cf0565b843561227181611d65565b8152848601356001600160601b038116811461228d5760008081fd5b8187015282526040939093019290840190612245565b8552509295945050505050565b6000806000606084860312156122c557600080fd5b83356122d081611d65565b92506020840135915060408401356001600160401b038111156122f257600080fd5b6122fe868287016121cf565b9150509250925092565b6000806040838503121561231b57600080fd5b82356001600160401b038082111561233257600080fd5b61233e868387016121cf565b9350602085013591508082111561235457600080fd5b50611f0d85828601611d7a565b60005b8381101561237c578181015183820152602001612364565b838111156109505750506000910152565b600082601f83011261239e57600080fd5b815160206123ae611d9b83611d42565b82815260059290921b840181019181810190868411156123cd57600080fd5b8286015b84811015611de35780516001600160401b038111156123f05760008081fd5b8701603f810189136124025760008081fd5b848101516040612414611d9b83611e59565b8281528b828486010111156124295760008081fd5b61243883898301848701612361565b86525050509183019183016123d1565b805161245381611e2a565b919050565b60008060006060848603121561246d57600080fd5b83516001600160401b038082111561248457600080fd5b818601915086601f83011261249857600080fd5b815160206124a8611d9b83611d42565b82815260059290921b8401810191818101908a8411156124c757600080fd5b948201945b838610156124ee5785516124df81611d65565b825294820194908201906124cc565b9189015191975090935050508082111561250757600080fd5b506125148682870161238d565b92505061252360408501612448565b90509250925092565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600060001982141561256c5761256c612542565b5060010190565b6001600160a01b038381168252604060208084018290528451918401829052600092858201929091906060860190855b818110156125c15785518516835294830194918301916001016125a3565b509098975050505050505050565b600060208083850312156125e257600080fd5b82516001600160401b038111156125f857600080fd5b8301601f8101851361260957600080fd5b8051612617611d9b82611d42565b81815260059190911b8201830190838101908783111561263657600080fd5b928401925b828410156126545783518252928401929084019061263b565b979650505050505050565b600081600019048311821515161561267957612679612542565b500290565b6000821982111561269157612691612542565b500190565b6000826126b357634e487b7160e01b600052601260045260246000fd5b500490565b600080821280156001600160ff1b03849003851316156126da576126da612542565b600160ff1b83900384128116156126f3576126f3612542565b50500190565b60008282101561270b5761270b612542565b500390565b60008151808452612728816020860160208601612361565b601f01601f19169290920160200192915050565b60018060a01b038316815260406020820152600082516060604084015261276660a0840182612710565b90506020840151606084015260408401516080840152809150509392505050565b60008161279657612796612542565b506000190190565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b6040815260006127fc6040830185611f17565b828103602084015261280e8185611f17565b95945050505050565b60008083128015600160ff1b85018412161561283557612835612542565b6001600160ff1b038401831381161561285057612850612542565b50500390565b634e487b7160e01b600052602160045260246000fd5b828152604060208201526000611e226040830184612710565b60008251612897818460208701612361565b9190910192915050565b6000602082840312156128b357600080fd5b81516001600160e01b03198116811461051f57600080fdfea264697066735822122041a1848da3a333d4821839e44ee9cccc30bf436bcb37f47a0d3f0d7c8247493364736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01rW`\x005`\xE0\x1C\x80cibU\xBE\x11a\0\xDEW\x80c\x98\xEC\x1A\xC9\x11a\0\x97W\x80c\xCD\xCD5\x81\x11a\0qW\x80c\xCD\xCD5\x81\x14a\x03\x15W\x80c\xDE\xC5\xD1\xF6\x14a\x03(W\x80c\xEC\x7F\xBB1\x14a\x03;W\x80c\xF2\xFD\xE3\x8B\x14a\x03wW`\0\x80\xFD[\x80c\x98\xEC\x1A\xC9\x14a\x02\xE7W\x80c\xAB\x11\x89\x95\x14a\x02\xFAW\x80c\xB93\xFAt\x14a\x03\rW`\0\x80\xFD[\x80cibU\xBE\x14a\x02\x8DW\x80cqP\x18\xA6\x14a\x02\xA0W\x80ct<1\xF4\x14a\x02\xA8W\x80c\x85}\xC1\x90\x14a\x02\xBBW\x80c\x8D\xA5\xCB[\x14a\x02\xC3W\x80c\x95_-\x90\x14a\x02\xD4W`\0\x80\xFD[\x80c;$.J\x11a\x010W\x80c;$.J\x14a\x02\x0EW\x80c=V\x11\xF6\x14a\x02!W\x80c@\xBF/\xB7\x14a\x024W\x80cQ@\xA5H\x14a\x02<W\x80c^\x10B\xE8\x14a\x02OW\x80c^\xF53)\x14a\x02zW`\0\x80\xFD[\x80b\xCF*\xB5\x14a\x01wW\x80c\r\xBA3\x94\x14a\x01\x8CW\x80c\x16&\xBA~\x14a\x01\xB2W\x80c\x17\x03\xA0\x18\x14a\x01\xDEW\x80c\x1EL\xD8^\x14a\x01\xF3W\x80c1O:I\x14a\x02\x06W[`\0\x80\xFD[a\x01\x8Aa\x01\x856`\x04a\x1D\xEEV[a\x03\x8AV[\0[a\x01\x9Fa\x01\x9A6`\x04a\x1E<V[a\x03\x96V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xC5a\x01\xC06`\x04a\x1E\xD1V[a\x03\xB2V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x01\xA9V[a\x01\xE6a\x03\xF0V[`@Qa\x01\xA9\x91\x90a\x1FzV[a\x01\x9Fa\x02\x016`\x04a\x1E<V[a\x04\x83V[a\x01\x9Fa\x04\x99V[a\x01\x9Fa\x02\x1C6`\x04a\x1F\x8DV[a\x04\xAAV[a\x01\x8Aa\x02/6`\x04a\x1F\xAAV[a\x04\xCBV[`gTa\x01\x9FV[a\x01\x8Aa\x02J6`\x04a WV[a\x04\xDAV[a\x02ba\x02]6`\x04a! V[a\x04\xFDV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xA9V[a\x01\x8Aa\x02\x886`\x04a!LV[a\x05&V[a\x01\x8Aa\x02\x9B6`\x04a!eV[a\x057V[a\x01\x8Aa\x05QV[a\x01\x8Aa\x02\xB66`\x04a\x1F\x8DV[a\x05eV[a\x01\x8Aa\x05\x9FV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02bV[a\x01\x9Fa\x02\xE26`\x04a!\xA1V[a\x05\xA8V[a\x01\x9Fa\x02\xF56`\x04a\x1F\x8DV[a\x05\xD3V[a\x01\x8Aa\x03\x086`\x04a\"\xB0V[a\x08:V[a\x01\x9Fa\tVV[a\x02ba\x03#6`\x04a\x1F\x8DV[a\tbV[a\x01\x8Aa\x0366`\x04a#\x08V[a\t\x83V[a\x03ga\x03I6`\x04a\x1F\x8DV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`n` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xA9V[a\x01\x8Aa\x03\x856`\x04a\x1F\x8DV[a\t\x94V[a\x03\x93\x81a\n\nV[PV[`\0a\x03\xAC`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[\x92\x91PPV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03\xCC\x91\x90a$XV[\x92P\x92P\x92Pa\x03\xDE\x86\x84\x84\x84a\x0BpV[Pc\x0B\x13]?`\xE1\x1B\x95\x94PPPPPV[`@\x80Q` \x81\x01\x90\x91R``\x81R`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R\x92\x93\x91\x92\x84\x92\x90\x91\x84\x91`\0\x90\x85\x01[\x82\x82\x10\x15a\x04vW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x04'V[PPPP\x81RPP\x90P\x90V[`\0a\x03\xAC`lc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[`\0a\x04\xA5`ka\x0C/V[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`m` R`@\x81 a\x03\xAC\x90a\x0C/V[a\x04\xD63\x83\x83a\x0C\x8BV[PPV[a\x04\xD6\x82`\0\x81Q\x81\x10a\x04\xF0Wa\x04\xF0a%,V[` \x02` \x01\x01Qa\r\xC3V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`j` R`@\x81 a\x05\x1F\x90\x83a\naV[\x93\x92PPPV[a\x05.a\r\xE6V[a\x03\x93\x81a\x0E@V[a\x05?a\r\xE6V[a\x05H\x82a\x0E\x83V[a\x04\xD6\x81a\n\nV[a\x05Ya\r\xE6V[a\x05c`\0a\x0E\xC9V[V[3`\0\x90\x81R`n` R`@\x90 T`\xFF\x16a\x05\x95W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x933\x82a\x0F\x1BV[a\x05c3a\x0F\xCEV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`m` R`@\x81 a\x05\x1F\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[`\0\x80`f`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06JW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x05\xFBV[PPPP\x90P`\0\x80\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06lWa\x06la\x1C\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\x95W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x06\xFEW\x83\x81\x81Q\x81\x10a\x06\xB6Wa\x06\xB6a%,V[` \x02` \x01\x01Q`\0\x01Q\x82\x82\x81Q\x81\x10a\x06\xD4Wa\x06\xD4a%,V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x06\xF6\x81a%XV[\x91PPa\x06\x9BV[P`@Qc\x90\x04\x13G`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x90\x04\x13G\x90a\x07P\x90\x89\x90\x86\x90`\x04\x01a%sV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\x95\x91\x90\x81\x01\x90a%\xCFV[\x90P`\0[\x84Q\x81\x10\x15a\x08\x0CW\x84\x81\x81Q\x81\x10a\x07\xB5Wa\x07\xB5a%,V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x07\xDCWa\x07\xDCa%,V[` \x02` \x01\x01Qa\x07\xEE\x91\x90a&_V[a\x07\xF8\x90\x85a&~V[\x93P\x80a\x08\x04\x81a%XV[\x91PPa\x07\x9AV[Pa\x08\x19a'\x10\x84a&\x96V[\x92P`gT\x83\x10a\x08.WP\x90\x94\x93PPPPV[P`\0\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x08ZWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x08tWP0;\x15\x80\x15a\x08tWP`\0T`\xFF\x16`\x01\x14[a\x08\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x08\xFFW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\t\n\x84\x84\x84a\x10\xF1V[\x80\x15a\tPW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[`\0a\x04\xA5`la\x0C/V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`j` R`@\x81 a\x03\xAC\x90a\x0C/V[a\t\x8Ba\r\xE6V[a\x05H\x82a\x11RV[a\t\x9Ca\r\xE6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\n\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\xD3V[a\x03\x93\x81a\x0E\xC9V[`\0\x80[\x82Q\x81\x10\x15a\nWWa\n9\x83\x82\x81Q\x81\x10a\n,Wa\n,a%,V[` \x02` \x01\x01Qa\x12\xB1V[a\nC\x90\x83a&\xB8V[\x91P\x80a\nO\x81a%XV[\x91PPa\n\x0EV[Pa\tP\x81a\x13\xD4V[`\0C\x82\x10a\n\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R`d\x01a\x08\xD3V[\x82T`\0[\x81\x81\x10\x15a\x0B\x17W`\0a\n\xCB\x82\x84a\x14@V[\x90P\x84\x86`\0\x01\x82\x81T\x81\x10a\n\xE3Wa\n\xE3a%,V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\x03W\x80\x92Pa\x0B\x11V[a\x0B\x0E\x81`\x01a&~V[\x91P[Pa\n\xB7V[\x81\x15a\x0B[W\x84a\x0B)`\x01\x84a&\xF9V[\x81T\x81\x10a\x0B9Wa\x0B9a%,V[`\0\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0B^V[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x95\x94PPPPPV[`\0\x83Q\x90P`\0\x80`\0\x80a\x0B\x87\x85\x88Qa\x14[V[`\0[\x85\x81\x10\x15a\x0C\x19W\x88\x81\x81Q\x81\x10a\x0B\xA4Wa\x0B\xA4a%,V[` \x02` \x01\x01Q\x94Pa\x0B\xB8\x85\x88a\x14\x9CV[\x92Pa\x0B\xC4\x84\x86a\x14\xEFV[a\x0B\xE8\x83\x8B\x8A\x84\x81Q\x81\x10a\x0B\xDBWa\x0B\xDBa%,V[` \x02` \x01\x01Qa\x15!V[\x84\x93P`\0a\x0B\xF7\x86\x89a\x15RV[\x90Pa\x0C\x03\x81\x84a&~V[\x92PP\x80\x80a\x0C\x11\x90a%XV[\x91PPa\x0B\x8AV[Pa\x0C$\x81\x87a\x15\xA5V[PPPPPPPPPV[\x80T`\0\x90\x80\x15a\x0CxW\x82a\x0CF`\x01\x83a&\xF9V[\x81T\x81\x10a\x0CVWa\x0CVa%,V[`\0\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0C{V[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`n` R`@\x90 T`\xFF\x16\x15a\x0C\xC5W`@QcB\xEEh\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90`\0a\x0C\xD5\x83a%XV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`n` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\r\x06\x84a\x12\xB1V[\x90Pa\r\x11\x81a\x13\xD4V[PPa\r\x1D\x84\x83a\x0F\x1BV[`hT`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x99&\xEE}\x90a\rO\x90\x87\x90\x87\x90`\x04\x01a'<V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\riW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r}W=`\0\x80>=`\0\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x87\x16\x91P\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90`\0\x90\xA3PPPPV[`eT\x81Q\x14a\x03\x8AW`@Qc\x16\x9E\xFB[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08\xD3V[a\x0EK`l\x82a\x16\x01V[PP`@Q\x81\x81R\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`g\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8F\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`j` R`@\x81 a\x0F<\x90a\x0C/V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0F]WPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`j` R`@\x90 a\x0F\x81\x91\x84\x16a\x16\x01V[PP`@Q`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82R\x80\x84\x16\x91C\x91\x86\x16\x90\x7F\xD0a\x16\x82R\xF4As6X\xF0\x9EM\x8F[-\x99\x8E\xD4\xEF$\xA2\xBB\xFDl\xEC\xA5.\xA11P\x02\x90` \x01`@Q\x80\x91\x03\x90\xA4PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`n` R`@\x90 T`\xFF\x16a\x10\x07W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90`\0a\x10\x17\x83a'\x87V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`n` R`@\x81 \x80T`\xFF\x19\x16\x90Ua\x10E\x82a\x12\xB1V[\x90Pa\x10P\x81a\x13\xD4V[PP`hT`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xA3d\xF4\xDA\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x99W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xADW=`\0\x80>=`\0\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x85\x16\x91P\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x11\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xD3\x90a'\x9EV[`h\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua\x11<\x82a\x0E@V[a\x11E\x81a\x11RV[a\x11Ma\x17,V[PPPV[a\x11[\x81a\x17[V[a\x11xW`@Qc\xD1sWy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R`\0\x94\x84\x92\x84\x91\x87\x90\x85\x01[\x82\x82\x10\x15a\x11\xEBW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x11\x9CV[PPP\x91RP\x90\x91P`f\x90P`\0a\x12\x04\x82\x82a\x1C\x84V[PP`\0[\x82QQ\x81\x10\x15a\x12\x7FW\x82Q\x80Q`f\x91\x90\x83\x90\x81\x10a\x12+Wa\x12+a%,V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x92\x82\x90 \x81Q\x91\x90\x92\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x91\x01U\x80a\x12w\x81a%XV[\x91PPa\x12\tV[P\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x81\x83`@Qa\x0E\xBD\x92\x91\x90a'\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`m` R`@\x81 \x81\x90\x81\x90\x81\x90a\x12\xD8\x90a\x0C/V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`n` R`@\x90 T\x90\x91P`\xFF\x16a\x13=Wa\x13\x05\x81\x84a(\x17V[\x92P\x82a\x13\x15WP\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`m` R`@\x81 a\x136\x91a\x16\x01V[PPa\x13\x87V[a\x13F\x85a\x05\xD3V[\x91Pa\x13R\x81\x83a(\x17V[\x92P\x82a\x13bWP\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`m` R`@\x90 a\x13\x84\x90\x83a\x16\x01V[PP[`@\x80Q\x82\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16\x91\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x91\x01`@Q\x80\x91\x03\x90\xA2P\x90\x93\x92PPPV[`\0\x80a\x13\xE1`ka\x0C/V[\x91P`\0a\x13\xEF\x84\x84a&\xB8V[\x91P\x81\x90Pa\x13\xFF`k\x82a\x16\x01V[PP`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B\x91\x01`@Q\x80\x91\x03\x90\xA1P\x91P\x91V[`\0a\x14O`\x02\x84\x84\x18a&\x96V[a\x05\x1F\x90\x84\x84\x16a&~V[\x80\x82\x14a\x14~W`@Q`\x01b\x13\x98\xB9`\xE3\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81a\x04\xD6W`@Qc%\x1FV\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x14\xC4W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`j` R`@\x90 a\x05\x1F\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x10a\x04\xD6W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x155`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a\x18+V[a\x11MW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x15zW`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`m` R`@\x90 a\x05\x1F\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[`\0a\x15\xB0\x82a\x19wV[\x90P\x80\x83\x11\x15a\x15\xD3W`@QcK\x05\xA0\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x15\xDE\x83a\x19\xB3V[\x90P\x83\x81\x11\x15a\tPW`@Qc\xE1!c/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T`\0\x90\x81\x90\x81a\x16\x12\x86a\x0C/V[\x90P`\0\x82\x11\x80\x15a\x16PWPC\x86a\x16,`\x01\x85a&\xF9V[\x81T\x81\x10a\x16<Wa\x16<a%,V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x16\xB0Wa\x16^\x85a\x19\xEFV[\x86a\x16j`\x01\x85a&\xF9V[\x81T\x81\x10a\x16zWa\x16za%,V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xE0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xE0\x1B\x03\x16\x02\x17\x90UPa\x17\x1EV[\x85`\0\x01`@Q\x80`@\x01`@R\x80a\x16\xC8Ca\x1A\\V[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x16\xDC\x88a\x19\xEFV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[\x92P\x83\x91PP[\x92P\x92\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x17SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xD3\x90a'\x9EV[a\x05ca\x1A\xC1V[\x80Q`\0\x90\x81\x80\x80\x80[\x84Q\x81\x10\x15a\x18\tW\x84\x81\x81Q\x81\x10a\x17\x80Wa\x17\x80a%,V[` \x02` \x01\x01Q`\0\x01Q\x92P\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x17\xC0W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x93P\x84\x81\x81Q\x81\x10a\x17\xD5Wa\x17\xD5a%,V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82a\x17\xF5\x91\x90a&~V[\x91P\x80a\x18\x01\x81a%XV[\x91PPa\x17eV[Pa'\x10\x81\x14a\x18\x1FWP`\0\x95\x94PPPPPV[P`\x01\x95\x94PPPPPV[`\0\x80`\0a\x18:\x85\x85a\x1A\xF1V[\x90\x92P\x90P`\0\x81`\x04\x81\x11\x15a\x18SWa\x18Sa(VV[\x14\x80\x15a\x18qWP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x18\x81W`\x01\x92PPPa\x05\x1FV[`\0\x80\x87`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01a\x18\xA9\x92\x91\x90a(lV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x18\xE7\x91\x90a(\x85V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x19\"W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19'V[``\x91P[P\x91P\x91P\x81\x80\x15a\x19:WP\x80Q` \x14[\x80\x15a\x19kWP\x80Qc\x0B\x13]?`\xE1\x1B\x90a\x19_\x90\x83\x01` \x90\x81\x01\x90\x84\x01a(\xA1V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14[\x98\x97PPPPPPPPV[`\0C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x19\x9FW`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xAC`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[`\0C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x19\xDBW`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xAC`lc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\na\x16V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x1AXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x08\xD3V[P\x90V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x1AXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\xD3V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xD3\x90a'\x9EV[a\x05c3a\x0E\xC9V[`\0\x80\x82Q`A\x14\x15a\x1B(W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x1B\x1C\x87\x82\x85\x85a\x1B^V[\x94P\x94PPPPa\x17%V[\x82Q`@\x14\x15a\x1BRW` \x83\x01Q`@\x84\x01Qa\x1BG\x86\x83\x83a\x1CKV[\x93P\x93PPPa\x17%V[P`\0\x90P`\x02a\x17%V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1B\x95WP`\0\x90P`\x03a\x1CBV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a\x1B\xADWP\x84`\xFF\x16`\x1C\x14\x15[\x15a\x1B\xBEWP`\0\x90P`\x04a\x1CBV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1C\x12W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C;W`\0`\x01\x92P\x92PPa\x1CBV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a\x1Ch`\xFF\x86\x90\x1C`\x1Ba&~V[\x90Pa\x1Cv\x87\x82\x88\x85a\x1B^V[\x93P\x93PPP\x93P\x93\x91PPV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x03\x93\x91\x90[\x80\x82\x11\x15a\x1AXW`\0\x81U`\x01\x01a\x1C\x9EV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xEAWa\x1C\xEAa\x1C\xB2V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xEAWa\x1C\xEAa\x1C\xB2V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D:Wa\x1D:a\x1C\xB2V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1D[Wa\x1D[a\x1C\xB2V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x93W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x1D\x8BW`\0\x80\xFD[\x815` a\x1D\xA0a\x1D\x9B\x83a\x1DBV[a\x1D\x12V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x1D\xBFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1D\xE3W\x805a\x1D\xD6\x81a\x1DeV[\x83R\x91\x83\x01\x91\x83\x01a\x1D\xC3V[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1E\0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\x16W`\0\x80\xFD[a\x1E\"\x84\x82\x85\x01a\x1DzV[\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\x93W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1ENW`\0\x80\xFD[\x815a\x05\x1F\x81a\x1E*V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1ErWa\x1Era\x1C\xB2V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1E\x91W`\0\x80\xFD[\x815a\x1E\x9Fa\x1D\x9B\x82a\x1EYV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1E\xB4W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1E\xE4W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\x01W`\0\x80\xFD[a\x1F\r\x85\x82\x86\x01a\x1E\x80V[\x91PP\x92P\x92\x90PV[\x80Q` \x80\x84R\x81Q\x84\x82\x01\x81\x90R`\0\x92`@\x91\x90\x83\x01\x90\x82\x87\x01\x90\x85[\x81\x81\x10\x15a\x1FmW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x86\x01Q`\x01`\x01``\x1B\x03\x16\x86\x84\x01R\x92\x85\x01\x92\x91\x84\x01\x91`\x01\x01a\x1F6V[P\x90\x97\x96PPPPPPPV[` \x81R`\0a\x05\x1F` \x83\x01\x84a\x1F\x17V[`\0` \x82\x84\x03\x12\x15a\x1F\x9FW`\0\x80\xFD[\x815a\x05\x1F\x81a\x1DeV[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\xBDW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1F\xD4W`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x1F\xE8W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a \x03Wa \x03a\x1C\xB2V[`@R\x825\x82\x81\x11\x15a \x15W`\0\x80\xFD[a !\x88\x82\x86\x01a\x1E\x80V[\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x94PPPP` \x83\x015a L\x81a\x1DeV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a jW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a \x81W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a \x95W`\0\x80\xFD[\x815` a \xA5a\x1D\x9B\x83a\x1DBV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a \xC4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a \xFCW\x805\x86\x81\x11\x15a \xE0W`\0\x80\x81\xFD[a \xEE\x8C\x86\x83\x8B\x01\x01a\x1DzV[\x84RP\x91\x83\x01\x91\x83\x01a \xC8V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a!\x13W`\0\x80\xFD[Pa\x1F\r\x85\x82\x86\x01a\x1E\x80V[`\0\x80`@\x83\x85\x03\x12\x15a!3W`\0\x80\xFD[\x825a!>\x81a\x1DeV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a!^W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a!xW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x95W`\0\x80\xFD[a\x1F\r\x85\x82\x86\x01a\x1DzV[`\0\x80`@\x83\x85\x03\x12\x15a!\xB4W`\0\x80\xFD[\x825a!\xBF\x81a\x1DeV[\x91P` \x83\x015a L\x81a\x1E*V[`\0` \x80\x83\x85\x03\x12\x15a!\xE2W`\0\x80\xFD[a!\xEAa\x1C\xC8V[\x91P\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x02W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\"\x13W`\0\x80\xFD[\x805a\"!a\x1D\x9B\x82a\x1DBV[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\"@W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\"\xA3W`@\x84\x89\x03\x12\x15a\"^W`\0\x80\x81\xFD[a\"fa\x1C\xF0V[\x845a\"q\x81a\x1DeV[\x81R\x84\x86\x015`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\"\x8DW`\0\x80\x81\xFD[\x81\x87\x01R\x82R`@\x93\x90\x93\x01\x92\x90\x84\x01\x90a\"EV[\x85RP\x92\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"\xC5W`\0\x80\xFD[\x835a\"\xD0\x81a\x1DeV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xF2W`\0\x80\xFD[a\"\xFE\x86\x82\x87\x01a!\xCFV[\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a#\x1BW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a#2W`\0\x80\xFD[a#>\x86\x83\x87\x01a!\xCFV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a#TW`\0\x80\xFD[Pa\x1F\r\x85\x82\x86\x01a\x1DzV[`\0[\x83\x81\x10\x15a#|W\x81\x81\x01Q\x83\x82\x01R` \x01a#dV[\x83\x81\x11\x15a\tPWPP`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a#\x9EW`\0\x80\xFD[\x81Q` a#\xAEa\x1D\x9B\x83a\x1DBV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a#\xCDW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1D\xE3W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xF0W`\0\x80\x81\xFD[\x87\x01`?\x81\x01\x89\x13a$\x02W`\0\x80\x81\xFD[\x84\x81\x01Q`@a$\x14a\x1D\x9B\x83a\x1EYV[\x82\x81R\x8B\x82\x84\x86\x01\x01\x11\x15a$)W`\0\x80\x81\xFD[a$8\x83\x89\x83\x01\x84\x87\x01a#aV[\x86RPPP\x91\x83\x01\x91\x83\x01a#\xD1V[\x80Qa$S\x81a\x1E*V[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a$mW`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a$\x84W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a$\x98W`\0\x80\xFD[\x81Q` a$\xA8a\x1D\x9B\x83a\x1DBV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a$\xC7W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a$\xEEW\x85Qa$\xDF\x81a\x1DeV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a$\xCCV[\x91\x89\x01Q\x91\x97P\x90\x93PPP\x80\x82\x11\x15a%\x07W`\0\x80\xFD[Pa%\x14\x86\x82\x87\x01a#\x8DV[\x92PPa%#`@\x85\x01a$HV[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a%lWa%la%BV[P`\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`@` \x80\x84\x01\x82\x90R\x84Q\x91\x84\x01\x82\x90R`\0\x92\x85\x82\x01\x92\x90\x91\x90``\x86\x01\x90\x85[\x81\x81\x10\x15a%\xC1W\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a%\xA3V[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a%\xE2W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xF8W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a&\tW`\0\x80\xFD[\x80Qa&\x17a\x1D\x9B\x82a\x1DBV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a&6W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a&TW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a&;V[\x97\x96PPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a&yWa&ya%BV[P\x02\x90V[`\0\x82\x19\x82\x11\x15a&\x91Wa&\x91a%BV[P\x01\x90V[`\0\x82a&\xB3WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80\x82\x12\x80\x15`\x01`\x01`\xFF\x1B\x03\x84\x90\x03\x85\x13\x16\x15a&\xDAWa&\xDAa%BV[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15a&\xF3Wa&\xF3a%BV[PP\x01\x90V[`\0\x82\x82\x10\x15a'\x0BWa'\x0Ba%BV[P\x03\x90V[`\0\x81Q\x80\x84Ra'(\x81` \x86\x01` \x86\x01a#aV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra'f`\xA0\x84\x01\x82a'\x10V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[`\0\x81a'\x96Wa'\x96a%BV[P`\0\x19\x01\x90V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R`\0a'\xFC`@\x83\x01\x85a\x1F\x17V[\x82\x81\x03` \x84\x01Ra(\x0E\x81\x85a\x1F\x17V[\x95\x94PPPPPV[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a(5Wa(5a%BV[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a(PWa(Pa%BV[PP\x03\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x82\x81R`@` \x82\x01R`\0a\x1E\"`@\x83\x01\x84a'\x10V[`\0\x82Qa(\x97\x81\x84` \x87\x01a#aV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a(\xB3W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x05\x1FW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 A\xA1\x84\x8D\xA3\xA33\xD4\x82\x189\xE4N\xE9\xCC\xCC0\xBFCk\xCB7\xF4z\r?\r|\x82GI3dsolcC\0\x08\x0C\x003",
    );
    /**```solidity
    struct Quorum { StrategyParams[] strategies; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Quorum {
        pub strategies:
            alloy::sol_types::private::Vec<<StrategyParams as alloy::sol_types::SolType>::RustType>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Array<StrategyParams>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<<StrategyParams as alloy::sol_types::SolType>::RustType>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Quorum> for UnderlyingRustTuple<'_> {
            fn from(value: Quorum) -> Self {
                (value.strategies,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Quorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategies: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Quorum {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Quorum {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Quorum {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Quorum {
            const NAME: &'static str = "Quorum";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("Quorum(StrategyParams[] strategies)")
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(<StrategyParams as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<StrategyParams as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                <alloy::sol_types::sol_data::Array<
                    StrategyParams,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Quorum {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyParams,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    StrategyParams,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct StrategyParams { address strategy; uint96 multiplier; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct StrategyParams {
        pub strategy: alloy::sol_types::private::Address,
        pub multiplier: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U96,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StrategyParams> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyParams) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.multiplier,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StrategyParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StrategyParams {
            const NAME: &'static str = "StrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyParams(address strategy,uint96 multiplier)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.multiplier)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StrategyParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.multiplier,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategy,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.multiplier,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**Custom error with signature `InsufficientSignedStake()` and selector `0xe121632f`.
    ```solidity
    error InsufficientSignedStake();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InsufficientSignedStake {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientSignedStake> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientSignedStake) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientSignedStake {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientSignedStake {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientSignedStake()";
            const SELECTOR: [u8; 4] = [225u8, 33u8, 99u8, 47u8];
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
    /**Custom error with signature `InsufficientWeight()` and selector `0xa8792fd1`.
    ```solidity
    error InsufficientWeight();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InsufficientWeight {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientWeight> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientWeight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientWeight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientWeight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientWeight()";
            const SELECTOR: [u8; 4] = [168u8, 121u8, 47u8, 209u8];
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
    /**Custom error with signature `InvalidLength()` and selector `0x947d5a84`.
    ```solidity
    error InvalidLength();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidLength {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidLength()";
            const SELECTOR: [u8; 4] = [148u8, 125u8, 90u8, 132u8];
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
    /**Custom error with signature `InvalidQuorum()` and selector `0xd1735779`.
    ```solidity
    error InvalidQuorum();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidQuorum {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidQuorum> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidQuorum) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidQuorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidQuorum {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidQuorum()";
            const SELECTOR: [u8; 4] = [209u8, 115u8, 87u8, 121u8];
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
    /**Custom error with signature `InvalidReferenceBlock()` and selector `0xe64f180f`.
    ```solidity
    error InvalidReferenceBlock();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidReferenceBlock {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidReferenceBlock> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidReferenceBlock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidReferenceBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidReferenceBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidReferenceBlock()";
            const SELECTOR: [u8; 4] = [230u8, 79u8, 24u8, 15u8];
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
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
    ```solidity
    error InvalidSignature();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `InvalidSignedWeight()` and selector `0x960b41ee`.
    ```solidity
    error InvalidSignedWeight();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidSignedWeight {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSignedWeight> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignedWeight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignedWeight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignedWeight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignedWeight()";
            const SELECTOR: [u8; 4] = [150u8, 11u8, 65u8, 238u8];
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
    /**Custom error with signature `InvalidThreshold()` and selector `0xaabd5a09`.
    ```solidity
    error InvalidThreshold();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidThreshold {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidThreshold()";
            const SELECTOR: [u8; 4] = [170u8, 189u8, 90u8, 9u8];
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
    /**Custom error with signature `LengthMismatch()` and selector `0xff633a38`.
    ```solidity
    error LengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct LengthMismatch {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<LengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: LengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LengthMismatch()";
            const SELECTOR: [u8; 4] = [255u8, 99u8, 58u8, 56u8];
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
    /**Custom error with signature `MustUpdateAllOperators()` and selector `0x2d3df6b6`.
    ```solidity
    error MustUpdateAllOperators();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct MustUpdateAllOperators {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MustUpdateAllOperators> for UnderlyingRustTuple<'_> {
            fn from(value: MustUpdateAllOperators) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MustUpdateAllOperators {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MustUpdateAllOperators {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MustUpdateAllOperators()";
            const SELECTOR: [u8; 4] = [45u8, 61u8, 246u8, 182u8];
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
    /**Custom error with signature `NotSorted()` and selector `0xba50f911`.
    ```solidity
    error NotSorted();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct NotSorted {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotSorted> for UnderlyingRustTuple<'_> {
            fn from(value: NotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotSorted()";
            const SELECTOR: [u8; 4] = [186u8, 80u8, 249u8, 17u8];
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
    /**Custom error with signature `OperatorAlreadyRegistered()` and selector `0x42ee68b5`.
    ```solidity
    error OperatorAlreadyRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OperatorAlreadyRegistered {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorAlreadyRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorAlreadyRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorAlreadyRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorAlreadyRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorAlreadyRegistered()";
            const SELECTOR: [u8; 4] = [66u8, 238u8, 104u8, 181u8];
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
    /**Custom error with signature `OperatorNotRegistered()` and selector `0x25ec6c1f`.
    ```solidity
    error OperatorNotRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OperatorNotRegistered {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorNotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotRegistered()";
            const SELECTOR: [u8; 4] = [37u8, 236u8, 108u8, 31u8];
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
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
    ```solidity
    event Initialized(uint8 version);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8,
                    56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8,
                    96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.version,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MinimumWeightUpdated(uint256,uint256)` and selector `0x713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f`.
    ```solidity
    event MinimumWeightUpdated(uint256 _old, uint256 _new);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct MinimumWeightUpdated {
        #[allow(missing_docs)]
        pub _old: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _new: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for MinimumWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MinimumWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    113u8, 60u8, 165u8, 59u8, 136u8, 214u8, 235u8, 99u8, 245u8, 177u8, 133u8, 76u8,
                    184u8, 203u8, 221u8, 115u8, 110u8, 197u8, 30u8, 218u8, 34u8, 94u8, 70u8, 121u8,
                    26u8, 169u8, 41u8, 139u8, 1u8, 96u8, 100u8, 143u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _old: data.0,
                    _new: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._old,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._new,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MinimumWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MinimumWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MinimumWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorDeregistered(address,address)` and selector `0x31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed580`.
    ```solidity
    event OperatorDeregistered(address indexed _operator, address indexed _avs);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct OperatorDeregistered {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _avs: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorDeregistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorDeregistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 224u8, 173u8, 254u8, 199u8, 27u8, 204u8, 238u8, 55u8, 182u8, 232u8, 58u8,
                    144u8, 194u8, 254u8, 219u8, 23u8, 216u8, 241u8, 105u8, 63u8, 238u8, 134u8,
                    60u8, 71u8, 113u8, 231u8, 191u8, 226u8, 174u8, 213u8, 128u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _operator: topics.1,
                    _avs: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self._operator.clone(),
                    self._avs.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorDeregistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorDeregistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorDeregistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRegistered(address,address)` and selector `0xa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c1`.
    ```solidity
    event OperatorRegistered(address indexed _operator, address indexed _avs);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct OperatorRegistered {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _avs: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorRegistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    164u8, 83u8, 219u8, 97u8, 42u8, 245u8, 158u8, 85u8, 33u8, 214u8, 171u8, 146u8,
                    132u8, 220u8, 62u8, 45u8, 6u8, 175u8, 40u8, 110u8, 177u8, 177u8, 183u8, 183u8,
                    113u8, 252u8, 228u8, 113u8, 108u8, 25u8, 242u8, 193u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _operator: topics.1,
                    _avs: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self._operator.clone(),
                    self._avs.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorRegistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRegistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorRegistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorWeightUpdated(address,uint256,uint256)` and selector `0x88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594`.
    ```solidity
    event OperatorWeightUpdated(address indexed _operator, uint256 oldWeight, uint256 newWeight);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct OperatorWeightUpdated {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorWeightUpdated(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    136u8, 119u8, 13u8, 200u8, 98u8, 228u8, 122u8, 126u8, 213u8, 134u8, 144u8,
                    120u8, 87u8, 235u8, 27u8, 117u8, 228u8, 197u8, 255u8, 200u8, 183u8, 7u8, 199u8,
                    238u8, 16u8, 235u8, 116u8, 214u8, 136u8, 95u8, 229u8, 148u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _operator: topics.1,
                    oldWeight: data.0,
                    newWeight: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.oldWeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newWeight,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self._operator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
    ```solidity
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8,
                    208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8,
                    175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumUpdated(((address,uint96)[]),((address,uint96)[]))` and selector `0x23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e`.
    ```solidity
    event QuorumUpdated(Quorum _old, Quorum _new);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct QuorumUpdated {
        #[allow(missing_docs)]
        pub _old: <Quorum as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _new: <Quorum as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for QuorumUpdated {
            type DataTuple<'a> = (Quorum, Quorum);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "QuorumUpdated(((address,uint96)[]),((address,uint96)[]))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    35u8, 170u8, 212u8, 230u8, 23u8, 68u8, 236u8, 225u8, 100u8, 19u8, 10u8, 164u8,
                    21u8, 193u8, 97u8, 110u8, 128u8, 19u8, 107u8, 15u8, 7u8, 112u8, 229u8, 101u8,
                    137u8, 67u8, 139u8, 144u8, 178u8, 105u8, 38u8, 94u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _old: data.0,
                    _new: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._old),
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._new),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SigningKeyUpdate(address,uint256,address,address)` and selector `0xd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea1315002`.
    ```solidity
    event SigningKeyUpdate(address indexed operator, uint256 indexed updateBlock, address indexed newSigningKey, address oldSigningKey);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct SigningKeyUpdate {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub updateBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newSigningKey: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldSigningKey: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SigningKeyUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SigningKeyUpdate(address,uint256,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    208u8, 97u8, 22u8, 130u8, 82u8, 244u8, 65u8, 115u8, 54u8, 88u8, 240u8, 158u8,
                    77u8, 143u8, 91u8, 45u8, 153u8, 142u8, 212u8, 239u8, 36u8, 162u8, 187u8, 253u8,
                    108u8, 236u8, 165u8, 46u8, 161u8, 49u8, 80u8, 2u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    updateBlock: topics.2,
                    newSigningKey: topics.3,
                    oldSigningKey: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.oldSigningKey,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.operator.clone(),
                    self.updateBlock.clone(),
                    self.newSigningKey.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.updateBlock);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newSigningKey,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SigningKeyUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SigningKeyUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SigningKeyUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ThresholdWeightUpdated(uint256)` and selector `0x9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b`.
    ```solidity
    event ThresholdWeightUpdated(uint256 _thresholdWeight);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct ThresholdWeightUpdated {
        #[allow(missing_docs)]
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ThresholdWeightUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ThresholdWeightUpdated(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    147u8, 36u8, 247u8, 229u8, 167u8, 192u8, 40u8, 136u8, 8u8, 166u8, 52u8, 204u8,
                    222u8, 68u8, 184u8, 233u8, 121u8, 103u8, 100u8, 116u8, 178u8, 46u8, 41u8,
                    238u8, 157u8, 213u8, 105u8, 181u8, 94u8, 121u8, 26u8, 75u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _thresholdWeight: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._thresholdWeight,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ThresholdWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ThresholdWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ThresholdWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TotalWeightUpdated(uint256,uint256)` and selector `0x86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b`.
    ```solidity
    event TotalWeightUpdated(uint256 oldTotalWeight, uint256 newTotalWeight);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct TotalWeightUpdated {
        #[allow(missing_docs)]
        pub oldTotalWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newTotalWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TotalWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TotalWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    134u8, 220u8, 248u8, 107u8, 18u8, 223u8, 238u8, 222u8, 167u8, 74u8, 233u8,
                    48u8, 13u8, 189u8, 170u8, 25u8, 59u8, 204u8, 229u8, 128u8, 147u8, 105u8, 200u8,
                    23u8, 126u8, 162u8, 244u8, 234u8, 170u8, 101u8, 114u8, 155u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldTotalWeight: data.0,
                    newTotalWeight: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.oldTotalWeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newTotalWeight,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TotalWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TotalWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TotalWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UpdateMinimumWeight(uint256,uint256)` and selector `0x1ea42186b305fa37310450d9fb87ea1e8f0c7f447e771479e3b27634bfe84dc1`.
    ```solidity
    event UpdateMinimumWeight(uint256 oldMinimumWeight, uint256 newMinimumWeight);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct UpdateMinimumWeight {
        #[allow(missing_docs)]
        pub oldMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UpdateMinimumWeight {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateMinimumWeight(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    30u8, 164u8, 33u8, 134u8, 179u8, 5u8, 250u8, 55u8, 49u8, 4u8, 80u8, 217u8,
                    251u8, 135u8, 234u8, 30u8, 143u8, 12u8, 127u8, 68u8, 126u8, 119u8, 20u8, 121u8,
                    227u8, 178u8, 118u8, 52u8, 191u8, 232u8, 77u8, 193u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldMinimumWeight: data.0,
                    newMinimumWeight: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.oldMinimumWeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newMinimumWeight,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UpdateMinimumWeight {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateMinimumWeight> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpdateMinimumWeight) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _delegationManager);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _delegationManager: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._delegationManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _delegationManager: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                        &self._delegationManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `deregisterOperator()` and selector `0x857dc190`.
    ```solidity
    function deregisterOperator() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {}
    ///Container type for the return parameters of the [`deregisterOperator()`](deregisterOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct deregisterOperatorReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator()";
            const SELECTOR: [u8; 4] = [133u8, 125u8, 193u8, 144u8];
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
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointOperatorWeight(address)` and selector `0x3b242e4a`.
    ```solidity
    function getLastCheckpointOperatorWeight(address _operator) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLastCheckpointOperatorWeightCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getLastCheckpointOperatorWeight(address)`](getLastCheckpointOperatorWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLastCheckpointOperatorWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointOperatorWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointOperatorWeightCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointOperatorWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointOperatorWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointOperatorWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointOperatorWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointOperatorWeightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointOperatorWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointOperatorWeight(address)";
            const SELECTOR: [u8; 4] = [59u8, 36u8, 46u8, 74u8];
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
                        &self._operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointThresholdWeight()` and selector `0xb933fa74`.
    ```solidity
    function getLastCheckpointThresholdWeight() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightCall {}
    ///Container type for the return parameters of the [`getLastCheckpointThresholdWeight()`](getLastCheckpointThresholdWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointThresholdWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointThresholdWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointThresholdWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointThresholdWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointThresholdWeight()";
            const SELECTOR: [u8; 4] = [185u8, 51u8, 250u8, 116u8];
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
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointThresholdWeightAtBlock(uint32)` and selector `0x1e4cd85e`.
    ```solidity
    function getLastCheckpointThresholdWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightAtBlockCall {
        pub _blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getLastCheckpointThresholdWeightAtBlock(uint32)`](getLastCheckpointThresholdWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightAtBlockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightAtBlockCall>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: getLastCheckpointThresholdWeightAtBlockCall) -> Self {
                    (value._blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for getLastCheckpointThresholdWeightAtBlockCall
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _blockNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightAtBlockReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: getLastCheckpointThresholdWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for getLastCheckpointThresholdWeightAtBlockReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointThresholdWeightAtBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointThresholdWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointThresholdWeightAtBlock(uint32)";
            const SELECTOR: [u8; 4] = [30u8, 76u8, 216u8, 94u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._blockNumber,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointTotalWeight()` and selector `0x314f3a49`.
    ```solidity
    function getLastCheckpointTotalWeight() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightCall {}
    ///Container type for the return parameters of the [`getLastCheckpointTotalWeight()`](getLastCheckpointTotalWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointTotalWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointTotalWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointTotalWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointTotalWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointTotalWeight()";
            const SELECTOR: [u8; 4] = [49u8, 79u8, 58u8, 73u8];
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
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointTotalWeightAtBlock(uint32)` and selector `0x0dba3394`.
    ```solidity
    function getLastCheckpointTotalWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightAtBlockCall {
        pub _blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getLastCheckpointTotalWeightAtBlock(uint32)`](getLastCheckpointTotalWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightAtBlockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightAtBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightAtBlockCall) -> Self {
                    (value._blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointTotalWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _blockNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightAtBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointTotalWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointTotalWeightAtBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointTotalWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointTotalWeightAtBlock(uint32)";
            const SELECTOR: [u8; 4] = [13u8, 186u8, 51u8, 148u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._blockNumber,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastestOperatorSigningKey(address)` and selector `0xcdcd3581`.
    ```solidity
    function getLastestOperatorSigningKey(address _operator) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLastestOperatorSigningKeyCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getLastestOperatorSigningKey(address)`](getLastestOperatorSigningKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLastestOperatorSigningKeyReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastestOperatorSigningKeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastestOperatorSigningKeyCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastestOperatorSigningKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastestOperatorSigningKeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastestOperatorSigningKeyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastestOperatorSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastestOperatorSigningKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastestOperatorSigningKeyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastestOperatorSigningKey(address)";
            const SELECTOR: [u8; 4] = [205u8, 205u8, 53u8, 129u8];
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
                        &self._operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorSigningKeyAtBlock(address,uint256)` and selector `0x5e1042e8`.
    ```solidity
    function getOperatorSigningKeyAtBlock(address _operator, uint256 _blockNumber) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getOperatorSigningKeyAtBlockCall {
        pub _operator: alloy::sol_types::private::Address,
        pub _blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getOperatorSigningKeyAtBlock(address,uint256)`](getOperatorSigningKeyAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getOperatorSigningKeyAtBlockReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSigningKeyAtBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSigningKeyAtBlockCall) -> Self {
                    (value._operator, value._blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSigningKeyAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operator: tuple.0,
                        _blockNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSigningKeyAtBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSigningKeyAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSigningKeyAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSigningKeyAtBlockCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSigningKeyAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSigningKeyAtBlock(address,uint256)";
            const SELECTOR: [u8; 4] = [94u8, 16u8, 66u8, 232u8];
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
                        &self._operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._blockNumber,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorWeight(address)` and selector `0x98ec1ac9`.
    ```solidity
    function getOperatorWeight(address _operator) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getOperatorWeightCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorWeight(address)`](getOperatorWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getOperatorWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorWeightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorWeight(address)";
            const SELECTOR: [u8; 4] = [152u8, 236u8, 26u8, 201u8];
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
                        &self._operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorWeightAtBlock(address,uint32)` and selector `0x955f2d90`.
    ```solidity
    function getOperatorWeightAtBlock(address _operator, uint32 _blockNumber) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getOperatorWeightAtBlockCall {
        pub _operator: alloy::sol_types::private::Address,
        pub _blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorWeightAtBlock(address,uint32)`](getOperatorWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getOperatorWeightAtBlockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightAtBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightAtBlockCall) -> Self {
                    (value._operator, value._blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operator: tuple.0,
                        _blockNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightAtBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorWeightAtBlockCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorWeightAtBlock(address,uint32)";
            const SELECTOR: [u8; 4] = [149u8, 95u8, 45u8, 144u8];
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
                        &self._operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._blockNumber,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,uint256,((address,uint96)[]))` and selector `0xab118995`.
    ```solidity
    function initialize(address _serviceManager, uint256 _thresholdWeight, Quorum memory _quorum) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _serviceManager: alloy::sol_types::private::Address,
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
        pub _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initialize(address,uint256,((address,uint96)[]))`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                Quorum,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <Quorum as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._serviceManager, value._thresholdWeight, value._quorum)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _serviceManager: tuple.0,
                        _thresholdWeight: tuple.1,
                        _quorum: tuple.2,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                Quorum,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,uint256,((address,uint96)[]))";
            const SELECTOR: [u8; 4] = [171u8, 17u8, 137u8, 149u8];
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
                        &self._serviceManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._thresholdWeight,
                    ),
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._quorum),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`.
    ```solidity
    function isValidSignature(bytes32 _dataHash, bytes memory _signatureData) external view returns (bytes4);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct isValidSignatureCall {
        pub _dataHash: alloy::sol_types::private::FixedBytes<32>,
        pub _signatureData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`isValidSignature(bytes32,bytes)`](isValidSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct isValidSignatureReturn {
        pub _0: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureCall> for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureCall) -> Self {
                    (value._dataHash, value._signatureData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _dataHash: tuple.0,
                        _signatureData: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidSignatureCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidSignatureReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidSignature(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [22u8, 38u8, 186u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._dataHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._signatureData,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `minimumWeight()` and selector `0x40bf2fb7`.
    ```solidity
    function minimumWeight() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct minimumWeightCall {}
    ///Container type for the return parameters of the [`minimumWeight()`](minimumWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct minimumWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: minimumWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: minimumWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minimumWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = minimumWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minimumWeight()";
            const SELECTOR: [u8; 4] = [64u8, 191u8, 47u8, 183u8];
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
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `operatorRegistered(address)` and selector `0xec7fbb31`.
    ```solidity
    function operatorRegistered(address _operator) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct operatorRegisteredCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorRegistered(address)`](operatorRegisteredCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct operatorRegisteredReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorRegisteredCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorRegisteredCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorRegisteredCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorRegisteredReturn> for UnderlyingRustTuple<'_> {
                fn from(value: operatorRegisteredReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorRegisteredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorRegisteredCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorRegisteredReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorRegistered(address)";
            const SELECTOR: [u8; 4] = [236u8, 127u8, 187u8, 49u8];
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
                        &self._operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
    ```solidity
    function owner() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ownerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `quorum()` and selector `0x1703a018`.
    ```solidity
    function quorum() external view returns (Quorum memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct quorumCall {}
    ///Container type for the return parameters of the [`quorum()`](quorumCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct quorumReturn {
        pub _0: <Quorum as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Quorum,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (<Quorum as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumReturn;
            type ReturnTuple<'a> = (Quorum,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorum()";
            const SELECTOR: [u8; 4] = [23u8, 3u8, 160u8, 24u8];
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
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `registerOperatorWithSignature((bytes,bytes32,uint256),address)` and selector `0x3d5611f6`.
    ```solidity
    function registerOperatorWithSignature(ISignatureUtils.SignatureWithSaltAndExpiry memory _operatorSignature, address _signingKey) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct registerOperatorWithSignatureCall {
        pub _operatorSignature:
            <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        pub _signingKey: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`registerOperatorWithSignature((bytes,bytes32,uint256),address)`](registerOperatorWithSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct registerOperatorWithSignatureReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ISignatureUtils::SignatureWithSaltAndExpiry,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorWithSignatureCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithSignatureCall) -> Self {
                    (value._operatorSignature, value._signingKey)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorWithSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operatorSignature: tuple.0,
                        _signingKey: tuple.1,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorWithSignatureReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithSignatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorWithSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorWithSignatureCall {
            type Parameters<'a> = (
                ISignatureUtils::SignatureWithSaltAndExpiry,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorWithSignatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "registerOperatorWithSignature((bytes,bytes32,uint256),address)";
            const SELECTOR: [u8; 4] = [61u8, 86u8, 17u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self._operatorSignature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._signingKey,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
    ```solidity
    function renounceOwnership() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
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
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
    ```solidity
    function transferOwnership(address newOwner) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
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
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateMinimumWeight(uint256,address[])` and selector `0x696255be`.
    ```solidity
    function updateMinimumWeight(uint256 _newMinimumWeight, address[] memory _operators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateMinimumWeightCall {
        pub _newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
        pub _operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`updateMinimumWeight(uint256,address[])`](updateMinimumWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateMinimumWeightReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateMinimumWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateMinimumWeightCall) -> Self {
                    (value._newMinimumWeight, value._operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateMinimumWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _newMinimumWeight: tuple.0,
                        _operators: tuple.1,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateMinimumWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateMinimumWeightReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateMinimumWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateMinimumWeightCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateMinimumWeightReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateMinimumWeight(uint256,address[])";
            const SELECTOR: [u8; 4] = [105u8, 98u8, 85u8, 190u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._newMinimumWeight),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperatorSigningKey(address)` and selector `0x743c31f4`.
    ```solidity
    function updateOperatorSigningKey(address _newSigningKey) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateOperatorSigningKeyCall {
        pub _newSigningKey: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`updateOperatorSigningKey(address)`](updateOperatorSigningKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateOperatorSigningKeyReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorSigningKeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorSigningKeyCall) -> Self {
                    (value._newSigningKey,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorSigningKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _newSigningKey: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorSigningKeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorSigningKeyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorSigningKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorSigningKeyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorSigningKey(address)";
            const SELECTOR: [u8; 4] = [116u8, 60u8, 49u8, 244u8];
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
                        &self._newSigningKey,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperators(address[])` and selector `0x00cf2ab5`.
    ```solidity
    function updateOperators(address[] memory _operators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateOperatorsCall {
        pub _operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`updateOperators(address[])`](updateOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateOperatorsReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsCall) -> Self {
                    (value._operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operators: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperators(address[])";
            const SELECTOR: [u8; 4] = [0u8, 207u8, 42u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::SolType>::tokenize(
                    &self._operators
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperatorsForQuorum(address[][],bytes)` and selector `0x5140a548`.
    ```solidity
    function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumCall {
        pub operatorsPerQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        pub _1: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`updateOperatorsForQuorum(address[][],bytes)`](updateOperatorsForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                >,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumCall) -> Self {
                    (value.operatorsPerQuorum, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorsPerQuorum: tuple.0,
                        _1: tuple.1,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorsForQuorum(address[][],bytes)";
            const SELECTOR: [u8; 4] = [81u8, 64u8, 165u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.operatorsPerQuorum
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateQuorumConfig(((address,uint96)[]),address[])` and selector `0xdec5d1f6`.
    ```solidity
    function updateQuorumConfig(Quorum memory _quorum, address[] memory _operators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateQuorumConfigCall {
        pub _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
        pub _operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`updateQuorumConfig(((address,uint96)[]),address[])`](updateQuorumConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateQuorumConfigReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                Quorum,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Quorum as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateQuorumConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumConfigCall) -> Self {
                    (value._quorum, value._operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateQuorumConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _quorum: tuple.0,
                        _operators: tuple.1,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateQuorumConfigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateQuorumConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateQuorumConfigCall {
            type Parameters<'a> = (
                Quorum,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateQuorumConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateQuorumConfig(((address,uint96)[]),address[])";
            const SELECTOR: [u8; 4] = [222u8, 197u8, 209u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._quorum),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateStakeThreshold(uint256)` and selector `0x5ef53329`.
    ```solidity
    function updateStakeThreshold(uint256 _thresholdWeight) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateStakeThresholdCall {
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateStakeThreshold(uint256)`](updateStakeThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateStakeThresholdReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateStakeThresholdCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateStakeThresholdCall) -> Self {
                    (value._thresholdWeight,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateStakeThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _thresholdWeight: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateStakeThresholdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateStakeThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateStakeThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateStakeThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateStakeThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateStakeThreshold(uint256)";
            const SELECTOR: [u8; 4] = [94u8, 245u8, 51u8, 41u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._thresholdWeight,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`ECDSAStakeRegistry`](self) function calls.
    pub enum ECDSAStakeRegistryCalls {
        deregisterOperator(deregisterOperatorCall),
        getLastCheckpointOperatorWeight(getLastCheckpointOperatorWeightCall),
        getLastCheckpointThresholdWeight(getLastCheckpointThresholdWeightCall),
        getLastCheckpointThresholdWeightAtBlock(getLastCheckpointThresholdWeightAtBlockCall),
        getLastCheckpointTotalWeight(getLastCheckpointTotalWeightCall),
        getLastCheckpointTotalWeightAtBlock(getLastCheckpointTotalWeightAtBlockCall),
        getLastestOperatorSigningKey(getLastestOperatorSigningKeyCall),
        getOperatorSigningKeyAtBlock(getOperatorSigningKeyAtBlockCall),
        getOperatorWeight(getOperatorWeightCall),
        getOperatorWeightAtBlock(getOperatorWeightAtBlockCall),
        initialize(initializeCall),
        isValidSignature(isValidSignatureCall),
        minimumWeight(minimumWeightCall),
        operatorRegistered(operatorRegisteredCall),
        owner(ownerCall),
        quorum(quorumCall),
        registerOperatorWithSignature(registerOperatorWithSignatureCall),
        renounceOwnership(renounceOwnershipCall),
        transferOwnership(transferOwnershipCall),
        updateMinimumWeight(updateMinimumWeightCall),
        updateOperatorSigningKey(updateOperatorSigningKeyCall),
        updateOperators(updateOperatorsCall),
        updateOperatorsForQuorum(updateOperatorsForQuorumCall),
        updateQuorumConfig(updateQuorumConfigCall),
        updateStakeThreshold(updateStakeThresholdCall),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 207u8, 42u8, 181u8],
            [13u8, 186u8, 51u8, 148u8],
            [22u8, 38u8, 186u8, 126u8],
            [23u8, 3u8, 160u8, 24u8],
            [30u8, 76u8, 216u8, 94u8],
            [49u8, 79u8, 58u8, 73u8],
            [59u8, 36u8, 46u8, 74u8],
            [61u8, 86u8, 17u8, 246u8],
            [64u8, 191u8, 47u8, 183u8],
            [81u8, 64u8, 165u8, 72u8],
            [94u8, 16u8, 66u8, 232u8],
            [94u8, 245u8, 51u8, 41u8],
            [105u8, 98u8, 85u8, 190u8],
            [113u8, 80u8, 24u8, 166u8],
            [116u8, 60u8, 49u8, 244u8],
            [133u8, 125u8, 193u8, 144u8],
            [141u8, 165u8, 203u8, 91u8],
            [149u8, 95u8, 45u8, 144u8],
            [152u8, 236u8, 26u8, 201u8],
            [171u8, 17u8, 137u8, 149u8],
            [185u8, 51u8, 250u8, 116u8],
            [205u8, 205u8, 53u8, 129u8],
            [222u8, 197u8, 209u8, 246u8],
            [236u8, 127u8, 187u8, 49u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ECDSAStakeRegistryCalls {
        const NAME: &'static str = "ECDSAStakeRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointOperatorWeight(_) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointThresholdWeight(_) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointThresholdWeightAtBlock(_) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointTotalWeight(_) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointTotalWeightAtBlock(_) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastestOperatorSigningKey(_) => {
                    <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSigningKeyAtBlock(_) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorWeight(_) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorWeightAtBlock(_) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidSignature(_) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minimumWeight(_) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorRegistered(_) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::quorum(_) => <quorumCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::registerOperatorWithSignature(_) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateMinimumWeight(_) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorSigningKey(_) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperators(_) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorsForQuorum(_) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateQuorumConfig(_) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateStakeThreshold(_) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::SELECTOR
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
            )
                -> alloy_sol_types::Result<ECDSAStakeRegistryCalls>] = &[
                {
                    fn updateOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::updateOperators)
                    }
                    updateOperators
                },
                {
                    fn getLastCheckpointTotalWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointTotalWeightAtBlock,
                            )
                    }
                    getLastCheckpointTotalWeightAtBlock
                },
                {
                    fn isValidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <isValidSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::isValidSignature)
                    }
                    isValidSignature
                },
                {
                    fn quorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <quorumCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryCalls::quorum)
                    }
                    quorum
                },
                {
                    fn getLastCheckpointThresholdWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointThresholdWeightAtBlock,
                            )
                    }
                    getLastCheckpointThresholdWeightAtBlock
                },
                {
                    fn getLastCheckpointTotalWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getLastCheckpointTotalWeight)
                    }
                    getLastCheckpointTotalWeight
                },
                {
                    fn getLastCheckpointOperatorWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointOperatorWeight,
                            )
                    }
                    getLastCheckpointOperatorWeight
                },
                {
                    fn registerOperatorWithSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::registerOperatorWithSignature)
                    }
                    registerOperatorWithSignature
                },
                {
                    fn minimumWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <minimumWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::minimumWeight)
                    }
                    minimumWeight
                },
                {
                    fn updateOperatorsForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::updateOperatorsForQuorum)
                    }
                    updateOperatorsForQuorum
                },
                {
                    fn getOperatorSigningKeyAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getOperatorSigningKeyAtBlock)
                    }
                    getOperatorSigningKeyAtBlock
                },
                {
                    fn updateStakeThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::updateStakeThreshold)
                    }
                    updateStakeThreshold
                },
                {
                    fn updateMinimumWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::updateMinimumWeight)
                    }
                    updateMinimumWeight
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn updateOperatorSigningKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::updateOperatorSigningKey)
                    }
                    updateOperatorSigningKey
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryCalls::owner)
                    }
                    owner
                },
                {
                    fn getOperatorWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::getOperatorWeightAtBlock)
                    }
                    getOperatorWeightAtBlock
                },
                {
                    fn getOperatorWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::getOperatorWeight)
                    }
                    getOperatorWeight
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getLastCheckpointThresholdWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointThresholdWeight,
                            )
                    }
                    getLastCheckpointThresholdWeight
                },
                {
                    fn getLastestOperatorSigningKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getLastestOperatorSigningKey)
                    }
                    getLastestOperatorSigningKey
                },
                {
                    fn updateQuorumConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::updateQuorumConfig)
                    }
                    updateQuorumConfig
                },
                {
                    fn operatorRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::operatorRegistered)
                    }
                    operatorRegistered
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::transferOwnership)
                    }
                    transferOwnership
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointOperatorWeight(inner) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointThresholdWeight(inner) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointThresholdWeightAtBlock(inner) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointTotalWeight(inner) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointTotalWeightAtBlock(inner) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastestOperatorSigningKey(inner) => {
                    <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSigningKeyAtBlock(inner) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorWeight(inner) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorWeightAtBlock(inner) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::minimumWeight(inner) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorRegistered(inner) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::quorum(inner) => {
                    <quorumCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerOperatorWithSignature(inner) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateMinimumWeight(inner) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorSigningKey(inner) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateQuorumConfig(inner) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateStakeThreshold(inner) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointOperatorWeight(inner) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointThresholdWeight(inner) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointThresholdWeightAtBlock(inner) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointTotalWeight(inner) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointTotalWeightAtBlock(inner) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastestOperatorSigningKey(inner) => {
                    <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorSigningKeyAtBlock(inner) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorWeight(inner) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorWeightAtBlock(inner) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minimumWeight(inner) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorRegistered(inner) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::quorum(inner) => {
                    <quorumCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerOperatorWithSignature(inner) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateMinimumWeight(inner) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorSigningKey(inner) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateQuorumConfig(inner) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateStakeThreshold(inner) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ECDSAStakeRegistry`](self) custom errors.
    pub enum ECDSAStakeRegistryErrors {
        InsufficientSignedStake(InsufficientSignedStake),
        InsufficientWeight(InsufficientWeight),
        InvalidLength(InvalidLength),
        InvalidQuorum(InvalidQuorum),
        InvalidReferenceBlock(InvalidReferenceBlock),
        InvalidSignature(InvalidSignature),
        InvalidSignedWeight(InvalidSignedWeight),
        InvalidThreshold(InvalidThreshold),
        LengthMismatch(LengthMismatch),
        MustUpdateAllOperators(MustUpdateAllOperators),
        NotSorted(NotSorted),
        OperatorAlreadyRegistered(OperatorAlreadyRegistered),
        OperatorNotRegistered(OperatorNotRegistered),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [37u8, 236u8, 108u8, 31u8],
            [45u8, 61u8, 246u8, 182u8],
            [66u8, 238u8, 104u8, 181u8],
            [139u8, 170u8, 87u8, 159u8],
            [148u8, 125u8, 90u8, 132u8],
            [150u8, 11u8, 65u8, 238u8],
            [168u8, 121u8, 47u8, 209u8],
            [170u8, 189u8, 90u8, 9u8],
            [186u8, 80u8, 249u8, 17u8],
            [209u8, 115u8, 87u8, 121u8],
            [225u8, 33u8, 99u8, 47u8],
            [230u8, 79u8, 24u8, 15u8],
            [255u8, 99u8, 58u8, 56u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ECDSAStakeRegistryErrors {
        const NAME: &'static str = "ECDSAStakeRegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InsufficientSignedStake(_) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientWeight(_) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidLength(_) => <InvalidLength as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidQuorum(_) => <InvalidQuorum as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidReferenceBlock(_) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignedWeight(_) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidThreshold(_) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LengthMismatch(_) => <LengthMismatch as alloy_sol_types::SolError>::SELECTOR,
                Self::MustUpdateAllOperators(_) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotSorted(_) => <NotSorted as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorAlreadyRegistered(_) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
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
            )
                -> alloy_sol_types::Result<ECDSAStakeRegistryErrors>] = &[
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn MustUpdateAllOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::MustUpdateAllOperators)
                    }
                    MustUpdateAllOperators
                },
                {
                    fn OperatorAlreadyRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::OperatorAlreadyRegistered)
                    }
                    OperatorAlreadyRegistered
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn InvalidLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidLength as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryErrors::InvalidLength)
                    }
                    InvalidLength
                },
                {
                    fn InvalidSignedWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidSignedWeight as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::InvalidSignedWeight)
                    }
                    InvalidSignedWeight
                },
                {
                    fn InsufficientWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InsufficientWeight as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::InsufficientWeight)
                    }
                    InsufficientWeight
                },
                {
                    fn InvalidThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::InvalidThreshold)
                    }
                    InvalidThreshold
                },
                {
                    fn NotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <NotSorted as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryErrors::NotSorted)
                    }
                    NotSorted
                },
                {
                    fn InvalidQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidQuorum as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryErrors::InvalidQuorum)
                    }
                    InvalidQuorum
                },
                {
                    fn InsufficientSignedStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InsufficientSignedStake as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::InsufficientSignedStake)
                    }
                    InsufficientSignedStake
                },
                {
                    fn InvalidReferenceBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::InvalidReferenceBlock)
                    }
                    InvalidReferenceBlock
                },
                {
                    fn LengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <LengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::LengthMismatch)
                    }
                    LengthMismatch
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidReferenceBlock(inner) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidReferenceBlock(inner) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`ECDSAStakeRegistry`](self) events.
    pub enum ECDSAStakeRegistryEvents {
        Initialized(Initialized),
        MinimumWeightUpdated(MinimumWeightUpdated),
        OperatorDeregistered(OperatorDeregistered),
        OperatorRegistered(OperatorRegistered),
        OperatorWeightUpdated(OperatorWeightUpdated),
        OwnershipTransferred(OwnershipTransferred),
        QuorumUpdated(QuorumUpdated),
        SigningKeyUpdate(SigningKeyUpdate),
        ThresholdWeightUpdated(ThresholdWeightUpdated),
        TotalWeightUpdated(TotalWeightUpdated),
        UpdateMinimumWeight(UpdateMinimumWeight),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                30u8, 164u8, 33u8, 134u8, 179u8, 5u8, 250u8, 55u8, 49u8, 4u8, 80u8, 217u8, 251u8,
                135u8, 234u8, 30u8, 143u8, 12u8, 127u8, 68u8, 126u8, 119u8, 20u8, 121u8, 227u8,
                178u8, 118u8, 52u8, 191u8, 232u8, 77u8, 193u8,
            ],
            [
                35u8, 170u8, 212u8, 230u8, 23u8, 68u8, 236u8, 225u8, 100u8, 19u8, 10u8, 164u8,
                21u8, 193u8, 97u8, 110u8, 128u8, 19u8, 107u8, 15u8, 7u8, 112u8, 229u8, 101u8,
                137u8, 67u8, 139u8, 144u8, 178u8, 105u8, 38u8, 94u8,
            ],
            [
                49u8, 224u8, 173u8, 254u8, 199u8, 27u8, 204u8, 238u8, 55u8, 182u8, 232u8, 58u8,
                144u8, 194u8, 254u8, 219u8, 23u8, 216u8, 241u8, 105u8, 63u8, 238u8, 134u8, 60u8,
                71u8, 113u8, 231u8, 191u8, 226u8, 174u8, 213u8, 128u8,
            ],
            [
                113u8, 60u8, 165u8, 59u8, 136u8, 214u8, 235u8, 99u8, 245u8, 177u8, 133u8, 76u8,
                184u8, 203u8, 221u8, 115u8, 110u8, 197u8, 30u8, 218u8, 34u8, 94u8, 70u8, 121u8,
                26u8, 169u8, 41u8, 139u8, 1u8, 96u8, 100u8, 143u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                134u8, 220u8, 248u8, 107u8, 18u8, 223u8, 238u8, 222u8, 167u8, 74u8, 233u8, 48u8,
                13u8, 189u8, 170u8, 25u8, 59u8, 204u8, 229u8, 128u8, 147u8, 105u8, 200u8, 23u8,
                126u8, 162u8, 244u8, 234u8, 170u8, 101u8, 114u8, 155u8,
            ],
            [
                136u8, 119u8, 13u8, 200u8, 98u8, 228u8, 122u8, 126u8, 213u8, 134u8, 144u8, 120u8,
                87u8, 235u8, 27u8, 117u8, 228u8, 197u8, 255u8, 200u8, 183u8, 7u8, 199u8, 238u8,
                16u8, 235u8, 116u8, 214u8, 136u8, 95u8, 229u8, 148u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                147u8, 36u8, 247u8, 229u8, 167u8, 192u8, 40u8, 136u8, 8u8, 166u8, 52u8, 204u8,
                222u8, 68u8, 184u8, 233u8, 121u8, 103u8, 100u8, 116u8, 178u8, 46u8, 41u8, 238u8,
                157u8, 213u8, 105u8, 181u8, 94u8, 121u8, 26u8, 75u8,
            ],
            [
                164u8, 83u8, 219u8, 97u8, 42u8, 245u8, 158u8, 85u8, 33u8, 214u8, 171u8, 146u8,
                132u8, 220u8, 62u8, 45u8, 6u8, 175u8, 40u8, 110u8, 177u8, 177u8, 183u8, 183u8,
                113u8, 252u8, 228u8, 113u8, 108u8, 25u8, 242u8, 193u8,
            ],
            [
                208u8, 97u8, 22u8, 130u8, 82u8, 244u8, 65u8, 115u8, 54u8, 88u8, 240u8, 158u8, 77u8,
                143u8, 91u8, 45u8, 153u8, 142u8, 212u8, 239u8, 36u8, 162u8, 187u8, 253u8, 108u8,
                236u8, 165u8, 46u8, 161u8, 49u8, 80u8, 2u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ECDSAStakeRegistryEvents {
        const NAME: &'static str = "ECDSAStakeRegistryEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<MinimumWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MinimumWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::MinimumWeightUpdated)
                }
                Some(<OperatorDeregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorDeregistered)
                }
                Some(<OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorRegistered)
                }
                Some(<OperatorWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorWeightUpdated)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<QuorumUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumUpdated)
                }
                Some(<SigningKeyUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SigningKeyUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::SigningKeyUpdate)
                }
                Some(<ThresholdWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ThresholdWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ThresholdWeightUpdated)
                }
                Some(<TotalWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TotalWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TotalWeightUpdated)
                }
                Some(<UpdateMinimumWeight as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <UpdateMinimumWeight as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::UpdateMinimumWeight)
                }
                _ => alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                    name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                    log: alloy_sol_types::private::Box::new(
                        alloy_sol_types::private::LogData::new_unchecked(
                            topics.to_vec(),
                            data.to_vec().into(),
                        ),
                    ),
                }),
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for ECDSAStakeRegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MinimumWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SigningKeyUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ThresholdWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TotalWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateMinimumWeight(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MinimumWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SigningKeyUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ThresholdWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TotalWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateMinimumWeight(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ECDSAStakeRegistry`](self) contract instance.

    See the [wrapper's documentation](`ECDSAStakeRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ECDSAStakeRegistryInstance<T, P, N> {
        ECDSAStakeRegistryInstance::<T, P, N>::new(address, provider)
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
        _delegationManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<ECDSAStakeRegistryInstance<T, P, N>>>
    {
        ECDSAStakeRegistryInstance::<T, P, N>::deploy(provider, _delegationManager)
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
    >(
        provider: P,
        _delegationManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        ECDSAStakeRegistryInstance::<T, P, N>::deploy_builder(provider, _delegationManager)
    }
    /**A [`ECDSAStakeRegistry`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ECDSAStakeRegistry`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ECDSAStakeRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ECDSAStakeRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ECDSAStakeRegistryInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ECDSAStakeRegistryInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ECDSAStakeRegistry`](self) contract instance.

        See the [wrapper's documentation](`ECDSAStakeRegistryInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<ECDSAStakeRegistryInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _delegationManager);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _delegationManager,
                    })[..],
                ]
                .concat()
                .into(),
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
    impl<T, P: ::core::clone::Clone, N> ECDSAStakeRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ECDSAStakeRegistryInstance<T, P, N> {
            ECDSAStakeRegistryInstance {
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
        > ECDSAStakeRegistryInstance<T, P, N>
    {
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
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(&deregisterOperatorCall {})
        }
        ///Creates a new call builder for the [`getLastCheckpointOperatorWeight`] function.
        pub fn getLastCheckpointOperatorWeight(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointOperatorWeightCall, N> {
            self.call_builder(&getLastCheckpointOperatorWeightCall { _operator })
        }
        ///Creates a new call builder for the [`getLastCheckpointThresholdWeight`] function.
        pub fn getLastCheckpointThresholdWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointThresholdWeightCall, N>
        {
            self.call_builder(&getLastCheckpointThresholdWeightCall {})
        }
        ///Creates a new call builder for the [`getLastCheckpointThresholdWeightAtBlock`] function.
        pub fn getLastCheckpointThresholdWeightAtBlock(
            &self,
            _blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointThresholdWeightAtBlockCall, N>
        {
            self.call_builder(&getLastCheckpointThresholdWeightAtBlockCall { _blockNumber })
        }
        ///Creates a new call builder for the [`getLastCheckpointTotalWeight`] function.
        pub fn getLastCheckpointTotalWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointTotalWeightCall, N> {
            self.call_builder(&getLastCheckpointTotalWeightCall {})
        }
        ///Creates a new call builder for the [`getLastCheckpointTotalWeightAtBlock`] function.
        pub fn getLastCheckpointTotalWeightAtBlock(
            &self,
            _blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointTotalWeightAtBlockCall, N>
        {
            self.call_builder(&getLastCheckpointTotalWeightAtBlockCall { _blockNumber })
        }
        ///Creates a new call builder for the [`getLastestOperatorSigningKey`] function.
        pub fn getLastestOperatorSigningKey(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastestOperatorSigningKeyCall, N> {
            self.call_builder(&getLastestOperatorSigningKeyCall { _operator })
        }
        ///Creates a new call builder for the [`getOperatorSigningKeyAtBlock`] function.
        pub fn getOperatorSigningKeyAtBlock(
            &self,
            _operator: alloy::sol_types::private::Address,
            _blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSigningKeyAtBlockCall, N> {
            self.call_builder(&getOperatorSigningKeyAtBlockCall {
                _operator,
                _blockNumber,
            })
        }
        ///Creates a new call builder for the [`getOperatorWeight`] function.
        pub fn getOperatorWeight(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorWeightCall, N> {
            self.call_builder(&getOperatorWeightCall { _operator })
        }
        ///Creates a new call builder for the [`getOperatorWeightAtBlock`] function.
        pub fn getOperatorWeightAtBlock(
            &self,
            _operator: alloy::sol_types::private::Address,
            _blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorWeightAtBlockCall, N> {
            self.call_builder(&getOperatorWeightAtBlockCall {
                _operator,
                _blockNumber,
            })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _serviceManager: alloy::sol_types::private::Address,
            _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
            _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                _serviceManager,
                _thresholdWeight,
                _quorum,
            })
        }
        ///Creates a new call builder for the [`isValidSignature`] function.
        pub fn isValidSignature(
            &self,
            _dataHash: alloy::sol_types::private::FixedBytes<32>,
            _signatureData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidSignatureCall, N> {
            self.call_builder(&isValidSignatureCall {
                _dataHash,
                _signatureData,
            })
        }
        ///Creates a new call builder for the [`minimumWeight`] function.
        pub fn minimumWeight(&self) -> alloy_contract::SolCallBuilder<T, &P, minimumWeightCall, N> {
            self.call_builder(&minimumWeightCall {})
        }
        ///Creates a new call builder for the [`operatorRegistered`] function.
        pub fn operatorRegistered(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorRegisteredCall, N> {
            self.call_builder(&operatorRegisteredCall { _operator })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`quorum`] function.
        pub fn quorum(&self) -> alloy_contract::SolCallBuilder<T, &P, quorumCall, N> {
            self.call_builder(&quorumCall {})
        }
        ///Creates a new call builder for the [`registerOperatorWithSignature`] function.
        pub fn registerOperatorWithSignature(
            &self,
            _operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
            _signingKey: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorWithSignatureCall, N> {
            self.call_builder(&registerOperatorWithSignatureCall {
                _operatorSignature,
                _signingKey,
            })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`updateMinimumWeight`] function.
        pub fn updateMinimumWeight(
            &self,
            _newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
            _operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateMinimumWeightCall, N> {
            self.call_builder(&updateMinimumWeightCall {
                _newMinimumWeight,
                _operators,
            })
        }
        ///Creates a new call builder for the [`updateOperatorSigningKey`] function.
        pub fn updateOperatorSigningKey(
            &self,
            _newSigningKey: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorSigningKeyCall, N> {
            self.call_builder(&updateOperatorSigningKeyCall { _newSigningKey })
        }
        ///Creates a new call builder for the [`updateOperators`] function.
        pub fn updateOperators(
            &self,
            _operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsCall, N> {
            self.call_builder(&updateOperatorsCall { _operators })
        }
        ///Creates a new call builder for the [`updateOperatorsForQuorum`] function.
        pub fn updateOperatorsForQuorum(
            &self,
            operatorsPerQuorum: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsForQuorumCall, N> {
            self.call_builder(&updateOperatorsForQuorumCall {
                operatorsPerQuorum,
                _1,
            })
        }
        ///Creates a new call builder for the [`updateQuorumConfig`] function.
        pub fn updateQuorumConfig(
            &self,
            _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
            _operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateQuorumConfigCall, N> {
            self.call_builder(&updateQuorumConfigCall {
                _quorum,
                _operators,
            })
        }
        ///Creates a new call builder for the [`updateStakeThreshold`] function.
        pub fn updateStakeThreshold(
            &self,
            _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateStakeThresholdCall, N> {
            self.call_builder(&updateStakeThresholdCall { _thresholdWeight })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ECDSAStakeRegistryInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`MinimumWeightUpdated`] event.
        pub fn MinimumWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MinimumWeightUpdated, N> {
            self.event_filter::<MinimumWeightUpdated>()
        }
        ///Creates a new event filter for the [`OperatorDeregistered`] event.
        pub fn OperatorDeregistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorDeregistered, N> {
            self.event_filter::<OperatorDeregistered>()
        }
        ///Creates a new event filter for the [`OperatorRegistered`] event.
        pub fn OperatorRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRegistered, N> {
            self.event_filter::<OperatorRegistered>()
        }
        ///Creates a new event filter for the [`OperatorWeightUpdated`] event.
        pub fn OperatorWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorWeightUpdated, N> {
            self.event_filter::<OperatorWeightUpdated>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`QuorumUpdated`] event.
        pub fn QuorumUpdated_filter(&self) -> alloy_contract::Event<T, &P, QuorumUpdated, N> {
            self.event_filter::<QuorumUpdated>()
        }
        ///Creates a new event filter for the [`SigningKeyUpdate`] event.
        pub fn SigningKeyUpdate_filter(&self) -> alloy_contract::Event<T, &P, SigningKeyUpdate, N> {
            self.event_filter::<SigningKeyUpdate>()
        }
        ///Creates a new event filter for the [`ThresholdWeightUpdated`] event.
        pub fn ThresholdWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ThresholdWeightUpdated, N> {
            self.event_filter::<ThresholdWeightUpdated>()
        }
        ///Creates a new event filter for the [`TotalWeightUpdated`] event.
        pub fn TotalWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TotalWeightUpdated, N> {
            self.event_filter::<TotalWeightUpdated>()
        }
        ///Creates a new event filter for the [`UpdateMinimumWeight`] event.
        pub fn UpdateMinimumWeight_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UpdateMinimumWeight, N> {
            self.event_filter::<UpdateMinimumWeight>()
        }
    }
}
