use eigensdk_client_avsregistry::reader::AvsRegistryChainReader;
use eigensdk_client_elcontracts::reader::ELChainReader;
use ethers_core::types::{Address, U256};
use std::collections::HashMap;

pub struct Collector {
    elreader: ELChainReader,
    avs_registry_reader: AvsRegistryChainReader,
    operator_addr: Address,
    operator_id: [u8; 32],
    quorum_names: HashMap<u8, String>,
}

impl Collector {
    pub fn new(
        elreader: ELChainReader,
        avs_registry_reader: AvsRegistryChainReader,
        operator_addr: Address,
        operator_id: [u8; 32],
        quorum_names: HashMap<u8, String>,
    ) -> Self {
        Self {
            elreader,
            avs_registry_reader,
            operator_addr,
            operator_id,
            quorum_names,
        }
    }

    pub fn describe(&self) {}

    pub fn init_operator_id(&self) {}

    pub fn collect(&self) {}
}
