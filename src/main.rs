use std::{env, fs};

struct Config {
    querry: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let querry = args[1].clone();
        let file_path = args[2].clone();

        Config { querry, file_path }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.querry);
    println!("In the file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Shoule have been able to read the file");

    println!("With text:\n{contents}");
}
