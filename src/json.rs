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