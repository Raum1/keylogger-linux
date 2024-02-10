use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
use std::path::Path;

const LOGDIR: &str = "../log";

// Assigning each keycode with corresponding character or button
const KEYCODES: [&str; 120] = [
    "reserved", "ESC", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "BACKSPACE", "TAB", 
    "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "Enter", "Left Ctrl", "A", "S", "D", "F",
    "G", "H", "J", "K", "L", ";", "'", "~", "LEFT SHIFT", "\\", "Z", "X", "C", "V", "B", "N", "M", ",", ".",
    "/", "RIGHT SHIFT", "*", "LEFT ALT", "SPACE", "CAPSLOCK", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9",
    "F10", "NUMLOCK", "SCROLL LOCK", "KP7", "KP8", "KP9", "KPMINUS", "KP4", "KP5", "KP6", "KPPLUS", "KP1", "KP2",
    "KP3", "KP0", "KPDOT", "ZENKAKUHANKAKU", "<", "F11", "F12", "RO", "KATAKANA", "HIRAGANA", "HENKAN",
    "KATAKANAHIRAGANA", "MUHENKAN", "KPJPCOMMA", "KPENTER", "RIGHTCTRL", "KPSLASH", "SYSRQ", "RIGHTALT",
    "LINEFEED", "HOME", "KEY UP", "KEY PAGEUP", "KEY LEFT", "KEY RIGHT", "KEY END", "KEY DOWN", "PAGEDOWN",
    "INSERT", "DELETE", "MACRO", "MUTE", "VOLUME DOWN", "VOLUME UP", "POWER", "KP EQUAL", "KP PLUS MINUS", 
    "KEY PAUSE", "KEY SCALE"
];

//Read by line
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Sort table by value
fn print_sorted_table(table: &HashMap<i32, i32>){
    let mut key_codes: Vec<_> = table.iter().collect();
    key_codes.sort_by(|a, b| b.1.cmp(a.1));
    for (key_code, num) in key_codes{
        if *key_code < 120{ 
            println!("key {} is pressed {} times", KEYCODES[*key_code as usize], num);
        }
    }
}

// Print all the characters 
fn print_long_string(table: &HashMap<i32, i32>){
    let mut string: String = "".to_owned();
    for letter in table.keys(){
        if *letter < 120 {
            let value = table.get(letter);
            if let Some(val) = value {
                let char = KEYCODES[*letter as usize];
                if char.len() == 1 {
                    let pushed_str = char.repeat(*val as usize);
                    string.push_str(&pushed_str);
                }
            }
        }
    }
    println!("{}", string);
}

fn main() {
    let mut table: HashMap<i32, i32> = HashMap::new();

    //Represent each line as an element in Vec
    if let Ok(lines) = read_lines(LOGDIR) {
        for line in lines.flatten(){
            let key_code = line.parse::<i32>().unwrap();
            //Check for presence of current key code
            if !table.contains_key(&key_code) {
                //Create and assign value of 1
                table.insert(key_code, 1);
            }
            //Add one to the key's value
            else {
                if let Some(value) = table.get_mut(&key_code) {
                    *value += 1;
                }
            }
        }
    }
    print_sorted_table(&table);
    //print_long_string(&table);
    
} 
