use crate::error::ElContractsError;
use crate::reader::ELChainReader;
use alloy_primitives::{ruint::aliases::U256, Address, Bytes, FixedBytes, TxHash};
use alloy_sol_types::SolValue;
use eigen_crypto_bls::{
    alloy_g1_point_to_g1_affine, convert_to_g1_point, convert_to_g2_point, BlsKeyPair,
};
pub use eigen_types::operator::Operator;
use eigen_utils::{
    allocationmanager::{AllocationManager, IAllocationManagerTypes},
    delegationmanager::DelegationManager,
    erc20::ERC20,
    get_signer,
    irewardscoordinator::{
        IRewardsCoordinator,
        IRewardsCoordinatorTypes::{self, RewardsMerkleClaim},
    },
    permissioncontroller::PermissionController,
    registrycoordinator::{IBLSApkRegistry::PubkeyRegistrationParams, RegistryCoordinator},
    strategymanager::StrategyManager,
};
use tracing::info;

/// Gas limit for registerAsOperator in [`DelegationManager`]
pub const GAS_LIMIT_REGISTER_AS_OPERATOR_DELEGATION_MANAGER: u128 = 300000;

/// Chain Writer to interact with EigenLayer contracts onchain
#[derive(Debug, Clone)]
pub struct ELChainWriter {
    delegation_manager: Address,
    strategy_manager: Address,
    rewards_coordinator: Address,
    permission_controller: Address,
    allocation_manager: Address,
    registry_coordinator: Address,
    el_chain_reader: ELChainReader,
    provider: String,
    signer: String,
}

