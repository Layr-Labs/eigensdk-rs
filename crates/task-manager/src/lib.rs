//! Task manager

/// Event decoder
pub mod event_decoder;
/// Task
pub mod task;
/// Task response
pub mod task_response;
/// Task response metadata
pub mod task_response_metadata_sol;

use alloy::primitives::B256;
use alloy::sol_types::SolValue;
pub use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
pub use eigen_utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
pub use eigen_utils::slashing::middleware::iblssignaturechecker::BN254::G1Point;
use std::{fmt::Debug, future::Future};
use task::Task;
use task_response::TaskResponse;
use task_response_metadata_sol::TaskResponseMetadataSol;

/// Error returned by the task processor
pub type TaskManagerError = Box<dyn core::error::Error + Send>;

/// Utility function for boxing errors
pub fn box_error<E: core::error::Error + Send + 'static>(e: E) -> TaskManagerError {
    Box::new(e)
}

/// Task manager contract definitions.
/// Defines the types and constants used in [`TaskManager`] and across the SDK.
pub trait TaskManagerDefs {
    /// Type for inputs of each task
    type Input: Clone + SolValue + Send + Sync + 'static + Debug;

    /// Type for outputs of each task
    type Output: Clone + SolValue + Send + Sync + 'static + Debug;

    /// New task event
    const NEW_TASK_EVENT_SELECTOR: B256;

    /// Task responded event
    const TASK_RESPONDED_EVENT_SELECTOR: B256;
}

/// Task manager contract trait. It wraps the contract's types and functions.
pub trait TaskManager: TaskManagerDefs {
    /// Respond to a task
    ///
    /// # Arguments
    ///
    /// * `task` - The task
    /// * `response` - The response
    /// * `non_signer_stakes_and_signature` - The non-signer stakes and signature
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskManagerError>` - The result of the operation
    fn respond_to_task(
        &self,
        task: Task<Self::Input>,
        response: TaskResponse<Self::Output>,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> impl Future<Output = Result<(), TaskManagerError>> + Send;

    /// Create a new task
    ///
    /// # Arguments
    ///
    /// * `input` - Generic input of the task
    /// * `quorum_threshold` - The quorum threshold for the task
    /// * `quorums` - The quorums for the task
    ///
    /// # Returns
    ///
    /// * `Result<N::ReceiptResponse, TaskManagerError>` - The result of the task
    fn create_new_task(
        &self,
        input: Self::Input,
        quorum_threshold: QuorumThresholdPercentage,
        quorums: Vec<QuorumNum>,
    ) -> impl Future<Output = Result<(), TaskManagerError>> + Send;

    /// Raise challenge
    ///
    /// # Arguments
    ///
    /// * `task` - The task
    /// * `task_response` - The task response
    /// * `task_response_metadata` - The task response metadata
    /// * `pubkeys_of_non_signing_operators` - The pubkeys of non-signing operators
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskManagerError>` - The result of the operation
    fn raise_challenge(
        &self,
        task: Task<Self::Input>,
        task_response: TaskResponse<Self::Output>,
        task_response_metadata: TaskResponseMetadataSol,
        pubkeys_of_non_signing_operators: Vec<G1Point>,
    ) -> impl Future<Output = Result<(), TaskManagerError>> + Send;
}

#[macro_export]
/// Implements the [`TaskManager`] trait for the given contract.
/// This requires the contract to have [`createNewTask`], [`respondToTask`] and [`raiseAndResolveChallenge`] functions.
macro_rules! impl_task_manager_from_defs_and_contract {
    ($defs:ty => $contract:ident) => {
        impl<T, P, N> $crate::TaskManagerDefs for $contract<T, P, N>
        where
            T: ::alloy::contract::private::Transport + Clone + Send + Sync,
            P: ::alloy::contract::private::Provider<T, N>,
            N: ::alloy::network::Network,
        {
            type Input = <$defs as TaskManagerDefs>::Input;
            type Output = <$defs as $crate::TaskManagerDefs>::Output;
            const NEW_TASK_EVENT_SELECTOR: ::alloy::primitives::B256 =
                <$defs as $crate::TaskManagerDefs>::NEW_TASK_EVENT_SELECTOR;
            const TASK_RESPONDED_EVENT_SELECTOR: ::alloy::primitives::B256 =
                <$defs as $crate::TaskManagerDefs>::TASK_RESPONDED_EVENT_SELECTOR;
        }

        impl<T, P, N> $crate::TaskManager for $contract<T, P, N>
        where
            T: ::alloy::contract::private::Transport + Clone + Send + Sync,
            P: ::alloy::contract::private::Provider<T, N>,
            N: ::alloy::network::Network,
        {
            $crate::default_contract_impl!();
        }
    };
}

