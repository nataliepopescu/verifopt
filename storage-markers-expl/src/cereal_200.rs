use std::{collections::HashMap, fs};
use std::hint::black_box;

type Graph = HashMap<String, Vec<String>>;

pub fn c_200()-> Result<(), Box<dyn std::error::Error>> {

    // for _i in 0..100 {
    let file_path = "/home/akalaba/verifopt/storage-markers-expl/src/200_graph.json";
    let contents = fs::read_to_string(file_path)?;
    // Convert the JSON string back to a Point.
    let deserialized: Graph = serde_json::from_str(&contents).unwrap();

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&deserialized).unwrap();

    // Use serialized string to ensure it is not optimized away
    black_box(&serialized);

    Ok(())
}

fn main() {
    c_200();
}