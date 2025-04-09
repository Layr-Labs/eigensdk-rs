use alloy::{
    contract::ContractInstance,
    network::Ethereum,
    primitives::Address,
    providers::{Identity, Provider, ProviderBuilder},
};

pub trait TaskGenerator {
    fn generate_task<P>(&self, task_manager_contract: &ContractInstance<P, Ethereum>);

    fn build_task_manager<P>(&self, provider: P, address: Address) -> ContractInstance<P, Ethereum>
    where
        P: Clone + Provider<Ethereum>;
}
