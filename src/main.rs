mod json;
mod file;

fn main() {
    json::read(&file::open("data.json")).unwrap();
}