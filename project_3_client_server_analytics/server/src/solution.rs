use analytics_lib::{dataset::Dataset, query::Query};

pub fn hello() -> String {
    println!("hello called");
    return String::from("hello");
}

pub fn slow_rpc(input_dataset: &Dataset) -> Dataset {
    println!("slow_rpc called");
    // Clone and return the entire raw dataset to the client.
    // The client is responsible for running the query on this data.
    // .clone() makes a full copy because we can't move out of a reference.
    return input_dataset.clone();
}

pub fn fast_rpc(input_dataset: &Dataset, query: Query) -> Dataset {
    println!("fast_rpc called");
    // Run the query HERE on the server, then return only the result.
    // This is "fast" because only a small result dataset travels the network
    // instead of the entire raw dataset.
    return analytics_lib::solution::compute_query_on_dataset(input_dataset, &query);
}