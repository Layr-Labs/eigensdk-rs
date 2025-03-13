use crate::error::ElContractsError;
use crate::reader::ELChainReader;
use alloy::dyn_abi::DynSolValue;
use alloy::primitives::{Address, FixedBytes, TxHash, U256};
use alloy::sol;
use eigen_common::get_signer;
use eigen_crypto_bls::{
    alloy_g1_point_to_g1_affine, convert_to_g1_point, convert_to_g2_point, BlsKeyPair,
};
pub use eigen_types::operator::Operator;

use eigen_utils::convert_allocation_operator_set_to_rewards_operator_set;
use eigen_utils::rewardsv2::core::delegationmanager::DelegationManager as RewardsV2DelegationManager;
use eigen_utils::rewardsv2::core::delegationmanager::IDelegationManager::OperatorDetails;
use eigen_utils::slashing::core::allocationmanager::AllocationManager::OperatorSet;
use eigen_utils::{
    slashing::core::{
        allocationmanager::{AllocationManager, IAllocationManagerTypes},
        delegationmanager::DelegationManager,
        irewardscoordinator::{IRewardsCoordinator, IRewardsCoordinatorTypes::RewardsMerkleClaim},
        permissioncontroller::PermissionController,
        strategymanager::StrategyManager,
    },
    slashing::middleware::{ierc20::IERC20, registrycoordinator::RegistryCoordinator},
};
use tracing::info;

/// Gas limit for registerAsOperator in [`DelegationManager`]
pub const GAS_LIMIT_REGISTER_AS_OPERATOR_DELEGATION_MANAGER: u128 = 300000;

sol! {
    #[allow(missing_docs)]
    #[derive(Debug)]
    /// Bar
    enum RegistrationType {
        NORMAL,
        CHURN,
    }
}
/// Chain Writer to interact with EigenLayer contracts onchain
#[derive(Debug, Clone)]
pub struct ELChainWriter {
    strategy_manager: Address,
    rewards_coordinator: Address,
    permission_controller: Option<Address>,
    allocation_manager: Option<Address>,
    registry_coordinator: Address,
    el_chain_reader: ELChainReader,
    provider: String,
    signer: String,
}

