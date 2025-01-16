/**

Generated by the following Solidity interface...
```solidity
interface ECDSAUpgradeable {}
```

...which was generated by the following JSON ABI:
```json
[]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod ECDSAUpgradeable {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
<<<<<<< HEAD:crates/utils/src/ecdsaupgradeable.rs
    ///0x6080806040523460175760399081601c823930815050f35b5f80fdfe5f80fdfea26469706673582212208f092c703b8d0e1953682404fcdc306910c631bb8e62c81304ce3e5fbf69520264736f6c634300081b0033
=======
    ///0x60566037600b82828239805160001a607314602a57634e487b7160e01b600052600060045260246000fd5b30600052607381538281f3fe73000000000000000000000000000000000000000030146080604052600080fdfea26469706673582212201c13cffea709e4fa993a49bb9b0cd01c331e918403ba59687963350ea73f47d764736f6c634300080c0033
>>>>>>> dev:crates/utils/src/middleware/ecdsaupgradeable.rs
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
<<<<<<< HEAD:crates/utils/src/ecdsaupgradeable.rs
        b"`\x80\x80`@R4`\x17W`9\x90\x81`\x1C\x8290\x81PP\xF3[_\x80\xFD\xFE_\x80\xFD\xFE\xA2dipfsX\"\x12 \x8F\t,p;\x8D\x0E\x19Sh$\x04\xFC\xDC0i\x10\xC61\xBB\x8Eb\xC8\x13\x04\xCE>_\xBFiR\x02dsolcC\0\x08\x1B\x003",
=======
        b"`V`7`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`*WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x1C\x13\xCF\xFE\xA7\t\xE4\xFA\x99:I\xBB\x9B\x0C\xD0\x1C3\x1E\x91\x84\x03\xBAYhyc5\x0E\xA7?G\xD7dsolcC\0\x08\x0C\x003",
>>>>>>> dev:crates/utils/src/middleware/ecdsaupgradeable.rs
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
<<<<<<< HEAD:crates/utils/src/ecdsaupgradeable.rs
    ///0x5f80fdfea26469706673582212208f092c703b8d0e1953682404fcdc306910c631bb8e62c81304ce3e5fbf69520264736f6c634300081b0033
=======
    ///0x73000000000000000000000000000000000000000030146080604052600080fdfea26469706673582212201c13cffea709e4fa993a49bb9b0cd01c331e918403ba59687963350ea73f47d764736f6c634300080c0033
>>>>>>> dev:crates/utils/src/middleware/ecdsaupgradeable.rs
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
<<<<<<< HEAD:crates/utils/src/ecdsaupgradeable.rs
        b"_\x80\xFD\xFE\xA2dipfsX\"\x12 \x8F\t,p;\x8D\x0E\x19Sh$\x04\xFC\xDC0i\x10\xC61\xBB\x8Eb\xC8\x13\x04\xCE>_\xBFiR\x02dsolcC\0\x08\x1B\x003",
=======
        b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x1C\x13\xCF\xFE\xA7\t\xE4\xFA\x99:I\xBB\x9B\x0C\xD0\x1C3\x1E\x91\x84\x03\xBAYhyc5\x0E\xA7?G\xD7dsolcC\0\x08\x0C\x003",
>>>>>>> dev:crates/utils/src/middleware/ecdsaupgradeable.rs
    );
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ECDSAUpgradeable`](self) contract instance.

    See the [wrapper's documentation](`ECDSAUpgradeableInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ECDSAUpgradeableInstance<T, P, N> {
        ECDSAUpgradeableInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<ECDSAUpgradeableInstance<T, P, N>>>
    {
        ECDSAUpgradeableInstance::<T, P, N>::deploy(provider)
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
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        ECDSAUpgradeableInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`ECDSAUpgradeable`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ECDSAUpgradeable`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ECDSAUpgradeableInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ECDSAUpgradeableInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ECDSAUpgradeableInstance")
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
        > ECDSAUpgradeableInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ECDSAUpgradeable`](self) contract instance.

        See the [wrapper's documentation](`ECDSAUpgradeableInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ECDSAUpgradeableInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> ECDSAUpgradeableInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ECDSAUpgradeableInstance<T, P, N> {
            ECDSAUpgradeableInstance {
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
        > ECDSAUpgradeableInstance<T, P, N>
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
        > ECDSAUpgradeableInstance<T, P, N>
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
