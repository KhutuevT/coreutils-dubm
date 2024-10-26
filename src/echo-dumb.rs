use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let echo_arg = &args[1];

    let words: Vec<_> = echo_arg
        .split(' ')
        .collect();

    let last_word = words[words.len()-1];

    println!("{}", echo_arg);

    for _ in 0..3 {
        println!("{}", last_word);
    }
}
