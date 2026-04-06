use analytics_lib::{dataset::Dataset, query::Query, solution::compute_query_on_dataset};
use interface::RPCInterfaceClient;
use tarpc::context::Context;

pub async fn run_hello(rpc_client: &RPCInterfaceClient) {
    let result = rpc_client.hello(Context::current()).await.unwrap();
    println!("The server says: `{}`", result);
}

pub async fn run_slow_rpc(rpc_client: &RPCInterfaceClient, query: Query) -> Dataset {
    println!("using slow_rpc");

   
    let complete_dataset = rpc_client.slow_rpc(Context::current()).await.unwrap();
    let query_dataset =  compute_query_on_dataset(&complete_dataset, &query);
    return query_dataset

    // What should you do to the dataset?
    // Hint: you have not used `query`, maybe you need to use it somehow?
}

//std 2
pub async fn run_fast_rpc(rpc_client: &RPCInterfaceClient, query: Query) -> Dataset {
    println!("using fast_rpc");

    // You should call fast_rpc here and not slow_rpc.
    let query_dataset = rpc_client.fast_rpc(Context::current(), query).await.unwrap();
    return query_dataset
}