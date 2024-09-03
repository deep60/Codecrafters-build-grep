use core::panic;
use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 0 {
        panic!("Unhandled pattern: {}", pattern)
    }

    match pattern {
        "\\d" => input_line.contains(|c: char| c.is_digit(10)),
        "\\w" => input_line.chars().any(|c| c.is_alphanumeric()),
        pattern => {
            if pattern.chars().count() != 1 {
                panic!("Unhandled pattern: {}", pattern)
            }

            let input_c = pattern.chars().nth(0).unwrap();
            input_line.contains(input_c)
        }
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    //println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    // if match_pattern(&input_line, &pattern) {
    //     process::exit(0)
    // } else {
    //     process::exit(1)
    // }
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
