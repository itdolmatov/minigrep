use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let content = fs::read_to_string(config.path).expect("file to read");
    println!("content {}", content);
}

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let path = args[2].clone();
        Config { query, path }
    }
}