#[macro_export]
/// This macro generates a default implementation of the [`TaskManager`] trait's methods.
/// This requires the contract to have [`createNewTask`], [`respondToTask`] and [`raiseAndResolveChallenge`] functions.
macro_rules! default_contract_impl {
    () => {
        async fn create_new_task(
            &self,
            input: Self::Input,
            quorum_threshold: $crate::QuorumThresholdPercentage,
            quorums: Vec<$crate::QuorumNum>,
        ) -> Result<(), $crate::TaskManagerError> {
            self.createNewTask(input, quorum_threshold.into(), quorums.into())
                .send()
                .await
                .map_err($crate::box_error)?
                .get_receipt()
                .await
                .map_err($crate::box_error)?;

            Ok(())
        }

        async fn respond_to_task(
            &self,
            task: $crate::task::Task<Self::Input>,
            task_response: $crate::task_response::TaskResponse<Self::Output>,
            non_signer_stakes_and_signature: $crate::NonSignerStakesAndSignature,
        ) -> Result<(), $crate::TaskManagerError> {
            let contract_task = (
                task.input,
                task.task_created_block,
                task.quorum_numbers,
                task.quorum_threshold_percentage,
            )
                .into();

            let contract_response = (task_response.task_index, task_response.response).into();

            let apk_g2 = (
                non_signer_stakes_and_signature.apkG2.X,
                non_signer_stakes_and_signature.apkG2.Y,
            )
                .into();

            let sigma = (
                non_signer_stakes_and_signature.sigma.X,
                non_signer_stakes_and_signature.sigma.Y,
            )
                .into();

            let quorum_apks = non_signer_stakes_and_signature
                .quorumApks
                .iter()
                .map(|apk| (apk.X, apk.Y).into())
                .collect();

            let non_signer_pubkeys = non_signer_stakes_and_signature
                .nonSignerPubkeys
                .iter()
                .map(|pubkey| (pubkey.X, pubkey.Y).into())
                .collect();

            let non_signer_stakes_and_signature = (
                non_signer_stakes_and_signature.nonSignerQuorumBitmapIndices,
                non_signer_pubkeys,
                quorum_apks,
                apk_g2,
                sigma,
                non_signer_stakes_and_signature.quorumApkIndices,
                non_signer_stakes_and_signature.totalStakeIndices,
                non_signer_stakes_and_signature.nonSignerStakeIndices,
            )
                .into();

            self.respondToTask(
                contract_task,
                contract_response,
                non_signer_stakes_and_signature,
            )
            .send()
            .await
            .map_err($crate::box_error)?
            .get_receipt()
            .await
            .map_err($crate::box_error)?;

            Ok(())
        }

        async fn raise_challenge(
            &self,
            task: $crate::task::Task<Self::Input>,
            task_response: $crate::task_response::TaskResponse<Self::Output>,
            task_response_metadata: $crate::task_response_metadata_sol::TaskResponseMetadataSol,
            pubkeys_of_non_signing_operators: Vec<$crate::G1Point>,
        ) -> Result<(), $crate::TaskManagerError> {
            let contract_task = (
                task.input,
                task.task_created_block,
                task.quorum_numbers,
                task.quorum_threshold_percentage,
            )
                .into();

            let contract_response = (task_response.task_index, task_response.response).into();

            let task_response_metadata = (
                task_response_metadata.taskResponsedBlock,
                task_response_metadata.hashOfNonSigners,
            )
                .into();

            let pubkey_non_signer = pubkeys_of_non_signing_operators
                .iter()
                .map(|p| (p.X, p.Y).into())
                .collect();

            self.raiseAndResolveChallenge(
                contract_task,
                contract_response,
                task_response_metadata,
                pubkey_non_signer,
            )
            .send()
            .await
            .map_err($crate::box_error)?
            .get_receipt()
            .await
            .map_err($crate::box_error)?;

            Ok(())
        }
    };
}
