use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    experiment_independent_vars: Vec<Vec<String>>,
    replication_protocol: Vec<String>,
    client_protocol_mode: Vec<String>,
    server_names: Vec<String>,
    consistency: Vec<String>,
    base_python_directory: Vec<String>,
    python_venv: String,
    python_app_name: String,
    num_instances: i32,
    shard_config: String,
    replica_config: String,
    network_config: String,
    replica_config_format_str: String,
    shard_config_format_str: String,
    fault_tolerance: i32,
    num_shards: i32,
    pin_client_processes: Vec<i32>,
    pin_server_processes: Vec<i32>,
}

pub fn c()-> Result<(), Box<dyn std::error::Error>> {

    for _i in 0..100 {
        let file_path = "/home/akalaba/verifopt/storage-markers-expl/src/config.json";
        let contents = fs::read_to_string(file_path)?;
        // Convert the JSON string back to a Point.
        let mut deserialized: Config = serde_json::from_str(&contents).unwrap();

        // Use read in config to ensure it is not optimized away
        deserialized.num_shards += 1;
        // Convert the Point to a JSON string.
        let serialized = serde_json::to_string(&deserialized).unwrap();

        // Prints serialized = {"x":1,"y":2}
        fs::write(file_path, serialized)?;
    }
    Ok(())
}

fn main() {
    c();
}