mod cli;
mod core;

use crate::core::file;

fn main() {
    let res = file::open("data.json");
    file::read(&res).unwrap();

    // cli::init();

    println!("\x1b[0;31mSO\x1b[0m");
}