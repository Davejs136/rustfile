mod json;
mod file;
mod cli;

fn main() {
    // let mut option = String::new();
    let res = file::open("data.json");
    json::read(&res).unwrap();

    loop {
        cli::prompt("> ");
        let option = cli::input().trim().to_string();

        dbg!(&option);

        match &option as &str {
            "help" | "h" => cli::help(),
            "clear" => cli::clear(),
            "exit" | "quit" | "q" => break,
            _ => println!("command no valid")
        }
    }
}