
use std::env;
use std::io;
use std::process;
use std::str::Chars;

// fn match_pattern(input_line: &str, pattern: &str) -> bool {
//     if pattern.chars().count() == 1 {
//         return input_line.contains(pattern);
//     } else if pattern == r"\d" {
//         return input_line.chars().any(|c| c.is_numeric());
//     } else if pattern == r"\w" {
//         return input_line.chars().any(|c| c.is_alphanumeric());
//     } else if pattern.starts_with("[") && pattern.ends_with("]") {
//         let chars = pattern[1..pattern.len() - 1].chars().collect::<Vec<char>>();
//         return input_line.chars().any(|c| chars.contains(&c));
//     } else {
//         panic!("Unhandled pattern: {}", pattern)
//     }
// }

// fn match_pattern(input_line: &str, pattern: &str) -> bool {
//     match pattern {
//         s if s.chars().count() == 1 => input_line.contains(pattern),
//         r#"\d"# => input_line.parse::<i64>().is_ok(),
//         r#"\w"# => input_line.chars().all(|x| char::is_ascii_alphanumeric(&x)),
//         s if s.starts_with("[^") && s.ends_with(']') => {
//             let cuttern = &pattern[2..pattern.len() - 1];
//             !input_line.chars().any(|c| cuttern.contains(c))
//         }
//         s if s.starts_with('[') && s.ends_with(']') => pattern[1..pattern.len() - 1]
//             .chars()
//             .any(|c| input_line.contains(c)),
//         _ => false,
//     }
// }

#[derive(Debug)]
enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Group(bool, String),
}
fn match_literal(chars: &mut Chars, literal: char) -> bool {
    let c = chars.next();
    c.is_some_and(|c| c == literal)
}
fn match_digit(chars: &mut Chars) -> bool {
    let c = chars.next();
    if c.is_none() {
        return false;
    }
    c.unwrap().is_digit(10)
}
fn match_alphanumeric(chars: &mut Chars) -> bool {
    let c = chars.next();
    c.is_some_and(|c| c.is_alphanumeric())
}
fn match_group(chars: &mut Chars, group: &str) -> bool {
    let c = chars.next();
    c.is_some_and(|c| group.contains(c))
}
fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let patterns = build_patterns(pattern);
    let input_line = input_line.trim_matches('\n');
    'input_iter: for i in 0..input_line.len() {
        let input = &input_line[i..];
        let mut iter = input.chars();
        for pattern in patterns.iter() {
            match pattern {
                Pattern::Literal(l) => {
                    if !match_literal(&mut iter, *l) {
                        continue 'input_iter;
                    }
                }
                Pattern::Digit => {
                    if !match_digit(&mut iter) {
                        continue 'input_iter;
                    }
                }
                Pattern::Alphanumeric => {
                    if !match_alphanumeric(&mut iter) {
                        continue 'input_iter;
                    }
                }
                Pattern::Group(positive, group) => {
                    if match_group(&mut iter, group) != *positive {
                        continue 'input_iter;
                    }
                }
            }
        }
        return true;
    }
    return false;
}
fn build_group_pattern(iter: &mut Chars) -> (bool, String) {
    let mut group = String::new();
    let mut positive = true;
    if iter.clone().next().is_some_and(|c| c == '^') {
        positive = false;
        iter.next();
    }
    loop {
        let member = iter.next();
        if member.is_none() {
            panic!("Incomplete character group");
        }
        let member = member.unwrap();
        if member != ']' {
            group.push(member);
            continue;
        }
        break;
    }
    (positive, group)
}
fn build_patterns(pattern: &str) -> Vec<Pattern> {
    let mut iter = pattern.chars();
    let mut patterns = Vec::new();
    loop {
        let current = iter.next();
        if current.is_none() {
            break;
        }
        patterns.push(match current.unwrap() {
            '\\' => {
                let special = iter.next();
                if special.is_none() {
                    panic!("Incomplete special character")
                }
                match special.unwrap() {
                    'd' => Pattern::Digit,
                    'w' => Pattern::Alphanumeric,
                    '\\' => Pattern::Literal('\\'),
                    _ => panic!("Invalid special character"),
                }
            }
            '[' => {
                let (positive, group) = build_group_pattern(&mut iter);
                Pattern::Group(positive, group)
            }
            l => Pattern::Literal(l),
        })
    }
    patterns
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
