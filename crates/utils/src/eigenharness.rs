///Module containing a contract's types and functions.
/**

```solidity
library ERC20VotesUpgradeable {
    struct Checkpoint { uint32 fromBlock; uint224 votes; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod ERC20VotesUpgradeable {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Checkpoint { uint32 fromBlock; uint224 votes; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Checkpoint {
        pub fromBlock: u32,
        pub votes: alloy::sol_types::private::primitives::aliases::U224,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<224>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
            alloy::sol_types::private::primitives::aliases::U224,
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
        impl ::core::convert::From<Checkpoint> for UnderlyingRustTuple<'_> {
            fn from(value: Checkpoint) -> Self {
                (value.fromBlock, value.votes)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Checkpoint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    fromBlock: tuple.0,
                    votes: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Checkpoint {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Checkpoint {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.fromBlock),
                    <alloy::sol_types::sol_data::Uint<
                        224,
                    > as alloy_sol_types::SolType>::tokenize(&self.votes),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Checkpoint {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Checkpoint {
            const NAME: &'static str = "Checkpoint";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Checkpoint(uint32 fromBlock,uint224 votes)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.fromBlock)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        224,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.votes)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Checkpoint {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fromBlock,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        224,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.votes)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.fromBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    224,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.votes,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ERC20VotesUpgradeable`](self) contract instance.

See the [wrapper's documentation](`ERC20VotesUpgradeableInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ERC20VotesUpgradeableInstance<T, P, N> {
        ERC20VotesUpgradeableInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ERC20VotesUpgradeable`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ERC20VotesUpgradeable`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ERC20VotesUpgradeableInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ERC20VotesUpgradeableInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ERC20VotesUpgradeableInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ERC20VotesUpgradeableInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ERC20VotesUpgradeable`](self) contract instance.

See the [wrapper's documentation](`ERC20VotesUpgradeableInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ERC20VotesUpgradeableInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ERC20VotesUpgradeableInstance<T, P, N> {
            ERC20VotesUpgradeableInstance {
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
    > ERC20VotesUpgradeableInstance<T, P, N> {
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
    > ERC20VotesUpgradeableInstance<T, P, N> {
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
library ERC20VotesUpgradeable {
    struct Checkpoint {
        uint32 fromBlock;
        uint224 votes;
    }
}

interface EigenHarness {
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);
    event DelegateVotesChanged(address indexed delegate, uint256 previousBalance, uint256 newBalance);
    event EIP712DomainChanged();
    event Initialized(uint8 version);
    event Mint(address indexed minter, uint256 amount);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event SetAllowedFrom(address indexed from, bool isAllowedFrom);
    event SetAllowedTo(address indexed to, bool isAllowedTo);
    event Transfer(address indexed from, address indexed to, uint256 value);
    event TransferRestrictionsDisabled();

    constructor(address _bEIGEN);

    function CLOCK_MODE() external pure returns (string memory);
    function DOMAIN_SEPARATOR() external view returns (bytes32);
    function allowance(address owner, address spender) external view returns (uint256);
    function allowedFrom(address) external view returns (bool);
    function allowedTo(address) external view returns (bool);
    function approve(address spender, uint256 amount) external returns (bool);
    function bEIGEN() external view returns (address);
    function balanceOf(address account) external view returns (uint256);
    function checkpoints(address account, uint32 pos) external view returns (ERC20VotesUpgradeable.Checkpoint memory);
    function clock() external view returns (uint48);
    function decimals() external view returns (uint8);
    function decreaseAllowance(address spender, uint256 subtractedValue) external returns (bool);
    function delegate(address delegatee) external;
    function delegateBySig(address delegatee, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s) external;
    function delegates(address account) external view returns (address);
    function disableTransferRestrictions() external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getPastTotalSupply(uint256 timepoint) external view returns (uint256);
    function getPastVotes(address account, uint256 timepoint) external view returns (uint256);
    function getVotes(address account) external view returns (uint256);
    function increaseAllowance(address spender, uint256 addedValue) external returns (bool);
    function initialize(address initialOwner, address[] memory minters, uint256[] memory mintingAllowances, uint256[] memory mintAllowedAfters) external;
    function mint() external;
    function mint(address to, uint256 amount) external;
    function mintAllowedAfter(address) external view returns (uint256);
    function mintingAllowance(address) external view returns (uint256);
    function multisend(address[] memory receivers, uint256[] memory amounts) external;
    function name() external view returns (string memory);
    function nonces(address owner) external view returns (uint256);
    function numCheckpoints(address account) external view returns (uint32);
    function owner() external view returns (address);
    function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
    function renounceOwnership() external;
    function setAllowedFrom(address from, bool isAllowedFrom) external;
    function setAllowedFromPermissionless(address from, bool isAllowedFrom) external;
    function setAllowedTo(address to, bool isAllowedTo) external;
    function setTransferRestrictionsDisabledAfterToMax() external;
    function symbol() external view returns (string memory);
    function totalSupply() external view returns (uint256);
    function transfer(address to, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
    function transferOwnership(address newOwner) external;
    function transferOwnershipPermissionless(address newOwner) external;
    function transferRestrictionsDisabledAfter() external view returns (uint256);
    function unwrap(uint256 amount) external;
    function wrap(uint256 amount) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_bEIGEN",
        "type": "address",
        "internalType": "contract IERC20"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "CLOCK_MODE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "DOMAIN_SEPARATOR",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "allowance",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "spender",
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
    "name": "allowedFrom",
    "inputs": [
      {
        "name": "",
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
    "name": "allowedTo",
    "inputs": [
      {
        "name": "",
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
    "name": "approve",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "bEIGEN",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IERC20"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "balanceOf",
    "inputs": [
      {
        "name": "account",
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
    "name": "checkpoints",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "pos",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct ERC20VotesUpgradeable.Checkpoint",
        "components": [
          {
            "name": "fromBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "votes",
            "type": "uint224",
            "internalType": "uint224"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "clock",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint48",
        "internalType": "uint48"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "decimals",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "decreaseAllowance",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "subtractedValue",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegate",
    "inputs": [
      {
        "name": "delegatee",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegateBySig",
    "inputs": [
      {
        "name": "delegatee",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "v",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "r",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegates",
    "inputs": [
      {
        "name": "account",
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
    "name": "disableTransferRestrictions",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "eip712Domain",
    "inputs": [],
    "outputs": [
      {
        "name": "fields",
        "type": "bytes1",
        "internalType": "bytes1"
      },
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "version",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "verifyingContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "extensions",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPastTotalSupply",
    "inputs": [
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "getPastVotes",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "getVotes",
    "inputs": [
      {
        "name": "account",
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
    "name": "increaseAllowance",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "addedValue",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "initialOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "minters",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "mintingAllowances",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "mintAllowedAfters",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "mint",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "mint",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "mintAllowedAfter",
    "inputs": [
      {
        "name": "",
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
    "name": "mintingAllowance",
    "inputs": [
      {
        "name": "",
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
    "name": "multisend",
    "inputs": [
      {
        "name": "receivers",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "amounts",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "name",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nonces",
    "inputs": [
      {
        "name": "owner",
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
    "name": "numCheckpoints",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "permit",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "v",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "r",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "setAllowedFrom",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "isAllowedFrom",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setAllowedFromPermissionless",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "isAllowedFrom",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setAllowedTo",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "isAllowedTo",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setTransferRestrictionsDisabledAfterToMax",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "symbol",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "totalSupply",
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
    "name": "transfer",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferFrom",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
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
    "name": "transferOwnershipPermissionless",
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
    "name": "transferRestrictionsDisabledAfter",
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
    "name": "unwrap",
    "inputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "wrap",
    "inputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "Approval",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DelegateChanged",
    "inputs": [
      {
        "name": "delegator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "fromDelegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "toDelegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DelegateVotesChanged",
    "inputs": [
      {
        "name": "delegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "previousBalance",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newBalance",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EIP712DomainChanged",
    "inputs": [],
    "anonymous": false
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
    "name": "Mint",
    "inputs": [
      {
        "name": "minter",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
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
    "name": "SetAllowedFrom",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "isAllowedFrom",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SetAllowedTo",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "isAllowedTo",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Transfer",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TransferRestrictionsDisabled",
    "inputs": [],
    "anonymous": false
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod EigenHarness {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a060405234801561001057600080fd5b506040516136b03803806136b083398101604081905261002f9161010b565b6001600160a01b0381166080528061004561004c565b505061013b565b600054610100900460ff16156100b85760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff90811614610109576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b60006020828403121561011d57600080fd5b81516001600160a01b038116811461013457600080fd5b9392505050565b60805161354561016b60003960008181610378015281816108c601528181611563015261165201526135456000f3fe608060405234801561001057600080fd5b506004361061028a5760003560e01c806384b0196e1161015c578063aad41a41116100ce578063de0e9a3e11610087578063de0e9a3e1461063a578063ea598cb01461064d578063eb415f4514610660578063ec08f9c414610668578063f1127ed81461067b578063f2fde38b146106b857600080fd5b8063aad41a41146105cf578063b3793585146105e2578063b8c25594146105ee578063c3cda52014610601578063d505accf14610614578063dd62ed3e1461062757600080fd5b806395d89b411161012057806395d89b41146105715780639ab24eb0146105795780639aec4bae1461058c578063a457c2d714610596578063a7d1195d146105a9578063a9059cbb146105bc57600080fd5b806384b0196e146105005780638da5cb5b1461051b5780638e539e8c1461052c57806391ddadf41461053f57806394b819dc1461055e57600080fd5b80633f4da4c6116102005780636fcfff45116101b95780636fcfff451461044f57806370a0823114610477578063715018a6146104a057806378aa33ba146104a85780637ecebe00146104cc57806381b97161146104df57600080fd5b80633f4da4c61461037357806340c10f19146103b25780634bf5d7e9146103c557806353957125146103ef578063587cde1e146104105780635c19a95c1461043c57600080fd5b80631ffacdef116102525780631ffacdef1461031057806323b872dd14610323578063313ce567146103365780633644e51514610345578063395093511461034d5780633a46b1a81461036057600080fd5b80630455e6941461028f57806306fdde03146102c8578063095ea7b3146102dd5780631249c58b146102f057806318160ddd146102fa575b600080fd5b6102b361029d366004612d17565b6101336020526000908152604090205460ff1681565b60405190151581526020015b60405180910390f35b6102d06106cb565b6040516102bf9190612d78565b6102b36102eb366004612d8b565b61075d565b6102f8610777565b005b6103026108c2565b6040519081526020016102bf565b6102f861031e366004612dc3565b61094b565b6102b3610331366004612dfa565b6109b4565b604051601281526020016102bf565b6103026109d8565b6102b361035b366004612d8b565b6109e2565b61030261036e366004612d8b565b610a04565b61039a7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016102bf565b6102f86103c0366004612d8b565b610a89565b60408051808201909152600e81526d06d6f64653d74696d657374616d760941b60208201526102d0565b6103026103fd366004612d17565b6101306020526000908152604090205481565b61039a61041e366004612d17565b6001600160a01b03908116600090815260fe60205260409020541690565b6102f861044a366004612d17565b610a97565b61046261045d366004612d17565b610aa4565b60405163ffffffff90911681526020016102bf565b610302610485366004612d17565b6001600160a01b031660009081526065602052604090205490565b6102f8610ac6565b6102b36104b6366004612d17565b6101346020526000908152604090205460ff1681565b6103026104da366004612d17565b610ada565b6103026104ed366004612d17565b6101316020526000908152604090205481565b610508610af8565b6040516102bf9796959493929190612e37565b6033546001600160a01b031661039a565b61030261053a366004612ecf565b610b96565b610547610bfe565b60405165ffffffffffff90911681526020016102bf565b6102f861056c366004612dc3565b610953565b6102d0610c09565b610302610587366004612d17565b610c18565b6103026101325481565b6102b36105a4366004612d8b565b610c9a565b6102f86105b7366004612fb6565b610d15565b6102b36105ca366004612d8b565b611138565b6102f86105dd366004613107565b611146565b6102f860001961013255565b6102f86105fc366004612dc3565b611217565b6102f861060f366004613189565b611278565b6102f86106223660046131e1565b6113ae565b61030261063536600461324c565b611512565b6102f8610648366004612ecf565b61153d565b6102f861065b366004612ecf565b611630565b6102f8611728565b6102f8610676366004612d17565b6117f1565b61068e61068936600461327f565b6117fa565b60408051825163ffffffff1681526020928301516001600160e01b031692810192909252016102bf565b6102f86106c6366004612d17565b61187e565b6060606880546106da906132b4565b80601f0160208091040260200160405190810160405280929190818152602001828054610706906132b4565b80156107535780601f1061072857610100808354040283529160200191610753565b820191906000526020600020905b81548152906001019060200180831161073657829003601f168201915b5050505050905090565b60003361076b8185856118eb565b60019150505b92915050565b33600090815261013160205260409020546107f15760405162461bcd60e51b815260206004820152602f60248201527f456967656e2e6d696e743a206d73672e73656e64657220686173206e6f206d6960448201526e6e74696e6720616c6c6f77616e636560881b60648201526084015b60405180910390fd5b3360009081526101306020526040902054421161086a5760405162461bcd60e51b815260206004820152603160248201527f456967656e2e6d696e743a206d73672e73656e646572206973206e6f7420616c6044820152701b1bddd959081d1bc81b5a5b9d081e595d607a1b60648201526084016107e8565b3360008181526101316020526040812080549190559061088a9082611a0f565b60405181815233907f0f6798a560793a54c3bcfe86a93cde1e73087d944c0ea20544137d41213968859060200160405180910390a250565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610922573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061094691906132e8565b905090565b610953611aa5565b6001600160a01b03821660008181526101336020908152604091829020805460ff191685151590811790915591519182527fcf20b1ecb604b0e8888d579c64e8a3b10e590d45c1c2dddb393bed284362227191015b60405180910390a25050565b6000336109c2858285611aff565b6109cd858585611b73565b506001949350505050565b6000610946611d2f565b60003361076b8185856109f58383611512565b6109ff9190613317565b6118eb565b6000610a0e610bfe565b65ffffffffffff168210610a605760405162461bcd60e51b815260206004820152601960248201527804552433230566f7465733a20667574757265206c6f6f6b757603c1b60448201526064016107e8565b6001600160a01b038316600090815260ff60205260409020610a829083611d39565b9392505050565b610a938282611a0f565b5050565b610aa13382611e22565b50565b6001600160a01b038116600090815260ff602052604081205461077190611e9c565b610ace611aa5565b610ad86000611f05565b565b6001600160a01b038116600090815260cb6020526040812054610771565b6000606080600080600060606097546000801b148015610b185750609854155b610b5c5760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b60448201526064016107e8565b610b64611f57565b610b6c611f66565b60408051600080825260208201909252600f60f81b9b939a50919850469750309650945092509050565b6000610ba0610bfe565b65ffffffffffff168210610bf25760405162461bcd60e51b815260206004820152601960248201527804552433230566f7465733a20667574757265206c6f6f6b757603c1b60448201526064016107e8565b61077161010083611d39565b600061094642611f75565b6060606980546106da906132b4565b6001600160a01b038116600090815260ff60205260408120548015610c87576001600160a01b038316600090815260ff6020526040902080546000198301908110610c6557610c6561332a565b60009182526020909120015464010000000090046001600160e01b0316610c8a565b60005b6001600160e01b03169392505050565b60003381610ca88286611512565b905083811015610d085760405162461bcd60e51b815260206004820152602560248201527f45524332303a2064656372656173656420616c6c6f77616e63652062656c6f77604482015264207a65726f60d81b60648201526084016107e8565b6109cd82868684036118eb565b600054610100900460ff1615808015610d355750600054600160ff909116105b80610d4f5750303b158015610d4f575060005460ff166001145b610db25760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016107e8565b6000805460ff191660011790558015610dd5576000805461ff0019166101001790555b610ddd611fdc565b610e216040518060400160405280600581526020016422b4b3b2b760d91b8152506040518060400160405280600581526020016422a4a3a2a760d91b81525061200b565b610e2a85611f05565b610e506040518060400160405280600581526020016422a4a3a2a760d91b81525061203c565b8251845114610ed75760405162461bcd60e51b815260206004820152604760248201527f456967656e2e696e697469616c697a653a206d696e7465727320616e64206d6960448201527f6e74696e67416c6c6f77616e636573206d757374206265207468652073616d65606482015266040d8cadccee8d60cb1b608482015260a4016107e8565b8151845114610f5e5760405162461bcd60e51b815260206004820152604760248201527f456967656e2e696e697469616c697a653a206d696e7465727320616e64206d6960448201527f6e74416c6c6f776564416674657273206d757374206265207468652073616d65606482015266040d8cadccee8d60cb1b608482015260a4016107e8565b60005b84518110156110e357838181518110610f7c57610f7c61332a565b60200260200101516101316000878481518110610f9b57610f9b61332a565b60200260200101516001600160a01b03166001600160a01b0316815260200190815260200160002081905550828181518110610fd957610fd961332a565b60200260200101516101306000878481518110610ff857610ff861332a565b60200260200101516001600160a01b03166001600160a01b03168152602001908152602001600020819055506001610133600087848151811061103d5761103d61332a565b60200260200101516001600160a01b03166001600160a01b0316815260200190815260200160002060006101000a81548160ff02191690831515021790555084818151811061108e5761108e61332a565b60200260200101516001600160a01b03167fcf20b1ecb604b0e8888d579c64e8a3b10e590d45c1c2dddb393bed284362227160016040516110d3911515815260200190565b60405180910390a2600101610f61565b50600019610132558015611131576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050505050565b60003361076b818585611b73565b8281146111bb5760405162461bcd60e51b815260206004820152603e60248201527f456967656e2e6d756c746973656e643a2072656365697665727320616e64206160448201527f6d6f756e7473206d757374206265207468652073616d65206c656e677468000060648201526084016107e8565b60005b838110156111315761120f338686848181106111dc576111dc61332a565b90506020020160208101906111f19190612d17565b8585858181106112035761120361332a565b90506020020135611b73565b6001016111be565b61121f611aa5565b6001600160a01b03821660008181526101346020908152604091829020805460ff191685151590811790915591519182527f72a561d1af7409467dae4f1e9fc52590a9335a1dda17727e2b6aa8c4db35109b91016109a8565b834211156112c85760405162461bcd60e51b815260206004820152601d60248201527f4552433230566f7465733a207369676e6174757265206578706972656400000060448201526064016107e8565b604080517fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf60208201526001600160a01b0388169181019190915260608101869052608081018590526000906113429061133a9060a00160405160208183030381529060405280519060200120612086565b8585856120b3565b905061134d816120db565b861461139b5760405162461bcd60e51b815260206004820152601960248201527f4552433230566f7465733a20696e76616c6964206e6f6e63650000000000000060448201526064016107e8565b6113a58188611e22565b50505050505050565b834211156113fe5760405162461bcd60e51b815260206004820152601d60248201527f45524332305065726d69743a206578706972656420646561646c696e6500000060448201526064016107e8565b60007f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c988888861142d8c6120db565b6040805160208101969096526001600160a01b0394851690860152929091166060840152608083015260a082015260c0810186905260e001604051602081830303815290604052805190602001209050600061148882612086565b90506000611498828787876120b3565b9050896001600160a01b0316816001600160a01b0316146114fb5760405162461bcd60e51b815260206004820152601e60248201527f45524332305065726d69743a20696e76616c6964207369676e6174757265000060448201526064016107e8565b6115068a8a8a6118eb565b50505050505050505050565b6001600160a01b03918216600090815260666020908152604080832093909416825291909152205490565b6115473382612103565b60405163a9059cbb60e01b8152336004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063a9059cbb906044016020604051808303816000875af11580156115b4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115d89190613340565b610aa15760405162461bcd60e51b8152602060048201526024808201527f456967656e2e756e777261703a2062454947454e207472616e736665722066616044820152631a5b195960e21b60648201526084016107e8565b6040516323b872dd60e01b8152336004820152306024820152604481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906323b872dd906064016020604051808303816000875af11580156116a3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116c79190613340565b61171e5760405162461bcd60e51b815260206004820152602260248201527f456967656e2e777261703a2062454947454e207472616e73666572206661696c604482015261195960f21b60648201526084016107e8565b610aa13382611a0f565b611730611aa5565b60001961013254146117c05760405162461bcd60e51b815260206004820152604d60248201527f456967656e2e64697361626c655472616e736665725265737472696374696f6e60448201527f733a207472616e73666572207265737472696374696f6e732061726520616c7260648201526c1958591e48191a5cd8589b1959609a1b608482015260a4016107e8565b60006101328190556040517f2b18986d3ba809db2f13a5d7bf17f60d357b37d9cbb55dd71cbbac8dc4060f649190a1565b610aa181611f05565b60408051808201909152600080825260208201526001600160a01b038316600090815260ff60205260409020805463ffffffff841690811061183e5761183e61332a565b60009182526020918290206040805180820190915291015463ffffffff8116825264010000000090046001600160e01b0316918101919091529392505050565b611886611aa5565b6001600160a01b0381166117f15760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016107e8565b6001600160a01b03831661194d5760405162461bcd60e51b8152602060048201526024808201527f45524332303a20617070726f76652066726f6d20746865207a65726f206164646044820152637265737360e01b60648201526084016107e8565b6001600160a01b0382166119ae5760405162461bcd60e51b815260206004820152602260248201527f45524332303a20617070726f766520746f20746865207a65726f206164647265604482015261737360f01b60648201526084016107e8565b6001600160a01b0383811660008181526066602090815260408083209487168084529482529182902085905590518481527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a3505050565b611a19828261211c565b6001600160e01b03611a296108c2565b1115611a905760405162461bcd60e51b815260206004820152603060248201527f4552433230566f7465733a20746f74616c20737570706c79207269736b73206f60448201526f766572666c6f77696e6720766f74657360801b60648201526084016107e8565b611a9f6101006121f1836121fd565b50505050565b6033546001600160a01b03163314610ad85760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016107e8565b6000611b0b8484611512565b90506000198114611a9f5781811015611b665760405162461bcd60e51b815260206004820152601d60248201527f45524332303a20696e73756666696369656e7420616c6c6f77616e636500000060448201526064016107e8565b611a9f84848484036118eb565b6001600160a01b038316611bd75760405162461bcd60e51b815260206004820152602560248201527f45524332303a207472616e736665722066726f6d20746865207a65726f206164604482015264647265737360d81b60648201526084016107e8565b6001600160a01b038216611c395760405162461bcd60e51b815260206004820152602360248201527f45524332303a207472616e7366657220746f20746865207a65726f206164647260448201526265737360e81b60648201526084016107e8565b611c44838383612372565b6001600160a01b03831660009081526065602052604090205481811015611cbc5760405162461bcd60e51b815260206004820152602660248201527f45524332303a207472616e7366657220616d6f756e7420657863656564732062604482015265616c616e636560d01b60648201526084016107e8565b6001600160a01b0380851660008181526065602052604080822086860390559286168082529083902080548601905591517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90611d1c9086815260200190565b60405180910390a3611a9f848484612458565b600061094661248a565b815460009081816005811115611d93576000611d54846124fe565b611d5e908561335d565b600088815260209020909150869082015463ffffffff161115611d8357809150611d91565b611d8e816001613317565b92505b505b80821015611de0576000611da783836125e6565b600088815260209020909150869082015463ffffffff161115611dcc57809150611dda565b611dd7816001613317565b92505b50611d93565b8015611e0c576000868152602090208101600019015464010000000090046001600160e01b0316611e0f565b60005b6001600160e01b03169695505050505050565b6001600160a01b03828116600081815260fe6020818152604080842080546065845282862054949093528787166001600160a01b03198416811790915590519190951694919391928592917f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9190a4611a9f828483612601565b600063ffffffff821115611f015760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b60648201526084016107e8565b5090565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6060609980546106da906132b4565b6060609a80546106da906132b4565b600065ffffffffffff821115611f015760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203460448201526538206269747360d01b60648201526084016107e8565b600054610100900460ff166120035760405162461bcd60e51b81526004016107e890613370565b610ad861273e565b600054610100900460ff166120325760405162461bcd60e51b81526004016107e890613370565b610a93828261276e565b600054610100900460ff166120635760405162461bcd60e51b81526004016107e890613370565b610aa181604051806040016040528060018152602001603160f81b8152506127ae565b6000610771612093611d2f565b8360405161190160f01b8152600281019290925260228201526042902090565b60008060006120c4878787876127fd565b915091506120d1816128c1565b5095945050505050565b6001600160a01b038116600090815260cb602052604090208054600181018255905b50919050565b61210d8282612a0b565b611a9f610100612b52836121fd565b6001600160a01b0382166121725760405162461bcd60e51b815260206004820152601f60248201527f45524332303a206d696e7420746f20746865207a65726f20616464726573730060448201526064016107e8565b61217e60008383612372565b80606760008282546121909190613317565b90915550506001600160a01b0382166000818152606560209081526040808320805486019055518481527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a3610a9360008383612458565b6000610a828284613317565b8254600090819081811561224a5760008781526020902082016000190160408051808201909152905463ffffffff8116825264010000000090046001600160e01b0316602082015261225f565b60408051808201909152600080825260208201525b905080602001516001600160e01b0316935061227f84868863ffffffff16565b92506000821180156122a95750612294610bfe565b65ffffffffffff16816000015163ffffffff16145b156122ee576122b783612b5e565b60008881526020902083016000190180546001600160e01b03929092166401000000000263ffffffff909216919091179055612368565b866040518060400160405280612312612305610bfe565b65ffffffffffff16611e9c565b63ffffffff16815260200161232686612b5e565b6001600160e01b0390811690915282546001810184556000938452602093849020835194909301519091166401000000000263ffffffff909316929092179101555b5050935093915050565b610132544211612453576001600160a01b038316158061239957506001600160a01b038216155b806123bd57506001600160a01b0383166000908152610133602052604090205460ff165b806123e157506001600160a01b0382166000908152610134602052604090205460ff165b6124535760405162461bcd60e51b815260206004820152603a60248201527f456967656e2e5f6265666f7265546f6b656e5472616e736665723a2066726f6d60448201527f206f7220746f206d7573742062652077686974656c697374656400000000000060648201526084016107e8565b505050565b6001600160a01b03838116600090815260fe602052604080822054858416835291205461245392918216911683612601565b60007f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6124b5612bc7565b6124bd612c20565b60408051602081019490945283019190915260608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b60008160000361251057506000919050565b6000600161251d84612c51565b901c6001901b90506001818481612536576125366133bb565b048201901c9050600181848161254e5761254e6133bb565b048201901c90506001818481612566576125666133bb565b048201901c9050600181848161257e5761257e6133bb565b048201901c90506001818481612596576125966133bb565b048201901c905060018184816125ae576125ae6133bb565b048201901c905060018184816125c6576125c66133bb565b048201901c9050610a82818285816125e0576125e06133bb565b04612ce5565b60006125f560028484186133d1565b610a8290848416613317565b816001600160a01b0316836001600160a01b0316141580156126235750600081115b15612453576001600160a01b038316156126b1576001600160a01b038316600090815260ff60205260408120819061265e90612b52856121fd565b91509150846001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72483836040516126a6929190918252602082015260400190565b60405180910390a250505b6001600160a01b03821615612453576001600160a01b038216600090815260ff6020526040812081906126e7906121f1856121fd565b91509150836001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724838360405161272f929190918252602082015260400190565b60405180910390a25050505050565b600054610100900460ff166127655760405162461bcd60e51b81526004016107e890613370565b610ad833611f05565b600054610100900460ff166127955760405162461bcd60e51b81526004016107e890613370565b60686127a1838261343a565b506069612453828261343a565b600054610100900460ff166127d55760405162461bcd60e51b81526004016107e890613370565b60996127e1838261343a565b50609a6127ee828261343a565b50506000609781905560985550565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083111561283457506000905060036128b8565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015612888573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166128b1576000600192509250506128b8565b9150600090505b94509492505050565b60008160048111156128d5576128d56134f9565b036128dd5750565b60018160048111156128f1576128f16134f9565b0361293e5760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e6174757265000000000000000060448201526064016107e8565b6002816004811115612952576129526134f9565b0361299f5760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e6774680060448201526064016107e8565b60038160048111156129b3576129b36134f9565b03610aa15760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b60648201526084016107e8565b6001600160a01b038216612a6b5760405162461bcd60e51b815260206004820152602160248201527f45524332303a206275726e2066726f6d20746865207a65726f206164647265736044820152607360f81b60648201526084016107e8565b612a7782600083612372565b6001600160a01b03821660009081526065602052604090205481811015612aeb5760405162461bcd60e51b815260206004820152602260248201527f45524332303a206275726e20616d6f756e7420657863656564732062616c616e604482015261636560f01b60648201526084016107e8565b6001600160a01b03831660008181526065602090815260408083208686039055606780548790039055518581529192917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a361245383600084612458565b6000610a82828461335d565b60006001600160e01b03821115611f015760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b60648201526084016107e8565b600080612bd2611f57565b805190915015612be9578051602090910120919050565b6097548015612bf85792915050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4709250505090565b600080612c2b611f66565b805190915015612c42578051602090910120919050565b6098548015612bf85792915050565b600080608083901c15612c6657608092831c92015b604083901c15612c7857604092831c92015b602083901c15612c8a57602092831c92015b601083901c15612c9c57601092831c92015b600883901c15612cae57600892831c92015b600483901c15612cc057600492831c92015b600283901c15612cd257600292831c92015b600183901c156107715760010192915050565b6000818310612cf45781610a82565b5090919050565b80356001600160a01b0381168114612d1257600080fd5b919050565b600060208284031215612d2957600080fd5b610a8282612cfb565b6000815180845260005b81811015612d5857602081850181015186830182015201612d3c565b506000602082860101526020601f19601f83011685010191505092915050565b602081526000610a826020830184612d32565b60008060408385031215612d9e57600080fd5b612da783612cfb565b946020939093013593505050565b8015158114610aa157600080fd5b60008060408385031215612dd657600080fd5b612ddf83612cfb565b91506020830135612def81612db5565b809150509250929050565b600080600060608486031215612e0f57600080fd5b612e1884612cfb565b9250612e2660208501612cfb565b929592945050506040919091013590565b60ff60f81b8816815260e060208201526000612e5660e0830189612d32565b8281036040840152612e688189612d32565b606084018890526001600160a01b038716608085015260a0840186905283810360c08501528451808252602080870193509091019060005b81811015612ebe578351835260209384019390920191600101612ea0565b50909b9a5050505050505050505050565b600060208284031215612ee157600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff81118282101715612f2757612f27612ee8565b604052919050565b600067ffffffffffffffff821115612f4957612f49612ee8565b5060051b60200190565b600082601f830112612f6457600080fd5b8135612f77612f7282612f2f565b612efe565b8082825260208201915060208360051b860101925085831115612f9957600080fd5b602085015b838110156120d1578035835260209283019201612f9e565b60008060008060808587031215612fcc57600080fd5b612fd585612cfb565b9350602085013567ffffffffffffffff811115612ff157600080fd5b8501601f8101871361300257600080fd5b8035613010612f7282612f2f565b8082825260208201915060208360051b85010192508983111561303257600080fd5b6020840193505b8284101561305b5761304a84612cfb565b825260209384019390910190613039565b9550505050604085013567ffffffffffffffff81111561307a57600080fd5b61308687828801612f53565b925050606085013567ffffffffffffffff8111156130a357600080fd5b6130af87828801612f53565b91505092959194509250565b60008083601f8401126130cd57600080fd5b50813567ffffffffffffffff8111156130e557600080fd5b6020830191508360208260051b850101111561310057600080fd5b9250929050565b6000806000806040858703121561311d57600080fd5b843567ffffffffffffffff81111561313457600080fd5b613140878288016130bb565b909550935050602085013567ffffffffffffffff81111561316057600080fd5b61316c878288016130bb565b95989497509550505050565b803560ff81168114612d1257600080fd5b60008060008060008060c087890312156131a257600080fd5b6131ab87612cfb565b955060208701359450604087013593506131c760608801613178565b9598949750929560808101359460a0909101359350915050565b600080600080600080600060e0888a0312156131fc57600080fd5b61320588612cfb565b965061321360208901612cfb565b9550604088013594506060880135935061322f60808901613178565b9699959850939692959460a0840135945060c09093013592915050565b6000806040838503121561325f57600080fd5b61326883612cfb565b915061327660208401612cfb565b90509250929050565b6000806040838503121561329257600080fd5b61329b83612cfb565b9150602083013563ffffffff81168114612def57600080fd5b600181811c908216806132c857607f821691505b6020821081036120fd57634e487b7160e01b600052602260045260246000fd5b6000602082840312156132fa57600080fd5b5051919050565b634e487b7160e01b600052601160045260246000fd5b8082018082111561077157610771613301565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561335257600080fd5b8151610a8281612db5565b8181038181111561077157610771613301565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b634e487b7160e01b600052601260045260246000fd5b6000826133ee57634e487b7160e01b600052601260045260246000fd5b500490565b601f82111561245357806000526020600020601f840160051c8101602085101561341a5750805b601f840160051c820191505b818110156111315760008155600101613426565b815167ffffffffffffffff81111561345457613454612ee8565b6134688161346284546132b4565b846133f3565b6020601f82116001811461349c57600083156134845750848201515b600019600385901b1c1916600184901b178455611131565b600084815260208120601f198516915b828110156134cc57878501518255602094850194600190920191016134ac565b50848210156134ea5786840151600019600387901b60f8161c191681555b50505050600190811b01905550565b634e487b7160e01b600052602160045260246000fdfea2646970667358221220613daad20a66c11953e089a40a5c91ca5ca5c5da8357fe79ef3dd97e5e2e027e64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa6\xB08\x03\x80a6\xB0\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R\x80a\0Ea\0LV[PPa\x01;V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x14a\x01\tW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15a\x01\x1DW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x014W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa5Ea\x01k`\09`\0\x81\x81a\x03x\x01R\x81\x81a\x08\xC6\x01R\x81\x81a\x15c\x01Ra\x16R\x01Ra5E`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x8AW`\x005`\xE0\x1C\x80c\x84\xB0\x19n\x11a\x01\\W\x80c\xAA\xD4\x1AA\x11a\0\xCEW\x80c\xDE\x0E\x9A>\x11a\0\x87W\x80c\xDE\x0E\x9A>\x14a\x06:W\x80c\xEAY\x8C\xB0\x14a\x06MW\x80c\xEBA_E\x14a\x06`W\x80c\xEC\x08\xF9\xC4\x14a\x06hW\x80c\xF1\x12~\xD8\x14a\x06{W\x80c\xF2\xFD\xE3\x8B\x14a\x06\xB8W`\0\x80\xFD[\x80c\xAA\xD4\x1AA\x14a\x05\xCFW\x80c\xB3y5\x85\x14a\x05\xE2W\x80c\xB8\xC2U\x94\x14a\x05\xEEW\x80c\xC3\xCD\xA5 \x14a\x06\x01W\x80c\xD5\x05\xAC\xCF\x14a\x06\x14W\x80c\xDDb\xED>\x14a\x06'W`\0\x80\xFD[\x80c\x95\xD8\x9BA\x11a\x01 W\x80c\x95\xD8\x9BA\x14a\x05qW\x80c\x9A\xB2N\xB0\x14a\x05yW\x80c\x9A\xECK\xAE\x14a\x05\x8CW\x80c\xA4W\xC2\xD7\x14a\x05\x96W\x80c\xA7\xD1\x19]\x14a\x05\xA9W\x80c\xA9\x05\x9C\xBB\x14a\x05\xBCW`\0\x80\xFD[\x80c\x84\xB0\x19n\x14a\x05\0W\x80c\x8D\xA5\xCB[\x14a\x05\x1BW\x80c\x8ES\x9E\x8C\x14a\x05,W\x80c\x91\xDD\xAD\xF4\x14a\x05?W\x80c\x94\xB8\x19\xDC\x14a\x05^W`\0\x80\xFD[\x80c?M\xA4\xC6\x11a\x02\0W\x80co\xCF\xFFE\x11a\x01\xB9W\x80co\xCF\xFFE\x14a\x04OW\x80cp\xA0\x821\x14a\x04wW\x80cqP\x18\xA6\x14a\x04\xA0W\x80cx\xAA3\xBA\x14a\x04\xA8W\x80c~\xCE\xBE\0\x14a\x04\xCCW\x80c\x81\xB9qa\x14a\x04\xDFW`\0\x80\xFD[\x80c?M\xA4\xC6\x14a\x03sW\x80c@\xC1\x0F\x19\x14a\x03\xB2W\x80cK\xF5\xD7\xE9\x14a\x03\xC5W\x80cS\x95q%\x14a\x03\xEFW\x80cX|\xDE\x1E\x14a\x04\x10W\x80c\\\x19\xA9\\\x14a\x04<W`\0\x80\xFD[\x80c\x1F\xFA\xCD\xEF\x11a\x02RW\x80c\x1F\xFA\xCD\xEF\x14a\x03\x10W\x80c#\xB8r\xDD\x14a\x03#W\x80c1<\xE5g\x14a\x036W\x80c6D\xE5\x15\x14a\x03EW\x80c9P\x93Q\x14a\x03MW\x80c:F\xB1\xA8\x14a\x03`W`\0\x80\xFD[\x80c\x04U\xE6\x94\x14a\x02\x8FW\x80c\x06\xFD\xDE\x03\x14a\x02\xC8W\x80c\t^\xA7\xB3\x14a\x02\xDDW\x80c\x12I\xC5\x8B\x14a\x02\xF0W\x80c\x18\x16\r\xDD\x14a\x02\xFAW[`\0\x80\xFD[a\x02\xB3a\x02\x9D6`\x04a-\x17V[a\x013` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xD0a\x06\xCBV[`@Qa\x02\xBF\x91\x90a-xV[a\x02\xB3a\x02\xEB6`\x04a-\x8BV[a\x07]V[a\x02\xF8a\x07wV[\0[a\x03\x02a\x08\xC2V[`@Q\x90\x81R` \x01a\x02\xBFV[a\x02\xF8a\x03\x1E6`\x04a-\xC3V[a\tKV[a\x02\xB3a\x0316`\x04a-\xFAV[a\t\xB4V[`@Q`\x12\x81R` \x01a\x02\xBFV[a\x03\x02a\t\xD8V[a\x02\xB3a\x03[6`\x04a-\x8BV[a\t\xE2V[a\x03\x02a\x03n6`\x04a-\x8BV[a\n\x04V[a\x03\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xBFV[a\x02\xF8a\x03\xC06`\x04a-\x8BV[a\n\x89V[`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x06\xD6\xF6FS\xD7F\x96\xD6W7F\x16\xD7`\x94\x1B` \x82\x01Ra\x02\xD0V[a\x03\x02a\x03\xFD6`\x04a-\x17V[a\x010` R`\0\x90\x81R`@\x90 T\x81V[a\x03\x9Aa\x04\x1E6`\x04a-\x17V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\xFE` R`@\x90 T\x16\x90V[a\x02\xF8a\x04J6`\x04a-\x17V[a\n\x97V[a\x04ba\x04]6`\x04a-\x17V[a\n\xA4V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xBFV[a\x03\x02a\x04\x856`\x04a-\x17V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`e` R`@\x90 T\x90V[a\x02\xF8a\n\xC6V[a\x02\xB3a\x04\xB66`\x04a-\x17V[a\x014` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\x02a\x04\xDA6`\x04a-\x17V[a\n\xDAV[a\x03\x02a\x04\xED6`\x04a-\x17V[a\x011` R`\0\x90\x81R`@\x90 T\x81V[a\x05\x08a\n\xF8V[`@Qa\x02\xBF\x97\x96\x95\x94\x93\x92\x91\x90a.7V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x9AV[a\x03\x02a\x05:6`\x04a.\xCFV[a\x0B\x96V[a\x05Ga\x0B\xFEV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xBFV[a\x02\xF8a\x05l6`\x04a-\xC3V[a\tSV[a\x02\xD0a\x0C\tV[a\x03\x02a\x05\x876`\x04a-\x17V[a\x0C\x18V[a\x03\x02a\x012T\x81V[a\x02\xB3a\x05\xA46`\x04a-\x8BV[a\x0C\x9AV[a\x02\xF8a\x05\xB76`\x04a/\xB6V[a\r\x15V[a\x02\xB3a\x05\xCA6`\x04a-\x8BV[a\x118V[a\x02\xF8a\x05\xDD6`\x04a1\x07V[a\x11FV[a\x02\xF8`\0\x19a\x012UV[a\x02\xF8a\x05\xFC6`\x04a-\xC3V[a\x12\x17V[a\x02\xF8a\x06\x0F6`\x04a1\x89V[a\x12xV[a\x02\xF8a\x06\"6`\x04a1\xE1V[a\x13\xAEV[a\x03\x02a\x0656`\x04a2LV[a\x15\x12V[a\x02\xF8a\x06H6`\x04a.\xCFV[a\x15=V[a\x02\xF8a\x06[6`\x04a.\xCFV[a\x160V[a\x02\xF8a\x17(V[a\x02\xF8a\x06v6`\x04a-\x17V[a\x17\xF1V[a\x06\x8Ea\x06\x896`\x04a2\x7FV[a\x17\xFAV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xE0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02\xBFV[a\x02\xF8a\x06\xC66`\x04a-\x17V[a\x18~V[```h\x80Ta\x06\xDA\x90a2\xB4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\x06\x90a2\xB4V[\x80\x15a\x07SW\x80`\x1F\x10a\x07(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07SV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x076W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x07k\x81\x85\x85a\x18\xEBV[`\x01\x91PP[\x92\x91PPV[3`\0\x90\x81Ra\x011` R`@\x90 Ta\x07\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FEigen.mint: msg.sender has no mi`D\x82\x01Rnnting allowance`\x88\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81Ra\x010` R`@\x90 TB\x11a\x08jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FEigen.mint: msg.sender is not al`D\x82\x01Rp\x1B\x1B\xDD\xD9Y\x08\x1D\x1B\xC8\x1BZ[\x9D\x08\x1EY]`z\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[3`\0\x81\x81Ra\x011` R`@\x81 \x80T\x91\x90U\x90a\x08\x8A\x90\x82a\x1A\x0FV[`@Q\x81\x81R3\x90\x7F\x0Fg\x98\xA5`y:T\xC3\xBC\xFE\x86\xA9<\xDE\x1Es\x08}\x94L\x0E\xA2\x05D\x13}A!9h\x85\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tF\x91\x90a2\xE8V[\x90P\x90V[a\tSa\x1A\xA5V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x013` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\xCF \xB1\xEC\xB6\x04\xB0\xE8\x88\x8DW\x9Cd\xE8\xA3\xB1\x0EY\rE\xC1\xC2\xDD\xDB9;\xED(Cb\"q\x91\x01[`@Q\x80\x91\x03\x90\xA2PPV[`\x003a\t\xC2\x85\x82\x85a\x1A\xFFV[a\t\xCD\x85\x85\x85a\x1BsV[P`\x01\x94\x93PPPPV[`\0a\tFa\x1D/V[`\x003a\x07k\x81\x85\x85a\t\xF5\x83\x83a\x15\x12V[a\t\xFF\x91\x90a3\x17V[a\x18\xEBV[`\0a\n\x0Ea\x0B\xFEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\n`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x04U$3#\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`<\x1B`D\x82\x01R`d\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x90 a\n\x82\x90\x83a\x1D9V[\x93\x92PPPV[a\n\x93\x82\x82a\x1A\x0FV[PPV[a\n\xA13\x82a\x1E\"V[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xFF` R`@\x81 Ta\x07q\x90a\x1E\x9CV[a\n\xCEa\x1A\xA5V[a\n\xD8`\0a\x1F\x05V[V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCB` R`@\x81 Ta\x07qV[`\0``\x80`\0\x80`\0```\x97T`\0\x80\x1B\x14\x80\x15a\x0B\x18WP`\x98T\x15[a\x0B\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01a\x07\xE8V[a\x0Bda\x1FWV[a\x0Bla\x1FfV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0a\x0B\xA0a\x0B\xFEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\x0B\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x04U$3#\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`<\x1B`D\x82\x01R`d\x01a\x07\xE8V[a\x07qa\x01\0\x83a\x1D9V[`\0a\tFBa\x1FuV[```i\x80Ta\x06\xDA\x90a2\xB4V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xFF` R`@\x81 T\x80\x15a\x0C\x87W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x90 \x80T`\0\x19\x83\x01\x90\x81\x10a\x0CeWa\x0Cea3*V[`\0\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0C\x8AV[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[`\x003\x81a\x0C\xA8\x82\x86a\x15\x12V[\x90P\x83\x81\x10\x15a\r\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[a\t\xCD\x82\x86\x86\x84\x03a\x18\xEBV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\r5WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\rOWP0;\x15\x80\x15a\rOWP`\0T`\xFF\x16`\x01\x14[a\r\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\r\xD5W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\r\xDDa\x1F\xDCV[a\x0E!`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\"\xB4\xB3\xB2\xB7`\xD9\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\"\xA4\xA3\xA2\xA7`\xD9\x1B\x81RPa \x0BV[a\x0E*\x85a\x1F\x05V[a\x0EP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\"\xA4\xA3\xA2\xA7`\xD9\x1B\x81RPa <V[\x82Q\x84Q\x14a\x0E\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEigen.initialize: minters and mi`D\x82\x01R\x7FntingAllowances must be the same`d\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`\x84\x82\x01R`\xA4\x01a\x07\xE8V[\x81Q\x84Q\x14a\x0F^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEigen.initialize: minters and mi`D\x82\x01R\x7FntAllowedAfters must be the same`d\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`\x84\x82\x01R`\xA4\x01a\x07\xE8V[`\0[\x84Q\x81\x10\x15a\x10\xE3W\x83\x81\x81Q\x81\x10a\x0F|Wa\x0F|a3*V[` \x02` \x01\x01Qa\x011`\0\x87\x84\x81Q\x81\x10a\x0F\x9BWa\x0F\x9Ba3*V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x82\x81\x81Q\x81\x10a\x0F\xD9Wa\x0F\xD9a3*V[` \x02` \x01\x01Qa\x010`\0\x87\x84\x81Q\x81\x10a\x0F\xF8Wa\x0F\xF8a3*V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01a\x013`\0\x87\x84\x81Q\x81\x10a\x10=Wa\x10=a3*V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x84\x81\x81Q\x81\x10a\x10\x8EWa\x10\x8Ea3*V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7F\xCF \xB1\xEC\xB6\x04\xB0\xE8\x88\x8DW\x9Cd\xE8\xA3\xB1\x0EY\rE\xC1\xC2\xDD\xDB9;\xED(Cb\"q`\x01`@Qa\x10\xD3\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a\x0FaV[P`\0\x19a\x012U\x80\x15a\x111W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`\x003a\x07k\x81\x85\x85a\x1BsV[\x82\x81\x14a\x11\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FEigen.multisend: receivers and a`D\x82\x01R\x7Fmounts must be the same length\0\0`d\x82\x01R`\x84\x01a\x07\xE8V[`\0[\x83\x81\x10\x15a\x111Wa\x12\x0F3\x86\x86\x84\x81\x81\x10a\x11\xDCWa\x11\xDCa3*V[\x90P` \x02\x01` \x81\x01\x90a\x11\xF1\x91\x90a-\x17V[\x85\x85\x85\x81\x81\x10a\x12\x03Wa\x12\x03a3*V[\x90P` \x02\x015a\x1BsV[`\x01\x01a\x11\xBEV[a\x12\x1Fa\x1A\xA5V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x014` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7Fr\xA5a\xD1\xAFt\tF}\xAEO\x1E\x9F\xC5%\x90\xA93Z\x1D\xDA\x17r~+j\xA8\xC4\xDB5\x10\x9B\x91\x01a\t\xA8V[\x83B\x11\x15a\x12\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Votes: signature expired\0\0\0`D\x82\x01R`d\x01a\x07\xE8V[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\0\x90a\x13B\x90a\x13:\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a \x86V[\x85\x85\x85a \xB3V[\x90Pa\x13M\x81a \xDBV[\x86\x14a\x13\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC20Votes: invalid nonce\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xE8V[a\x13\xA5\x81\x88a\x1E\"V[PPPPPPPV[\x83B\x11\x15a\x13\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Permit: expired deadline\0\0\0`D\x82\x01R`d\x01a\x07\xE8V[`\0\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a\x14-\x8Ca \xDBV[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x14\x88\x82a \x86V[\x90P`\0a\x14\x98\x82\x87\x87\x87a \xB3V[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FERC20Permit: invalid signature\0\0`D\x82\x01R`d\x01a\x07\xE8V[a\x15\x06\x8A\x8A\x8Aa\x18\xEBV[PPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`f` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x15G3\x82a!\x03V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x15\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xD8\x91\x90a3@V[a\n\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FEigen.unwrap: bEIGEN transfer fa`D\x82\x01Rc\x1A[\x19Y`\xE2\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x16\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xC7\x91\x90a3@V[a\x17\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEigen.wrap: bEIGEN transfer fail`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[a\n\xA13\x82a\x1A\x0FV[a\x170a\x1A\xA5V[`\0\x19a\x012T\x14a\x17\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FEigen.disableTransferRestriction`D\x82\x01R\x7Fs: transfer restrictions are alr`d\x82\x01Rl\x19XY\x1EH\x19\x1A\\\xD8X\x9B\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x07\xE8V[`\0a\x012\x81\x90U`@Q\x7F+\x18\x98m;\xA8\t\xDB/\x13\xA5\xD7\xBF\x17\xF6\r5{7\xD9\xCB\xB5]\xD7\x1C\xBB\xAC\x8D\xC4\x06\x0Fd\x91\x90\xA1V[a\n\xA1\x81a\x1F\x05V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x18>Wa\x18>a3*V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x82Rd\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x18\x86a\x1A\xA5V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x17\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x19MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x19\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`f` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x1A\x19\x82\x82a!\x1CV[`\x01`\x01`\xE0\x1B\x03a\x1A)a\x08\xC2V[\x11\x15a\x1A\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FERC20Votes: total supply risks o`D\x82\x01Roverflowing votes`\x80\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[a\x1A\x9Fa\x01\0a!\xF1\x83a!\xFDV[PPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07\xE8V[`\0a\x1B\x0B\x84\x84a\x15\x12V[\x90P`\0\x19\x81\x14a\x1A\x9FW\x81\x81\x10\x15a\x1BfW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x07\xE8V[a\x1A\x9F\x84\x84\x84\x84\x03a\x18\xEBV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x1B\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1C9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[a\x1CD\x83\x83\x83a#rV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`e` R`@\x90 T\x81\x81\x10\x15a\x1C\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`e` R`@\x80\x82 \x86\x86\x03\x90U\x92\x86\x16\x80\x82R\x90\x83\x90 \x80T\x86\x01\x90U\x91Q\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x1D\x1C\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x1A\x9F\x84\x84\x84a$XV[`\0a\tFa$\x8AV[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a\x1D\x93W`\0a\x1DT\x84a$\xFEV[a\x1D^\x90\x85a3]V[`\0\x88\x81R` \x90 \x90\x91P\x86\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1D\x83W\x80\x91Pa\x1D\x91V[a\x1D\x8E\x81`\x01a3\x17V[\x92P[P[\x80\x82\x10\x15a\x1D\xE0W`\0a\x1D\xA7\x83\x83a%\xE6V[`\0\x88\x81R` \x90 \x90\x91P\x86\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1D\xCCW\x80\x91Pa\x1D\xDAV[a\x1D\xD7\x81`\x01a3\x17V[\x92P[Pa\x1D\x93V[\x80\x15a\x1E\x0CW`\0\x86\x81R` \x90 \x81\x01`\0\x19\x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x1E\x0FV[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\xFE` \x81\x81R`@\x80\x84 \x80T`e\x84R\x82\x86 T\x94\x90\x93R\x87\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x84\x16\x81\x17\x90\x91U\x90Q\x91\x90\x95\x16\x94\x91\x93\x91\x92\x85\x92\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\x1A\x9F\x82\x84\x83a&\x01V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x1F\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[P\x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[```\x99\x80Ta\x06\xDA\x90a2\xB4V[```\x9A\x80Ta\x06\xDA\x90a2\xB4V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1F\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\0Ta\x01\0\x90\x04`\xFF\x16a \x03W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xE8\x90a3pV[a\n\xD8a'>V[`\0Ta\x01\0\x90\x04`\xFF\x16a 2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xE8\x90a3pV[a\n\x93\x82\x82a'nV[`\0Ta\x01\0\x90\x04`\xFF\x16a cW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xE8\x90a3pV[a\n\xA1\x81`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RPa'\xAEV[`\0a\x07qa \x93a\x1D/V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a \xC4\x87\x87\x87\x87a'\xFDV[\x91P\x91Pa \xD1\x81a(\xC1V[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCB` R`@\x90 \x80T`\x01\x81\x01\x82U\x90[P\x91\x90PV[a!\r\x82\x82a*\x0BV[a\x1A\x9Fa\x01\0a+R\x83a!\xFDV[`\x01`\x01`\xA0\x1B\x03\x82\x16a!rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x07\xE8V[a!~`\0\x83\x83a#rV[\x80`g`\0\x82\x82Ta!\x90\x91\x90a3\x17V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`e` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\n\x93`\0\x83\x83a$XV[`\0a\n\x82\x82\x84a3\x17V[\x82T`\0\x90\x81\x90\x81\x81\x15a\"JW`\0\x87\x81R` \x90 \x82\x01`\0\x19\x01`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x82Rd\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16` \x82\x01Ra\"_V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R[\x90P\x80` \x01Q`\x01`\x01`\xE0\x1B\x03\x16\x93Pa\"\x7F\x84\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x92P`\0\x82\x11\x80\x15a\"\xA9WPa\"\x94a\x0B\xFEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x14[\x15a\"\xEEWa\"\xB7\x83a+^V[`\0\x88\x81R` \x90 \x83\x01`\0\x19\x01\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua#hV[\x86`@Q\x80`@\x01`@R\x80a#\x12a#\x05a\x0B\xFEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1E\x9CV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a#&\x86a+^V[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[PP\x93P\x93\x91PPV[a\x012TB\x11a$SW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80a#\x99WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15[\x80a#\xBDWP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x013` R`@\x90 T`\xFF\x16[\x80a#\xE1WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x014` R`@\x90 T`\xFF\x16[a$SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FEigen._beforeTokenTransfer: from`D\x82\x01R\x7F or to must be whitelisted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xE8V[PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\xFE` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta$S\x92\x91\x82\x16\x91\x16\x83a&\x01V[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa$\xB5a+\xC7V[a$\xBDa, V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x81`\0\x03a%\x10WP`\0\x91\x90PV[`\0`\x01a%\x1D\x84a,QV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a%6Wa%6a3\xBBV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a%NWa%Na3\xBBV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a%fWa%fa3\xBBV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a%~Wa%~a3\xBBV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a%\x96Wa%\x96a3\xBBV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a%\xAEWa%\xAEa3\xBBV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a%\xC6Wa%\xC6a3\xBBV[\x04\x82\x01\x90\x1C\x90Pa\n\x82\x81\x82\x85\x81a%\xE0Wa%\xE0a3\xBBV[\x04a,\xE5V[`\0a%\xF5`\x02\x84\x84\x18a3\xD1V[a\n\x82\x90\x84\x84\x16a3\x17V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a&#WP`\0\x81\x11[\x15a$SW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a&\xB1W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x81 \x81\x90a&^\x90a+R\x85a!\xFDV[\x91P\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa&\xA6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a$SW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xFF` R`@\x81 \x81\x90a&\xE7\x90a!\xF1\x85a!\xFDV[\x91P\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa'/\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a'eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xE8\x90a3pV[a\n\xD83a\x1F\x05V[`\0Ta\x01\0\x90\x04`\xFF\x16a'\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xE8\x90a3pV[`ha'\xA1\x83\x82a4:V[P`ia$S\x82\x82a4:V[`\0Ta\x01\0\x90\x04`\xFF\x16a'\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xE8\x90a3pV[`\x99a'\xE1\x83\x82a4:V[P`\x9Aa'\xEE\x82\x82a4:V[PP`\0`\x97\x81\x90U`\x98UPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a(4WP`\0\x90P`\x03a(\xB8V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a(\x88W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a(\xB1W`\0`\x01\x92P\x92PPa(\xB8V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a(\xD5Wa(\xD5a4\xF9V[\x03a(\xDDWPV[`\x01\x81`\x04\x81\x11\x15a(\xF1Wa(\xF1a4\xF9V[\x03a)>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xE8V[`\x02\x81`\x04\x81\x11\x15a)RWa)Ra4\xF9V[\x03a)\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x07\xE8V[`\x03\x81`\x04\x81\x11\x15a)\xB3Wa)\xB3a4\xF9V[\x03a\n\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x82\x16a*kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[a*w\x82`\0\x83a#rV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`e` R`@\x90 T\x81\x81\x10\x15a*\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`e` \x90\x81R`@\x80\x83 \x86\x86\x03\x90U`g\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a$S\x83`\0\x84a$XV[`\0a\n\x82\x82\x84a3]V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x1F\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\0\x80a+\xD2a\x1FWV[\x80Q\x90\x91P\x15a+\xE9W\x80Q` \x90\x91\x01 \x91\x90PV[`\x97T\x80\x15a+\xF8W\x92\x91PPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x92PPP\x90V[`\0\x80a,+a\x1FfV[\x80Q\x90\x91P\x15a,BW\x80Q` \x90\x91\x01 \x91\x90PV[`\x98T\x80\x15a+\xF8W\x92\x91PPV[`\0\x80`\x80\x83\x90\x1C\x15a,fW`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a,xW`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a,\x8AW` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a,\x9CW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a,\xAEW`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a,\xC0W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a,\xD2W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x07qW`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a,\xF4W\x81a\n\x82V[P\x90\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-\x12W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a-)W`\0\x80\xFD[a\n\x82\x82a,\xFBV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a-XW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a-<V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\n\x82` \x83\x01\x84a-2V[`\0\x80`@\x83\x85\x03\x12\x15a-\x9EW`\0\x80\xFD[a-\xA7\x83a,\xFBV[\x94` \x93\x90\x93\x015\x93PPPV[\x80\x15\x15\x81\x14a\n\xA1W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a-\xD6W`\0\x80\xFD[a-\xDF\x83a,\xFBV[\x91P` \x83\x015a-\xEF\x81a-\xB5V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a.\x0FW`\0\x80\xFD[a.\x18\x84a,\xFBV[\x92Pa.&` \x85\x01a,\xFBV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a.V`\xE0\x83\x01\x89a-2V[\x82\x81\x03`@\x84\x01Ra.h\x81\x89a-2V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90`\0[\x81\x81\x10\x15a.\xBEW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a.\xA0V[P\x90\x9B\x9APPPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a.\xE1W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a/'Wa/'a.\xE8V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a/IWa/Ia.\xE8V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a/dW`\0\x80\xFD[\x815a/wa/r\x82a//V[a.\xFEV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a/\x99W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a \xD1W\x805\x83R` \x92\x83\x01\x92\x01a/\x9EV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a/\xCCW`\0\x80\xFD[a/\xD5\x85a,\xFBV[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\xF1W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a0\x02W`\0\x80\xFD[\x805a0\x10a/r\x82a//V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x89\x83\x11\x15a02W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a0[Wa0J\x84a,\xFBV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a09V[\x95PPPP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0zW`\0\x80\xFD[a0\x86\x87\x82\x88\x01a/SV[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xA3W`\0\x80\xFD[a0\xAF\x87\x82\x88\x01a/SV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80\x83`\x1F\x84\x01\x12a0\xCDW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xE5W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a1\0W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a1\x1DW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a14W`\0\x80\xFD[a1@\x87\x82\x88\x01a0\xBBV[\x90\x95P\x93PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1`W`\0\x80\xFD[a1l\x87\x82\x88\x01a0\xBBV[\x95\x98\x94\x97P\x95PPPPV[\x805`\xFF\x81\x16\x81\x14a-\x12W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a1\xA2W`\0\x80\xFD[a1\xAB\x87a,\xFBV[\x95P` \x87\x015\x94P`@\x87\x015\x93Pa1\xC7``\x88\x01a1xV[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a1\xFCW`\0\x80\xFD[a2\x05\x88a,\xFBV[\x96Pa2\x13` \x89\x01a,\xFBV[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa2/`\x80\x89\x01a1xV[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a2_W`\0\x80\xFD[a2h\x83a,\xFBV[\x91Pa2v` \x84\x01a,\xFBV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a2\x92W`\0\x80\xFD[a2\x9B\x83a,\xFBV[\x91P` \x83\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a-\xEFW`\0\x80\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a2\xC8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a \xFDWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a2\xFAW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07qWa\x07qa3\x01V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a3RW`\0\x80\xFD[\x81Qa\n\x82\x81a-\xB5V[\x81\x81\x03\x81\x81\x11\x15a\x07qWa\x07qa3\x01V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a3\xEEWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x1F\x82\x11\x15a$SW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a4\x1AWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x111W`\0\x81U`\x01\x01a4&V[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4TWa4Ta.\xE8V[a4h\x81a4b\x84Ta2\xB4V[\x84a3\xF3V[` `\x1F\x82\x11`\x01\x81\x14a4\x9CW`\0\x83\x15a4\x84WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x111V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a4\xCCW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a4\xACV[P\x84\x82\x10\x15a4\xEAW\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 a=\xAA\xD2\nf\xC1\x19S\xE0\x89\xA4\n\\\x91\xCA\\\xA5\xC5\xDA\x83W\xFEy\xEF=\xD9~^.\x02~dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b506004361061028a5760003560e01c806384b0196e1161015c578063aad41a41116100ce578063de0e9a3e11610087578063de0e9a3e1461063a578063ea598cb01461064d578063eb415f4514610660578063ec08f9c414610668578063f1127ed81461067b578063f2fde38b146106b857600080fd5b8063aad41a41146105cf578063b3793585146105e2578063b8c25594146105ee578063c3cda52014610601578063d505accf14610614578063dd62ed3e1461062757600080fd5b806395d89b411161012057806395d89b41146105715780639ab24eb0146105795780639aec4bae1461058c578063a457c2d714610596578063a7d1195d146105a9578063a9059cbb146105bc57600080fd5b806384b0196e146105005780638da5cb5b1461051b5780638e539e8c1461052c57806391ddadf41461053f57806394b819dc1461055e57600080fd5b80633f4da4c6116102005780636fcfff45116101b95780636fcfff451461044f57806370a0823114610477578063715018a6146104a057806378aa33ba146104a85780637ecebe00146104cc57806381b97161146104df57600080fd5b80633f4da4c61461037357806340c10f19146103b25780634bf5d7e9146103c557806353957125146103ef578063587cde1e146104105780635c19a95c1461043c57600080fd5b80631ffacdef116102525780631ffacdef1461031057806323b872dd14610323578063313ce567146103365780633644e51514610345578063395093511461034d5780633a46b1a81461036057600080fd5b80630455e6941461028f57806306fdde03146102c8578063095ea7b3146102dd5780631249c58b146102f057806318160ddd146102fa575b600080fd5b6102b361029d366004612d17565b6101336020526000908152604090205460ff1681565b60405190151581526020015b60405180910390f35b6102d06106cb565b6040516102bf9190612d78565b6102b36102eb366004612d8b565b61075d565b6102f8610777565b005b6103026108c2565b6040519081526020016102bf565b6102f861031e366004612dc3565b61094b565b6102b3610331366004612dfa565b6109b4565b604051601281526020016102bf565b6103026109d8565b6102b361035b366004612d8b565b6109e2565b61030261036e366004612d8b565b610a04565b61039a7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016102bf565b6102f86103c0366004612d8b565b610a89565b60408051808201909152600e81526d06d6f64653d74696d657374616d760941b60208201526102d0565b6103026103fd366004612d17565b6101306020526000908152604090205481565b61039a61041e366004612d17565b6001600160a01b03908116600090815260fe60205260409020541690565b6102f861044a366004612d17565b610a97565b61046261045d366004612d17565b610aa4565b60405163ffffffff90911681526020016102bf565b610302610485366004612d17565b6001600160a01b031660009081526065602052604090205490565b6102f8610ac6565b6102b36104b6366004612d17565b6101346020526000908152604090205460ff1681565b6103026104da366004612d17565b610ada565b6103026104ed366004612d17565b6101316020526000908152604090205481565b610508610af8565b6040516102bf9796959493929190612e37565b6033546001600160a01b031661039a565b61030261053a366004612ecf565b610b96565b610547610bfe565b60405165ffffffffffff90911681526020016102bf565b6102f861056c366004612dc3565b610953565b6102d0610c09565b610302610587366004612d17565b610c18565b6103026101325481565b6102b36105a4366004612d8b565b610c9a565b6102f86105b7366004612fb6565b610d15565b6102b36105ca366004612d8b565b611138565b6102f86105dd366004613107565b611146565b6102f860001961013255565b6102f86105fc366004612dc3565b611217565b6102f861060f366004613189565b611278565b6102f86106223660046131e1565b6113ae565b61030261063536600461324c565b611512565b6102f8610648366004612ecf565b61153d565b6102f861065b366004612ecf565b611630565b6102f8611728565b6102f8610676366004612d17565b6117f1565b61068e61068936600461327f565b6117fa565b60408051825163ffffffff1681526020928301516001600160e01b031692810192909252016102bf565b6102f86106c6366004612d17565b61187e565b6060606880546106da906132b4565b80601f0160208091040260200160405190810160405280929190818152602001828054610706906132b4565b80156107535780601f1061072857610100808354040283529160200191610753565b820191906000526020600020905b81548152906001019060200180831161073657829003601f168201915b5050505050905090565b60003361076b8185856118eb565b60019150505b92915050565b33600090815261013160205260409020546107f15760405162461bcd60e51b815260206004820152602f60248201527f456967656e2e6d696e743a206d73672e73656e64657220686173206e6f206d6960448201526e6e74696e6720616c6c6f77616e636560881b60648201526084015b60405180910390fd5b3360009081526101306020526040902054421161086a5760405162461bcd60e51b815260206004820152603160248201527f456967656e2e6d696e743a206d73672e73656e646572206973206e6f7420616c6044820152701b1bddd959081d1bc81b5a5b9d081e595d607a1b60648201526084016107e8565b3360008181526101316020526040812080549190559061088a9082611a0f565b60405181815233907f0f6798a560793a54c3bcfe86a93cde1e73087d944c0ea20544137d41213968859060200160405180910390a250565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610922573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061094691906132e8565b905090565b610953611aa5565b6001600160a01b03821660008181526101336020908152604091829020805460ff191685151590811790915591519182527fcf20b1ecb604b0e8888d579c64e8a3b10e590d45c1c2dddb393bed284362227191015b60405180910390a25050565b6000336109c2858285611aff565b6109cd858585611b73565b506001949350505050565b6000610946611d2f565b60003361076b8185856109f58383611512565b6109ff9190613317565b6118eb565b6000610a0e610bfe565b65ffffffffffff168210610a605760405162461bcd60e51b815260206004820152601960248201527804552433230566f7465733a20667574757265206c6f6f6b757603c1b60448201526064016107e8565b6001600160a01b038316600090815260ff60205260409020610a829083611d39565b9392505050565b610a938282611a0f565b5050565b610aa13382611e22565b50565b6001600160a01b038116600090815260ff602052604081205461077190611e9c565b610ace611aa5565b610ad86000611f05565b565b6001600160a01b038116600090815260cb6020526040812054610771565b6000606080600080600060606097546000801b148015610b185750609854155b610b5c5760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b60448201526064016107e8565b610b64611f57565b610b6c611f66565b60408051600080825260208201909252600f60f81b9b939a50919850469750309650945092509050565b6000610ba0610bfe565b65ffffffffffff168210610bf25760405162461bcd60e51b815260206004820152601960248201527804552433230566f7465733a20667574757265206c6f6f6b757603c1b60448201526064016107e8565b61077161010083611d39565b600061094642611f75565b6060606980546106da906132b4565b6001600160a01b038116600090815260ff60205260408120548015610c87576001600160a01b038316600090815260ff6020526040902080546000198301908110610c6557610c6561332a565b60009182526020909120015464010000000090046001600160e01b0316610c8a565b60005b6001600160e01b03169392505050565b60003381610ca88286611512565b905083811015610d085760405162461bcd60e51b815260206004820152602560248201527f45524332303a2064656372656173656420616c6c6f77616e63652062656c6f77604482015264207a65726f60d81b60648201526084016107e8565b6109cd82868684036118eb565b600054610100900460ff1615808015610d355750600054600160ff909116105b80610d4f5750303b158015610d4f575060005460ff166001145b610db25760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016107e8565b6000805460ff191660011790558015610dd5576000805461ff0019166101001790555b610ddd611fdc565b610e216040518060400160405280600581526020016422b4b3b2b760d91b8152506040518060400160405280600581526020016422a4a3a2a760d91b81525061200b565b610e2a85611f05565b610e506040518060400160405280600581526020016422a4a3a2a760d91b81525061203c565b8251845114610ed75760405162461bcd60e51b815260206004820152604760248201527f456967656e2e696e697469616c697a653a206d696e7465727320616e64206d6960448201527f6e74696e67416c6c6f77616e636573206d757374206265207468652073616d65606482015266040d8cadccee8d60cb1b608482015260a4016107e8565b8151845114610f5e5760405162461bcd60e51b815260206004820152604760248201527f456967656e2e696e697469616c697a653a206d696e7465727320616e64206d6960448201527f6e74416c6c6f776564416674657273206d757374206265207468652073616d65606482015266040d8cadccee8d60cb1b608482015260a4016107e8565b60005b84518110156110e357838181518110610f7c57610f7c61332a565b60200260200101516101316000878481518110610f9b57610f9b61332a565b60200260200101516001600160a01b03166001600160a01b0316815260200190815260200160002081905550828181518110610fd957610fd961332a565b60200260200101516101306000878481518110610ff857610ff861332a565b60200260200101516001600160a01b03166001600160a01b03168152602001908152602001600020819055506001610133600087848151811061103d5761103d61332a565b60200260200101516001600160a01b03166001600160a01b0316815260200190815260200160002060006101000a81548160ff02191690831515021790555084818151811061108e5761108e61332a565b60200260200101516001600160a01b03167fcf20b1ecb604b0e8888d579c64e8a3b10e590d45c1c2dddb393bed284362227160016040516110d3911515815260200190565b60405180910390a2600101610f61565b50600019610132558015611131576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050505050565b60003361076b818585611b73565b8281146111bb5760405162461bcd60e51b815260206004820152603e60248201527f456967656e2e6d756c746973656e643a2072656365697665727320616e64206160448201527f6d6f756e7473206d757374206265207468652073616d65206c656e677468000060648201526084016107e8565b60005b838110156111315761120f338686848181106111dc576111dc61332a565b90506020020160208101906111f19190612d17565b8585858181106112035761120361332a565b90506020020135611b73565b6001016111be565b61121f611aa5565b6001600160a01b03821660008181526101346020908152604091829020805460ff191685151590811790915591519182527f72a561d1af7409467dae4f1e9fc52590a9335a1dda17727e2b6aa8c4db35109b91016109a8565b834211156112c85760405162461bcd60e51b815260206004820152601d60248201527f4552433230566f7465733a207369676e6174757265206578706972656400000060448201526064016107e8565b604080517fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf60208201526001600160a01b0388169181019190915260608101869052608081018590526000906113429061133a9060a00160405160208183030381529060405280519060200120612086565b8585856120b3565b905061134d816120db565b861461139b5760405162461bcd60e51b815260206004820152601960248201527f4552433230566f7465733a20696e76616c6964206e6f6e63650000000000000060448201526064016107e8565b6113a58188611e22565b50505050505050565b834211156113fe5760405162461bcd60e51b815260206004820152601d60248201527f45524332305065726d69743a206578706972656420646561646c696e6500000060448201526064016107e8565b60007f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c988888861142d8c6120db565b6040805160208101969096526001600160a01b0394851690860152929091166060840152608083015260a082015260c0810186905260e001604051602081830303815290604052805190602001209050600061148882612086565b90506000611498828787876120b3565b9050896001600160a01b0316816001600160a01b0316146114fb5760405162461bcd60e51b815260206004820152601e60248201527f45524332305065726d69743a20696e76616c6964207369676e6174757265000060448201526064016107e8565b6115068a8a8a6118eb565b50505050505050505050565b6001600160a01b03918216600090815260666020908152604080832093909416825291909152205490565b6115473382612103565b60405163a9059cbb60e01b8152336004820152602481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063a9059cbb906044016020604051808303816000875af11580156115b4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115d89190613340565b610aa15760405162461bcd60e51b8152602060048201526024808201527f456967656e2e756e777261703a2062454947454e207472616e736665722066616044820152631a5b195960e21b60648201526084016107e8565b6040516323b872dd60e01b8152336004820152306024820152604481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906323b872dd906064016020604051808303816000875af11580156116a3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116c79190613340565b61171e5760405162461bcd60e51b815260206004820152602260248201527f456967656e2e777261703a2062454947454e207472616e73666572206661696c604482015261195960f21b60648201526084016107e8565b610aa13382611a0f565b611730611aa5565b60001961013254146117c05760405162461bcd60e51b815260206004820152604d60248201527f456967656e2e64697361626c655472616e736665725265737472696374696f6e60448201527f733a207472616e73666572207265737472696374696f6e732061726520616c7260648201526c1958591e48191a5cd8589b1959609a1b608482015260a4016107e8565b60006101328190556040517f2b18986d3ba809db2f13a5d7bf17f60d357b37d9cbb55dd71cbbac8dc4060f649190a1565b610aa181611f05565b60408051808201909152600080825260208201526001600160a01b038316600090815260ff60205260409020805463ffffffff841690811061183e5761183e61332a565b60009182526020918290206040805180820190915291015463ffffffff8116825264010000000090046001600160e01b0316918101919091529392505050565b611886611aa5565b6001600160a01b0381166117f15760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016107e8565b6001600160a01b03831661194d5760405162461bcd60e51b8152602060048201526024808201527f45524332303a20617070726f76652066726f6d20746865207a65726f206164646044820152637265737360e01b60648201526084016107e8565b6001600160a01b0382166119ae5760405162461bcd60e51b815260206004820152602260248201527f45524332303a20617070726f766520746f20746865207a65726f206164647265604482015261737360f01b60648201526084016107e8565b6001600160a01b0383811660008181526066602090815260408083209487168084529482529182902085905590518481527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a3505050565b611a19828261211c565b6001600160e01b03611a296108c2565b1115611a905760405162461bcd60e51b815260206004820152603060248201527f4552433230566f7465733a20746f74616c20737570706c79207269736b73206f60448201526f766572666c6f77696e6720766f74657360801b60648201526084016107e8565b611a9f6101006121f1836121fd565b50505050565b6033546001600160a01b03163314610ad85760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016107e8565b6000611b0b8484611512565b90506000198114611a9f5781811015611b665760405162461bcd60e51b815260206004820152601d60248201527f45524332303a20696e73756666696369656e7420616c6c6f77616e636500000060448201526064016107e8565b611a9f84848484036118eb565b6001600160a01b038316611bd75760405162461bcd60e51b815260206004820152602560248201527f45524332303a207472616e736665722066726f6d20746865207a65726f206164604482015264647265737360d81b60648201526084016107e8565b6001600160a01b038216611c395760405162461bcd60e51b815260206004820152602360248201527f45524332303a207472616e7366657220746f20746865207a65726f206164647260448201526265737360e81b60648201526084016107e8565b611c44838383612372565b6001600160a01b03831660009081526065602052604090205481811015611cbc5760405162461bcd60e51b815260206004820152602660248201527f45524332303a207472616e7366657220616d6f756e7420657863656564732062604482015265616c616e636560d01b60648201526084016107e8565b6001600160a01b0380851660008181526065602052604080822086860390559286168082529083902080548601905591517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90611d1c9086815260200190565b60405180910390a3611a9f848484612458565b600061094661248a565b815460009081816005811115611d93576000611d54846124fe565b611d5e908561335d565b600088815260209020909150869082015463ffffffff161115611d8357809150611d91565b611d8e816001613317565b92505b505b80821015611de0576000611da783836125e6565b600088815260209020909150869082015463ffffffff161115611dcc57809150611dda565b611dd7816001613317565b92505b50611d93565b8015611e0c576000868152602090208101600019015464010000000090046001600160e01b0316611e0f565b60005b6001600160e01b03169695505050505050565b6001600160a01b03828116600081815260fe6020818152604080842080546065845282862054949093528787166001600160a01b03198416811790915590519190951694919391928592917f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9190a4611a9f828483612601565b600063ffffffff821115611f015760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b60648201526084016107e8565b5090565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6060609980546106da906132b4565b6060609a80546106da906132b4565b600065ffffffffffff821115611f015760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203460448201526538206269747360d01b60648201526084016107e8565b600054610100900460ff166120035760405162461bcd60e51b81526004016107e890613370565b610ad861273e565b600054610100900460ff166120325760405162461bcd60e51b81526004016107e890613370565b610a93828261276e565b600054610100900460ff166120635760405162461bcd60e51b81526004016107e890613370565b610aa181604051806040016040528060018152602001603160f81b8152506127ae565b6000610771612093611d2f565b8360405161190160f01b8152600281019290925260228201526042902090565b60008060006120c4878787876127fd565b915091506120d1816128c1565b5095945050505050565b6001600160a01b038116600090815260cb602052604090208054600181018255905b50919050565b61210d8282612a0b565b611a9f610100612b52836121fd565b6001600160a01b0382166121725760405162461bcd60e51b815260206004820152601f60248201527f45524332303a206d696e7420746f20746865207a65726f20616464726573730060448201526064016107e8565b61217e60008383612372565b80606760008282546121909190613317565b90915550506001600160a01b0382166000818152606560209081526040808320805486019055518481527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a3610a9360008383612458565b6000610a828284613317565b8254600090819081811561224a5760008781526020902082016000190160408051808201909152905463ffffffff8116825264010000000090046001600160e01b0316602082015261225f565b60408051808201909152600080825260208201525b905080602001516001600160e01b0316935061227f84868863ffffffff16565b92506000821180156122a95750612294610bfe565b65ffffffffffff16816000015163ffffffff16145b156122ee576122b783612b5e565b60008881526020902083016000190180546001600160e01b03929092166401000000000263ffffffff909216919091179055612368565b866040518060400160405280612312612305610bfe565b65ffffffffffff16611e9c565b63ffffffff16815260200161232686612b5e565b6001600160e01b0390811690915282546001810184556000938452602093849020835194909301519091166401000000000263ffffffff909316929092179101555b5050935093915050565b610132544211612453576001600160a01b038316158061239957506001600160a01b038216155b806123bd57506001600160a01b0383166000908152610133602052604090205460ff165b806123e157506001600160a01b0382166000908152610134602052604090205460ff165b6124535760405162461bcd60e51b815260206004820152603a60248201527f456967656e2e5f6265666f7265546f6b656e5472616e736665723a2066726f6d60448201527f206f7220746f206d7573742062652077686974656c697374656400000000000060648201526084016107e8565b505050565b6001600160a01b03838116600090815260fe602052604080822054858416835291205461245392918216911683612601565b60007f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6124b5612bc7565b6124bd612c20565b60408051602081019490945283019190915260608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b60008160000361251057506000919050565b6000600161251d84612c51565b901c6001901b90506001818481612536576125366133bb565b048201901c9050600181848161254e5761254e6133bb565b048201901c90506001818481612566576125666133bb565b048201901c9050600181848161257e5761257e6133bb565b048201901c90506001818481612596576125966133bb565b048201901c905060018184816125ae576125ae6133bb565b048201901c905060018184816125c6576125c66133bb565b048201901c9050610a82818285816125e0576125e06133bb565b04612ce5565b60006125f560028484186133d1565b610a8290848416613317565b816001600160a01b0316836001600160a01b0316141580156126235750600081115b15612453576001600160a01b038316156126b1576001600160a01b038316600090815260ff60205260408120819061265e90612b52856121fd565b91509150846001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72483836040516126a6929190918252602082015260400190565b60405180910390a250505b6001600160a01b03821615612453576001600160a01b038216600090815260ff6020526040812081906126e7906121f1856121fd565b91509150836001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724838360405161272f929190918252602082015260400190565b60405180910390a25050505050565b600054610100900460ff166127655760405162461bcd60e51b81526004016107e890613370565b610ad833611f05565b600054610100900460ff166127955760405162461bcd60e51b81526004016107e890613370565b60686127a1838261343a565b506069612453828261343a565b600054610100900460ff166127d55760405162461bcd60e51b81526004016107e890613370565b60996127e1838261343a565b50609a6127ee828261343a565b50506000609781905560985550565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083111561283457506000905060036128b8565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015612888573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166128b1576000600192509250506128b8565b9150600090505b94509492505050565b60008160048111156128d5576128d56134f9565b036128dd5750565b60018160048111156128f1576128f16134f9565b0361293e5760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e6174757265000000000000000060448201526064016107e8565b6002816004811115612952576129526134f9565b0361299f5760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e6774680060448201526064016107e8565b60038160048111156129b3576129b36134f9565b03610aa15760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b60648201526084016107e8565b6001600160a01b038216612a6b5760405162461bcd60e51b815260206004820152602160248201527f45524332303a206275726e2066726f6d20746865207a65726f206164647265736044820152607360f81b60648201526084016107e8565b612a7782600083612372565b6001600160a01b03821660009081526065602052604090205481811015612aeb5760405162461bcd60e51b815260206004820152602260248201527f45524332303a206275726e20616d6f756e7420657863656564732062616c616e604482015261636560f01b60648201526084016107e8565b6001600160a01b03831660008181526065602090815260408083208686039055606780548790039055518581529192917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a361245383600084612458565b6000610a82828461335d565b60006001600160e01b03821115611f015760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b60648201526084016107e8565b600080612bd2611f57565b805190915015612be9578051602090910120919050565b6097548015612bf85792915050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4709250505090565b600080612c2b611f66565b805190915015612c42578051602090910120919050565b6098548015612bf85792915050565b600080608083901c15612c6657608092831c92015b604083901c15612c7857604092831c92015b602083901c15612c8a57602092831c92015b601083901c15612c9c57601092831c92015b600883901c15612cae57600892831c92015b600483901c15612cc057600492831c92015b600283901c15612cd257600292831c92015b600183901c156107715760010192915050565b6000818310612cf45781610a82565b5090919050565b80356001600160a01b0381168114612d1257600080fd5b919050565b600060208284031215612d2957600080fd5b610a8282612cfb565b6000815180845260005b81811015612d5857602081850181015186830182015201612d3c565b506000602082860101526020601f19601f83011685010191505092915050565b602081526000610a826020830184612d32565b60008060408385031215612d9e57600080fd5b612da783612cfb565b946020939093013593505050565b8015158114610aa157600080fd5b60008060408385031215612dd657600080fd5b612ddf83612cfb565b91506020830135612def81612db5565b809150509250929050565b600080600060608486031215612e0f57600080fd5b612e1884612cfb565b9250612e2660208501612cfb565b929592945050506040919091013590565b60ff60f81b8816815260e060208201526000612e5660e0830189612d32565b8281036040840152612e688189612d32565b606084018890526001600160a01b038716608085015260a0840186905283810360c08501528451808252602080870193509091019060005b81811015612ebe578351835260209384019390920191600101612ea0565b50909b9a5050505050505050505050565b600060208284031215612ee157600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff81118282101715612f2757612f27612ee8565b604052919050565b600067ffffffffffffffff821115612f4957612f49612ee8565b5060051b60200190565b600082601f830112612f6457600080fd5b8135612f77612f7282612f2f565b612efe565b8082825260208201915060208360051b860101925085831115612f9957600080fd5b602085015b838110156120d1578035835260209283019201612f9e565b60008060008060808587031215612fcc57600080fd5b612fd585612cfb565b9350602085013567ffffffffffffffff811115612ff157600080fd5b8501601f8101871361300257600080fd5b8035613010612f7282612f2f565b8082825260208201915060208360051b85010192508983111561303257600080fd5b6020840193505b8284101561305b5761304a84612cfb565b825260209384019390910190613039565b9550505050604085013567ffffffffffffffff81111561307a57600080fd5b61308687828801612f53565b925050606085013567ffffffffffffffff8111156130a357600080fd5b6130af87828801612f53565b91505092959194509250565b60008083601f8401126130cd57600080fd5b50813567ffffffffffffffff8111156130e557600080fd5b6020830191508360208260051b850101111561310057600080fd5b9250929050565b6000806000806040858703121561311d57600080fd5b843567ffffffffffffffff81111561313457600080fd5b613140878288016130bb565b909550935050602085013567ffffffffffffffff81111561316057600080fd5b61316c878288016130bb565b95989497509550505050565b803560ff81168114612d1257600080fd5b60008060008060008060c087890312156131a257600080fd5b6131ab87612cfb565b955060208701359450604087013593506131c760608801613178565b9598949750929560808101359460a0909101359350915050565b600080600080600080600060e0888a0312156131fc57600080fd5b61320588612cfb565b965061321360208901612cfb565b9550604088013594506060880135935061322f60808901613178565b9699959850939692959460a0840135945060c09093013592915050565b6000806040838503121561325f57600080fd5b61326883612cfb565b915061327660208401612cfb565b90509250929050565b6000806040838503121561329257600080fd5b61329b83612cfb565b9150602083013563ffffffff81168114612def57600080fd5b600181811c908216806132c857607f821691505b6020821081036120fd57634e487b7160e01b600052602260045260246000fd5b6000602082840312156132fa57600080fd5b5051919050565b634e487b7160e01b600052601160045260246000fd5b8082018082111561077157610771613301565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561335257600080fd5b8151610a8281612db5565b8181038181111561077157610771613301565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b634e487b7160e01b600052601260045260246000fd5b6000826133ee57634e487b7160e01b600052601260045260246000fd5b500490565b601f82111561245357806000526020600020601f840160051c8101602085101561341a5750805b601f840160051c820191505b818110156111315760008155600101613426565b815167ffffffffffffffff81111561345457613454612ee8565b6134688161346284546132b4565b846133f3565b6020601f82116001811461349c57600083156134845750848201515b600019600385901b1c1916600184901b178455611131565b600084815260208120601f198516915b828110156134cc57878501518255602094850194600190920191016134ac565b50848210156134ea5786840151600019600387901b60f8161c191681555b50505050600190811b01905550565b634e487b7160e01b600052602160045260246000fdfea2646970667358221220613daad20a66c11953e089a40a5c91ca5ca5c5da8357fe79ef3dd97e5e2e027e64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x8AW`\x005`\xE0\x1C\x80c\x84\xB0\x19n\x11a\x01\\W\x80c\xAA\xD4\x1AA\x11a\0\xCEW\x80c\xDE\x0E\x9A>\x11a\0\x87W\x80c\xDE\x0E\x9A>\x14a\x06:W\x80c\xEAY\x8C\xB0\x14a\x06MW\x80c\xEBA_E\x14a\x06`W\x80c\xEC\x08\xF9\xC4\x14a\x06hW\x80c\xF1\x12~\xD8\x14a\x06{W\x80c\xF2\xFD\xE3\x8B\x14a\x06\xB8W`\0\x80\xFD[\x80c\xAA\xD4\x1AA\x14a\x05\xCFW\x80c\xB3y5\x85\x14a\x05\xE2W\x80c\xB8\xC2U\x94\x14a\x05\xEEW\x80c\xC3\xCD\xA5 \x14a\x06\x01W\x80c\xD5\x05\xAC\xCF\x14a\x06\x14W\x80c\xDDb\xED>\x14a\x06'W`\0\x80\xFD[\x80c\x95\xD8\x9BA\x11a\x01 W\x80c\x95\xD8\x9BA\x14a\x05qW\x80c\x9A\xB2N\xB0\x14a\x05yW\x80c\x9A\xECK\xAE\x14a\x05\x8CW\x80c\xA4W\xC2\xD7\x14a\x05\x96W\x80c\xA7\xD1\x19]\x14a\x05\xA9W\x80c\xA9\x05\x9C\xBB\x14a\x05\xBCW`\0\x80\xFD[\x80c\x84\xB0\x19n\x14a\x05\0W\x80c\x8D\xA5\xCB[\x14a\x05\x1BW\x80c\x8ES\x9E\x8C\x14a\x05,W\x80c\x91\xDD\xAD\xF4\x14a\x05?W\x80c\x94\xB8\x19\xDC\x14a\x05^W`\0\x80\xFD[\x80c?M\xA4\xC6\x11a\x02\0W\x80co\xCF\xFFE\x11a\x01\xB9W\x80co\xCF\xFFE\x14a\x04OW\x80cp\xA0\x821\x14a\x04wW\x80cqP\x18\xA6\x14a\x04\xA0W\x80cx\xAA3\xBA\x14a\x04\xA8W\x80c~\xCE\xBE\0\x14a\x04\xCCW\x80c\x81\xB9qa\x14a\x04\xDFW`\0\x80\xFD[\x80c?M\xA4\xC6\x14a\x03sW\x80c@\xC1\x0F\x19\x14a\x03\xB2W\x80cK\xF5\xD7\xE9\x14a\x03\xC5W\x80cS\x95q%\x14a\x03\xEFW\x80cX|\xDE\x1E\x14a\x04\x10W\x80c\\\x19\xA9\\\x14a\x04<W`\0\x80\xFD[\x80c\x1F\xFA\xCD\xEF\x11a\x02RW\x80c\x1F\xFA\xCD\xEF\x14a\x03\x10W\x80c#\xB8r\xDD\x14a\x03#W\x80c1<\xE5g\x14a\x036W\x80c6D\xE5\x15\x14a\x03EW\x80c9P\x93Q\x14a\x03MW\x80c:F\xB1\xA8\x14a\x03`W`\0\x80\xFD[\x80c\x04U\xE6\x94\x14a\x02\x8FW\x80c\x06\xFD\xDE\x03\x14a\x02\xC8W\x80c\t^\xA7\xB3\x14a\x02\xDDW\x80c\x12I\xC5\x8B\x14a\x02\xF0W\x80c\x18\x16\r\xDD\x14a\x02\xFAW[`\0\x80\xFD[a\x02\xB3a\x02\x9D6`\x04a-\x17V[a\x013` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xD0a\x06\xCBV[`@Qa\x02\xBF\x91\x90a-xV[a\x02\xB3a\x02\xEB6`\x04a-\x8BV[a\x07]V[a\x02\xF8a\x07wV[\0[a\x03\x02a\x08\xC2V[`@Q\x90\x81R` \x01a\x02\xBFV[a\x02\xF8a\x03\x1E6`\x04a-\xC3V[a\tKV[a\x02\xB3a\x0316`\x04a-\xFAV[a\t\xB4V[`@Q`\x12\x81R` \x01a\x02\xBFV[a\x03\x02a\t\xD8V[a\x02\xB3a\x03[6`\x04a-\x8BV[a\t\xE2V[a\x03\x02a\x03n6`\x04a-\x8BV[a\n\x04V[a\x03\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xBFV[a\x02\xF8a\x03\xC06`\x04a-\x8BV[a\n\x89V[`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x06\xD6\xF6FS\xD7F\x96\xD6W7F\x16\xD7`\x94\x1B` \x82\x01Ra\x02\xD0V[a\x03\x02a\x03\xFD6`\x04a-\x17V[a\x010` R`\0\x90\x81R`@\x90 T\x81V[a\x03\x9Aa\x04\x1E6`\x04a-\x17V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\xFE` R`@\x90 T\x16\x90V[a\x02\xF8a\x04J6`\x04a-\x17V[a\n\x97V[a\x04ba\x04]6`\x04a-\x17V[a\n\xA4V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xBFV[a\x03\x02a\x04\x856`\x04a-\x17V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`e` R`@\x90 T\x90V[a\x02\xF8a\n\xC6V[a\x02\xB3a\x04\xB66`\x04a-\x17V[a\x014` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\x02a\x04\xDA6`\x04a-\x17V[a\n\xDAV[a\x03\x02a\x04\xED6`\x04a-\x17V[a\x011` R`\0\x90\x81R`@\x90 T\x81V[a\x05\x08a\n\xF8V[`@Qa\x02\xBF\x97\x96\x95\x94\x93\x92\x91\x90a.7V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x9AV[a\x03\x02a\x05:6`\x04a.\xCFV[a\x0B\x96V[a\x05Ga\x0B\xFEV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xBFV[a\x02\xF8a\x05l6`\x04a-\xC3V[a\tSV[a\x02\xD0a\x0C\tV[a\x03\x02a\x05\x876`\x04a-\x17V[a\x0C\x18V[a\x03\x02a\x012T\x81V[a\x02\xB3a\x05\xA46`\x04a-\x8BV[a\x0C\x9AV[a\x02\xF8a\x05\xB76`\x04a/\xB6V[a\r\x15V[a\x02\xB3a\x05\xCA6`\x04a-\x8BV[a\x118V[a\x02\xF8a\x05\xDD6`\x04a1\x07V[a\x11FV[a\x02\xF8`\0\x19a\x012UV[a\x02\xF8a\x05\xFC6`\x04a-\xC3V[a\x12\x17V[a\x02\xF8a\x06\x0F6`\x04a1\x89V[a\x12xV[a\x02\xF8a\x06\"6`\x04a1\xE1V[a\x13\xAEV[a\x03\x02a\x0656`\x04a2LV[a\x15\x12V[a\x02\xF8a\x06H6`\x04a.\xCFV[a\x15=V[a\x02\xF8a\x06[6`\x04a.\xCFV[a\x160V[a\x02\xF8a\x17(V[a\x02\xF8a\x06v6`\x04a-\x17V[a\x17\xF1V[a\x06\x8Ea\x06\x896`\x04a2\x7FV[a\x17\xFAV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xE0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02\xBFV[a\x02\xF8a\x06\xC66`\x04a-\x17V[a\x18~V[```h\x80Ta\x06\xDA\x90a2\xB4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\x06\x90a2\xB4V[\x80\x15a\x07SW\x80`\x1F\x10a\x07(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07SV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x076W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x07k\x81\x85\x85a\x18\xEBV[`\x01\x91PP[\x92\x91PPV[3`\0\x90\x81Ra\x011` R`@\x90 Ta\x07\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FEigen.mint: msg.sender has no mi`D\x82\x01Rnnting allowance`\x88\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81Ra\x010` R`@\x90 TB\x11a\x08jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FEigen.mint: msg.sender is not al`D\x82\x01Rp\x1B\x1B\xDD\xD9Y\x08\x1D\x1B\xC8\x1BZ[\x9D\x08\x1EY]`z\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[3`\0\x81\x81Ra\x011` R`@\x81 \x80T\x91\x90U\x90a\x08\x8A\x90\x82a\x1A\x0FV[`@Q\x81\x81R3\x90\x7F\x0Fg\x98\xA5`y:T\xC3\xBC\xFE\x86\xA9<\xDE\x1Es\x08}\x94L\x0E\xA2\x05D\x13}A!9h\x85\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tF\x91\x90a2\xE8V[\x90P\x90V[a\tSa\x1A\xA5V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x013` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\xCF \xB1\xEC\xB6\x04\xB0\xE8\x88\x8DW\x9Cd\xE8\xA3\xB1\x0EY\rE\xC1\xC2\xDD\xDB9;\xED(Cb\"q\x91\x01[`@Q\x80\x91\x03\x90\xA2PPV[`\x003a\t\xC2\x85\x82\x85a\x1A\xFFV[a\t\xCD\x85\x85\x85a\x1BsV[P`\x01\x94\x93PPPPV[`\0a\tFa\x1D/V[`\x003a\x07k\x81\x85\x85a\t\xF5\x83\x83a\x15\x12V[a\t\xFF\x91\x90a3\x17V[a\x18\xEBV[`\0a\n\x0Ea\x0B\xFEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\n`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x04U$3#\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`<\x1B`D\x82\x01R`d\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x90 a\n\x82\x90\x83a\x1D9V[\x93\x92PPPV[a\n\x93\x82\x82a\x1A\x0FV[PPV[a\n\xA13\x82a\x1E\"V[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xFF` R`@\x81 Ta\x07q\x90a\x1E\x9CV[a\n\xCEa\x1A\xA5V[a\n\xD8`\0a\x1F\x05V[V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCB` R`@\x81 Ta\x07qV[`\0``\x80`\0\x80`\0```\x97T`\0\x80\x1B\x14\x80\x15a\x0B\x18WP`\x98T\x15[a\x0B\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01a\x07\xE8V[a\x0Bda\x1FWV[a\x0Bla\x1FfV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0a\x0B\xA0a\x0B\xFEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\x0B\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x04U$3#\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`<\x1B`D\x82\x01R`d\x01a\x07\xE8V[a\x07qa\x01\0\x83a\x1D9V[`\0a\tFBa\x1FuV[```i\x80Ta\x06\xDA\x90a2\xB4V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xFF` R`@\x81 T\x80\x15a\x0C\x87W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x90 \x80T`\0\x19\x83\x01\x90\x81\x10a\x0CeWa\x0Cea3*V[`\0\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0C\x8AV[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[`\x003\x81a\x0C\xA8\x82\x86a\x15\x12V[\x90P\x83\x81\x10\x15a\r\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[a\t\xCD\x82\x86\x86\x84\x03a\x18\xEBV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\r5WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\rOWP0;\x15\x80\x15a\rOWP`\0T`\xFF\x16`\x01\x14[a\r\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\r\xD5W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\r\xDDa\x1F\xDCV[a\x0E!`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\"\xB4\xB3\xB2\xB7`\xD9\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\"\xA4\xA3\xA2\xA7`\xD9\x1B\x81RPa \x0BV[a\x0E*\x85a\x1F\x05V[a\x0EP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\"\xA4\xA3\xA2\xA7`\xD9\x1B\x81RPa <V[\x82Q\x84Q\x14a\x0E\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEigen.initialize: minters and mi`D\x82\x01R\x7FntingAllowances must be the same`d\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`\x84\x82\x01R`\xA4\x01a\x07\xE8V[\x81Q\x84Q\x14a\x0F^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEigen.initialize: minters and mi`D\x82\x01R\x7FntAllowedAfters must be the same`d\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`\x84\x82\x01R`\xA4\x01a\x07\xE8V[`\0[\x84Q\x81\x10\x15a\x10\xE3W\x83\x81\x81Q\x81\x10a\x0F|Wa\x0F|a3*V[` \x02` \x01\x01Qa\x011`\0\x87\x84\x81Q\x81\x10a\x0F\x9BWa\x0F\x9Ba3*V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x82\x81\x81Q\x81\x10a\x0F\xD9Wa\x0F\xD9a3*V[` \x02` \x01\x01Qa\x010`\0\x87\x84\x81Q\x81\x10a\x0F\xF8Wa\x0F\xF8a3*V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01a\x013`\0\x87\x84\x81Q\x81\x10a\x10=Wa\x10=a3*V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x84\x81\x81Q\x81\x10a\x10\x8EWa\x10\x8Ea3*V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7F\xCF \xB1\xEC\xB6\x04\xB0\xE8\x88\x8DW\x9Cd\xE8\xA3\xB1\x0EY\rE\xC1\xC2\xDD\xDB9;\xED(Cb\"q`\x01`@Qa\x10\xD3\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a\x0FaV[P`\0\x19a\x012U\x80\x15a\x111W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`\x003a\x07k\x81\x85\x85a\x1BsV[\x82\x81\x14a\x11\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FEigen.multisend: receivers and a`D\x82\x01R\x7Fmounts must be the same length\0\0`d\x82\x01R`\x84\x01a\x07\xE8V[`\0[\x83\x81\x10\x15a\x111Wa\x12\x0F3\x86\x86\x84\x81\x81\x10a\x11\xDCWa\x11\xDCa3*V[\x90P` \x02\x01` \x81\x01\x90a\x11\xF1\x91\x90a-\x17V[\x85\x85\x85\x81\x81\x10a\x12\x03Wa\x12\x03a3*V[\x90P` \x02\x015a\x1BsV[`\x01\x01a\x11\xBEV[a\x12\x1Fa\x1A\xA5V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x014` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7Fr\xA5a\xD1\xAFt\tF}\xAEO\x1E\x9F\xC5%\x90\xA93Z\x1D\xDA\x17r~+j\xA8\xC4\xDB5\x10\x9B\x91\x01a\t\xA8V[\x83B\x11\x15a\x12\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Votes: signature expired\0\0\0`D\x82\x01R`d\x01a\x07\xE8V[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\0\x90a\x13B\x90a\x13:\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a \x86V[\x85\x85\x85a \xB3V[\x90Pa\x13M\x81a \xDBV[\x86\x14a\x13\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC20Votes: invalid nonce\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xE8V[a\x13\xA5\x81\x88a\x1E\"V[PPPPPPPV[\x83B\x11\x15a\x13\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Permit: expired deadline\0\0\0`D\x82\x01R`d\x01a\x07\xE8V[`\0\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a\x14-\x8Ca \xDBV[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x14\x88\x82a \x86V[\x90P`\0a\x14\x98\x82\x87\x87\x87a \xB3V[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FERC20Permit: invalid signature\0\0`D\x82\x01R`d\x01a\x07\xE8V[a\x15\x06\x8A\x8A\x8Aa\x18\xEBV[PPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`f` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x15G3\x82a!\x03V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x15\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xD8\x91\x90a3@V[a\n\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FEigen.unwrap: bEIGEN transfer fa`D\x82\x01Rc\x1A[\x19Y`\xE2\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x16\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xC7\x91\x90a3@V[a\x17\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEigen.wrap: bEIGEN transfer fail`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[a\n\xA13\x82a\x1A\x0FV[a\x170a\x1A\xA5V[`\0\x19a\x012T\x14a\x17\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FEigen.disableTransferRestriction`D\x82\x01R\x7Fs: transfer restrictions are alr`d\x82\x01Rl\x19XY\x1EH\x19\x1A\\\xD8X\x9B\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x07\xE8V[`\0a\x012\x81\x90U`@Q\x7F+\x18\x98m;\xA8\t\xDB/\x13\xA5\xD7\xBF\x17\xF6\r5{7\xD9\xCB\xB5]\xD7\x1C\xBB\xAC\x8D\xC4\x06\x0Fd\x91\x90\xA1V[a\n\xA1\x81a\x1F\x05V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x18>Wa\x18>a3*V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x82Rd\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x18\x86a\x1A\xA5V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x17\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x19MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x19\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`f` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x1A\x19\x82\x82a!\x1CV[`\x01`\x01`\xE0\x1B\x03a\x1A)a\x08\xC2V[\x11\x15a\x1A\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FERC20Votes: total supply risks o`D\x82\x01Roverflowing votes`\x80\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[a\x1A\x9Fa\x01\0a!\xF1\x83a!\xFDV[PPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07\xE8V[`\0a\x1B\x0B\x84\x84a\x15\x12V[\x90P`\0\x19\x81\x14a\x1A\x9FW\x81\x81\x10\x15a\x1BfW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x07\xE8V[a\x1A\x9F\x84\x84\x84\x84\x03a\x18\xEBV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x1B\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1C9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[a\x1CD\x83\x83\x83a#rV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`e` R`@\x90 T\x81\x81\x10\x15a\x1C\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`e` R`@\x80\x82 \x86\x86\x03\x90U\x92\x86\x16\x80\x82R\x90\x83\x90 \x80T\x86\x01\x90U\x91Q\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x1D\x1C\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x1A\x9F\x84\x84\x84a$XV[`\0a\tFa$\x8AV[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a\x1D\x93W`\0a\x1DT\x84a$\xFEV[a\x1D^\x90\x85a3]V[`\0\x88\x81R` \x90 \x90\x91P\x86\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1D\x83W\x80\x91Pa\x1D\x91V[a\x1D\x8E\x81`\x01a3\x17V[\x92P[P[\x80\x82\x10\x15a\x1D\xE0W`\0a\x1D\xA7\x83\x83a%\xE6V[`\0\x88\x81R` \x90 \x90\x91P\x86\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1D\xCCW\x80\x91Pa\x1D\xDAV[a\x1D\xD7\x81`\x01a3\x17V[\x92P[Pa\x1D\x93V[\x80\x15a\x1E\x0CW`\0\x86\x81R` \x90 \x81\x01`\0\x19\x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x1E\x0FV[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\xFE` \x81\x81R`@\x80\x84 \x80T`e\x84R\x82\x86 T\x94\x90\x93R\x87\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x84\x16\x81\x17\x90\x91U\x90Q\x91\x90\x95\x16\x94\x91\x93\x91\x92\x85\x92\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\x1A\x9F\x82\x84\x83a&\x01V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x1F\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[P\x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[```\x99\x80Ta\x06\xDA\x90a2\xB4V[```\x9A\x80Ta\x06\xDA\x90a2\xB4V[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1F\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\0Ta\x01\0\x90\x04`\xFF\x16a \x03W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xE8\x90a3pV[a\n\xD8a'>V[`\0Ta\x01\0\x90\x04`\xFF\x16a 2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xE8\x90a3pV[a\n\x93\x82\x82a'nV[`\0Ta\x01\0\x90\x04`\xFF\x16a cW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xE8\x90a3pV[a\n\xA1\x81`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RPa'\xAEV[`\0a\x07qa \x93a\x1D/V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a \xC4\x87\x87\x87\x87a'\xFDV[\x91P\x91Pa \xD1\x81a(\xC1V[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCB` R`@\x90 \x80T`\x01\x81\x01\x82U\x90[P\x91\x90PV[a!\r\x82\x82a*\x0BV[a\x1A\x9Fa\x01\0a+R\x83a!\xFDV[`\x01`\x01`\xA0\x1B\x03\x82\x16a!rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x07\xE8V[a!~`\0\x83\x83a#rV[\x80`g`\0\x82\x82Ta!\x90\x91\x90a3\x17V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`e` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\n\x93`\0\x83\x83a$XV[`\0a\n\x82\x82\x84a3\x17V[\x82T`\0\x90\x81\x90\x81\x81\x15a\"JW`\0\x87\x81R` \x90 \x82\x01`\0\x19\x01`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x82Rd\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16` \x82\x01Ra\"_V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R[\x90P\x80` \x01Q`\x01`\x01`\xE0\x1B\x03\x16\x93Pa\"\x7F\x84\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x92P`\0\x82\x11\x80\x15a\"\xA9WPa\"\x94a\x0B\xFEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x14[\x15a\"\xEEWa\"\xB7\x83a+^V[`\0\x88\x81R` \x90 \x83\x01`\0\x19\x01\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua#hV[\x86`@Q\x80`@\x01`@R\x80a#\x12a#\x05a\x0B\xFEV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1E\x9CV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a#&\x86a+^V[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[PP\x93P\x93\x91PPV[a\x012TB\x11a$SW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80a#\x99WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15[\x80a#\xBDWP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x013` R`@\x90 T`\xFF\x16[\x80a#\xE1WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x014` R`@\x90 T`\xFF\x16[a$SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FEigen._beforeTokenTransfer: from`D\x82\x01R\x7F or to must be whitelisted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xE8V[PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\xFE` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta$S\x92\x91\x82\x16\x91\x16\x83a&\x01V[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa$\xB5a+\xC7V[a$\xBDa, V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x81`\0\x03a%\x10WP`\0\x91\x90PV[`\0`\x01a%\x1D\x84a,QV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a%6Wa%6a3\xBBV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a%NWa%Na3\xBBV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a%fWa%fa3\xBBV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a%~Wa%~a3\xBBV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a%\x96Wa%\x96a3\xBBV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a%\xAEWa%\xAEa3\xBBV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a%\xC6Wa%\xC6a3\xBBV[\x04\x82\x01\x90\x1C\x90Pa\n\x82\x81\x82\x85\x81a%\xE0Wa%\xE0a3\xBBV[\x04a,\xE5V[`\0a%\xF5`\x02\x84\x84\x18a3\xD1V[a\n\x82\x90\x84\x84\x16a3\x17V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a&#WP`\0\x81\x11[\x15a$SW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a&\xB1W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x81 \x81\x90a&^\x90a+R\x85a!\xFDV[\x91P\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa&\xA6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a$SW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xFF` R`@\x81 \x81\x90a&\xE7\x90a!\xF1\x85a!\xFDV[\x91P\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa'/\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a'eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xE8\x90a3pV[a\n\xD83a\x1F\x05V[`\0Ta\x01\0\x90\x04`\xFF\x16a'\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xE8\x90a3pV[`ha'\xA1\x83\x82a4:V[P`ia$S\x82\x82a4:V[`\0Ta\x01\0\x90\x04`\xFF\x16a'\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xE8\x90a3pV[`\x99a'\xE1\x83\x82a4:V[P`\x9Aa'\xEE\x82\x82a4:V[PP`\0`\x97\x81\x90U`\x98UPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a(4WP`\0\x90P`\x03a(\xB8V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a(\x88W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a(\xB1W`\0`\x01\x92P\x92PPa(\xB8V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a(\xD5Wa(\xD5a4\xF9V[\x03a(\xDDWPV[`\x01\x81`\x04\x81\x11\x15a(\xF1Wa(\xF1a4\xF9V[\x03a)>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\xE8V[`\x02\x81`\x04\x81\x11\x15a)RWa)Ra4\xF9V[\x03a)\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x07\xE8V[`\x03\x81`\x04\x81\x11\x15a)\xB3Wa)\xB3a4\xF9V[\x03a\n\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x82\x16a*kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[a*w\x82`\0\x83a#rV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`e` R`@\x90 T\x81\x81\x10\x15a*\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`e` \x90\x81R`@\x80\x83 \x86\x86\x03\x90U`g\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a$S\x83`\0\x84a$XV[`\0a\n\x82\x82\x84a3]V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x1F\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x07\xE8V[`\0\x80a+\xD2a\x1FWV[\x80Q\x90\x91P\x15a+\xE9W\x80Q` \x90\x91\x01 \x91\x90PV[`\x97T\x80\x15a+\xF8W\x92\x91PPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x92PPP\x90V[`\0\x80a,+a\x1FfV[\x80Q\x90\x91P\x15a,BW\x80Q` \x90\x91\x01 \x91\x90PV[`\x98T\x80\x15a+\xF8W\x92\x91PPV[`\0\x80`\x80\x83\x90\x1C\x15a,fW`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a,xW`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a,\x8AW` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a,\x9CW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a,\xAEW`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a,\xC0W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a,\xD2W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x07qW`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a,\xF4W\x81a\n\x82V[P\x90\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-\x12W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a-)W`\0\x80\xFD[a\n\x82\x82a,\xFBV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a-XW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a-<V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\n\x82` \x83\x01\x84a-2V[`\0\x80`@\x83\x85\x03\x12\x15a-\x9EW`\0\x80\xFD[a-\xA7\x83a,\xFBV[\x94` \x93\x90\x93\x015\x93PPPV[\x80\x15\x15\x81\x14a\n\xA1W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a-\xD6W`\0\x80\xFD[a-\xDF\x83a,\xFBV[\x91P` \x83\x015a-\xEF\x81a-\xB5V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a.\x0FW`\0\x80\xFD[a.\x18\x84a,\xFBV[\x92Pa.&` \x85\x01a,\xFBV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a.V`\xE0\x83\x01\x89a-2V[\x82\x81\x03`@\x84\x01Ra.h\x81\x89a-2V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90`\0[\x81\x81\x10\x15a.\xBEW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a.\xA0V[P\x90\x9B\x9APPPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a.\xE1W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a/'Wa/'a.\xE8V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a/IWa/Ia.\xE8V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a/dW`\0\x80\xFD[\x815a/wa/r\x82a//V[a.\xFEV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a/\x99W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a \xD1W\x805\x83R` \x92\x83\x01\x92\x01a/\x9EV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a/\xCCW`\0\x80\xFD[a/\xD5\x85a,\xFBV[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\xF1W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a0\x02W`\0\x80\xFD[\x805a0\x10a/r\x82a//V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x89\x83\x11\x15a02W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a0[Wa0J\x84a,\xFBV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a09V[\x95PPPP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0zW`\0\x80\xFD[a0\x86\x87\x82\x88\x01a/SV[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xA3W`\0\x80\xFD[a0\xAF\x87\x82\x88\x01a/SV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80\x83`\x1F\x84\x01\x12a0\xCDW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xE5W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a1\0W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a1\x1DW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a14W`\0\x80\xFD[a1@\x87\x82\x88\x01a0\xBBV[\x90\x95P\x93PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1`W`\0\x80\xFD[a1l\x87\x82\x88\x01a0\xBBV[\x95\x98\x94\x97P\x95PPPPV[\x805`\xFF\x81\x16\x81\x14a-\x12W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a1\xA2W`\0\x80\xFD[a1\xAB\x87a,\xFBV[\x95P` \x87\x015\x94P`@\x87\x015\x93Pa1\xC7``\x88\x01a1xV[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a1\xFCW`\0\x80\xFD[a2\x05\x88a,\xFBV[\x96Pa2\x13` \x89\x01a,\xFBV[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa2/`\x80\x89\x01a1xV[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a2_W`\0\x80\xFD[a2h\x83a,\xFBV[\x91Pa2v` \x84\x01a,\xFBV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a2\x92W`\0\x80\xFD[a2\x9B\x83a,\xFBV[\x91P` \x83\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a-\xEFW`\0\x80\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a2\xC8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a \xFDWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a2\xFAW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07qWa\x07qa3\x01V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a3RW`\0\x80\xFD[\x81Qa\n\x82\x81a-\xB5V[\x81\x81\x03\x81\x81\x11\x15a\x07qWa\x07qa3\x01V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a3\xEEWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x1F\x82\x11\x15a$SW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a4\x1AWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x111W`\0\x81U`\x01\x01a4&V[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4TWa4Ta.\xE8V[a4h\x81a4b\x84Ta2\xB4V[\x84a3\xF3V[` `\x1F\x82\x11`\x01\x81\x14a4\x9CW`\0\x83\x15a4\x84WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x111V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a4\xCCW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a4\xACV[P\x84\x82\x10\x15a4\xEAW\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 a=\xAA\xD2\nf\xC1\x19S\xE0\x89\xA4\n\\\x91\xCA\\\xA5\xC5\xDA\x83W\xFEy\xEF=\xD9~^.\x02~dsolcC\0\x08\x1B\x003",
    );
    /**Event with signature `Approval(address,address,uint256)` and selector `0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925`.
```solidity
event Approval(address indexed owner, address indexed spender, uint256 value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Approval {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Approval {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Approval(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                140u8,
                91u8,
                225u8,
                229u8,
                235u8,
                236u8,
                125u8,
                91u8,
                209u8,
                79u8,
                113u8,
                66u8,
                125u8,
                30u8,
                132u8,
                243u8,
                221u8,
                3u8,
                20u8,
                192u8,
                247u8,
                178u8,
                41u8,
                30u8,
                91u8,
                32u8,
                10u8,
                200u8,
                199u8,
                195u8,
                185u8,
                37u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    owner: topics.1,
                    spender: topics.2,
                    value: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.owner.clone(), self.spender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.owner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.spender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Approval {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Approval> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Approval) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DelegateChanged(address,address,address)` and selector `0x3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f`.
```solidity
event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DelegateChanged {
        #[allow(missing_docs)]
        pub delegator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub fromDelegate: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub toDelegate: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DelegateChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DelegateChanged(address,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8,
                52u8,
                232u8,
                162u8,
                230u8,
                217u8,
                126u8,
                146u8,
                154u8,
                126u8,
                84u8,
                1u8,
                30u8,
                165u8,
                72u8,
                93u8,
                125u8,
                25u8,
                109u8,
                213u8,
                240u8,
                186u8,
                77u8,
                78u8,
                249u8,
                88u8,
                3u8,
                232u8,
                227u8,
                252u8,
                37u8,
                127u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    delegator: topics.1,
                    fromDelegate: topics.2,
                    toDelegate: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
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
                    self.delegator.clone(),
                    self.fromDelegate.clone(),
                    self.toDelegate.clone(),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.delegator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.fromDelegate,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.toDelegate,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DelegateChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelegateChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DelegateChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DelegateVotesChanged(address,uint256,uint256)` and selector `0xdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724`.
```solidity
event DelegateVotesChanged(address indexed delegate, uint256 previousBalance, uint256 newBalance);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DelegateVotesChanged {
        #[allow(missing_docs)]
        pub delegate: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub previousBalance: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newBalance: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DelegateVotesChanged {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DelegateVotesChanged(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                222u8,
                194u8,
                186u8,
                205u8,
                210u8,
                240u8,
                91u8,
                89u8,
                222u8,
                52u8,
                218u8,
                155u8,
                82u8,
                61u8,
                255u8,
                139u8,
                228u8,
                46u8,
                94u8,
                56u8,
                232u8,
                24u8,
                200u8,
                47u8,
                219u8,
                11u8,
                174u8,
                119u8,
                67u8,
                135u8,
                167u8,
                36u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    delegate: topics.1,
                    previousBalance: data.0,
                    newBalance: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.previousBalance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newBalance),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.delegate.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.delegate,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DelegateVotesChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelegateVotesChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DelegateVotesChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EIP712DomainChanged()` and selector `0x0a6387c9ea3628b88a633bb4f3b151770f70085117a15f9bf3787cda53f13d31`.
```solidity
event EIP712DomainChanged();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EIP712DomainChanged {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EIP712DomainChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EIP712DomainChanged()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                10u8,
                99u8,
                135u8,
                201u8,
                234u8,
                54u8,
                40u8,
                184u8,
                138u8,
                99u8,
                59u8,
                180u8,
                243u8,
                177u8,
                81u8,
                119u8,
                15u8,
                112u8,
                8u8,
                81u8,
                23u8,
                161u8,
                95u8,
                155u8,
                243u8,
                120u8,
                124u8,
                218u8,
                83u8,
                241u8,
                61u8,
                49u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EIP712DomainChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EIP712DomainChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EIP712DomainChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
```solidity
event Initialized(uint8 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
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
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
    /**Event with signature `Mint(address,uint256)` and selector `0x0f6798a560793a54c3bcfe86a93cde1e73087d944c0ea20544137d4121396885`.
```solidity
event Mint(address indexed minter, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Mint {
        #[allow(missing_docs)]
        pub minter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Mint {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Mint(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                15u8,
                103u8,
                152u8,
                165u8,
                96u8,
                121u8,
                58u8,
                84u8,
                195u8,
                188u8,
                254u8,
                134u8,
                169u8,
                60u8,
                222u8,
                30u8,
                115u8,
                8u8,
                125u8,
                148u8,
                76u8,
                14u8,
                162u8,
                5u8,
                68u8,
                19u8,
                125u8,
                65u8,
                33u8,
                57u8,
                104u8,
                133u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    minter: topics.1,
                    amount: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.minter.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.minter,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Mint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Mint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Mint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
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
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
    /**Event with signature `SetAllowedFrom(address,bool)` and selector `0xcf20b1ecb604b0e8888d579c64e8a3b10e590d45c1c2dddb393bed2843622271`.
```solidity
event SetAllowedFrom(address indexed from, bool isAllowedFrom);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SetAllowedFrom {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub isAllowedFrom: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SetAllowedFrom {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SetAllowedFrom(address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                207u8,
                32u8,
                177u8,
                236u8,
                182u8,
                4u8,
                176u8,
                232u8,
                136u8,
                141u8,
                87u8,
                156u8,
                100u8,
                232u8,
                163u8,
                177u8,
                14u8,
                89u8,
                13u8,
                69u8,
                193u8,
                194u8,
                221u8,
                219u8,
                57u8,
                59u8,
                237u8,
                40u8,
                67u8,
                98u8,
                34u8,
                113u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    from: topics.1,
                    isAllowedFrom: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isAllowedFrom,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.from.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SetAllowedFrom {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SetAllowedFrom> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SetAllowedFrom) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SetAllowedTo(address,bool)` and selector `0x72a561d1af7409467dae4f1e9fc52590a9335a1dda17727e2b6aa8c4db35109b`.
```solidity
event SetAllowedTo(address indexed to, bool isAllowedTo);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SetAllowedTo {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub isAllowedTo: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SetAllowedTo {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SetAllowedTo(address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                114u8,
                165u8,
                97u8,
                209u8,
                175u8,
                116u8,
                9u8,
                70u8,
                125u8,
                174u8,
                79u8,
                30u8,
                159u8,
                197u8,
                37u8,
                144u8,
                169u8,
                51u8,
                90u8,
                29u8,
                218u8,
                23u8,
                114u8,
                126u8,
                43u8,
                106u8,
                168u8,
                196u8,
                219u8,
                53u8,
                16u8,
                155u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    to: topics.1,
                    isAllowedTo: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isAllowedTo,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.to.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SetAllowedTo {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SetAllowedTo> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SetAllowedTo) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Transfer(address,address,uint256)` and selector `0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef`.
```solidity
event Transfer(address indexed from, address indexed to, uint256 value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Transfer {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Transfer {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Transfer(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8,
                242u8,
                82u8,
                173u8,
                27u8,
                226u8,
                200u8,
                155u8,
                105u8,
                194u8,
                176u8,
                104u8,
                252u8,
                55u8,
                141u8,
                170u8,
                149u8,
                43u8,
                167u8,
                241u8,
                99u8,
                196u8,
                161u8,
                22u8,
                40u8,
                245u8,
                90u8,
                77u8,
                245u8,
                35u8,
                179u8,
                239u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    from: topics.1,
                    to: topics.2,
                    value: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.from.clone(), self.to.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Transfer {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Transfer> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Transfer) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TransferRestrictionsDisabled()` and selector `0x2b18986d3ba809db2f13a5d7bf17f60d357b37d9cbb55dd71cbbac8dc4060f64`.
```solidity
event TransferRestrictionsDisabled();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransferRestrictionsDisabled {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TransferRestrictionsDisabled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TransferRestrictionsDisabled()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                43u8,
                24u8,
                152u8,
                109u8,
                59u8,
                168u8,
                9u8,
                219u8,
                47u8,
                19u8,
                165u8,
                215u8,
                191u8,
                23u8,
                246u8,
                13u8,
                53u8,
                123u8,
                55u8,
                217u8,
                203u8,
                181u8,
                93u8,
                215u8,
                28u8,
                187u8,
                172u8,
                141u8,
                196u8,
                6u8,
                15u8,
                100u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TransferRestrictionsDisabled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransferRestrictionsDisabled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &TransferRestrictionsDisabled,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _bEIGEN);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _bEIGEN: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._bEIGEN,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _bEIGEN: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        &self._bEIGEN,
                    ),
                )
            }
        }
    };
    /**Function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`.
```solidity
function CLOCK_MODE() external pure returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODECall {}
    ///Container type for the return parameters of the [`CLOCK_MODE()`](CLOCK_MODECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODEReturn {
        pub _0: alloy::sol_types::private::String,
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
            impl ::core::convert::From<CLOCK_MODECall> for UnderlyingRustTuple<'_> {
                fn from(value: CLOCK_MODECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOCK_MODECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
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
            impl ::core::convert::From<CLOCK_MODEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: CLOCK_MODEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOCK_MODEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CLOCK_MODECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = CLOCK_MODEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CLOCK_MODE()";
            const SELECTOR: [u8; 4] = [75u8, 245u8, 215u8, 233u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`.
```solidity
function DOMAIN_SEPARATOR() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORCall {}
    ///Container type for the return parameters of the [`DOMAIN_SEPARATOR()`](DOMAIN_SEPARATORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORReturn {
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
            impl ::core::convert::From<DOMAIN_SEPARATORCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_SEPARATORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DOMAIN_SEPARATORCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<DOMAIN_SEPARATORReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_SEPARATORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DOMAIN_SEPARATORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DOMAIN_SEPARATORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DOMAIN_SEPARATORReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DOMAIN_SEPARATOR()";
            const SELECTOR: [u8; 4] = [54u8, 68u8, 229u8, 21u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `allowance(address,address)` and selector `0xdd62ed3e`.
```solidity
function allowance(address owner, address spender) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allowanceCall {
        pub owner: alloy::sol_types::private::Address,
        pub spender: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`allowance(address,address)`](allowanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allowanceReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<allowanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: allowanceCall) -> Self {
                    (value.owner, value.spender)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        spender: tuple.1,
                    }
                }
            }
        }
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
            impl ::core::convert::From<allowanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allowanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allowanceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allowanceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allowance(address,address)";
            const SELECTOR: [u8; 4] = [221u8, 98u8, 237u8, 62u8];
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
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
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
    /**Function with signature `allowedFrom(address)` and selector `0x0455e694`.
```solidity
function allowedFrom(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allowedFromCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`allowedFrom(address)`](allowedFromCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allowedFromReturn {
        pub _0: bool,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<allowedFromCall> for UnderlyingRustTuple<'_> {
                fn from(value: allowedFromCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowedFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<allowedFromReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allowedFromReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowedFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allowedFromCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allowedFromReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allowedFrom(address)";
            const SELECTOR: [u8; 4] = [4u8, 85u8, 230u8, 148u8];
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
                        &self._0,
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
    /**Function with signature `allowedTo(address)` and selector `0x78aa33ba`.
```solidity
function allowedTo(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allowedToCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`allowedTo(address)`](allowedToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allowedToReturn {
        pub _0: bool,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<allowedToCall> for UnderlyingRustTuple<'_> {
                fn from(value: allowedToCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowedToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<allowedToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allowedToReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowedToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allowedToCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allowedToReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allowedTo(address)";
            const SELECTOR: [u8; 4] = [120u8, 170u8, 51u8, 186u8];
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
                        &self._0,
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
    /**Function with signature `approve(address,uint256)` and selector `0x095ea7b3`.
```solidity
function approve(address spender, uint256 amount) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveCall {
        pub spender: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`approve(address,uint256)`](approveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveReturn {
        pub _0: bool,
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
            impl ::core::convert::From<approveCall> for UnderlyingRustTuple<'_> {
                fn from(value: approveCall) -> Self {
                    (value.spender, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        spender: tuple.0,
                        amount: tuple.1,
                    }
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
            impl ::core::convert::From<approveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: approveReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for approveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = approveReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "approve(address,uint256)";
            const SELECTOR: [u8; 4] = [9u8, 94u8, 167u8, 179u8];
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
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `bEIGEN()` and selector `0x3f4da4c6`.
```solidity
function bEIGEN() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGENCall {}
    ///Container type for the return parameters of the [`bEIGEN()`](bEIGENCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGENReturn {
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<bEIGENCall> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGENCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGENCall {
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
            impl ::core::convert::From<bEIGENReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGENReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGENReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bEIGENCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGENReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bEIGEN()";
            const SELECTOR: [u8; 4] = [63u8, 77u8, 164u8, 198u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `balanceOf(address)` and selector `0x70a08231`.
```solidity
function balanceOf(address account) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfCall {
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`balanceOf(address)`](balanceOfCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<balanceOfCall> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<balanceOfReturn> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for balanceOfCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = balanceOfReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "balanceOf(address)";
            const SELECTOR: [u8; 4] = [112u8, 160u8, 130u8, 49u8];
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
                        &self.account,
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
    /**Function with signature `checkpoints(address,uint32)` and selector `0xf1127ed8`.
```solidity
function checkpoints(address account, uint32 pos) external view returns (ERC20VotesUpgradeable.Checkpoint memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointsCall {
        pub account: alloy::sol_types::private::Address,
        pub pos: u32,
    }
    ///Container type for the return parameters of the [`checkpoints(address,uint32)`](checkpointsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointsReturn {
        pub _0: <ERC20VotesUpgradeable::Checkpoint as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
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
            impl ::core::convert::From<checkpointsCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkpointsCall) -> Self {
                    (value.account, value.pos)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkpointsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        pos: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ERC20VotesUpgradeable::Checkpoint,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ERC20VotesUpgradeable::Checkpoint as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<checkpointsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkpointsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkpointsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkpointsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkpointsReturn;
            type ReturnTuple<'a> = (ERC20VotesUpgradeable::Checkpoint,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkpoints(address,uint32)";
            const SELECTOR: [u8; 4] = [241u8, 18u8, 126u8, 216u8];
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
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.pos),
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
    /**Function with signature `clock()` and selector `0x91ddadf4`.
```solidity
function clock() external view returns (uint48);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockCall {}
    ///Container type for the return parameters of the [`clock()`](clockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U48,
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
            impl ::core::convert::From<clockCall> for UnderlyingRustTuple<'_> {
                fn from(value: clockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U48,
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
            impl ::core::convert::From<clockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: clockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = clockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clock()";
            const SELECTOR: [u8; 4] = [145u8, 221u8, 173u8, 244u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `decimals()` and selector `0x313ce567`.
```solidity
function decimals() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decimalsCall {}
    ///Container type for the return parameters of the [`decimals()`](decimalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decimalsReturn {
        pub _0: u8,
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
            impl ::core::convert::From<decimalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: decimalsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decimalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
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
            impl ::core::convert::From<decimalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: decimalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decimalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for decimalsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = decimalsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "decimals()";
            const SELECTOR: [u8; 4] = [49u8, 60u8, 229u8, 103u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`.
```solidity
function decreaseAllowance(address spender, uint256 subtractedValue) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decreaseAllowanceCall {
        pub spender: alloy::sol_types::private::Address,
        pub subtractedValue: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`decreaseAllowance(address,uint256)`](decreaseAllowanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decreaseAllowanceReturn {
        pub _0: bool,
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
            impl ::core::convert::From<decreaseAllowanceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: decreaseAllowanceCall) -> Self {
                    (value.spender, value.subtractedValue)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for decreaseAllowanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        spender: tuple.0,
                        subtractedValue: tuple.1,
                    }
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
            impl ::core::convert::From<decreaseAllowanceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: decreaseAllowanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for decreaseAllowanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for decreaseAllowanceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = decreaseAllowanceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "decreaseAllowance(address,uint256)";
            const SELECTOR: [u8; 4] = [164u8, 87u8, 194u8, 215u8];
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
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.subtractedValue),
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
    /**Function with signature `delegate(address)` and selector `0x5c19a95c`.
```solidity
function delegate(address delegatee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateCall {
        pub delegatee: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegate(address)`](delegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<delegateCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateCall) -> Self {
                    (value.delegatee,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { delegatee: tuple.0 }
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
            impl ::core::convert::From<delegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegate(address)";
            const SELECTOR: [u8; 4] = [92u8, 25u8, 169u8, 92u8];
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
                        &self.delegatee,
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
    /**Function with signature `delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xc3cda520`.
```solidity
function delegateBySig(address delegatee, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateBySigCall {
        pub delegatee: alloy::sol_types::private::Address,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
        pub v: u8,
        pub r: alloy::sol_types::private::FixedBytes<32>,
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)`](delegateBySigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateBySigReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<delegateBySigCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateBySigCall) -> Self {
                    (
                        value.delegatee,
                        value.nonce,
                        value.expiry,
                        value.v,
                        value.r,
                        value.s,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateBySigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        delegatee: tuple.0,
                        nonce: tuple.1,
                        expiry: tuple.2,
                        v: tuple.3,
                        r: tuple.4,
                        s: tuple.5,
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
            impl ::core::convert::From<delegateBySigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateBySigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateBySigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateBySigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateBySigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [195u8, 205u8, 165u8, 32u8];
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
                        &self.delegatee,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.v),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.r),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
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
    /**Function with signature `delegates(address)` and selector `0x587cde1e`.
```solidity
function delegates(address account) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatesCall {
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegates(address)`](delegatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatesReturn {
        pub _0: alloy::sol_types::private::Address,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<delegatesCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegatesCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
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
            impl ::core::convert::From<delegatesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegatesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegatesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegatesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegates(address)";
            const SELECTOR: [u8; 4] = [88u8, 124u8, 222u8, 30u8];
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
                        &self.account,
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
    /**Function with signature `disableTransferRestrictions()` and selector `0xeb415f45`.
```solidity
function disableTransferRestrictions() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableTransferRestrictionsCall {}
    ///Container type for the return parameters of the [`disableTransferRestrictions()`](disableTransferRestrictionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableTransferRestrictionsReturn {}
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
            impl ::core::convert::From<disableTransferRestrictionsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: disableTransferRestrictionsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disableTransferRestrictionsCall {
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
            impl ::core::convert::From<disableTransferRestrictionsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: disableTransferRestrictionsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for disableTransferRestrictionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disableTransferRestrictionsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = disableTransferRestrictionsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disableTransferRestrictions()";
            const SELECTOR: [u8; 4] = [235u8, 65u8, 95u8, 69u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
```solidity
function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainCall {}
    ///Container type for the return parameters of the [`eip712Domain()`](eip712DomainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainReturn {
        pub fields: alloy::sol_types::private::FixedBytes<1>,
        pub name: alloy::sol_types::private::String,
        pub version: alloy::sol_types::private::String,
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        pub verifyingContract: alloy::sol_types::private::Address,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub extensions: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
            impl ::core::convert::From<eip712DomainCall> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<1>,
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<eip712DomainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainReturn) -> Self {
                    (
                        value.fields,
                        value.name,
                        value.version,
                        value.chainId,
                        value.verifyingContract,
                        value.salt,
                        value.extensions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        fields: tuple.0,
                        name: tuple.1,
                        version: tuple.2,
                        chainId: tuple.3,
                        verifyingContract: tuple.4,
                        salt: tuple.5,
                        extensions: tuple.6,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip712DomainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eip712DomainReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip712Domain()";
            const SELECTOR: [u8; 4] = [132u8, 176u8, 25u8, 110u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getPastTotalSupply(uint256)` and selector `0x8e539e8c`.
```solidity
function getPastTotalSupply(uint256 timepoint) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastTotalSupplyCall {
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getPastTotalSupply(uint256)`](getPastTotalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastTotalSupplyReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getPastTotalSupplyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPastTotalSupplyCall) -> Self {
                    (value.timepoint,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPastTotalSupplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { timepoint: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<getPastTotalSupplyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPastTotalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPastTotalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPastTotalSupplyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPastTotalSupplyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPastTotalSupply(uint256)";
            const SELECTOR: [u8; 4] = [142u8, 83u8, 158u8, 140u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
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
    /**Function with signature `getPastVotes(address,uint256)` and selector `0x3a46b1a8`.
```solidity
function getPastVotes(address account, uint256 timepoint) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotesCall {
        pub account: alloy::sol_types::private::Address,
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getPastVotes(address,uint256)`](getPastVotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotesReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getPastVotesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotesCall) -> Self {
                    (value.account, value.timepoint)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPastVotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        timepoint: tuple.1,
                    }
                }
            }
        }
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
            impl ::core::convert::From<getPastVotesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPastVotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPastVotesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPastVotesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPastVotes(address,uint256)";
            const SELECTOR: [u8; 4] = [58u8, 70u8, 177u8, 168u8];
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
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
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
    /**Function with signature `getVotes(address)` and selector `0x9ab24eb0`.
```solidity
function getVotes(address account) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesCall {
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getVotes(address)`](getVotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<getVotesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getVotesCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<getVotesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getVotesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getVotesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getVotesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getVotes(address)";
            const SELECTOR: [u8; 4] = [154u8, 178u8, 78u8, 176u8];
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
                        &self.account,
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
    /**Function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`.
```solidity
function increaseAllowance(address spender, uint256 addedValue) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct increaseAllowanceCall {
        pub spender: alloy::sol_types::private::Address,
        pub addedValue: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`increaseAllowance(address,uint256)`](increaseAllowanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct increaseAllowanceReturn {
        pub _0: bool,
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
            impl ::core::convert::From<increaseAllowanceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: increaseAllowanceCall) -> Self {
                    (value.spender, value.addedValue)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for increaseAllowanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        spender: tuple.0,
                        addedValue: tuple.1,
                    }
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
            impl ::core::convert::From<increaseAllowanceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: increaseAllowanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for increaseAllowanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for increaseAllowanceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = increaseAllowanceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "increaseAllowance(address,uint256)";
            const SELECTOR: [u8; 4] = [57u8, 80u8, 147u8, 81u8];
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
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.addedValue),
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
    /**Function with signature `initialize(address,address[],uint256[],uint256[])` and selector `0xa7d1195d`.
```solidity
function initialize(address initialOwner, address[] memory minters, uint256[] memory mintingAllowances, uint256[] memory mintAllowedAfters) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub initialOwner: alloy::sol_types::private::Address,
        pub minters: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub mintingAllowances: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        pub mintAllowedAfters: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`initialize(address,address[],uint256[],uint256[])`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value.initialOwner,
                        value.minters,
                        value.mintingAllowances,
                        value.mintAllowedAfters,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        minters: tuple.1,
                        mintingAllowances: tuple.2,
                        mintAllowedAfters: tuple.3,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address[],uint256[],uint256[])";
            const SELECTOR: [u8; 4] = [167u8, 209u8, 25u8, 93u8];
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
                        &self.initialOwner,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.minters),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.mintingAllowances),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.mintAllowedAfters),
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
    /**Function with signature `mint()` and selector `0x1249c58b`.
```solidity
function mint() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mint_0Call {}
    ///Container type for the return parameters of the [`mint()`](mint_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mint_0Return {}
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
            impl ::core::convert::From<mint_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: mint_0Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mint_0Call {
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
            impl ::core::convert::From<mint_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: mint_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mint_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mint_0Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mint_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mint()";
            const SELECTOR: [u8; 4] = [18u8, 73u8, 197u8, 139u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `mint(address,uint256)` and selector `0x40c10f19`.
```solidity
function mint(address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mint_1Call {
        pub to: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`mint(address,uint256)`](mint_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mint_1Return {}
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<mint_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: mint_1Call) -> Self {
                    (value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mint_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        amount: tuple.1,
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
            impl ::core::convert::From<mint_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: mint_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mint_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mint_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mint_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mint(address,uint256)";
            const SELECTOR: [u8; 4] = [64u8, 193u8, 15u8, 25u8];
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
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `mintAllowedAfter(address)` and selector `0x53957125`.
```solidity
function mintAllowedAfter(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mintAllowedAfterCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`mintAllowedAfter(address)`](mintAllowedAfterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mintAllowedAfterReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<mintAllowedAfterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: mintAllowedAfterCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mintAllowedAfterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<mintAllowedAfterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: mintAllowedAfterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mintAllowedAfterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mintAllowedAfterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mintAllowedAfterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mintAllowedAfter(address)";
            const SELECTOR: [u8; 4] = [83u8, 149u8, 113u8, 37u8];
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
                        &self._0,
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
    /**Function with signature `mintingAllowance(address)` and selector `0x81b97161`.
```solidity
function mintingAllowance(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mintingAllowanceCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`mintingAllowance(address)`](mintingAllowanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mintingAllowanceReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<mintingAllowanceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: mintingAllowanceCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mintingAllowanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<mintingAllowanceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: mintingAllowanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mintingAllowanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mintingAllowanceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mintingAllowanceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mintingAllowance(address)";
            const SELECTOR: [u8; 4] = [129u8, 185u8, 113u8, 97u8];
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
                        &self._0,
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
    /**Function with signature `multisend(address[],uint256[])` and selector `0xaad41a41`.
```solidity
function multisend(address[] memory receivers, uint256[] memory amounts) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multisendCall {
        pub receivers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub amounts: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`multisend(address[],uint256[])`](multisendCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multisendReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<multisendCall> for UnderlyingRustTuple<'_> {
                fn from(value: multisendCall) -> Self {
                    (value.receivers, value.amounts)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multisendCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        receivers: tuple.0,
                        amounts: tuple.1,
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
            impl ::core::convert::From<multisendReturn> for UnderlyingRustTuple<'_> {
                fn from(value: multisendReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multisendReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for multisendCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = multisendReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "multisend(address[],uint256[])";
            const SELECTOR: [u8; 4] = [170u8, 212u8, 26u8, 65u8];
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.receivers),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.amounts),
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
    /**Function with signature `name()` and selector `0x06fdde03`.
```solidity
function name() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameCall {}
    ///Container type for the return parameters of the [`name()`](nameCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameReturn {
        pub _0: alloy::sol_types::private::String,
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
            impl ::core::convert::From<nameCall> for UnderlyingRustTuple<'_> {
                fn from(value: nameCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
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
            impl ::core::convert::From<nameReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nameReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nameCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = nameReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "name()";
            const SELECTOR: [u8; 4] = [6u8, 253u8, 222u8, 3u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `nonces(address)` and selector `0x7ecebe00`.
```solidity
function nonces(address owner) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noncesCall {
        pub owner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`nonces(address)`](noncesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noncesReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<noncesCall> for UnderlyingRustTuple<'_> {
                fn from(value: noncesCall) -> Self {
                    (value.owner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noncesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { owner: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<noncesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: noncesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noncesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for noncesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = noncesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nonces(address)";
            const SELECTOR: [u8; 4] = [126u8, 206u8, 190u8, 0u8];
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
                        &self.owner,
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
    /**Function with signature `numCheckpoints(address)` and selector `0x6fcfff45`.
```solidity
function numCheckpoints(address account) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numCheckpointsCall {
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`numCheckpoints(address)`](numCheckpointsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numCheckpointsReturn {
        pub _0: u32,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<numCheckpointsCall> for UnderlyingRustTuple<'_> {
                fn from(value: numCheckpointsCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for numCheckpointsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<numCheckpointsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: numCheckpointsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for numCheckpointsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for numCheckpointsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = numCheckpointsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "numCheckpoints(address)";
            const SELECTOR: [u8; 4] = [111u8, 207u8, 255u8, 69u8];
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
                        &self.account,
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
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        pub _0: alloy::sol_types::private::Address,
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`.
```solidity
function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permitCall {
        pub owner: alloy::sol_types::private::Address,
        pub spender: alloy::sol_types::private::Address,
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
        pub v: u8,
        pub r: alloy::sol_types::private::FixedBytes<32>,
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`permit(address,address,uint256,uint256,uint8,bytes32,bytes32)`](permitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permitReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<permitCall> for UnderlyingRustTuple<'_> {
                fn from(value: permitCall) -> Self {
                    (
                        value.owner,
                        value.spender,
                        value.value,
                        value.deadline,
                        value.v,
                        value.r,
                        value.s,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        spender: tuple.1,
                        value: tuple.2,
                        deadline: tuple.3,
                        v: tuple.4,
                        r: tuple.5,
                        s: tuple.6,
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
            impl ::core::convert::From<permitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: permitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permitCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = permitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [213u8, 5u8, 172u8, 207u8];
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
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.v),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.r),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
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
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
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
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
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
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setAllowedFrom(address,bool)` and selector `0x1ffacdef`.
```solidity
function setAllowedFrom(address from, bool isAllowedFrom) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllowedFromCall {
        pub from: alloy::sol_types::private::Address,
        pub isAllowedFrom: bool,
    }
    ///Container type for the return parameters of the [`setAllowedFrom(address,bool)`](setAllowedFromCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllowedFromReturn {}
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
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, bool);
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
            impl ::core::convert::From<setAllowedFromCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAllowedFromCall) -> Self {
                    (value.from, value.isAllowedFrom)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAllowedFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        isAllowedFrom: tuple.1,
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
            impl ::core::convert::From<setAllowedFromReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAllowedFromReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAllowedFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAllowedFromCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAllowedFromReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAllowedFrom(address,bool)";
            const SELECTOR: [u8; 4] = [31u8, 250u8, 205u8, 239u8];
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isAllowedFrom,
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
    /**Function with signature `setAllowedFromPermissionless(address,bool)` and selector `0x94b819dc`.
```solidity
function setAllowedFromPermissionless(address from, bool isAllowedFrom) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllowedFromPermissionlessCall {
        pub from: alloy::sol_types::private::Address,
        pub isAllowedFrom: bool,
    }
    ///Container type for the return parameters of the [`setAllowedFromPermissionless(address,bool)`](setAllowedFromPermissionlessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllowedFromPermissionlessReturn {}
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
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, bool);
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
            impl ::core::convert::From<setAllowedFromPermissionlessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAllowedFromPermissionlessCall) -> Self {
                    (value.from, value.isAllowedFrom)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAllowedFromPermissionlessCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        isAllowedFrom: tuple.1,
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
            impl ::core::convert::From<setAllowedFromPermissionlessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAllowedFromPermissionlessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAllowedFromPermissionlessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAllowedFromPermissionlessCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAllowedFromPermissionlessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAllowedFromPermissionless(address,bool)";
            const SELECTOR: [u8; 4] = [148u8, 184u8, 25u8, 220u8];
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isAllowedFrom,
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
    /**Function with signature `setAllowedTo(address,bool)` and selector `0xb8c25594`.
```solidity
function setAllowedTo(address to, bool isAllowedTo) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllowedToCall {
        pub to: alloy::sol_types::private::Address,
        pub isAllowedTo: bool,
    }
    ///Container type for the return parameters of the [`setAllowedTo(address,bool)`](setAllowedToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllowedToReturn {}
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
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, bool);
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
            impl ::core::convert::From<setAllowedToCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAllowedToCall) -> Self {
                    (value.to, value.isAllowedTo)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAllowedToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        isAllowedTo: tuple.1,
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
            impl ::core::convert::From<setAllowedToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setAllowedToReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAllowedToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAllowedToCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAllowedToReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAllowedTo(address,bool)";
            const SELECTOR: [u8; 4] = [184u8, 194u8, 85u8, 148u8];
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
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isAllowedTo,
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
    /**Function with signature `setTransferRestrictionsDisabledAfterToMax()` and selector `0xb3793585`.
```solidity
function setTransferRestrictionsDisabledAfterToMax() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setTransferRestrictionsDisabledAfterToMaxCall {}
    ///Container type for the return parameters of the [`setTransferRestrictionsDisabledAfterToMax()`](setTransferRestrictionsDisabledAfterToMaxCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setTransferRestrictionsDisabledAfterToMaxReturn {}
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
            impl ::core::convert::From<setTransferRestrictionsDisabledAfterToMaxCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setTransferRestrictionsDisabledAfterToMaxCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setTransferRestrictionsDisabledAfterToMaxCall {
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
            impl ::core::convert::From<setTransferRestrictionsDisabledAfterToMaxReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setTransferRestrictionsDisabledAfterToMaxReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setTransferRestrictionsDisabledAfterToMaxReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setTransferRestrictionsDisabledAfterToMaxCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setTransferRestrictionsDisabledAfterToMaxReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setTransferRestrictionsDisabledAfterToMax()";
            const SELECTOR: [u8; 4] = [179u8, 121u8, 53u8, 133u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `symbol()` and selector `0x95d89b41`.
```solidity
function symbol() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct symbolCall {}
    ///Container type for the return parameters of the [`symbol()`](symbolCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct symbolReturn {
        pub _0: alloy::sol_types::private::String,
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
            impl ::core::convert::From<symbolCall> for UnderlyingRustTuple<'_> {
                fn from(value: symbolCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
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
            impl ::core::convert::From<symbolReturn> for UnderlyingRustTuple<'_> {
                fn from(value: symbolReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for symbolCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = symbolReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "symbol()";
            const SELECTOR: [u8; 4] = [149u8, 216u8, 155u8, 65u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `totalSupply()` and selector `0x18160ddd`.
```solidity
function totalSupply() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalSupplyCall {}
    ///Container type for the return parameters of the [`totalSupply()`](totalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalSupplyReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<totalSupplyCall> for UnderlyingRustTuple<'_> {
                fn from(value: totalSupplyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<totalSupplyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: totalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalSupplyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = totalSupplyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalSupply()";
            const SELECTOR: [u8; 4] = [24u8, 22u8, 13u8, 221u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transfer(address,uint256)` and selector `0xa9059cbb`.
```solidity
function transfer(address to, uint256 amount) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferCall {
        pub to: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`transfer(address,uint256)`](transferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferReturn {
        pub _0: bool,
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
            impl ::core::convert::From<transferCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferCall) -> Self {
                    (value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        amount: tuple.1,
                    }
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
            impl ::core::convert::From<transferReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transfer(address,uint256)";
            const SELECTOR: [u8; 4] = [169u8, 5u8, 156u8, 187u8];
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
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`.
```solidity
function transferFrom(address from, address to, uint256 amount) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferFromCall {
        pub from: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`transferFrom(address,address,uint256)`](transferFromCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferFromReturn {
        pub _0: bool,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<transferFromCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromCall) -> Self {
                    (value.from, value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        to: tuple.1,
                        amount: tuple.2,
                    }
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
            impl ::core::convert::From<transferFromReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferFromCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferFromReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferFrom(address,address,uint256)";
            const SELECTOR: [u8; 4] = [35u8, 184u8, 114u8, 221u8];
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
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
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnershipPermissionless(address)` and selector `0xec08f9c4`.
```solidity
function transferOwnershipPermissionless(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipPermissionlessCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnershipPermissionless(address)`](transferOwnershipPermissionlessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipPermissionlessReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<transferOwnershipPermissionlessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipPermissionlessCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipPermissionlessCall {
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
            impl ::core::convert::From<transferOwnershipPermissionlessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipPermissionlessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipPermissionlessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipPermissionlessCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipPermissionlessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnershipPermissionless(address)";
            const SELECTOR: [u8; 4] = [236u8, 8u8, 249u8, 196u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transferRestrictionsDisabledAfter()` and selector `0x9aec4bae`.
```solidity
function transferRestrictionsDisabledAfter() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferRestrictionsDisabledAfterCall {}
    ///Container type for the return parameters of the [`transferRestrictionsDisabledAfter()`](transferRestrictionsDisabledAfterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferRestrictionsDisabledAfterReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<transferRestrictionsDisabledAfterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferRestrictionsDisabledAfterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferRestrictionsDisabledAfterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<transferRestrictionsDisabledAfterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferRestrictionsDisabledAfterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferRestrictionsDisabledAfterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferRestrictionsDisabledAfterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferRestrictionsDisabledAfterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferRestrictionsDisabledAfter()";
            const SELECTOR: [u8; 4] = [154u8, 236u8, 75u8, 174u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `unwrap(uint256)` and selector `0xde0e9a3e`.
```solidity
function unwrap(uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unwrapCall {
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`unwrap(uint256)`](unwrapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unwrapReturn {}
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
            impl ::core::convert::From<unwrapCall> for UnderlyingRustTuple<'_> {
                fn from(value: unwrapCall) -> Self {
                    (value.amount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unwrapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amount: tuple.0 }
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
            impl ::core::convert::From<unwrapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unwrapReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unwrapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unwrapCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unwrapReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unwrap(uint256)";
            const SELECTOR: [u8; 4] = [222u8, 14u8, 154u8, 62u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `wrap(uint256)` and selector `0xea598cb0`.
```solidity
function wrap(uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wrapCall {
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`wrap(uint256)`](wrapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wrapReturn {}
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
            impl ::core::convert::From<wrapCall> for UnderlyingRustTuple<'_> {
                fn from(value: wrapCall) -> Self {
                    (value.amount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for wrapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amount: tuple.0 }
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
            impl ::core::convert::From<wrapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: wrapReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for wrapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for wrapCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = wrapReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "wrap(uint256)";
            const SELECTOR: [u8; 4] = [234u8, 89u8, 140u8, 176u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    ///Container for all the [`EigenHarness`](self) function calls.
    pub enum EigenHarnessCalls {
        CLOCK_MODE(CLOCK_MODECall),
        DOMAIN_SEPARATOR(DOMAIN_SEPARATORCall),
        allowance(allowanceCall),
        allowedFrom(allowedFromCall),
        allowedTo(allowedToCall),
        approve(approveCall),
        bEIGEN(bEIGENCall),
        balanceOf(balanceOfCall),
        checkpoints(checkpointsCall),
        clock(clockCall),
        decimals(decimalsCall),
        decreaseAllowance(decreaseAllowanceCall),
        delegate(delegateCall),
        delegateBySig(delegateBySigCall),
        delegates(delegatesCall),
        disableTransferRestrictions(disableTransferRestrictionsCall),
        eip712Domain(eip712DomainCall),
        getPastTotalSupply(getPastTotalSupplyCall),
        getPastVotes(getPastVotesCall),
        getVotes(getVotesCall),
        increaseAllowance(increaseAllowanceCall),
        initialize(initializeCall),
        mint_0(mint_0Call),
        mint_1(mint_1Call),
        mintAllowedAfter(mintAllowedAfterCall),
        mintingAllowance(mintingAllowanceCall),
        multisend(multisendCall),
        name(nameCall),
        nonces(noncesCall),
        numCheckpoints(numCheckpointsCall),
        owner(ownerCall),
        permit(permitCall),
        renounceOwnership(renounceOwnershipCall),
        setAllowedFrom(setAllowedFromCall),
        setAllowedFromPermissionless(setAllowedFromPermissionlessCall),
        setAllowedTo(setAllowedToCall),
        setTransferRestrictionsDisabledAfterToMax(
            setTransferRestrictionsDisabledAfterToMaxCall,
        ),
        symbol(symbolCall),
        totalSupply(totalSupplyCall),
        transfer(transferCall),
        transferFrom(transferFromCall),
        transferOwnership(transferOwnershipCall),
        transferOwnershipPermissionless(transferOwnershipPermissionlessCall),
        transferRestrictionsDisabledAfter(transferRestrictionsDisabledAfterCall),
        unwrap(unwrapCall),
        wrap(wrapCall),
    }
    #[automatically_derived]
    impl EigenHarnessCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 85u8, 230u8, 148u8],
            [6u8, 253u8, 222u8, 3u8],
            [9u8, 94u8, 167u8, 179u8],
            [18u8, 73u8, 197u8, 139u8],
            [24u8, 22u8, 13u8, 221u8],
            [31u8, 250u8, 205u8, 239u8],
            [35u8, 184u8, 114u8, 221u8],
            [49u8, 60u8, 229u8, 103u8],
            [54u8, 68u8, 229u8, 21u8],
            [57u8, 80u8, 147u8, 81u8],
            [58u8, 70u8, 177u8, 168u8],
            [63u8, 77u8, 164u8, 198u8],
            [64u8, 193u8, 15u8, 25u8],
            [75u8, 245u8, 215u8, 233u8],
            [83u8, 149u8, 113u8, 37u8],
            [88u8, 124u8, 222u8, 30u8],
            [92u8, 25u8, 169u8, 92u8],
            [111u8, 207u8, 255u8, 69u8],
            [112u8, 160u8, 130u8, 49u8],
            [113u8, 80u8, 24u8, 166u8],
            [120u8, 170u8, 51u8, 186u8],
            [126u8, 206u8, 190u8, 0u8],
            [129u8, 185u8, 113u8, 97u8],
            [132u8, 176u8, 25u8, 110u8],
            [141u8, 165u8, 203u8, 91u8],
            [142u8, 83u8, 158u8, 140u8],
            [145u8, 221u8, 173u8, 244u8],
            [148u8, 184u8, 25u8, 220u8],
            [149u8, 216u8, 155u8, 65u8],
            [154u8, 178u8, 78u8, 176u8],
            [154u8, 236u8, 75u8, 174u8],
            [164u8, 87u8, 194u8, 215u8],
            [167u8, 209u8, 25u8, 93u8],
            [169u8, 5u8, 156u8, 187u8],
            [170u8, 212u8, 26u8, 65u8],
            [179u8, 121u8, 53u8, 133u8],
            [184u8, 194u8, 85u8, 148u8],
            [195u8, 205u8, 165u8, 32u8],
            [213u8, 5u8, 172u8, 207u8],
            [221u8, 98u8, 237u8, 62u8],
            [222u8, 14u8, 154u8, 62u8],
            [234u8, 89u8, 140u8, 176u8],
            [235u8, 65u8, 95u8, 69u8],
            [236u8, 8u8, 249u8, 196u8],
            [241u8, 18u8, 126u8, 216u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EigenHarnessCalls {
        const NAME: &'static str = "EigenHarnessCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 46usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CLOCK_MODE(_) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DOMAIN_SEPARATOR(_) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allowance(_) => {
                    <allowanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allowedFrom(_) => {
                    <allowedFromCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allowedTo(_) => {
                    <allowedToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::approve(_) => <approveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::bEIGEN(_) => <bEIGENCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::balanceOf(_) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkpoints(_) => {
                    <checkpointsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::clock(_) => <clockCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::decimals(_) => <decimalsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::decreaseAllowance(_) => {
                    <decreaseAllowanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegate(_) => <delegateCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegateBySig(_) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegates(_) => {
                    <delegatesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disableTransferRestrictions(_) => {
                    <disableTransferRestrictionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPastTotalSupply(_) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPastVotes(_) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVotes(_) => <getVotesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::increaseAllowance(_) => {
                    <increaseAllowanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mint_0(_) => <mint_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::mint_1(_) => <mint_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::mintAllowedAfter(_) => {
                    <mintAllowedAfterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mintingAllowance(_) => {
                    <mintingAllowanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::multisend(_) => {
                    <multisendCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::name(_) => <nameCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nonces(_) => <noncesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::numCheckpoints(_) => {
                    <numCheckpointsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::permit(_) => <permitCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAllowedFrom(_) => {
                    <setAllowedFromCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAllowedFromPermissionless(_) => {
                    <setAllowedFromPermissionlessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAllowedTo(_) => {
                    <setAllowedToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setTransferRestrictionsDisabledAfterToMax(_) => {
                    <setTransferRestrictionsDisabledAfterToMaxCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::symbol(_) => <symbolCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::totalSupply(_) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transfer(_) => <transferCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transferFrom(_) => {
                    <transferFromCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnershipPermissionless(_) => {
                    <transferOwnershipPermissionlessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferRestrictionsDisabledAfter(_) => {
                    <transferRestrictionsDisabledAfterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unwrap(_) => <unwrapCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::wrap(_) => <wrapCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<EigenHarnessCalls>] = &[
                {
                    fn allowedFrom(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <allowedFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::allowedFrom)
                    }
                    allowedFrom
                },
                {
                    fn name(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <nameCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::name)
                    }
                    name
                },
                {
                    fn approve(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <approveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::approve)
                    }
                    approve
                },
                {
                    fn mint_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <mint_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::mint_0)
                    }
                    mint_0
                },
                {
                    fn totalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <totalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::totalSupply)
                    }
                    totalSupply
                },
                {
                    fn setAllowedFrom(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <setAllowedFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::setAllowedFrom)
                    }
                    setAllowedFrom
                },
                {
                    fn transferFrom(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <transferFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::transferFrom)
                    }
                    transferFrom
                },
                {
                    fn decimals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <decimalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::decimals)
                    }
                    decimals
                },
                {
                    fn DOMAIN_SEPARATOR(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::DOMAIN_SEPARATOR)
                    }
                    DOMAIN_SEPARATOR
                },
                {
                    fn increaseAllowance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <increaseAllowanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::increaseAllowance)
                    }
                    increaseAllowance
                },
                {
                    fn getPastVotes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <getPastVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::getPastVotes)
                    }
                    getPastVotes
                },
                {
                    fn bEIGEN(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <bEIGENCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::bEIGEN)
                    }
                    bEIGEN
                },
                {
                    fn mint_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <mint_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::mint_1)
                    }
                    mint_1
                },
                {
                    fn CLOCK_MODE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::CLOCK_MODE)
                    }
                    CLOCK_MODE
                },
                {
                    fn mintAllowedAfter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <mintAllowedAfterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::mintAllowedAfter)
                    }
                    mintAllowedAfter
                },
                {
                    fn delegates(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <delegatesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::delegates)
                    }
                    delegates
                },
                {
                    fn delegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <delegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::delegate)
                    }
                    delegate
                },
                {
                    fn numCheckpoints(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <numCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::numCheckpoints)
                    }
                    numCheckpoints
                },
                {
                    fn balanceOf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn allowedTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <allowedToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::allowedTo)
                    }
                    allowedTo
                },
                {
                    fn nonces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <noncesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::nonces)
                    }
                    nonces
                },
                {
                    fn mintingAllowance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <mintingAllowanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::mintingAllowance)
                    }
                    mintingAllowance
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::owner)
                    }
                    owner
                },
                {
                    fn getPastTotalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::getPastTotalSupply)
                    }
                    getPastTotalSupply
                },
                {
                    fn clock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <clockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::clock)
                    }
                    clock
                },
                {
                    fn setAllowedFromPermissionless(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <setAllowedFromPermissionlessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::setAllowedFromPermissionless)
                    }
                    setAllowedFromPermissionless
                },
                {
                    fn symbol(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <symbolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::symbol)
                    }
                    symbol
                },
                {
                    fn getVotes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <getVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::getVotes)
                    }
                    getVotes
                },
                {
                    fn transferRestrictionsDisabledAfter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <transferRestrictionsDisabledAfterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::transferRestrictionsDisabledAfter)
                    }
                    transferRestrictionsDisabledAfter
                },
                {
                    fn decreaseAllowance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <decreaseAllowanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::decreaseAllowance)
                    }
                    decreaseAllowance
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::initialize)
                    }
                    initialize
                },
                {
                    fn transfer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <transferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::transfer)
                    }
                    transfer
                },
                {
                    fn multisend(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <multisendCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::multisend)
                    }
                    multisend
                },
                {
                    fn setTransferRestrictionsDisabledAfterToMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <setTransferRestrictionsDisabledAfterToMaxCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                EigenHarnessCalls::setTransferRestrictionsDisabledAfterToMax,
                            )
                    }
                    setTransferRestrictionsDisabledAfterToMax
                },
                {
                    fn setAllowedTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <setAllowedToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::setAllowedTo)
                    }
                    setAllowedTo
                },
                {
                    fn delegateBySig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <delegateBySigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::delegateBySig)
                    }
                    delegateBySig
                },
                {
                    fn permit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <permitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::permit)
                    }
                    permit
                },
                {
                    fn allowance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <allowanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::allowance)
                    }
                    allowance
                },
                {
                    fn unwrap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <unwrapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::unwrap)
                    }
                    unwrap
                },
                {
                    fn wrap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <wrapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::wrap)
                    }
                    wrap
                },
                {
                    fn disableTransferRestrictions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <disableTransferRestrictionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::disableTransferRestrictions)
                    }
                    disableTransferRestrictions
                },
                {
                    fn transferOwnershipPermissionless(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <transferOwnershipPermissionlessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::transferOwnershipPermissionless)
                    }
                    transferOwnershipPermissionless
                },
                {
                    fn checkpoints(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <checkpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::checkpoints)
                    }
                    checkpoints
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenHarnessCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenHarnessCalls::transferOwnership)
                    }
                    transferOwnership
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
                Self::CLOCK_MODE(inner) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::DOMAIN_SEPARATOR(inner) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allowance(inner) => {
                    <allowanceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::allowedFrom(inner) => {
                    <allowedFromCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allowedTo(inner) => {
                    <allowedToCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bEIGEN(inner) => {
                    <bEIGENCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::checkpoints(inner) => {
                    <checkpointsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::clock(inner) => {
                    <clockCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::decimals(inner) => {
                    <decimalsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::decreaseAllowance(inner) => {
                    <decreaseAllowanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegate(inner) => {
                    <delegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegateBySig(inner) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegates(inner) => {
                    <delegatesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::disableTransferRestrictions(inner) => {
                    <disableTransferRestrictionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPastTotalSupply(inner) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPastVotes(inner) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getVotes(inner) => {
                    <getVotesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::increaseAllowance(inner) => {
                    <increaseAllowanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::mint_0(inner) => {
                    <mint_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::mint_1(inner) => {
                    <mint_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::mintAllowedAfter(inner) => {
                    <mintAllowedAfterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::mintingAllowance(inner) => {
                    <mintingAllowanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::multisend(inner) => {
                    <multisendCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nonces(inner) => {
                    <noncesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::numCheckpoints(inner) => {
                    <numCheckpointsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::permit(inner) => {
                    <permitCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAllowedFrom(inner) => {
                    <setAllowedFromCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAllowedFromPermissionless(inner) => {
                    <setAllowedFromPermissionlessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAllowedTo(inner) => {
                    <setAllowedToCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setTransferRestrictionsDisabledAfterToMax(inner) => {
                    <setTransferRestrictionsDisabledAfterToMaxCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::symbol(inner) => {
                    <symbolCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::totalSupply(inner) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnershipPermissionless(inner) => {
                    <transferOwnershipPermissionlessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferRestrictionsDisabledAfter(inner) => {
                    <transferRestrictionsDisabledAfterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unwrap(inner) => {
                    <unwrapCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::wrap(inner) => {
                    <wrapCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CLOCK_MODE(inner) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DOMAIN_SEPARATOR(inner) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allowance(inner) => {
                    <allowanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allowedFrom(inner) => {
                    <allowedFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allowedTo(inner) => {
                    <allowedToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::bEIGEN(inner) => {
                    <bEIGENCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkpoints(inner) => {
                    <checkpointsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::clock(inner) => {
                    <clockCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::decimals(inner) => {
                    <decimalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::decreaseAllowance(inner) => {
                    <decreaseAllowanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegate(inner) => {
                    <delegateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegateBySig(inner) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegates(inner) => {
                    <delegatesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disableTransferRestrictions(inner) => {
                    <disableTransferRestrictionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPastTotalSupply(inner) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPastVotes(inner) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getVotes(inner) => {
                    <getVotesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::increaseAllowance(inner) => {
                    <increaseAllowanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::mint_0(inner) => {
                    <mint_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::mint_1(inner) => {
                    <mint_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::mintAllowedAfter(inner) => {
                    <mintAllowedAfterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mintingAllowance(inner) => {
                    <mintingAllowanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::multisend(inner) => {
                    <multisendCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nonces(inner) => {
                    <noncesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::numCheckpoints(inner) => {
                    <numCheckpointsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::permit(inner) => {
                    <permitCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAllowedFrom(inner) => {
                    <setAllowedFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAllowedFromPermissionless(inner) => {
                    <setAllowedFromPermissionlessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAllowedTo(inner) => {
                    <setAllowedToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setTransferRestrictionsDisabledAfterToMax(inner) => {
                    <setTransferRestrictionsDisabledAfterToMaxCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::symbol(inner) => {
                    <symbolCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::totalSupply(inner) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::transferOwnershipPermissionless(inner) => {
                    <transferOwnershipPermissionlessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferRestrictionsDisabledAfter(inner) => {
                    <transferRestrictionsDisabledAfterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unwrap(inner) => {
                    <unwrapCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::wrap(inner) => {
                    <wrapCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`EigenHarness`](self) events.
    pub enum EigenHarnessEvents {
        Approval(Approval),
        DelegateChanged(DelegateChanged),
        DelegateVotesChanged(DelegateVotesChanged),
        EIP712DomainChanged(EIP712DomainChanged),
        Initialized(Initialized),
        Mint(Mint),
        OwnershipTransferred(OwnershipTransferred),
        SetAllowedFrom(SetAllowedFrom),
        SetAllowedTo(SetAllowedTo),
        Transfer(Transfer),
        TransferRestrictionsDisabled(TransferRestrictionsDisabled),
    }
    #[automatically_derived]
    impl EigenHarnessEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8,
                99u8,
                135u8,
                201u8,
                234u8,
                54u8,
                40u8,
                184u8,
                138u8,
                99u8,
                59u8,
                180u8,
                243u8,
                177u8,
                81u8,
                119u8,
                15u8,
                112u8,
                8u8,
                81u8,
                23u8,
                161u8,
                95u8,
                155u8,
                243u8,
                120u8,
                124u8,
                218u8,
                83u8,
                241u8,
                61u8,
                49u8,
            ],
            [
                15u8,
                103u8,
                152u8,
                165u8,
                96u8,
                121u8,
                58u8,
                84u8,
                195u8,
                188u8,
                254u8,
                134u8,
                169u8,
                60u8,
                222u8,
                30u8,
                115u8,
                8u8,
                125u8,
                148u8,
                76u8,
                14u8,
                162u8,
                5u8,
                68u8,
                19u8,
                125u8,
                65u8,
                33u8,
                57u8,
                104u8,
                133u8,
            ],
            [
                43u8,
                24u8,
                152u8,
                109u8,
                59u8,
                168u8,
                9u8,
                219u8,
                47u8,
                19u8,
                165u8,
                215u8,
                191u8,
                23u8,
                246u8,
                13u8,
                53u8,
                123u8,
                55u8,
                217u8,
                203u8,
                181u8,
                93u8,
                215u8,
                28u8,
                187u8,
                172u8,
                141u8,
                196u8,
                6u8,
                15u8,
                100u8,
            ],
            [
                49u8,
                52u8,
                232u8,
                162u8,
                230u8,
                217u8,
                126u8,
                146u8,
                154u8,
                126u8,
                84u8,
                1u8,
                30u8,
                165u8,
                72u8,
                93u8,
                125u8,
                25u8,
                109u8,
                213u8,
                240u8,
                186u8,
                77u8,
                78u8,
                249u8,
                88u8,
                3u8,
                232u8,
                227u8,
                252u8,
                37u8,
                127u8,
            ],
            [
                114u8,
                165u8,
                97u8,
                209u8,
                175u8,
                116u8,
                9u8,
                70u8,
                125u8,
                174u8,
                79u8,
                30u8,
                159u8,
                197u8,
                37u8,
                144u8,
                169u8,
                51u8,
                90u8,
                29u8,
                218u8,
                23u8,
                114u8,
                126u8,
                43u8,
                106u8,
                168u8,
                196u8,
                219u8,
                53u8,
                16u8,
                155u8,
            ],
            [
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ],
            [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
            [
                140u8,
                91u8,
                225u8,
                229u8,
                235u8,
                236u8,
                125u8,
                91u8,
                209u8,
                79u8,
                113u8,
                66u8,
                125u8,
                30u8,
                132u8,
                243u8,
                221u8,
                3u8,
                20u8,
                192u8,
                247u8,
                178u8,
                41u8,
                30u8,
                91u8,
                32u8,
                10u8,
                200u8,
                199u8,
                195u8,
                185u8,
                37u8,
            ],
            [
                207u8,
                32u8,
                177u8,
                236u8,
                182u8,
                4u8,
                176u8,
                232u8,
                136u8,
                141u8,
                87u8,
                156u8,
                100u8,
                232u8,
                163u8,
                177u8,
                14u8,
                89u8,
                13u8,
                69u8,
                193u8,
                194u8,
                221u8,
                219u8,
                57u8,
                59u8,
                237u8,
                40u8,
                67u8,
                98u8,
                34u8,
                113u8,
            ],
            [
                221u8,
                242u8,
                82u8,
                173u8,
                27u8,
                226u8,
                200u8,
                155u8,
                105u8,
                194u8,
                176u8,
                104u8,
                252u8,
                55u8,
                141u8,
                170u8,
                149u8,
                43u8,
                167u8,
                241u8,
                99u8,
                196u8,
                161u8,
                22u8,
                40u8,
                245u8,
                90u8,
                77u8,
                245u8,
                35u8,
                179u8,
                239u8,
            ],
            [
                222u8,
                194u8,
                186u8,
                205u8,
                210u8,
                240u8,
                91u8,
                89u8,
                222u8,
                52u8,
                218u8,
                155u8,
                82u8,
                61u8,
                255u8,
                139u8,
                228u8,
                46u8,
                94u8,
                56u8,
                232u8,
                24u8,
                200u8,
                47u8,
                219u8,
                11u8,
                174u8,
                119u8,
                67u8,
                135u8,
                167u8,
                36u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for EigenHarnessEvents {
        const NAME: &'static str = "EigenHarnessEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Approval as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Approval as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Approval)
                }
                Some(<DelegateChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DelegateChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DelegateChanged)
                }
                Some(
                    <DelegateVotesChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DelegateVotesChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DelegateVotesChanged)
                }
                Some(
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EIP712DomainChanged)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(<Mint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Mint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Mint)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<SetAllowedFrom as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SetAllowedFrom as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SetAllowedFrom)
                }
                Some(<SetAllowedTo as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SetAllowedTo as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SetAllowedTo)
                }
                Some(<Transfer as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Transfer as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Transfer)
                }
                Some(
                    <TransferRestrictionsDisabled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransferRestrictionsDisabled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TransferRestrictionsDisabled)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for EigenHarnessEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DelegateChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DelegateVotesChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Mint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SetAllowedFrom(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SetAllowedTo(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransferRestrictionsDisabled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DelegateChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DelegateVotesChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Mint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SetAllowedFrom(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SetAllowedTo(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransferRestrictionsDisabled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`EigenHarness`](self) contract instance.

See the [wrapper's documentation](`EigenHarnessInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EigenHarnessInstance<T, P, N> {
        EigenHarnessInstance::<T, P, N>::new(address, provider)
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
        _bEIGEN: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<EigenHarnessInstance<T, P, N>>,
    > {
        EigenHarnessInstance::<T, P, N>::deploy(provider, _bEIGEN)
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
        _bEIGEN: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        EigenHarnessInstance::<T, P, N>::deploy_builder(provider, _bEIGEN)
    }
    /**A [`EigenHarness`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EigenHarness`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EigenHarnessInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EigenHarnessInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EigenHarnessInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EigenHarnessInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EigenHarness`](self) contract instance.

See the [wrapper's documentation](`EigenHarnessInstance`) for more details.*/
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
            _bEIGEN: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<EigenHarnessInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _bEIGEN);
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
            _bEIGEN: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { _bEIGEN },
                    )[..],
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
    impl<T, P: ::core::clone::Clone, N> EigenHarnessInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EigenHarnessInstance<T, P, N> {
            EigenHarnessInstance {
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
    > EigenHarnessInstance<T, P, N> {
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
        ///Creates a new call builder for the [`CLOCK_MODE`] function.
        pub fn CLOCK_MODE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, CLOCK_MODECall, N> {
            self.call_builder(&CLOCK_MODECall {})
        }
        ///Creates a new call builder for the [`DOMAIN_SEPARATOR`] function.
        pub fn DOMAIN_SEPARATOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DOMAIN_SEPARATORCall, N> {
            self.call_builder(&DOMAIN_SEPARATORCall {})
        }
        ///Creates a new call builder for the [`allowance`] function.
        pub fn allowance(
            &self,
            owner: alloy::sol_types::private::Address,
            spender: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, allowanceCall, N> {
            self.call_builder(&allowanceCall { owner, spender })
        }
        ///Creates a new call builder for the [`allowedFrom`] function.
        pub fn allowedFrom(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, allowedFromCall, N> {
            self.call_builder(&allowedFromCall { _0 })
        }
        ///Creates a new call builder for the [`allowedTo`] function.
        pub fn allowedTo(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, allowedToCall, N> {
            self.call_builder(&allowedToCall { _0 })
        }
        ///Creates a new call builder for the [`approve`] function.
        pub fn approve(
            &self,
            spender: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, approveCall, N> {
            self.call_builder(&approveCall { spender, amount })
        }
        ///Creates a new call builder for the [`bEIGEN`] function.
        pub fn bEIGEN(&self) -> alloy_contract::SolCallBuilder<T, &P, bEIGENCall, N> {
            self.call_builder(&bEIGENCall {})
        }
        ///Creates a new call builder for the [`balanceOf`] function.
        pub fn balanceOf(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, balanceOfCall, N> {
            self.call_builder(&balanceOfCall { account })
        }
        ///Creates a new call builder for the [`checkpoints`] function.
        pub fn checkpoints(
            &self,
            account: alloy::sol_types::private::Address,
            pos: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkpointsCall, N> {
            self.call_builder(&checkpointsCall { account, pos })
        }
        ///Creates a new call builder for the [`clock`] function.
        pub fn clock(&self) -> alloy_contract::SolCallBuilder<T, &P, clockCall, N> {
            self.call_builder(&clockCall {})
        }
        ///Creates a new call builder for the [`decimals`] function.
        pub fn decimals(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, decimalsCall, N> {
            self.call_builder(&decimalsCall {})
        }
        ///Creates a new call builder for the [`decreaseAllowance`] function.
        pub fn decreaseAllowance(
            &self,
            spender: alloy::sol_types::private::Address,
            subtractedValue: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, decreaseAllowanceCall, N> {
            self.call_builder(
                &decreaseAllowanceCall {
                    spender,
                    subtractedValue,
                },
            )
        }
        ///Creates a new call builder for the [`delegate`] function.
        pub fn delegate(
            &self,
            delegatee: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateCall, N> {
            self.call_builder(&delegateCall { delegatee })
        }
        ///Creates a new call builder for the [`delegateBySig`] function.
        pub fn delegateBySig(
            &self,
            delegatee: alloy::sol_types::private::Address,
            nonce: alloy::sol_types::private::primitives::aliases::U256,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
            v: u8,
            r: alloy::sol_types::private::FixedBytes<32>,
            s: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateBySigCall, N> {
            self.call_builder(
                &delegateBySigCall {
                    delegatee,
                    nonce,
                    expiry,
                    v,
                    r,
                    s,
                },
            )
        }
        ///Creates a new call builder for the [`delegates`] function.
        pub fn delegates(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegatesCall, N> {
            self.call_builder(&delegatesCall { account })
        }
        ///Creates a new call builder for the [`disableTransferRestrictions`] function.
        pub fn disableTransferRestrictions(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, disableTransferRestrictionsCall, N> {
            self.call_builder(&disableTransferRestrictionsCall {})
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall {})
        }
        ///Creates a new call builder for the [`getPastTotalSupply`] function.
        pub fn getPastTotalSupply(
            &self,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPastTotalSupplyCall, N> {
            self.call_builder(
                &getPastTotalSupplyCall {
                    timepoint,
                },
            )
        }
        ///Creates a new call builder for the [`getPastVotes`] function.
        pub fn getPastVotes(
            &self,
            account: alloy::sol_types::private::Address,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPastVotesCall, N> {
            self.call_builder(
                &getPastVotesCall {
                    account,
                    timepoint,
                },
            )
        }
        ///Creates a new call builder for the [`getVotes`] function.
        pub fn getVotes(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getVotesCall, N> {
            self.call_builder(&getVotesCall { account })
        }
        ///Creates a new call builder for the [`increaseAllowance`] function.
        pub fn increaseAllowance(
            &self,
            spender: alloy::sol_types::private::Address,
            addedValue: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, increaseAllowanceCall, N> {
            self.call_builder(
                &increaseAllowanceCall {
                    spender,
                    addedValue,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            minters: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            mintingAllowances: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            mintAllowedAfters: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    initialOwner,
                    minters,
                    mintingAllowances,
                    mintAllowedAfters,
                },
            )
        }
        ///Creates a new call builder for the [`mint_0`] function.
        pub fn mint_0(&self) -> alloy_contract::SolCallBuilder<T, &P, mint_0Call, N> {
            self.call_builder(&mint_0Call {})
        }
        ///Creates a new call builder for the [`mint_1`] function.
        pub fn mint_1(
            &self,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, mint_1Call, N> {
            self.call_builder(&mint_1Call { to, amount })
        }
        ///Creates a new call builder for the [`mintAllowedAfter`] function.
        pub fn mintAllowedAfter(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, mintAllowedAfterCall, N> {
            self.call_builder(&mintAllowedAfterCall { _0 })
        }
        ///Creates a new call builder for the [`mintingAllowance`] function.
        pub fn mintingAllowance(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, mintingAllowanceCall, N> {
            self.call_builder(&mintingAllowanceCall { _0 })
        }
        ///Creates a new call builder for the [`multisend`] function.
        pub fn multisend(
            &self,
            receivers: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            amounts: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, multisendCall, N> {
            self.call_builder(
                &multisendCall {
                    receivers,
                    amounts,
                },
            )
        }
        ///Creates a new call builder for the [`name`] function.
        pub fn name(&self) -> alloy_contract::SolCallBuilder<T, &P, nameCall, N> {
            self.call_builder(&nameCall {})
        }
        ///Creates a new call builder for the [`nonces`] function.
        pub fn nonces(
            &self,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, noncesCall, N> {
            self.call_builder(&noncesCall { owner })
        }
        ///Creates a new call builder for the [`numCheckpoints`] function.
        pub fn numCheckpoints(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, numCheckpointsCall, N> {
            self.call_builder(&numCheckpointsCall { account })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`permit`] function.
        pub fn permit(
            &self,
            owner: alloy::sol_types::private::Address,
            spender: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
            v: u8,
            r: alloy::sol_types::private::FixedBytes<32>,
            s: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, permitCall, N> {
            self.call_builder(
                &permitCall {
                    owner,
                    spender,
                    value,
                    deadline,
                    v,
                    r,
                    s,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`setAllowedFrom`] function.
        pub fn setAllowedFrom(
            &self,
            from: alloy::sol_types::private::Address,
            isAllowedFrom: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAllowedFromCall, N> {
            self.call_builder(
                &setAllowedFromCall {
                    from,
                    isAllowedFrom,
                },
            )
        }
        ///Creates a new call builder for the [`setAllowedFromPermissionless`] function.
        pub fn setAllowedFromPermissionless(
            &self,
            from: alloy::sol_types::private::Address,
            isAllowedFrom: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAllowedFromPermissionlessCall, N> {
            self.call_builder(
                &setAllowedFromPermissionlessCall {
                    from,
                    isAllowedFrom,
                },
            )
        }
        ///Creates a new call builder for the [`setAllowedTo`] function.
        pub fn setAllowedTo(
            &self,
            to: alloy::sol_types::private::Address,
            isAllowedTo: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAllowedToCall, N> {
            self.call_builder(
                &setAllowedToCall {
                    to,
                    isAllowedTo,
                },
            )
        }
        ///Creates a new call builder for the [`setTransferRestrictionsDisabledAfterToMax`] function.
        pub fn setTransferRestrictionsDisabledAfterToMax(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            setTransferRestrictionsDisabledAfterToMaxCall,
            N,
        > {
            self.call_builder(
                &setTransferRestrictionsDisabledAfterToMaxCall {
                },
            )
        }
        ///Creates a new call builder for the [`symbol`] function.
        pub fn symbol(&self) -> alloy_contract::SolCallBuilder<T, &P, symbolCall, N> {
            self.call_builder(&symbolCall {})
        }
        ///Creates a new call builder for the [`totalSupply`] function.
        pub fn totalSupply(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, totalSupplyCall, N> {
            self.call_builder(&totalSupplyCall {})
        }
        ///Creates a new call builder for the [`transfer`] function.
        pub fn transfer(
            &self,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferCall, N> {
            self.call_builder(&transferCall { to, amount })
        }
        ///Creates a new call builder for the [`transferFrom`] function.
        pub fn transferFrom(
            &self,
            from: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferFromCall, N> {
            self.call_builder(
                &transferFromCall {
                    from,
                    to,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`transferOwnershipPermissionless`] function.
        pub fn transferOwnershipPermissionless(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            transferOwnershipPermissionlessCall,
            N,
        > {
            self.call_builder(
                &transferOwnershipPermissionlessCall {
                    newOwner,
                },
            )
        }
        ///Creates a new call builder for the [`transferRestrictionsDisabledAfter`] function.
        pub fn transferRestrictionsDisabledAfter(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            transferRestrictionsDisabledAfterCall,
            N,
        > {
            self.call_builder(
                &transferRestrictionsDisabledAfterCall {
                },
            )
        }
        ///Creates a new call builder for the [`unwrap`] function.
        pub fn unwrap(
            &self,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, unwrapCall, N> {
            self.call_builder(&unwrapCall { amount })
        }
        ///Creates a new call builder for the [`wrap`] function.
        pub fn wrap(
            &self,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, wrapCall, N> {
            self.call_builder(&wrapCall { amount })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EigenHarnessInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Approval`] event.
        pub fn Approval_filter(&self) -> alloy_contract::Event<T, &P, Approval, N> {
            self.event_filter::<Approval>()
        }
        ///Creates a new event filter for the [`DelegateChanged`] event.
        pub fn DelegateChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DelegateChanged, N> {
            self.event_filter::<DelegateChanged>()
        }
        ///Creates a new event filter for the [`DelegateVotesChanged`] event.
        pub fn DelegateVotesChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DelegateVotesChanged, N> {
            self.event_filter::<DelegateVotesChanged>()
        }
        ///Creates a new event filter for the [`EIP712DomainChanged`] event.
        pub fn EIP712DomainChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EIP712DomainChanged, N> {
            self.event_filter::<EIP712DomainChanged>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`Mint`] event.
        pub fn Mint_filter(&self) -> alloy_contract::Event<T, &P, Mint, N> {
            self.event_filter::<Mint>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`SetAllowedFrom`] event.
        pub fn SetAllowedFrom_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SetAllowedFrom, N> {
            self.event_filter::<SetAllowedFrom>()
        }
        ///Creates a new event filter for the [`SetAllowedTo`] event.
        pub fn SetAllowedTo_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SetAllowedTo, N> {
            self.event_filter::<SetAllowedTo>()
        }
        ///Creates a new event filter for the [`Transfer`] event.
        pub fn Transfer_filter(&self) -> alloy_contract::Event<T, &P, Transfer, N> {
            self.event_filter::<Transfer>()
        }
        ///Creates a new event filter for the [`TransferRestrictionsDisabled`] event.
        pub fn TransferRestrictionsDisabled_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TransferRestrictionsDisabled, N> {
            self.event_filter::<TransferRestrictionsDisabled>()
        }
    }
}
