use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};


// Returns true if row satisfies the condition, false otherwise.
// handle conditions (Not, And, Or).
fn row_matches(row: &Row, dataset: &Dataset, condition: &Condition) -> bool {
    match condition {

    // find column index corresponds to col_name to compare
    // the row's value at that index against the expected value.
        Condition::Equal(col_name, expected) => {
            let idx = dataset.column_index(col_name);
            row.get_value(idx) == expected
        }
     // Negate the result of the inner condition.
        Condition::Not(inner) => !row_matches(row, dataset, inner),
    // must be true (short-circuits on the first false).
        Condition::And(left, right) => {
            row_matches(row, dataset, left) && row_matches(row, dataset, right)
        }
        Condition::Or(left, right) => {
            row_matches(row, dataset, left) || row_matches(row, dataset, right)
        }
    }
}
    // Returns a Dataset containing only rows work with the filter condition.
pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {

    // Start with an empty dataset that has the same columns as the input.
    let mut result = Dataset::new(dataset.columns().clone());

     // Borrow each row and check it against the condition.
    for row in dataset.iter() {
        if row_matches(row, dataset, filter) {

     // Clone the row because we're BORROWING from the input dataset.
            result.add_row(row.clone());
        }
    }
    result
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

// Aggregates each group in the HashMap into a single value.
// Returns a HashMap mapping each group key to aggregated result.
pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut result = HashMap::new();

    // key = the group-by value, group = all rows in that bucket.
    for (key, group) in dataset {
        let value = match aggregation {

            // count the number of rows in this group.
            Aggregation::Count(_) => Value::Integer(group.len() as i32),

            // sum add up all integer values in the specified column across each row.
            Aggregation::Sum(col_name) => {
                let idx = group.column_index(col_name);
                let sum: i32 = group.iter().map(|row| match row.get_value(idx) {
                    Value::Integer(n) => *n,
                    _ => panic!("Sum on non-integer column"),
                }).sum();
                Value::Integer(sum)
            }
             // average sum divided by count
            Aggregation::Average(col_name) => {
                let idx = group.column_index(col_name);
                let count = group.len() as i32;
                let sum: i32 = group.iter().map(|row| match row.get_value(idx) {
                    Value::Integer(n) => *n,
                    _ => panic!("Average on non-integer column"),
                }).sum();
                Value::Integer(sum / count)
            }
        };
        result.insert(key, value);
    }
    result
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}