impl ELChainWriter {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        delegation_manager: Address,
        strategy_manager: Address,
        rewards_coordinator: Address,
        permission_controller: Address,
        allocation_manager: Address,
        registry_coordinator: Address,
        el_chain_reader: ELChainReader,
        provider: String,
        signer: String,
    ) -> Self {
        Self {
            delegation_manager,
            strategy_manager,
            rewards_coordinator,
            permission_controller,
            allocation_manager,
            registry_coordinator,
            el_chain_reader,
            provider,
            signer,
        }
    }

    /// Register an operator to EigenLayer, and wait for the transaction to be mined.
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator to register
    ///
    /// # Returns
    ///
    /// * `FixedBytes<32>` - The transaction hash if successful, otherwise an error
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn register_as_operator(
        &self,
        operator: Operator,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        info!("registering operator {:?} to EigenLayer", operator.address);
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let binding = {
            let contract_call = contract_delegation_manager.registerAsOperator(
                operator.address,
                operator.allocation_delay,
                operator.metadata_url.unwrap_or_default(),
            );
            contract_call.gas(300000)
        };

        let binding_tx = binding
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let receipt = binding_tx
            .get_receipt()
            .await
            .map_err(ElContractsError::AlloyPendingTransactionError)?;

        let tx_status = receipt.status();
        let hash = receipt.transaction_hash;
        if tx_status {
            info!(tx_hash = %receipt.transaction_hash, "tx successfully included");
        } else {
            info!(tx_hash = %receipt.transaction_hash, "failed to register operator");
        };
        Ok(hash)
    }

    /// Update operator details on EigenLayer
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator to update
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash if successful, otherwise an error
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn update_operator_details(
        &self,
        operator: Operator,
    ) -> Result<TxHash, ElContractsError> {
        info!(
            "updating operator detils of operator {:?} to EigenLayer",
            operator.address
        );

        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let contract_call_modify_operator_details = contract_delegation_manager
            .modifyOperatorDetails(operator.address, operator.delegation_approver_address);

        let modify_operator_tx = contract_call_modify_operator_details
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        info!(tx_hash = %modify_operator_tx.tx_hash(), operator = %operator.address, "updated operator details tx");

        let contract_call_update_metadata_uri = contract_delegation_manager
            .updateOperatorMetadataURI(operator.address, operator.metadata_url.unwrap_or_default());

        let metadata_tx = contract_call_update_metadata_uri.send().await?;

        Ok(*metadata_tx.tx_hash())
    }

    /// Deposit ERC20 tokens into a strategy on EigenLayer
    ///
    /// # Arguments
    ///
    /// * `strategy_addr` - The address of the strategy to deposit into
    /// * `amount` - The amount of tokens to deposit
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash if successful, otherwise an error
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn deposit_erc20_into_strategy(
        &self,
        strategy_addr: Address,
        amount: U256,
    ) -> Result<TxHash, ElContractsError> {
        info!("depositing {amount:?} tokens into strategy {strategy_addr:?}");
        let (_strategy, token_address) = self
            .el_chain_reader
            .get_strategy_and_underlying_token(strategy_addr)
            .await?;
        let provider = get_signer(&self.signer.clone(), &self.provider);
        let token_contract = ERC20::new(token_address, &provider);

        let contract_call = token_contract.approve(self.strategy_manager, amount);

        let _approve = contract_call.send().await?;

        let contract_strategy_manager = StrategyManager::new(self.strategy_manager, &provider);

        let deposit_contract_call =
            contract_strategy_manager.depositIntoStrategy(strategy_addr, token_address, amount);

        let tx = deposit_contract_call.send().await?;

        info!("deposited {amount:?} tokens into strategy {strategy_addr:?}");
        Ok(*tx.tx_hash())
    }

    /// Set a claimer for a given address on EigenLayer
    ///
    /// # Arguments
    ///
    /// * `claimer` - The address to set as the claimer
    ///
    /// # Returns
    ///
    /// * `FixedBytes<32>` - The transaction hash if the operation is sent, otherwise an error
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn set_claimer_for(
        &self,
        claimer: Address,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let set_claimer_for_call = contract_rewards_coordinator.setClaimerFor_0(claimer);

        let tx = set_claimer_for_call.send().await?;
        Ok(*tx.tx_hash())
    }

    /// Process a claim for rewards for a given earner address. Checks the claim against a given root
    /// (determined by the root_index on the claim). Earnings are cumulative so earners can claim to
    /// the latest distribution root and the contract will compute the difference between their earning
    /// and claimed amounts. The difference is transferred to the earner address.
    /// If a claimer has not been set (see [`set_claimer_for`]), only the earner can claim. Otherwise, only
    /// the claimer can claim.
    ///
    /// # Arguments
    ///
    /// * `earnerAddress` - The address of the earner for whom to process the claim.
    /// * `claim` - The RewardsMerkleClaim object containing the claim.
    ///
    /// # Returns
    ///
    /// * `Result<FixedBytes<32>, ElContractsError>` - The transaction hash if the claim is sent, otherwise an error.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails. Also fails if no root has been submitted yet.
    pub async fn process_claim(
        &self,
        earner_address: Address,
        claim: RewardsMerkleClaim,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let process_claim_call = contract_rewards_coordinator.processClaim(claim, earner_address);

        let tx = process_claim_call.send().await?;
        Ok(*tx.tx_hash())
    }

    /// Get the latest claimable distribution root.
    ///
    /// # Returns
    /// * `Result<DistributionRoot, ElContractsError>` - The latest claimable distribution root if the call is successful.
    ///
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn get_current_claimable_distribution_root(
        &self,
    ) -> Result<IRewardsCoordinatorTypes::DistributionRoot, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let cumulative_claimed_for_root_call = contract_rewards_coordinator
            .getCurrentClaimableDistributionRoot()
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IRewardsCoordinator::getCurrentClaimableDistributionRootReturn {
            _0: cumulative_claimed_for_root_ret,
        } = cumulative_claimed_for_root_call;

        Ok(cumulative_claimed_for_root_ret)
    }

    /// Get the length of the distribution roots.
    ///
    /// # Returns
    ///
    /// * `Result<U256, ElContractsError>` - The length of the distribution roots if the call is successful,
    ///   otherwise an error.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn get_distribution_roots_length(&self) -> Result<U256, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let distribution_roots_lenght_call = contract_rewards_coordinator
            .getDistributionRootsLength()
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IRewardsCoordinator::getDistributionRootsLengthReturn {
            _0: distribution_roots_length,
        } = distribution_roots_lenght_call;

        Ok(distribution_roots_length)
    }

    /// Get the current rewards calculation end timestamp (the timestamp until which rewards have been calculated).
    ///
    /// # Returns
    ///
    /// * `Result<u32, ElContractsError>` - The current rewards calculation
    ///   end timestamp if the call is successful.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn get_curr_rewards_calculation_end_timestamp(
        &self,
    ) -> Result<u32, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let distribution_roots_lenght_call = contract_rewards_coordinator
            .currRewardsCalculationEndTimestamp()
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IRewardsCoordinator::currRewardsCalculationEndTimestampReturn {
            _0: timestamp_return,
        } = distribution_roots_lenght_call;

        Ok(timestamp_return)
    }

    /// Get the root index from a given hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash to get the root index from.
    ///
    /// # Returns
    ///
    /// * `Result<u32, ElContractsError>` - The root index if the
    ///   call is successful.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn get_root_index_from_hash(
        &self,
        hash: FixedBytes<32>,
    ) -> Result<u32, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let get_root_index_from_hash_call = contract_rewards_coordinator
            .getRootIndexFromHash(hash)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IRewardsCoordinator::getRootIndexFromHashReturn { _0: root_index } =
            get_root_index_from_hash_call;

        Ok(root_index)
    }

    /// Check if a claim would currently pass the validations in `process_claim`
    ///
    /// # Arguments
    ///
    /// * `claim` - The claim to check
    ///
    /// # Returns
    ///
    /// * `Result<bool, ElContractsError>` - True if the claim would pass the validations, false otherwise
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails. Also fails if no root has been submitted yet.
    pub async fn check_claim(&self, claim: RewardsMerkleClaim) -> Result<bool, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let check_claim_call = contract_rewards_coordinator
            .checkClaim(claim)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IRewardsCoordinator::checkClaimReturn { _0: claim_ret } = check_claim_call;

        Ok(claim_ret)
    }

    /// Get the cumulative claimed amount for a given earner address and token.
    ///
    /// # Arguments
    ///
    /// * `earner_address` - The address of the earner.
    /// * `token` - The address of the token.
    ///
    /// # Returns
    ///
    /// * `Result<U256, ElContractsError>` - The cumulative claimed amount if the call is successful.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn get_cumulative_claimed(
        &self,
        earner_address: Address,
        token: Address,
    ) -> Result<U256, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let cumulative_claimed_call = contract_rewards_coordinator
            .cumulativeClaimed(earner_address, token)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IRewardsCoordinator::cumulativeClaimedReturn {
            _0: cumulative_claim_ret,
        } = cumulative_claimed_call;

        Ok(cumulative_claim_ret)
    }

    /// Removes permission of an appointee on a target contract, given an account address.
    ///
    /// # Arguments
    ///
    /// * `account_address` - account address from which to remove permission
    /// * `appointee_address` - Address to remove
    /// * `target` - contract address that the appointee has permission to
    /// * `selector` - The selector of the function to remove permissions for
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the generated transaction.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn remove_permission(
        &self,
        account_address: Address,
        appointee_address: Address,
        target: Address,
        selector: FixedBytes<4>,
    ) -> Result<TxHash, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let permission_controller_contract =
            PermissionController::new(self.permission_controller, provider);

        let tx = permission_controller_contract
            .removeAppointee(account_address, appointee_address, target, selector)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
    }

    /// Set an appointee for a given account. Only the admin of the account can set an appointee.
    /// The appointee will be able to call the target contract function with the given selector.
    /// # Arguments
    /// * `account_address` - account address set appointee for
    /// * `appointee_address` - appointee address to set
    /// * `target` - target contract address
    /// * `selector` - function selector
    /// # Returns
    /// * `TxHash` - The transaction hash of the generated transaction.
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn set_permission(
        &self,
        account_address: Address,
        appointee_address: Address,
        target: Address,
        selector: FixedBytes<4>,
    ) -> Result<TxHash, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let permission_controller_contract =
            PermissionController::new(self.permission_controller, provider);

        let tx = permission_controller_contract
            .setAppointee(account_address, appointee_address, target, selector)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
    }

    /// Remove pending admin. Only the admin of the account can remove a pending admin
    ///
    /// # Arguments
    ///
    /// * `account_address` - account address
    /// * `admin_address` - admin address to remove
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the generated transaction.
    pub async fn remove_pending_admin(
        &self,
        account_address: Address,
        admin_address: Address,
    ) -> Result<TxHash, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let permission_controller_contract =
            PermissionController::new(self.permission_controller, provider);

        let tx = permission_controller_contract
            .removePendingAdmin(account_address, admin_address)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
    }

    /// Set a pending admin. Multiple admins can be set for an account.
    /// The caller must be an admin. If the account does not have an admin, the caller must be the account.
    ///
    /// # Arguments
    ///
    /// * `account_address` - account address
    /// * `admin_address` - admin address to set
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the generated transaction.
    pub async fn add_pending_admin(
        &self,
        account_address: Address,
        admin_address: Address,
    ) -> Result<TxHash, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let permission_controller_contract =
            PermissionController::new(self.permission_controller, provider);

        let tx = permission_controller_contract
            .addPendingAdmin(account_address, admin_address)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
    }

    /// Accept a pending admin. The sender of the transaction must be the pending admin.
    ///
    /// # Arguments
    ///
    /// * `account` - account to accept admin for
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the generated transaction.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn accept_admin(&self, account: Address) -> Result<TxHash, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let permission_controller_contract =
            PermissionController::new(self.permission_controller, provider);

        let tx = permission_controller_contract
            .acceptAdmin(account)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
    }

    /// Remove an admin. The sender of the transaction must be an admin.
    ///
    /// # Arguments
    ///
    /// * `account` - account to remove admin from
    /// * `admin` - admin to remove
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the generated transaction.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails. Fails if the admin being removed is the only admin.
    pub async fn remove_admin(
        &self,
        account: Address,
        admin: Address,
    ) -> Result<TxHash, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let permission_controller_contract =
            PermissionController::new(self.permission_controller, provider);

        let tx = permission_controller_contract
            .removeAdmin(account, admin)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
    }

    /// Register an operator for one or more operator sets for an AVS. If the operator
    /// has any stake allocated to these operator sets, it immediately becomes slashable.
    ///
    /// # Arguments
    ///
    /// * `operator_address` - operator address to register
    /// * `avs_address` - AVS address
    /// * `operator_set_ids` - operator set ids to register on
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the generated transaction.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn register_for_operator_sets(
        &self,
        operator_address: Address,
        avs_address: Address,
        operator_set_ids: Vec<u32>,
        bls_key_pair: BlsKeyPair,
        socket: &str,
    ) -> Result<TxHash, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);
        let allocation_manager_contract =
            AllocationManager::new(self.allocation_manager, provider.clone());
        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator, provider);

        let g1_hashed_msg_to_sign = contract_registry_coordinator
            .pubkeyRegistrationMessageHash(operator_address)
            .call()
            .await?
            ._0;

        let sig = bls_key_pair
            .sign_hashed_to_curve_message(alloy_g1_point_to_g1_affine(g1_hashed_msg_to_sign))
            .g1_point();
        let alloy_g1_point_signed_msg =
            convert_to_g1_point(sig.g1()).map_err(|_| ElContractsError::BLSKeyPairInvalid)?;
        let g1_pub_key_bn254 = convert_to_g1_point(bls_key_pair.public_key().g1())
            .map_err(|_| ElContractsError::BLSKeyPairInvalid)?;
        let g2_pub_key_bn254 = convert_to_g2_point(bls_key_pair.public_key_g2().g2())
            .map_err(|_| ElContractsError::BLSKeyPairInvalid)?;

        let params = PubkeyRegistrationParams {
            pubkeyRegistrationSignature: alloy_g1_point_signed_msg,
            pubkeyG1: g1_pub_key_bn254,
            pubkeyG2: g2_pub_key_bn254,
        };

        let mut data: Bytes = (socket, params).abi_encode().into();

        // The encoder is prepending 32 bytes to the data as if it was used in a dynamic function parameter.
        // This is not used when decoding the bytes directly, so we need to remove it.
        data = data.slice(32..);

        let params = IAllocationManagerTypes::RegisterParams {
            avs: avs_address,
            operatorSetIds: operator_set_ids,
            data,
        };
        let tx = allocation_manager_contract
            .registerForOperatorSets(operator_address, params)
            .send()
            .await?;

        Ok(*tx.tx_hash())
    }

    /// Deregister an operator from one or more of the AVS's operator sets. If the operator
    /// has any slashable stake allocated to the AVS, it remains slashable until the deallocation delay has passed.
    ///
    /// # Arguments
    ///
    /// * `operator_address` - operator address to deregister
    /// * `avs_address` - AVS address
    /// * `operator_set_ids` - operator set ids to deregister from
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the generated transaction.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn deregister_from_operator_sets(
        &self,
        operator_address: Address,
        avs_address: Address,
        operator_set_ids: Vec<u32>,
    ) -> Result<TxHash, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);
        let allocation_manager_contract = AllocationManager::new(self.allocation_manager, provider);

        let params = IAllocationManagerTypes::DeregisterParams {
            operator: operator_address,
            avs: avs_address,
            operatorSetIds: operator_set_ids,
        };
        let tx = allocation_manager_contract
            .deregisterFromOperatorSets(params)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
    }

    /// Set the allocation delay for an operator. It is the number of blocks between an operator
    /// allocating magnitude to an operator set, and the magnitude becoming slashable
    ///
    /// # Arguments
    ///
    /// * `operator_address` - operator address to set allocation delay for
    /// * `delay` - delay in blocks
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the generated transaction.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn set_allocation_delay(
        &self,
        operator_address: Address,
        delay: u32,
    ) -> Result<TxHash, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);
        let allocation_manager_contract = AllocationManager::new(self.allocation_manager, provider);

        let tx = allocation_manager_contract
            .setAllocationDelay(operator_address, delay)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
    }

    /// Modifiy the proportions of slashable stake allocated to an operator set from a list of strategies.
    /// # Arguments
    /// * `operator_address` - operator address to modify allocations for
    /// * `allocations` - list of magnitude adjustments for one or more operator sets
    /// # Returns
    /// * `TxHash` - The transaction hash of the generated transaction.
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn modify_allocations(
        &self,
        operator_address: Address,
        allocations: Vec<IAllocationManagerTypes::AllocateParams>,
    ) -> Result<TxHash, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);
        let allocation_manager_contract = AllocationManager::new(self.allocation_manager, provider);

        let tx = allocation_manager_contract
            .modifyAllocations(operator_address, allocations)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
    }
}

