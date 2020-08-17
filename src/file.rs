use std::fs::{read_to_string};
use serde::{Deserialize, Serialize};
use serde_json::{Result};

#[derive(Serialize, Deserialize)]
struct Data {
    name: String,
    version: String,
    author: String,
}

pub fn read(file: &str) -> Result<()> {
    let res: Data = serde_json::from_str(file)?;

    println!("Welcome to: {}", res.name);
    println!("Version: {}", res.version);
    println!("By {}", res.author);

    Ok(())
}

pub fn create() {
    println!("Creating files");   
}

pub fn open(path: &str) -> String {
    let path = path;
    let res = read_to_string(path).unwrap();
    res
}

// pub fn create<T>(data: T) {
//     println!("Creating new data");
// }