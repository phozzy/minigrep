use std::{env, fs};

fn parse_config(args: &[String]) -> (&str, &str) {
    let querry = &args[1];
    let file_path = &args[2];

    (querry, file_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (querry, file_path) = parse_config(&args);

    println!("Searching for {}", querry);
    println!("In the file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Shoule have been able to read the file");

    println!("With text:\n{contents}");
}
