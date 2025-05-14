mod config;
mod checker;
mod worker;
 
use crate::config::Config;
use crate::worker::start_worker_pool;
use crate::checker::generate_json;

use std::{
    fs::File,
    io::Write,
    process::exit,
};

fn main(){
    let config = Config::parse().unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(2);

    });
    
    let results = start_worker_pool(&config);
    let json_data = generate_json(&results);
    let mut file = File::create("status.json").expect("Could not create status.json");
    file.write_all(json_data.as_bytes()).expect("Failed to write JSON");
}
