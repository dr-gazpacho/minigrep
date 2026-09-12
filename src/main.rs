use std::{dbg, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
