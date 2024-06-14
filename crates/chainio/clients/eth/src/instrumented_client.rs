use eigen_metrics_collectors_rpc_calls::RpcCalls as RpcCallsCollector;
use ethers::providers::{Http, Middleware, Provider};

pub struct InstrumentedClient {
    client: Provider<Http>,
    rpc_calls_collector: RpcCallsCollector,
    client_and_version: String,
}

impl InstrumentedClient {
    // pub fn new_instrumented_client(rpc: String, rpc_calls_collector: RpcCallsCollector) {

    //     let client = Provider::<Http>::try_from(rpc).unwrap();

    //     let client_and_version =
    // }

    //  pub async fn get_client_and_version(&self, client : Provider<Http>) -> String{

    //     let client_version = client.get_net_version().await.unwrap();
    //     let s = client.
    //     client_version

    // }
}
