use analytics_lib::{dataset::Dataset, query::Query};


// This trait defines the public RPC interface — the "menu" of functions a client can call.
// #[tarpc::service] auto-generates the client stub (RPCInterfaceClient) and server wiring.
#[tarpc::service]
pub trait RPCInterface {
    // hello: a simple test — server returns the string "hello".
    async fn hello() -> String;

    // slow_rpc: server sends the ENTIRE raw dataset to the client.
    // The client then runs the query locally. Slow because lots of data travels the network.
    async fn slow_rpc() -> Dataset;

    // fast_rpc: client sends the query to the server.
    // The server runs it and only returns the small result. Fast because less data travels.
    async fn fast_rpc(query: Query) -> Dataset;
}