#[cfg(test)]
mod tests {
    use super::ELChainWriter;
    use crate::reader::ELChainReader;
    use alloy::{hex::FromHex, providers::Provider};
    use alloy_primitives::{address, aliases::U96, Address, Bytes, FixedBytes, U256};
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::{
        anvil::{mine_anvil_blocks, set_account_balance, start_anvil_container},
        anvil_constants::{
            get_allocation_manager_address, get_avs_directory_address,
            get_delegation_manager_address, get_erc20_mock_strategy,
            get_registry_coordinator_address, get_rewards_coordinator_address,
            get_strategy_manager_address,
        },
        transaction::wait_transaction,
    };
    use eigen_types::operator::Operator;
    use eigen_utils::{
        allocationmanager::{
            AllocationManager::{self, OperatorSet},
            IAllocationManagerTypes,
        },
        delegationmanager::DelegationManager,
        get_provider, get_signer,
        irewardscoordinator::{
            IRewardsCoordinator,
            IRewardsCoordinatorTypes::{
                EarnerTreeMerkleLeaf, RewardsMerkleClaim, TokenTreeMerkleLeaf,
            },
        },
        registrycoordinator::{
            IRegistryCoordinator::OperatorSetParam, IStakeRegistry::StrategyParams,
            RegistryCoordinator,
        },
    };
    use std::str::FromStr;

