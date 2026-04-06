extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::dataset::Value;
use analytics_lib::query::{Aggregation, Condition, Query};
use client::{start_client, solution};

// Breaks the query string into a flat list of tokens.
// For example: `FILTER section == "A1" GROUP BY grade COUNT name`
// becomes: ["FILTER", "section", "==", "\"A1\"", "GROUP", "BY", "grade", "COUNT", "name"]
fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // Skip whitespace
            ' ' | '\t' => { chars.next(); }

            // Single-character punctuation — each becomes its own token
            '(' => { tokens.push("(".to_string()); chars.next(); }
            ')' => { tokens.push(")".to_string()); chars.next(); }
            '!' => { tokens.push("!".to_string()); chars.next(); }

            // `==` is two characters, consume both
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push("==".to_string());
                }
            }

            // Quoted string value like "A1" — keep the quotes in the token
            // so we can tell it apart from a column name later
            '"' => {
                chars.next(); // consume opening quote
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '"' { chars.next(); break; }
                    s.push(c);
                    chars.next();
                }
                // Store with surrounding quotes e.g. `"A1"`
                tokens.push(format!("\"{}\"", s));
            }

            // Words (column names, keywords, integers)
            _ if c.is_alphanumeric() || c == '_' => {
                let mut word = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        word.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(word);
            }

            _ => { chars.next(); }
        }
    }
    tokens
}

// Parses a Value (the right-hand side of `column == value`).
// Quoted tokens like `"A1"` become Value::String("A1").
// Numeric tokens like `42` become Value::Integer(42).
// Unquoted words are treated as strings too.
fn parse_value(tokens: &[String], pos: usize) -> (Value, usize) {
    let token = &tokens[pos];
    if token.starts_with('"') {
        // Strip surrounding quotes and return as String value
        let inner = token[1..token.len() - 1].to_string();
        return (Value::String(inner), pos + 1);
    }
    if let Ok(n) = token.parse::<i32>() {
        return (Value::Integer(n), pos + 1);
    }
    // Fallback: unquoted word treated as string
    return (Value::String(token.clone()), pos + 1);
}

// Recursively parses a Condition starting at `pos` in the token list.
// Returns the parsed Condition and the position of the next unconsumed token.
//
// Handles three forms:
//   !(condition)                  → Condition::Not
//   (condition AND condition)     → Condition::And
//   (condition OR  condition)     → Condition::Or
//   column == value               → Condition::Equal  (base case, no parens)
fn parse_condition(tokens: &[String], pos: usize) -> (Condition, usize) {
    // NOT: !(condition)
    if tokens[pos] == "!" {
        let pos = pos + 1; // skip `!`
        assert_eq!(tokens[pos], "(", "Expected `(` after `!`");
        let pos = pos + 1; // skip `(`
        let (inner, pos) = parse_condition(tokens, pos);
        assert_eq!(tokens[pos], ")", "Expected `)` to close `!(`");
        return (Condition::Not(Box::new(inner)), pos + 1);
    }

    // AND / OR: (left AND/OR right)
    if tokens[pos] == "(" {
        let pos = pos + 1; // skip `(`
        let (left, pos) = parse_condition(tokens, pos);

        // The next token must be AND or OR
        let op = tokens[pos].clone();
        let pos = pos + 1; // skip operator

        let (right, pos) = parse_condition(tokens, pos);
        assert_eq!(tokens[pos], ")", "Expected `)` to close `(condition AND/OR condition)`");
        let pos = pos + 1; // skip `)`

        let condition = match op.as_str() {
            "AND" => Condition::And(Box::new(left), Box::new(right)),
            "OR"  => Condition::Or(Box::new(left), Box::new(right)),
            _     => panic!("Expected AND or OR, got: {}", op),
        };
        return (condition, pos);
    }

    // Base case: column == value
    let column = tokens[pos].clone();
    let pos = pos + 1; // skip column name
    assert_eq!(tokens[pos], "==", "Expected `==` in condition");
    let pos = pos + 1; // skip `==`
    let (value, pos) = parse_value(tokens, pos);
    return (Condition::Equal(column, value), pos);
}

// Parses a full query string into a Query object.
//
// Expected format:
//   FILTER <condition> GROUP BY <column> <COUNT|SUM|AVERAGE> <column>
//
// Examples:
//   FILTER section == "A1" GROUP BY grade COUNT name
//   FILTER (section == "A1" OR section == "B1") GROUP BY section AVERAGE grade
//   FILTER (!(band == "Meshuggah") AND !(band == "Vildhjarta")) GROUP BY album AVERAGE rating
fn parse_query_from_string(input: String) -> Query {
    let tokens = tokenize(&input);
    let mut pos = 0;

    // Every query starts with FILTER
    assert_eq!(tokens[pos], "FILTER", "Query must start with FILTER");
    pos += 1;

    // Parse the condition — this consumes tokens until GROUP is reached
    let (condition, new_pos) = parse_condition(&tokens, pos);
    pos = new_pos;

    // Expect GROUP BY
    assert_eq!(tokens[pos], "GROUP", "Expected GROUP after condition");
    pos += 1;
    assert_eq!(tokens[pos], "BY", "Expected BY after GROUP");
    pos += 1;

    // The column to group results by
    let group_by = tokens[pos].clone();
    pos += 1;

    // The aggregation type: COUNT, SUM, or AVERAGE
    let agg_type = tokens[pos].clone();
    pos += 1;

    // The column the aggregation operates on
    let agg_col = tokens[pos].clone();

    let aggregation = match agg_type.as_str() {
        "COUNT"   => Aggregation::Count(agg_col),
        "SUM"     => Aggregation::Sum(agg_col),
        "AVERAGE" => Aggregation::Average(agg_col),
        _         => panic!("Unknown aggregation type: {}. Use COUNT, SUM, or AVERAGE", agg_type),
    };

    Query::new(condition, group_by, aggregation)
}

// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    // Establish connection to server.
    let rpc_client = start_client().await;

    // Get a handle to the standard input stream
    let stdin = std::io::stdin();

    // Lock the handle to gain access to BufRead methods like lines()
    println!("Enter your query:");
    for line_result in stdin.lock().lines() {
        // Handle potential errors when reading a line
        match line_result {
            Ok(query) => {
                if query == "exit" {
                    break;
                }

                // parse query.
                let query = parse_query_from_string(query);

                // Carry out query.
                let time = Instant::now();
                let dataset = solution::run_fast_rpc(&rpc_client, query).await;
                let duration = time.elapsed();

                // Print results.
                println!("{}", dataset);
                println!("Query took {:?} to executed", duration);
                println!("Enter your next query (or enter exit to stop):");
            },
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}