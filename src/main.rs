use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let querry = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", querry);
    println!("In the file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Shoule have been able to read the file");

    println!("With text:\n{contents}");
}
