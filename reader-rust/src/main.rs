use std::fs;

const LOGDIR: &str = "../";

fn main() {
    println!("Reading keys input...");
    let log = fs::read_to_string(LOGDIR).expect("Should have been able to read the file");;
    println!("{log}");

}