    const OPERATOR_ADDRESS: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    const OPERATOR_PRIVATE_KEY: &str =
        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    /// Returns a new instance of ELChainWriter
    async fn setup_el_chain_reader(http_endpoint: String) -> ELChainReader {
        let delegation_manager_address =
            get_delegation_manager_address(http_endpoint.clone()).await;
        let avs_directory_address = get_avs_directory_address(http_endpoint.clone()).await;

        ELChainReader::build(
            get_test_logger().clone(),
            delegation_manager_address,
            avs_directory_address,
            &http_endpoint,
        )
        .await
        .unwrap()
    }

    async fn new_test_writer(http_endpoint: String, private_key: String) -> ELChainWriter {
        let el_chain_reader = setup_el_chain_reader(http_endpoint.clone()).await;
        let strategy_manager = get_strategy_manager_address(http_endpoint.clone()).await;
        let rewards_coordinator = get_rewards_coordinator_address(http_endpoint.clone()).await;
        let delegation_manager = get_delegation_manager_address(http_endpoint.clone()).await;
        let allocation_manager = get_allocation_manager_address(http_endpoint.clone()).await;
        let contract_delegation_manager =
            DelegationManager::new(delegation_manager, get_provider(&http_endpoint));
        let permission_controller = contract_delegation_manager
            .permissionController()
            .call()
            .await
            .unwrap()
            ._0;
        let registry_coordinator = get_registry_coordinator_address(http_endpoint.clone()).await;

        ELChainWriter::new(
            delegation_manager,
            strategy_manager,
            rewards_coordinator,
            permission_controller,
            allocation_manager,
            registry_coordinator,
            el_chain_reader,
            http_endpoint.clone(),
            private_key,
        )
    }

