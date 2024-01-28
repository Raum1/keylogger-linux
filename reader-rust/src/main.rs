use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const LOGDIR: &str = "../log";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("Reading keys input...");
    let mut table: Vec<Vec<i32>> = Vec::with_capacity(2);
    if let Ok(lines) = read_lines(LOGDIR) {
        for line in lines.flatten(){
            let key_code = line.parse::<i32>().unwrap();
            println!("{}",line);
            if let Some(index) = table[0].iter().position(|&x| x == key_code){
                table[1][index] += 1;
            }
            else {
                table[0].push(key_code);
                table[1].push(1);
            }
        }
    }
    
}   
