use std::fs;

const LOGDIR: &str = "../";

fn main() {
    println!("Reading keys input...");
    let log = fs::read_to_string(LOGDIR).unwrap().lines();
    println!("{log}");

}