    async fn new_claim(http_endpoint: &str) -> (FixedBytes<32>, RewardsMerkleClaim) {
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        let earner_address = address!("25a1b7322f9796b26a4bec125913b34c292b28d6");
        let claim = RewardsMerkleClaim {
            rootIndex: 0,
            earnerIndex: 7,
            earnerTreeProof: Bytes::from_hex("4bf5e16eaabbc36964f1e1639808669420f55d60e51adb7e9695b77145c479fd6777be59643947bb24d78e69d6605bf369c515b479f3a8967dd68a97c5bb4a4a262b28002eeb6cbbffb7e79e5741bf2be189a6073440a62fabcd8af4dbda94e3").unwrap(),
            earnerLeaf: EarnerTreeMerkleLeaf {
                earner: earner_address,
                earnerTokenRoot: FixedBytes::from_hex(
                    "f8e7e20b32aae1d818dcb593b98982841e9a0ed12c161ad603e3ee3948746cba",
                )
                .unwrap(),
            },
            tokenIndices: vec![7],
            tokenTreeProofs: vec![
                Bytes::from_hex("3cd04e8fc6f23812c570fe12292a30bb9e105e00f5913ac4b4938f23e65d8d10e6b1403d58c9d5450952e7d96c81305dad9fb966e8a27d3a42058e3958a0d30033148e91b455542d05deb81b8305b672e742cd3145f7022a0089bad2e6af9173").unwrap(),
            ],
            tokenLeaves: vec![TokenTreeMerkleLeaf {
                token: address!("7fbfdd1dfd80730385aee232cc9f79b8ae12a654"),
                cumulativeEarnings: U256::from_str("3000000000000000000").unwrap(),
            }],
        };

        // Using test data taken from
        // https://github.com/Layr-Labs/eigenlayer-contracts/blob/a888a1cd1479438dda4b138245a69177b125a973/src/test/test-data/rewardsCoordinator/processClaimProofs_MaxEarnerAndLeafIndices.json
        let root = FixedBytes::from_hex(
            "37550707c80f3d8907c467999730e52127ab89be3f17a5017a3f1ffb73a1445f",
        )
        .unwrap();

        let key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let rewards_coordinator = IRewardsCoordinator::new(
            get_rewards_coordinator_address(http_endpoint.to_string()).await,
            get_signer(key, http_endpoint),
        );
        let curr_rewards_calculation_end_timestamp = el_chain_writer
            .get_curr_rewards_calculation_end_timestamp()
            .await
            .unwrap();
        let submit_tx = rewards_coordinator
            .submitRoot(root, curr_rewards_calculation_end_timestamp + 1)
            .send()
            .await
            .unwrap();
        let submit_status = submit_tx.get_receipt().await.unwrap().status();
        assert!(submit_status);

        (root, claim)
    }

