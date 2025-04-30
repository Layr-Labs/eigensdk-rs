use crate::{
    challenger::ChallengerTaskProcessor, error::ChallengerError, task_manager::TaskManagerContract,
};
use alloy::dyn_abi::SolType;
use alloy::{
    contract::private::{Provider, Transport},
    network::Network,
    primitives::{Bytes, FixedBytes},
    rpc::types::Log,
    sol_types::SolValue,
};
use eigen_task_processor::{task::Task, task_response::TaskResponse};
use eigen_utils::slashing::middleware::iblssignaturechecker::BN254::G1Point;
use std::collections::HashMap; // Check which type to use

#[derive(Debug)]
pub struct TaskResponseData<Output>
where
    Output: Clone + SolValue + Send + Sync + 'static,
{
    task_response: TaskResponse<Output>,
    task_response_metadata: TaskResponseMetadata,
    non_signing_operator_pub_keys: Vec<G1Point>,
}

#[derive(Debug)]
pub struct TaskResponseMetadata {
    pub task_responded_block: u32,
    pub hash_of_non_signers: FixedBytes<32>,
}

pub struct IndexingChallengerProcessor<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static,
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
    N: Network,
{
    task_manager: TM,
    tasks: HashMap<u32, Task<TM::Input>>,
    task_responses: HashMap<u32, TaskResponseData<TM::Output>>,
}

impl<TM, T, P, N> ChallengerTaskProcessor for IndexingChallengerProcessor<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static,
    <TM as TaskManagerContract<T, P, N>>::Input: From<
        <<<TM as TaskManagerContract<T, P, N>>::Input as SolValue>::SolType as SolType>::RustType,
    >,
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
    N: Network,
{
    type NewTaskEvent = TM::NewTaskEvent;

    type TaskResponseEvent = TM::TaskRespondedEvent;

    type Input = TM::Input;

    type Output = TM::Output;

    async fn handle_task_creation(&mut self, log: Log) -> Result<(), ChallengerError> {
        // event NewTaskCreated(uint32 indexed taskIndex, Task task);
        // Since taskIndex is indexed type, it is present in the topics array
        // The first element of the topic is the event hash signature, the second is the taskIndex
        let task_index_bytes: [u8; 32] = log.topics().get(1).unwrap().0;

        // u32 values are stored in the last 4 bytes of a 32 bytes array (left-padded).
        let u32_bytes: [u8; 4] = task_index_bytes[28..32].try_into().unwrap();
        let task_index = u32::from_be_bytes(u32_bytes);

        // Skip the first 32 bytes of the ABI-encoded data (the dynamic offset pointer)
        // so we can decode the actual tuple payload that follows.
        let data = log.inner.data.data.0.get(32..).unwrap();

        let (input, task_created_block, quorum_numbers, quorum_threshold_percentage) =
            <(
                <TM::Input as SolValue>::SolType,
                <u32 as SolValue>::SolType,
                <Bytes as SolValue>::SolType,
                <u32 as SolValue>::SolType,
            )>::abi_decode_params(data, false)?;

        let task = Task::<TM::Input> {
            input: input.into(),
            task_created_block,
            quorum_numbers,
            quorum_threshold_percentage,
        };
        self.tasks.insert(task_index, task);

        Ok(())
    }

    async fn handle_task_response(
        &mut self,
        log: Log,
        check_response: impl Fn(
            Task<TM::Input>,
            TaskResponse<TM::Output>,
        ) -> Result<bool, ChallengerError>,
    ) -> Result<(), ChallengerError> {
        // TaskResponded(TaskResponse taskResponse, TaskResponseMetadata taskResponseMetadata);

        // Skip the first 32 bytes of the ABI-encoded data (the dynamic offset pointer)
        // so we can decode the actual tuple payload that follows.
        let data = log.inner.data.data.0.get(32..).unwrap();

        // Decode TaskResponse<Output>
        let (task_response, task_response_metadata) = <((
            <TM::Output as SolValue>::SolType,
            <TM::TaskResponseMetadata as SolValue>::SolType,
        ))>::abi_decode_params(data, false)?;

        // let (input, task_created_block, quorum_numbers, quorum_threshold_percentage) =
        //     <(
        //         <TM::Input as SolValue>::SolType,
        //         <u32 as SolValue>::SolType,
        //         <Bytes as SolValue>::SolType,
        //         <u32 as SolValue>::SolType,
        //     )>::abi_decode_params(data, false)?;

        self.task_responses.insert(task_index, task);

        Ok(())
    }
}

// THIS CODE IS COMMENTED BECAUSE RIGHT NOW IT DOESN'T COMPILE
// TODO: MAKE TRAIT ASYNC AND TRY TO MOVE CODE TO THE TASK MANAGER CONTRACT TRAIT IN TASK PROCESSOR

