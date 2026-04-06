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
    return query_dataset;

    // Step 1: call slow_rpc on the server — it returns the ENTIRE raw dataset.
    // Context::current() is required by tarpc for every RPC call (tracks deadlines/metadata).
    // .await waits for the async network call to finish.
    // .unwrap() extracts the value, or panics if the connection failed.

    // Step 2: the server didn't run the query — WE have to run it here on the client.
    // This is what makes slow_rpc "slow": all the raw data came over the network,
    // and now the client has to do all the filtering/grouping/aggregation work.
}

//std 2
pub async fn run_fast_rpc(rpc_client: &RPCInterfaceClient, query: Query) -> Dataset {
    println!("using fast_rpc");

    // Send the query to the server — it runs compute_query_on_dataset() over there
    // and returns only the small result. We just receive and return it directly.
    // This is "fast" because only the final result (e.g., 5 rows) travels the network,
    // not the thousands of rows in the raw dataset.
    return rpc_client.fast_rpc(Context::current(), query).await.unwrap();
}