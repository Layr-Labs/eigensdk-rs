// THIS CODE IS COMMENTED BECAUSE RIGHT NOW IT DOESN'T COMPILE
// TODO: MAKE TRAIT ASYNC AND TRY TO MOVE CODE TO THE TASK MANAGER CONTRACT TRAIT IN TASK PROCESSOR

fn main() {
    println!("TODO: Implement challenger!");
}

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
