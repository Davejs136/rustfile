use std::io::{self, Write};
use std::process::Command;

fn prompt(chr: &str) {
    print!("{}", chr);
    io::stdout().flush().unwrap();
}

fn input() -> String {
    let mut entry = String::new();
    io::stdin().read_line(&mut entry).unwrap();

    entry
}

fn output(word: &str) {
    println!("\n{}\n", word);
    io::stdout().flush().unwrap();
}

fn clear() {
    Command::new("clear").status().unwrap();
}

fn help() {
    println!("new [newdb] ---> create a new DB");
    println!("new [newdb]  ---> Create a new DB");
    println!("use [dbname] ---> Use database selected");
    println!("show         ---> Show all databases");
    println!("h            ---> Show this thelp");
    println!("Using some database");
}

pub fn init() {
    output("Hi");
    loop {
        prompt("> ");
        let option = input().trim().to_string();

        dbg!(&option);

        match &option as &str {
            "help" | "h" => help(),
            "clear" => clear(),
            "exit" | "quit" | "q" => break,
            _ => println!("command no valid"),
        }
    }
}