// use alloy::{
//     consensus::Transaction,
//     contract::private::{Provider as PrivateProvider, Transport},
//     network::Network,
//     providers::Provider,
//     sol_types::SolCall,
// };
// use eigensdk::{
//     challenger::{challenger::ChallengerTaskProcessor, Challenger},
//     common::{get_provider, get_signer},
// };
// use incredible_bindings::incrediblesquaringtaskmanager::{
//     IIncredibleSquaringTaskManager::{Task, TaskResponse, TaskResponseMetadata},
//     IncredibleSquaringTaskManager::{
//         respondToTaskCall, IncredibleSquaringTaskManagerInstance, NewTaskCreated, TaskResponded,
//     },
//     BN254::G1Point,
// };
// use incredible_dot_product::config::Config;
// use std::collections::HashMap;

// pub struct TaskResponseData {
//     task_response: TaskResponse,
//     task_response_metadata: TaskResponseMetadata,
//     non_signing_operator_pub_keys: Vec<G1Point>,
// }

// struct ChallengerTaskProcessorImpl<T, P, N>
// where
//     T: Transport + Clone + Send + Sync,
//     P: PrivateProvider<T, N>,
//     N: Network,
// {
//     tasks: HashMap<u32, Task>,
//     task_responses: HashMap<u32, TaskResponseData>,
//     contract: IncredibleSquaringTaskManagerInstance<T, P, N>,
//     rpc_url: String,
// }

// // TODO: Try to move logic to TaskManagerContract trait
// impl<T, P, N> ChallengerTaskProcessor for ChallengerTaskProcessorImpl<T, P, N>
// where
//     T: Transport + Clone + Send + Sync,
//     P: PrivateProvider<T, N>,
//     N: Network,
// {
//     type NewTaskEvent = NewTaskCreated;

//     type TaskResponseEvent = TaskResponded;

//     // TODO: Make method async in the trait
//     async fn handle_task_creation(&mut self, decoded: alloy::rpc::types::Log<Self::NewTaskEvent>) {
//         let data = decoded.data();

//         self.tasks.insert(data.taskIndex, data.task.clone());
//     }

//     // TODO: Make method async in the trait
//     async fn handle_task_response(
//         &mut self,
//         decoded: alloy::rpc::types::Log<Self::TaskResponseEvent>,
//     ) {
//         let data = decoded.data().clone();
//         let task_index = data.taskResponse.referenceTaskIndex;

//         let non_signing_operator_pub_keys = self.get_non_signing_operator_pub_keys(decoded).await;

//         self.task_responses.insert(
//             task_index,
//             TaskResponseData {
//                 task_response: data.taskResponse.clone(),
//                 task_response_metadata: data.taskResponseMetadata.clone(),
//                 non_signing_operator_pub_keys,
//             },
//         );

//         if self.tasks.contains_key(&task_index) && self.check_task_response(task_index) {
//             self.raise_challenge(task_index).await;
//         }
//     }
// }

// impl<T, P, N> ChallengerTaskProcessorImpl<T, P, N>
// where
//     T: Transport + Clone + Send + Sync,
//     P: PrivateProvider<T, N>,
//     N: Network,
// {
//     pub fn new(contract: IncredibleSquaringTaskManagerInstance<T, P, N>, rpc_url: String) -> Self {
//         Self {
//             tasks: HashMap::new(),
//             task_responses: HashMap::new(),
//             contract,
//             rpc_url,
//         }
//     }

//     fn check_task_response(&self, _task_index: u32) -> bool {
//         todo!()
//     }

//     // Since this is the method that calls the contract, it should be implemented in the TaskManagerContract trait
//     async fn raise_challenge(&self, task_index: u32) {
//         let task = self.tasks.get(&task_index).unwrap();
//         let task_response = self.task_responses.get(&task_index).unwrap();

//         self.contract
//             .raiseAndResolveChallenge(
//                 task.clone(),
//                 task_response.task_response.clone(),
//                 task_response.task_response_metadata.clone(),
//                 task_response.non_signing_operator_pub_keys.clone(),
//             )
//             .send()
//             .await
//             .unwrap()
//             .get_receipt()
//             .await
//             .unwrap();
//     }

//     pub async fn get_non_signing_operator_pub_keys(
//         &self,
//         log: alloy::rpc::types::Log<TaskResponded>,
//     ) -> Vec<G1Point> {
//         let tx_hash = log.transaction_hash.unwrap();

//         let provider = get_provider(&self.rpc_url);
//         let tx = provider
//             .get_transaction_by_hash(tx_hash)
//             .await
//             .unwrap()
//             .unwrap();

//         let decoded = respondToTaskCall::abi_decode(tx.inner.input(), false).unwrap();

//         decoded
//             .nonSignerStakesAndSignature
//             .nonSignerPubkeys
//             .into_iter()
//             .map(|pk| G1Point { X: pk.X, Y: pk.Y })
//             .collect()
//     }
// }

// #[tokio::main]
// async fn main() {
//     let config = Config::load_from("config.toml");
//     let wallet = get_signer(&config.rpc_config.signer, &config.rpc_config.http_rpc_url);

//     let contract =
//         IncredibleSquaringTaskManagerInstance::new(config.contract_address.task_manager, wallet);
//     let task_processor =
//         ChallengerTaskProcessorImpl::new(contract, config.rpc_config.ws_rpc_url.to_string());
//     let mut challenger = Challenger::new(config.rpc_config.ws_rpc_url.to_string(), task_processor);
//     challenger.start_challenger().await.unwrap();
// }
