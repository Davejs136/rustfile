use std::fs::{read_to_string};

pub fn open(path: &str) -> String {
    let path = path;
    let res = read_to_string(path).unwrap();
    res
}

pub fn create<T>(data: T) {
    println!("Creating new data");
}