    #[tokio::test]
    async fn test_register_operator() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_reader = setup_el_chain_reader(http_endpoint.clone()).await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        let operator = Operator {
            address: OPERATOR_ADDRESS, // can only register the address corresponding to the signer used in the writer
            staker_opt_out_window_blocks: 3,
            delegation_approver_address: OPERATOR_ADDRESS,
            metadata_url: Some("metadata_uri".to_string()),
            allocation_delay: 1,
        };
        el_chain_writer
            .register_as_operator(operator)
            .await
            .unwrap();

        let is_registered = el_chain_reader
            .is_operator_registered(OPERATOR_ADDRESS)
            .await
            .unwrap();
        assert!(is_registered);
    }

    #[tokio::test]
    async fn test_register_and_update_operator() {
        let (container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let provider = get_provider(&http_endpoint);

        let address_str = "009440d62dc85c73dbf889b7ad1f4da8b231d2ef";
        let private_key = "6b35c6d8110c888de06575b45181bf3f9e6c73451fa5cde812c95a6b31e66ddf";
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), private_key.to_string()).await;

        set_account_balance(&container, address_str).await;
        let address = Address::from_str(address_str).unwrap();

        let operator = Operator {
            address,
            staker_opt_out_window_blocks: 3,
            delegation_approver_address: Address::ZERO,
            metadata_url: Some("eigensdk-rs".to_string()),
            allocation_delay: 1,
        };

        // First test: register as an operator
        let tx_hash = el_chain_writer
            .register_as_operator(operator)
            .await
            .unwrap();

        let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap();
        assert!(receipt.unwrap().status());

        let operator_modified = Operator {
            address,
            staker_opt_out_window_blocks: 3,
            delegation_approver_address: Address::ZERO,
            metadata_url: Some("new-metadata".to_string()),
            allocation_delay: 1,
        };

        // Second test: update operator details
        let tx_hash = el_chain_writer
            .update_operator_details(operator_modified)
            .await
            .unwrap();
        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }

    #[tokio::test]
    async fn test_deposit_erc20_into_strategy() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        let amount = U256::from_str("100").unwrap();
        let strategy_addr = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let tx_hash = el_chain_writer
            .deposit_erc20_into_strategy(strategy_addr, amount)
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }

    #[tokio::test]
    async fn test_set_claimer_for() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        let claimer = address!("5eb15C0992734B5e77c888D713b4FC67b3D679A2");

        let tx_hash = el_chain_writer.set_claimer_for(claimer).await.unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }

    #[tokio::test]
    async fn test_check_claim() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        let (_, claim) = new_claim(&http_endpoint).await;

        let distribution_roots_length = el_chain_writer
            .get_distribution_roots_length()
            .await
            .unwrap();
        assert_eq!(distribution_roots_length, U256::from(1));

        let valid_claim = el_chain_writer.check_claim(claim.clone()).await.unwrap();
        assert!(valid_claim);
    }

    #[tokio::test]
    async fn test_get_distribution_roots_length() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;
        new_claim(&http_endpoint).await;

        let distribution_roots_length_ret = el_chain_writer
            .get_distribution_roots_length()
            .await
            .unwrap();

        assert_eq!(distribution_roots_length_ret, U256::from(1));
    }

    #[tokio::test]
    async fn test_get_root_index_from_hash() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;
        let (root, _) = new_claim(&http_endpoint).await;

        let index = el_chain_writer
            .get_root_index_from_hash(root)
            .await
            .unwrap();

        assert_eq!(index, 0);
    }

    #[tokio::test]
    async fn test_get_cumulative_claimed() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        let earner_address = address!("F2288D736d27C1584Ebf7be5f52f9E4d47251AeE");
        let (_, _, token_address) = el_chain_writer
            .el_chain_reader
            .get_strategy_and_underlying_erc20_token(
                get_erc20_mock_strategy(http_endpoint.clone()).await,
            )
            .await
            .unwrap();

        let cumulative_claimed_ret = el_chain_writer
            .get_cumulative_claimed(earner_address, token_address)
            .await
            .unwrap();

        // No claims so cumulative claimed should be zero
        assert_eq!(cumulative_claimed_ret, U256::from(0));
    }

    #[tokio::test]
    async fn test_get_cumulative_claimed_for_root() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;
        let (root, _) = new_claim(&http_endpoint).await;

        let distribution_root = el_chain_writer
            .get_current_claimable_distribution_root()
            .await
            .unwrap();

        assert_eq!(distribution_root.root, root);
    }

    #[tokio::test]
    async fn test_add_and_remove_pending_admin() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        let pending_admin = address!("009440d62dc85c73dbf889b7ad1f4da8b231d2ef");
        let tx_hash = el_chain_writer
            .add_pending_admin(OPERATOR_ADDRESS, pending_admin)
            .await
            .unwrap();
        wait_transaction(&http_endpoint, tx_hash).await.unwrap();

        let is_pending_admin = el_chain_writer
            .el_chain_reader
            .is_pending_admin(OPERATOR_ADDRESS, pending_admin)
            .await
            .unwrap();
        assert!(is_pending_admin);

        let tx_hash = el_chain_writer
            .remove_pending_admin(OPERATOR_ADDRESS, pending_admin)
            .await
            .unwrap();
        wait_transaction(&http_endpoint, tx_hash).await.unwrap();

        let is_admin = el_chain_writer
            .el_chain_reader
            .is_pending_admin(OPERATOR_ADDRESS, pending_admin)
            .await
            .unwrap();
        assert!(!is_admin);
    }

    #[tokio::test]
    async fn test_accept_admin() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let account_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        let pending_admin = address!("14dC79964da2C08b23698B3D3cc7Ca32193d9955");
        let pending_admin_key =
            "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";

        account_writer
            .add_pending_admin(OPERATOR_ADDRESS, pending_admin)
            .await
            .unwrap();

        let admin_writer =
            new_test_writer(http_endpoint.to_string(), pending_admin_key.to_string()).await;

        admin_writer.accept_admin(OPERATOR_ADDRESS).await.unwrap();

        let is_admin = admin_writer
            .el_chain_reader
            .is_admin(OPERATOR_ADDRESS, pending_admin)
            .await
            .unwrap();
        assert!(is_admin);
    }

    #[tokio::test]
    async fn test_remove_admin() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        let pending_admin_1 = address!("14dC79964da2C08b23698B3D3cc7Ca32193d9955");
        let pending_admin_1_key =
            "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";

        let pending_admin_2 = address!("23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f");
        let pending_admin_2_key =
            "0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97";

        // Adding two admins and removing one. Cannot remove the last admin, so one must remain
        el_chain_writer
            .add_pending_admin(OPERATOR_ADDRESS, pending_admin_1)
            .await
            .unwrap();
        el_chain_writer
            .add_pending_admin(OPERATOR_ADDRESS, pending_admin_2)
            .await
            .unwrap();

        let admin_1_writer =
            new_test_writer(http_endpoint.to_string(), pending_admin_1_key.to_string()).await;
        admin_1_writer.accept_admin(OPERATOR_ADDRESS).await.unwrap();
        let admin_2_writer =
            new_test_writer(http_endpoint.to_string(), pending_admin_2_key.to_string()).await;
        admin_2_writer.accept_admin(OPERATOR_ADDRESS).await.unwrap();

        admin_1_writer
            .remove_admin(OPERATOR_ADDRESS, pending_admin_2)
            .await
            .unwrap();

        let is_admin = el_chain_writer
            .el_chain_reader
            .is_admin(OPERATOR_ADDRESS, pending_admin_2)
            .await
            .unwrap();
        assert!(!is_admin);
    }

    #[tokio::test]
    async fn test_set_and_remove_permission() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let account_address = OPERATOR_ADDRESS;
        let appointee_address = address!("009440d62dc85c73dbf889b7ad1f4da8b231d2ef");
        let appointee_key = "6b35c6d8110c888de06575b45181bf3f9e6c73451fa5cde812c95a6b31e66ddf";
        let target = address!("14dC79964da2C08b23698B3D3cc7Ca32193d9955");
        let selector = [0, 1, 2, 3].into();

        // add an admin
        let account_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        // set permission
        let tx_hash = account_writer
            .set_permission(account_address, appointee_address, target, selector)
            .await
            .unwrap();
        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        // check if appointee can call the set target
        let appointee_writer =
            new_test_writer(http_endpoint.to_string(), appointee_key.to_string()).await;
        let can_call = appointee_writer
            .el_chain_reader
            .can_call(account_address, appointee_address, target, selector)
            .await
            .unwrap();
        assert!(can_call);

        // test remove permission
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        let tx_hash = el_chain_writer
            .remove_permission(account_address, appointee_address, target, selector)
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }

    async fn create_operator_set(http_endpoint: &str, avs_address: Address, operator_set_id: u32) {
        let allocation_manager_addr =
            get_allocation_manager_address(http_endpoint.to_string()).await;
        let signer = get_signer(OPERATOR_PRIVATE_KEY, http_endpoint);
        let allocation_manager = AllocationManager::new(allocation_manager_addr, signer.clone());
        let registry_coordinator_addr =
            get_registry_coordinator_address(http_endpoint.to_string()).await;

        allocation_manager
            .setAVSRegistrar(avs_address, registry_coordinator_addr)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();

        // Enable operator sets in Registry Coordinator
        let registry_coordinator =
            RegistryCoordinator::new(registry_coordinator_addr, signer.clone());
        registry_coordinator
            .enableOperatorSets()
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();

        // Create slashable quorum
        let contract_registry_coordinator =
            RegistryCoordinator::new(registry_coordinator_addr, signer.clone());
        let operator_set_params = OperatorSetParam {
            maxOperatorCount: 10,
            kickBIPsOfOperatorStake: 100,
            kickBIPsOfTotalStake: 1000,
        };
        let strategy = get_erc20_mock_strategy(http_endpoint.to_string()).await;
        let strategy_params = StrategyParams {
            strategy,
            multiplier: U96::from(1),
        };
        contract_registry_coordinator
            .createSlashableStakeQuorum(operator_set_params, U96::from(0), vec![strategy_params], 0)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();

        // Create operator set
        let params = IAllocationManagerTypes::CreateSetParams {
            operatorSetId: operator_set_id,
            strategies: vec![strategy],
        };
        allocation_manager
            .createOperatorSets(avs_address, vec![params])
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_register_for_operator_sets() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;

        let avs_address = OPERATOR_ADDRESS;
        let operator_set_id = 1;
        create_operator_set(http_endpoint.as_str(), avs_address, operator_set_id).await;

        let operator_addr = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");
        let operator_private_key =
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
        let el_chain_writer =
            new_test_writer(http_endpoint.clone(), operator_private_key.to_string()).await;
        let bls_key = BlsKeyPair::new("1".to_string()).unwrap();

        let tx_hash = el_chain_writer
            .register_for_operator_sets(
                operator_addr,
                avs_address,
                vec![operator_set_id],
                bls_key,
                "socket",
            )
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let operator_set = OperatorSet {
            avs: avs_address,
            id: operator_set_id,
        };
        let is_registered = el_chain_writer
            .el_chain_reader
            .is_operator_registered_with_operator_set(operator_addr, operator_set.clone())
            .await
            .unwrap();
        assert!(is_registered);
    }

    #[tokio::test]
    async fn test_set_allocation_delay() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        let delay = 10;

        let tx_hash = el_chain_writer
            .set_allocation_delay(OPERATOR_ADDRESS, delay)
            .await
            .unwrap();
        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let allocation_delay = el_chain_writer
            .el_chain_reader
            .get_allocation_delay(OPERATOR_ADDRESS)
            .await
            .unwrap();

        assert_eq!(allocation_delay, delay);
    }

    #[tokio::test]
    async fn test_modify_allocations() {
        let (container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), OPERATOR_PRIVATE_KEY.to_string()).await;

        let operator_address = OPERATOR_ADDRESS;
        let strategy_addr = get_erc20_mock_strategy(http_endpoint.clone()).await;

        let avs_address = OPERATOR_ADDRESS;
        let operator_set_id = 1;
        create_operator_set(http_endpoint.as_str(), avs_address, operator_set_id).await;

        let new_allocation = 100;
        let allocate_params = IAllocationManagerTypes::AllocateParams {
            strategies: vec![strategy_addr],
            operatorSet: OperatorSet {
                avs: avs_address,
                id: operator_set_id,
            },
            newMagnitudes: vec![new_allocation],
        };
        let tx_hash = el_chain_writer
            .modify_allocations(operator_address, vec![allocate_params])
            .await
            .unwrap();
        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let allocation_info = el_chain_writer
            .el_chain_reader
            .get_allocation_info(operator_address, strategy_addr)
            .await
            .unwrap();

        // Allocation should be pending
        assert_eq!(allocation_info[0].pending_diff, U256::from(new_allocation));

        let allocation_delay = el_chain_writer
            .el_chain_reader
            .get_allocation_delay(OPERATOR_ADDRESS)
            .await
            .unwrap();
        mine_anvil_blocks(&container, allocation_delay).await;

        let allocation_info = el_chain_writer
            .el_chain_reader
            .get_allocation_info(operator_address, strategy_addr)
            .await
            .unwrap();

        // After the allocation delay blocks, the allocation should be set
        assert_eq!(
            allocation_info[0].current_magnitude,
            U256::from(new_allocation)
        );
    }
}
