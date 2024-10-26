use std::env::args;

fn main() {
    for arg in args().skip(1) {
        arg.split_whitespace().for_each(|word| { println!("{word}") });
    }
    match args().last() {
        Some(last_arg) => if let Some(last_word) = last_arg
            .split_whitespace()
            .last() {
                for _ in 0..3 {
                    println!("{last_word}");
                }
        }
        _ => (),
    }
}
