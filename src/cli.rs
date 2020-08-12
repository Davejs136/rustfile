use std::io::{self, Write};
use std::process::{Command};

pub fn prompt(chr: &str) {
  print!("{}", chr);
  io::stdout().flush().unwrap();
}

pub fn input() -> String {
  let mut entry = String::new();
  io::stdin().read_line(&mut entry).unwrap();

  entry
}

pub fn output(word: &str) {
  println!("\n{}\n", word);
  io::stdout().flush().unwrap();
}

pub fn clear() {
  Command::new("clear").status().unwrap();
}

pub fn help() {
  println!("new [newdb]  ---> Create a new DB");
  println!("use [dbname] ---> Use database selected");
  println!("show         ---> Show all databases");
  println!("h            ---> Show this thelp");
  println!("Using some database");
}