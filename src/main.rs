use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let path: &String = &args[2];
    
    println!("Searching for {query}");
    println!("In file {path}");
}
