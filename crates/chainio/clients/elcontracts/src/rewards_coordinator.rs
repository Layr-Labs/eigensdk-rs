use alloy_primitives::{Address, FixedBytes};
//use alloy_provider::provider::wallet::WalletProvider;
use crate::error::ElContractsError;
use alloy_provider::WalletProvider;
use eigen_logging::logger::SharedLogger;
use eigen_utils::binding::RewardsCoordinator::{self, RewardsMerkleClaim};
use eigen_utils::get_signer;

#[derive(Debug, Clone)]
pub struct ELRewardsCoordinator {
    rewards_coordinator: Address,
    provider: String,
    signer: String,
    logger: SharedLogger,
}

impl ELRewardsCoordinator {
    pub async fn set_claimed_for(
        &self,
        claimer: Address,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let provider = get_signer(self.signer.clone(), &self.provider);

        let contract_rewards_coordinator =
            RewardsCoordinator::new(self.rewards_coordinator, &provider);

        let set_claimer_for_call = contract_rewards_coordinator.setClaimerFor(claimer);

        let receipt = set_claimer_for_call
            .send()
            .await?
            .get_receipt()
            .await
            .map_err(|e| {
                ElContractsError::AlloyContractError(alloy_contract::Error::TransportError(e))
            })?;
        let hash = receipt.transaction_hash;
        let signer_address = provider.default_signer_address();
        if receipt.status() {
            self.logger.info(
                &format!("Successfully set {claimer} as claimer for {signer_address}"),
                "eigen-client-elcontracts.set_claimed_for",
            );
        } else {
            self.logger.info(
                &format!("Failed to set {claimer} as claimer for {signer_address}"),
                "eigen-client-elcontracts.set_claimed_for",
            );
        }
        Ok(hash)
    }

    pub async fn process_claim(
        &self,
        recipient: Address,
        claim: RewardsMerkleClaim,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let provider = get_signer(self.signer.clone(), &self.provider);

        let contract_rewards_coordinator =
            RewardsCoordinator::new(self.rewards_coordinator, &provider);
        let process_claim_call = contract_rewards_coordinator.processClaim(claim, recipient);

        let receipt = process_claim_call
            .send()
            .await?
            .get_receipt()
            .await
            .map_err(|e| {
                ElContractsError::AlloyContractError(alloy_contract::Error::TransportError(e))
            })?;

        let hash = receipt.transaction_hash;
        if receipt.status() {
            self.logger.info(
                &format!("Successfully processed claim for recipient {recipient}"),
                "eigen-client-elcontracts.process_claim",
            );
        } else {
            self.logger.info(
                &format!("Failed to  process claim for recipient {recipient}"),
                "eigen-client-elcontracts.process_claim",
            );
        };
        Ok(hash)
    }
}

/*
    /// TODO(supernova): This method is not availabe in dev branch of eigenlayer-contracts, so skipping this till then
    pub fn force_deregister_from_operator_set(
        &self,
        _operator: Address,
        _avs: Address,
        _operator_set_ids: Vec<u32>,
        _operator_signature: AVSDirectory::SignatureWithSaltAndExpiry,
    ) {
        //     let provider = get_signer(self.signer.clone(), &self.provider);
        //     let contract_avs_directory = AVSDirectory::new(self.el_chain_reader.get_avs_directory_address(),provider);
        //     // contract_avs_directory

        todo!()
    }

    /// TODO(supernova): This method is not available in dev branch of eigenlayer-contracts , so skipping till then
    pub fn set_operator_commission_bips(&self) {
        // let provider = get_signer(self.signer.clone(), &self.provider);

        // let contract_rewards_coordinator =
        //     RewardsCoordinator::new(self.rewards_coordinator, &provider);

        // let tx = contract_rewards_coordinator.setOpe
        todo!()
    }
*/
