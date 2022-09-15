use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let querry = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", querry);
    println!("In the file {}", file_path);
}
