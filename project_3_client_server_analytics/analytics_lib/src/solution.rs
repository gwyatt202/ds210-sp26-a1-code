use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};


fn row_matches(dataset: &Dataset, row: &Row, condition: &Condition) -> bool {
    
    match condition {
        Condition::Equal(col_name, value) => {
            let col_index = dataset.column_index(col_name);
            let row_vals = row.get_value(col_index);
            return row_vals  == value  
            // if the value in the row equals the value in the equal condition, true returned   
                  
            
        },
        Condition::Not(not_inner) => {
            !row_matches(dataset, row, not_inner) 
            //recursivley calls function returning F if not_inner is T
            //Even though we can't see the condition in these lines, it is wrapped (Box) 
            //so we cant directly see it so we need to call the row_matches and access the condition like in Equal

        },

        Condition::And(and_inner_a, and_inner_b) => {
            row_matches(dataset, row, and_inner_a) && row_matches(dataset, row, and_inner_b)
            // recursively calls row_matches, returns true if both and_inner_a and and_inner_b are true
            },
        
        Condition::Or(or_inner_a, or_inner_b) => {
            row_matches(dataset, row, or_inner_a) || row_matches(dataset, row, or_inner_b)
            // recursivley calls func, returns T if or_inner_a OR or_inner_b is T

        },
    }
}



//both students
pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    //you must write correct code that looks at the rows in the given dataset, 
    //checks whether they meet the filter condition, and return a new Dataset 
    //that only contains matching rows.
    let columns: Vec<(String, ColumnType)> = dataset.columns().clone(); 
    //cloning the columns becasue we dont want to change original but still need the object because of the output type
    let mut new_dataset = Dataset::new(columns);
    //making a new dataset to output using the newly created columns
    for row in dataset.iter() { //.iter() allows to iterate over the rows of dataset
    if row_matches(dataset, row, filter) { 
        new_dataset.add_row(row.clone()); //if condition true, row is added to the new dataset
    }
    }
    return new_dataset
}



// std 1
pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    //make an empty hashmap
    //take the group_by_column and attribute this to the keys of the hashmap
    //iterate over the group_by_column
    //nested iteration over the rows in the dataset
    //if they contain that specific group_by_column in the outside loop, move them into the values of the hashmap
    //return the Hashmap

    let mut map: HashMap<Value, Dataset> = HashMap::new(); //create hashmap
    let col_index = dataset.column_index(group_by_column); //getting the index of the group_by_column
    
    for row in dataset.iter() { //iterating over the rows of old dataset
        let columns = dataset.columns().clone(); 
        let mut new_dataset = Dataset::new(columns); //create new dataset in the for loop for every row (one row (dataset) per iteration)
        let key = row.get_value(col_index); //one key per iteration
        if !map.contains_key(key) { //if map does not contain the key
            new_dataset.add_row(row.clone()); //add cloned row to not change original (not ref)
            map.insert(key.clone(), new_dataset);  //insert both the cloned key (cloned as key is from og data) 
                                                        //and new dataset into hash
        } else { //map contains the key
            let existing_dataset = map.get_mut(key).unwrap(); //takes value from key as mutable
            existing_dataset.add_row(row.clone());  // adds the cloned row to the existing dataset
        }
    }
    return map;
}

//std 2
pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    todo!("Implement this!");
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