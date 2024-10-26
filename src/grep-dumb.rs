use std::io::{self, BufRead};
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let grep_arg = &args[1];

    let sdin = io::stdin();
    let handle = sdin.lock();

    for line in handle.lines() {
        match line {
            Ok(input) => {
                if input == *grep_arg {
                    println!("I found: {}", grep_arg);
                } else {
                    println!("I didn't find: {}", grep_arg);
                } 
            }
            Err(e) => {
                eprintln!("Error: {}", e)
            }
        }
    }
}
