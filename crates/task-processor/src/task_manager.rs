use std::{fmt::Debug, future::Future};

use crate::{
    task::Task, task_response::TaskResponse, task_response_metadata_sol::TaskResponseMetadataSol,
};
use alloy::sol_types::{SolEvent, SolValue};
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use eigen_utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use eigen_utils::slashing::middleware::iblssignaturechecker::BN254::G1Point;
use serde::de::DeserializeOwned;

/// Error returned by the task processor
pub type TaskManagerError = Box<dyn core::error::Error + Send>;

/// Utility function for boxing errors
pub fn box_error<E: core::error::Error + Send + 'static>(e: E) -> TaskManagerError {
    Box::new(e)
}

/// Task manager contract trait. It wraps the contract's types and functions.
pub trait TaskManagerContract {
    /// Type for inputs of each task
    type Input: Clone + SolValue + Send + Sync + 'static + Debug;

    /// Type for outputs of each task
    type Output: Clone + SolValue + Send + Sync + 'static + Debug + DeserializeOwned;

    /// New task event
    type NewTaskEvent: SolEvent;

    /// Task responded event
    type TaskRespondedEvent: SolEvent;

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