impl ELChainWriter {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        strategy_manager: Address,
        rewards_coordinator: Address,
        permission_controller: Option<Address>,
        allocation_manager: Option<Address>,
        registry_coordinator: Address,
        el_chain_reader: ELChainReader,
        provider: String,
        signer: String,
    ) -> Self {
        Self {
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

    /// Sets signer for ELChainWriter
    ///
    /// # Arguments
    ///
    /// * `signer`: signer string
    ///
    pub fn set_signer(&mut self, signer: String) {
        self.signer = signer;
    }

    pub async fn register_as_operator_preslashing(
        &self,
        operator: Operator,
    ) -> Result<TxHash, ElContractsError> {
        info!("registering operator {:?} to EigenLayer", operator.address);
        let provider = get_signer(&self.signer.clone(), &self.provider);

        if let Some(staker_opt_out_blocks) = operator.staker_opt_out_window_blocks {
            let contract_delegation_manager =
                RewardsV2DelegationManager::new(self.el_chain_reader.delegation_manager, provider);
            let operator_details = OperatorDetails {
                __deprecated_earningsReceiver: operator
                    ._deprecated_earnings_receiver_address
                    .unwrap_or(Address::ZERO),
                delegationApprover: operator.delegation_approver_address,
                stakerOptOutWindowBlocks: staker_opt_out_blocks,
            };
            let binding = {
                let contract_call = contract_delegation_manager
                    .registerAsOperator(operator_details, operator.metadata_url);
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
        } else {
            Err(ElContractsError::StakerOptOutWindowBlocksNotSet)
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
    /// * `TxHash` - The transaction hash if successful, otherwise an error
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn register_as_operator(
        &self,
        operator: Operator,
    ) -> Result<TxHash, ElContractsError> {
        info!("registering operator {:?} to EigenLayer", operator.address);
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_delegation_manager =
            DelegationManager::new(self.el_chain_reader.delegation_manager, provider);

        if let Some(allocation_delay) = operator.allocation_delay {
            let binding = {
                let contract_call = contract_delegation_manager.registerAsOperator(
                    operator.address,
                    allocation_delay,
                    operator.metadata_url,
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
        } else {
            Err(ElContractsError::AllocationDelayNotSet)
        }
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

        let contract_delegation_manager =
            DelegationManager::new(self.el_chain_reader.delegation_manager, provider);

        let contract_call_modify_operator_details = contract_delegation_manager
            .modifyOperatorDetails(operator.address, operator.delegation_approver_address);

        let modify_operator_tx = contract_call_modify_operator_details
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        info!(tx_hash = %modify_operator_tx.tx_hash(), operator = %operator.address, "updated operator details tx");

        let contract_call_update_metadata_uri = contract_delegation_manager
            .updateOperatorMetadataURI(operator.address, operator.metadata_url);

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
        let token_contract = IERC20::new(token_address, &provider);

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
    /// * `claim` - The RewardsMerkleClaim object containing the claim.
    /// * `earnerAddress` - The address of the earner for whom to process the claim.
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
        claim: RewardsMerkleClaim,
        earner_address: Address,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let process_claim_call = contract_rewards_coordinator.processClaim(claim, earner_address);

        let tx = process_claim_call.send().await?;
        Ok(*tx.tx_hash())
    }

    /// Process multiple claim for rewards for a given earner address. Checks the claim against a given root
    /// (determined by the root_index on the claim). Earnings are cumulative so earners can claim to
    /// the latest distribution root and the contract will compute the difference between their earning
    /// and claimed amounts. The difference is transferred to the earner address.
    /// If a claimer has not been set (see [`set_claimer_for`]), only the earner can claim. Otherwise, only
    /// the claimer can claim.
    ///
    /// # Arguments
    ///
    /// * `claims` - A [`Vec`] of RewardsMerkleClaim objects containing the claims.
    /// * `earnerAddress` - The address of the earner for whom to process the claims.
    ///
    /// # Returns
    ///
    /// * `Result<FixedBytes<32>, ElContractsError>` - The transaction hash if the claim is sent, otherwise an error.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails. Also fails if no root has been submitted yet.
    pub async fn process_claims(
        &self,
        claims: Vec<RewardsMerkleClaim>,
        earner_address: Address,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let process_claim_call = contract_rewards_coordinator.processClaims(claims, earner_address);

        let tx = process_claim_call.send().await?;
        Ok(*tx.tx_hash())
    }

    /// Sets the split of a specific `operator` for a specific `avs`
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator address
    /// * `avs` - The AVS address
    /// * `split` - The new split of the operator for the AVS
    ///
    /// # Returns
    ///
    /// * `Result<FixedBytes<32>, ElContractsError>` - The transaction hash if the transaction is sent, otherwise an error.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn set_operator_avs_split(
        &self,
        operator: Address,
        avs: Address,
        split: u16,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let signer = get_signer(&self.signer, &self.provider);

        let rewards_coordinator = IRewardsCoordinator::new(self.rewards_coordinator, signer);

        let tx = rewards_coordinator
            .setOperatorAVSSplit(operator, avs, split)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
    }

    /// Sets the split for a specific `operator` for a specific `operatorSet`
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator address
    /// * `OperatorSet` - The operator set which consists of avs address and id.
    /// * `split` - The split for the operator for the specific operatorSet in bips.
    ///
    /// # Returns
    ///
    /// * `Result<FixedBytes<32>, ElContractsError>` - The transaction hash if the transaction is sent, otherwise an error.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn set_operator_set_split(
        &self,
        operator: Address,
        operator_set: OperatorSet,
        split: u16,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let signer = get_signer(&self.signer, &self.provider);

        let rewards_coordinator = IRewardsCoordinator::new(self.rewards_coordinator, signer);
        let operator_set_rewards =
            convert_allocation_operator_set_to_rewards_operator_set(operator_set);
        let tx = rewards_coordinator
            .setOperatorSetSplit(operator, operator_set_rewards, split)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
    }

    /// sets the split of a specific `operator` for Programmatic Incentives
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator address
    /// * `split` - The new split of the operator for PI
    ///
    /// # Returns
    ///
    /// * `Result<FixedBytes<32>, ElContractsError>` - The transaction hash if the transaction is sent, otherwise an error.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn set_operator_pi_split(
        &self,
        operator: Address,
        split: u16,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let signer = get_signer(&self.signer, &self.provider);

        let rewards_coordinator = IRewardsCoordinator::new(self.rewards_coordinator, signer);

        let tx = rewards_coordinator
            .setOperatorPISplit(operator, split)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
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
        let permission_controller_contract = PermissionController::new(
            self.permission_controller
                .ok_or(ElContractsError::MissingParameter)?,
            provider,
        );

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
        let permission_controller_contract = PermissionController::new(
            self.permission_controller
                .ok_or(ElContractsError::MissingParameter)?,
            provider,
        );

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
        let permission_controller_contract = PermissionController::new(
            self.permission_controller
                .ok_or(ElContractsError::MissingParameter)?,
            provider,
        );

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
        let permission_controller_contract = PermissionController::new(
            self.permission_controller
                .ok_or(ElContractsError::MissingParameter)?,
            provider,
        );

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
        let permission_controller_contract = PermissionController::new(
            self.permission_controller
                .ok_or(ElContractsError::MissingParameter)?,
            provider,
        );

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
        let permission_controller_contract = PermissionController::new(
            self.permission_controller
                .ok_or(ElContractsError::MissingParameter)?,
            provider,
        );

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
        let contract_allocation_manager = AllocationManager::new(
            self.allocation_manager
                .ok_or(ElContractsError::MissingParameter)?,
            provider.clone(),
        );
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

        let g2_point_x: Vec<DynSolValue> = vec![
            DynSolValue::Uint(g2_pub_key_bn254.X[0], 256),
            DynSolValue::Uint(g2_pub_key_bn254.X[1], 256),
        ];
        let g2_point_y: Vec<DynSolValue> = vec![
            DynSolValue::Uint(g2_pub_key_bn254.Y[0], 256),
            DynSolValue::Uint(g2_pub_key_bn254.Y[1], 256),
        ];
        let encoded_params_with_socket = DynSolValue::Tuple(vec![
            DynSolValue::Uint(U256::from(0), 256),
            DynSolValue::String(socket.to_string()),
            DynSolValue::Uint(alloy_g1_point_signed_msg.X, 256),
            DynSolValue::Uint(alloy_g1_point_signed_msg.Y, 256),
            DynSolValue::Uint(g1_pub_key_bn254.X, 256),
            DynSolValue::Uint(g1_pub_key_bn254.Y, 256),
            DynSolValue::FixedArray(g2_point_x),
            DynSolValue::FixedArray(g2_point_y),
        ])
        .abi_encode_params();

        let params = IAllocationManagerTypes::RegisterParams {
            avs: avs_address,
            operatorSetIds: operator_set_ids,
            data: encoded_params_with_socket.into(),
        };
        let tx = contract_allocation_manager
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
        let contract_allocation_manager = AllocationManager::new(
            self.allocation_manager
                .ok_or(ElContractsError::MissingParameter)?,
            provider,
        );

        let params = IAllocationManagerTypes::DeregisterParams {
            operator: operator_address,
            avs: avs_address,
            operatorSetIds: operator_set_ids,
        };
        let tx = contract_allocation_manager
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
        let contract_allocation_manager = AllocationManager::new(
            self.allocation_manager
                .ok_or(ElContractsError::MissingParameter)?,
            provider,
        );

        let tx = contract_allocation_manager
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
        let contract_allocation_manager = AllocationManager::new(
            self.allocation_manager
                .ok_or(ElContractsError::MissingParameter)?,
            provider,
        );

        let tx = contract_allocation_manager
            .modifyAllocations(operator_address, allocations)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        Ok(*tx.tx_hash())
    }

    /// Removes from the deallocationQueue all clearable deallocations up to max `num_to_clear` number of deallocations.
    ///
    /// `len(strategies)` must be equal to `len(num_to_clear)`
    ///
    /// # Arguments
    /// * `operator` - operator address to clear deallocations for
    /// * `strategies` - list of strategies to clear deallocations for
    /// * `num_to_clear` - list of number of pending deallocations to clear for each strategy
    ///
    /// # Returns
    /// * `TxHash` - The transaction hash of the generated transaction.
    ///
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn clear_deallocation_queue(
        &self,
        operator: Address,
        strategies: Vec<Address>,
        num_to_clear: Vec<u16>,
    ) -> Result<TxHash, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);
        let allocation_manager_contract = AllocationManager::new(
            self.allocation_manager
                .ok_or(ElContractsError::MissingParameter)?,
            provider,
        );

        allocation_manager_contract
            .clearDeallocationQueue(operator, strategies, num_to_clear)
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)
            .map(|tx| *tx.tx_hash())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{
        build_el_chain_reader, new_claim, new_test_writer, new_test_writer_preslashing,
        OPERATOR_ADDRESS, OPERATOR_PRIVATE_KEY,
    };
    use alloy::{
        primitives::{address, aliases::U96, Address, U256},
        providers::{Provider, WalletProvider},
        sol_types::SolCall,
    };
    use eigen_common::{get_provider, get_signer};
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_testing_utils::{
        anvil::{
            mine_anvil_blocks, set_account_balance, start_anvil_container, start_m2_anvil_container,
        },
        anvil_constants::{
            get_allocation_manager_address, get_erc20_mock_strategy,
            get_registry_coordinator_address, get_service_manager_address, FIRST_ADDRESS,
            FIRST_PRIVATE_KEY,
        },
        transaction::wait_transaction,
    };
    use eigen_types::operator::Operator;
    use eigen_utils::{
        convert_allocation_operator_set_to_rewards_operator_set,
        slashing::{
            core::allocationmanager::{
                AllocationManager::{self, OperatorSet},
                IAllocationManagerTypes,
            },
            middleware::registrycoordinator::{
                ISlashingRegistryCoordinatorTypes::OperatorSetParam,
                IStakeRegistryTypes::StrategyParams, RegistryCoordinator,
            },
            sdk::mockavsservicemanager::MockAvsServiceManager,
        },
    };
    use std::str::FromStr;

    #[tokio::test]
    async fn test_register_operator() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_reader = build_el_chain_reader(http_endpoint.clone()).await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;

        let operator = Operator {
            address: FIRST_ADDRESS, // can only register the address corresponding to the signer used in the writer
            delegation_approver_address: FIRST_ADDRESS,
            metadata_url: "metadata_uri".to_string(),
            allocation_delay: Some(1),
            _deprecated_earnings_receiver_address: None,
            staker_opt_out_window_blocks: None,
        };
        el_chain_writer
            .register_as_operator(operator)
            .await
            .unwrap();

        let is_registered = el_chain_reader
            .is_operator_registered(FIRST_ADDRESS)
            .await
            .unwrap();
        assert!(is_registered);
    }

    #[tokio::test]
    async fn test_register_operator_preslashing() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let el_chain_reader = build_el_chain_reader(http_endpoint.clone()).await;
        let el_chain_writer =
            new_test_writer_preslashing(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string())
                .await;

        let operator = Operator {
            address: FIRST_ADDRESS, // can only register the address corresponding to the signer used in the writer
            delegation_approver_address: FIRST_ADDRESS,
            metadata_url: "metadata_uri".to_string(),
            allocation_delay: None,
            _deprecated_earnings_receiver_address: None,
            staker_opt_out_window_blocks: Some(0u32),
        };
        el_chain_writer
            .register_as_operator_preslashing(operator)
            .await
            .unwrap();

        let is_registered = el_chain_reader
            .is_operator_registered(FIRST_ADDRESS)
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
            delegation_approver_address: Address::ZERO,
            metadata_url: "eigensdk-rs".to_string(),
            allocation_delay: Some(1),
            _deprecated_earnings_receiver_address: None,
            staker_opt_out_window_blocks: None,
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
            delegation_approver_address: Address::ZERO,
            metadata_url: "new-metadata".to_string(),
            allocation_delay: Some(1),
            staker_opt_out_window_blocks: None,
            _deprecated_earnings_receiver_address: None,
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
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;

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
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;

        let claimer = address!("5eb15C0992734B5e77c888D713b4FC67b3D679A2");

        let tx_hash = el_chain_writer.set_claimer_for(claimer).await.unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }

    #[tokio::test]
    async fn test_process_claim() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;

        let (_root, claim) = new_claim(&http_endpoint, U256::from(42)).await;

        let tx_hash = el_chain_writer
            .process_claim(claim, FIRST_ADDRESS)
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }

    #[tokio::test]
    async fn test_process_claims() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;

        let (_root, claim0) = new_claim(&http_endpoint, U256::from(42)).await;
        let (_root, claim1) = new_claim(&http_endpoint, U256::from(4256)).await;

        let tx_hash = el_chain_writer
            .process_claims(vec![claim0, claim1], FIRST_ADDRESS)
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }

    #[tokio::test]
    async fn test_add_and_remove_pending_admin() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;

        let pending_admin = address!("009440d62dc85c73dbf889b7ad1f4da8b231d2ef");
        let tx_hash = el_chain_writer
            .add_pending_admin(FIRST_ADDRESS, pending_admin)
            .await
            .unwrap();
        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let is_pending_admin = el_chain_writer
            .el_chain_reader
            .is_pending_admin(FIRST_ADDRESS, pending_admin)
            .await
            .unwrap();
        assert!(is_pending_admin);

        let tx_hash = el_chain_writer
            .remove_pending_admin(FIRST_ADDRESS, pending_admin)
            .await
            .unwrap();
        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let is_admin = el_chain_writer
            .el_chain_reader
            .is_pending_admin(FIRST_ADDRESS, pending_admin)
            .await
            .unwrap();
        assert!(!is_admin);
    }

    #[tokio::test]
    async fn test_accept_admin() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let account_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;

        let pending_admin = address!("14dC79964da2C08b23698B3D3cc7Ca32193d9955");
        let pending_admin_key =
            "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";

        let tx_hash = account_writer
            .add_pending_admin(FIRST_ADDRESS, pending_admin)
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let admin_writer =
            new_test_writer(http_endpoint.to_string(), pending_admin_key.to_string()).await;

        let tx_hash = admin_writer.accept_admin(FIRST_ADDRESS).await.unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let is_admin = admin_writer
            .el_chain_reader
            .is_admin(FIRST_ADDRESS, pending_admin)
            .await
            .unwrap();
        assert!(is_admin);
    }

    #[tokio::test]
    async fn test_remove_admin() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;

        let pending_admin_1 = address!("14dC79964da2C08b23698B3D3cc7Ca32193d9955");
        let pending_admin_1_key =
            "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";

        let pending_admin_2 = address!("23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f");
        let pending_admin_2_key =
            "0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97";

        // Adding two admins and removing one. Cannot remove the last admin, so one must remain
        let tx_hash = el_chain_writer
            .add_pending_admin(FIRST_ADDRESS, pending_admin_1)
            .await
            .unwrap();
        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let tx_hash = el_chain_writer
            .add_pending_admin(FIRST_ADDRESS, pending_admin_2)
            .await
            .unwrap();
        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let admin_1_writer =
            new_test_writer(http_endpoint.to_string(), pending_admin_1_key.to_string()).await;
        admin_1_writer.accept_admin(FIRST_ADDRESS).await.unwrap();
        let admin_2_writer =
            new_test_writer(http_endpoint.to_string(), pending_admin_2_key.to_string()).await;
        admin_2_writer.accept_admin(FIRST_ADDRESS).await.unwrap();

        let tx_hash = admin_1_writer
            .remove_admin(FIRST_ADDRESS, pending_admin_2)
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let is_admin = el_chain_writer
            .el_chain_reader
            .is_admin(FIRST_ADDRESS, pending_admin_2)
            .await
            .unwrap();
        assert!(!is_admin);
    }

    #[tokio::test]
    async fn test_set_and_remove_permission() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let account_address = FIRST_ADDRESS;
        let appointee_address = address!("009440d62dc85c73dbf889b7ad1f4da8b231d2ef");
        let appointee_key = "6b35c6d8110c888de06575b45181bf3f9e6c73451fa5cde812c95a6b31e66ddf";
        let target = address!("14dC79964da2C08b23698B3D3cc7Ca32193d9955");
        let selector = [0, 1, 2, 3].into();

        // add an admin
        let account_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;

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
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;

        let tx_hash = el_chain_writer
            .remove_permission(account_address, appointee_address, target, selector)
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }

    async fn create_operator_set(http_endpoint: &str, avs_address: Address) {
        let allocation_manager_addr =
            get_allocation_manager_address(http_endpoint.to_string()).await;
        let default_signer = get_signer(FIRST_PRIVATE_KEY, http_endpoint);
        let allocation_manager =
            AllocationManager::new(allocation_manager_addr, default_signer.clone());
        let registry_coordinator_addr =
            get_registry_coordinator_address(http_endpoint.to_string()).await;
        let service_manager_address = get_service_manager_address(http_endpoint.to_string()).await;
        let service_manager =
            MockAvsServiceManager::new(service_manager_address, default_signer.clone());
        service_manager
            .setAppointee(
                default_signer.default_signer_address(),
                allocation_manager_addr,
                alloy::primitives::FixedBytes(AllocationManager::setAVSRegistrarCall::SELECTOR),
            )
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
        allocation_manager
            .setAVSRegistrar(avs_address, registry_coordinator_addr)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();

        // Create slashable quorum
        let contract_registry_coordinator =
            RegistryCoordinator::new(registry_coordinator_addr, default_signer.clone());
        let operator_set_params = OperatorSetParam {
            maxOperatorCount: 10,
            kickBIPsOfOperatorStake: 100,
            kickBIPsOfTotalStake: 1000,
        };
        let strategy = get_erc20_mock_strategy(http_endpoint.to_string()).await;
        service_manager
            .setAppointee(
                registry_coordinator_addr,
                allocation_manager_addr,
                alloy::primitives::FixedBytes(AllocationManager::createOperatorSetsCall::SELECTOR),
            )
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
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
    }

    #[tokio::test]
    async fn test_register_for_operator_sets() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        let operator_set_id = 0;
        create_operator_set(http_endpoint.as_str(), avs_address).await;

        let operator_addr = OPERATOR_ADDRESS;
        let operator_private_key = OPERATOR_PRIVATE_KEY;
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

        let tx_hash = el_chain_writer
            .deregister_from_operator_sets(operator_addr, avs_address, vec![operator_set_id])
            .await
            .unwrap();
        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let is_registered = el_chain_writer
            .el_chain_reader
            .is_operator_registered_with_operator_set(operator_addr, operator_set.clone())
            .await
            .unwrap();
        assert!(!is_registered);
    }

    #[tokio::test]
    async fn test_set_allocation_delay() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;

        let delay = 10;

        let tx_hash = el_chain_writer
            .set_allocation_delay(FIRST_ADDRESS, delay)
            .await
            .unwrap();
        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
        let current_block = get_provider(&http_endpoint)
            .get_block_number()
            .await
            .unwrap();
        mine_anvil_blocks(&_container, (current_block as u32) + 2).await;
        let allocation_delay = el_chain_writer
            .el_chain_reader
            .get_allocation_delay(FIRST_ADDRESS)
            .await
            .unwrap();

        assert_eq!(allocation_delay, delay);
    }

    #[tokio::test]
    async fn test_modify_allocations() {
        let (container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;

        let operator_address = FIRST_ADDRESS;
        let strategy_addr = get_erc20_mock_strategy(http_endpoint.clone()).await;

        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        let operator_set_id = 0;
        create_operator_set(http_endpoint.as_str(), avs_address).await;

        let new_allocation = 100;
        let allocate_params = IAllocationManagerTypes::AllocateParams {
            strategies: vec![strategy_addr],
            operatorSet:
                eigen_utils::slashing::core::allocationmanager::AllocationManager::OperatorSet {
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
            .get_allocation_delay(FIRST_ADDRESS)
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

    #[tokio::test]
    async fn test_set_operator_avs_split() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;
        let new_split = 5;
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;

        let split = el_chain_writer
            .el_chain_reader
            .get_operator_avs_split(FIRST_ADDRESS, avs_address)
            .await
            .unwrap();

        assert_eq!(split, 1); // not initialized case

        el_chain_writer
            .set_operator_avs_split(FIRST_ADDRESS, avs_address, new_split)
            .await
            .unwrap();
        let split = el_chain_writer
            .el_chain_reader
            .get_operator_avs_split(FIRST_ADDRESS, avs_address)
            .await
            .unwrap();
        assert_eq!(split, 5); // initialized && activated
    }

    #[tokio::test]
    async fn test_set_operator_set_split() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        let operator_set_id = 0;
        create_operator_set(http_endpoint.as_str(), avs_address).await;

        let operator_addr = OPERATOR_ADDRESS;
        let operator_private_key = OPERATOR_PRIVATE_KEY;
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
            id: 0,
        };

        let new_split = 5;
        let tx_hash = el_chain_writer
            .set_operator_set_split(OPERATOR_ADDRESS, operator_set.clone(), new_split)
            .await
            .unwrap();
        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
        let rewards_operator_set =
            convert_allocation_operator_set_to_rewards_operator_set(operator_set.clone());
        let split = el_chain_writer
            .el_chain_reader
            .get_operator_set_split(OPERATOR_ADDRESS, rewards_operator_set.clone())
            .await
            .unwrap();

        assert_eq!(split, new_split); // initialized && activated
    }

    #[tokio::test]
    async fn test_set_operator_pi_split() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;
        let new_split = 5;

        let split = el_chain_writer
            .el_chain_reader
            .get_operator_pi_split(FIRST_ADDRESS)
            .await
            .unwrap();

        assert_eq!(split, 1); // not initialized case

        let tx_hash = el_chain_writer
            .set_operator_pi_split(FIRST_ADDRESS, new_split)
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let split = el_chain_writer
            .el_chain_reader
            .get_operator_pi_split(FIRST_ADDRESS)
            .await
            .unwrap();

        assert_eq!(split, new_split);
    }

    #[tokio::test]
    async fn test_clear_deallocation_queue() {
        let (_contianer, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        create_operator_set(http_endpoint.as_str(), avs_address).await;

        let operator_address = FIRST_ADDRESS;
        let strategy_addr = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let operator_set_id = 0;

        let new_allocation = 100;
        let allocate_params = IAllocationManagerTypes::AllocateParams {
            strategies: vec![strategy_addr],
            operatorSet: OperatorSet {
                avs: avs_address,
                id: operator_set_id,
            },
            newMagnitudes: vec![new_allocation],
        };
        let tx_hash_alloc = el_chain_writer
            .modify_allocations(operator_address, vec![allocate_params.clone()])
            .await
            .unwrap();
        let receipt_alloc = wait_transaction(&http_endpoint, tx_hash_alloc)
            .await
            .unwrap();
        assert!(receipt_alloc.status());

        let allocation_info_before = el_chain_writer
            .el_chain_reader
            .get_allocation_info(operator_address, strategy_addr)
            .await
            .unwrap();

        assert_eq!(
            allocation_info_before[0].pending_diff,
            U256::from(new_allocation)
        );

        let tx_hash_clear = el_chain_writer
            .clear_deallocation_queue(
                operator_address,
                vec![strategy_addr],
                vec![new_allocation as u16],
            )
            .await
            .unwrap();
        let receipt_clear = wait_transaction(&http_endpoint, tx_hash_clear)
            .await
            .unwrap();
        assert!(receipt_clear.status(),);

        let allocation_info_after = el_chain_writer
            .el_chain_reader
            .get_allocation_info(operator_address, strategy_addr)
            .await
            .unwrap();

        assert_eq!(allocation_info_after[0].pending_diff, U256::ZERO);
        assert_eq!(
            allocation_info_after[0].current_magnitude,
            U256::from(new_allocation)
        );
    }
}
