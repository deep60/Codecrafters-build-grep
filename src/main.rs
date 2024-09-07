
// // fn match_pattern(input_line: &str, pattern: &str) -> bool {
// //     if pattern.chars().count() == 1 {
// //         return input_line.contains(pattern);
// //     } else if pattern == r"\d" {
// //         return input_line.chars().any(|c| c.is_numeric());
// //     } else if pattern == r"\w" {
// //         return input_line.chars().any(|c| c.is_alphanumeric());
// //     } else if pattern.starts_with("[") && pattern.ends_with("]") {
// //         let chars = pattern[1..pattern.len() - 1].chars().collect::<Vec<char>>();
// //         return input_line.chars().any(|c| chars.contains(&c));
// //     } else {
// //         panic!("Unhandled pattern: {}", pattern)
// //     }
// // }

// // fn match_pattern(input_line: &str, pattern: &str) -> bool {
// //     match pattern {
// //         s if s.chars().count() == 1 => input_line.contains(pattern),
// //         r#"\d"# => input_line.parse::<i64>().is_ok(),
// //         r#"\w"# => input_line.chars().all(|x| char::is_ascii_alphanumeric(&x)),
// //         s if s.starts_with("[^") && s.ends_with(']') => {
// //             let cuttern = &pattern[2..pattern.len() - 1];
// //             !input_line.chars().any(|c| cuttern.contains(c))
// //         }
// //         s if s.starts_with('[') && s.ends_with(']') => pattern[1..pattern.len() - 1]
// //             .chars()
// //             .any(|c| input_line.contains(c)),
// //         _ => false,
// //     }
// // }

// // #[derive(Debug)]
// // enum Pattern {
// //     Literal(char),
// //     Digit,
// //     Alphanumeric,
// //     Group(bool, String),
// // }
// // fn match_literal(chars: &mut Chars, literal: char) -> bool {
// //     let c = chars.next();
// //     c.is_some_and(|c| c == literal)
// // }
// // fn match_digit(chars: &mut Chars) -> bool {
// //     let c = chars.next();
// //     if c.is_none() {
// //         return false;
// //     }
// //     c.unwrap().is_digit(10)
// // }
// // fn match_alphanumeric(chars: &mut Chars) -> bool {
// //     let c = chars.next();
// //     c.is_some_and(|c| c.is_alphanumeric())
// // }
// // fn match_group(chars: &mut Chars, group: &str) -> bool {
// //     let c = chars.next();
// //     c.is_some_and(|c| group.contains(c))
// // }
// // fn match_pattern(input_line: &str, pattern: &str) -> bool {
// //     let patterns = build_patterns(pattern);
// //     let input_line = input_line.trim_matches('\n');
// //     'input_iter: for i in 0..input_line.len() {
// //         let input = &input_line[i..];
// //         let mut iter = input.chars();
// //         for pattern in patterns.iter() {
// //             match pattern {
// //                 Pattern::Literal(l) => {
// //                     if !match_literal(&mut iter, *l) {
// //                         continue 'input_iter;
// //                     }
// //                 }
// //                 Pattern::Digit => {
// //                     if !match_digit(&mut iter) {
// //                         continue 'input_iter;
// //                     }
// //                 }
// //                 Pattern::Alphanumeric => {
// //                     if !match_alphanumeric(&mut iter) {
// //                         continue 'input_iter;
// //                     }
// //                 }
// //                 Pattern::Group(positive, group) => {
// //                     if match_group(&mut iter, group) != *positive {
// //                         continue 'input_iter;
// //                     }
// //                 }
// //             }
// //         }
// //         return true;
// //     }
// //     return false;
// // }
// // fn build_group_pattern(iter: &mut Chars) -> (bool, String) {
// //     let mut group = String::new();
// //     let mut positive = true;
// //     if iter.clone().next().is_some_and(|c| c == '^') {
// //         positive = false;
// //         iter.next();
// //     }
// //     loop {
// //         let member = iter.next();
// //         if member.is_none() {
// //             panic!("Incomplete character group");
// //         }
// //         let member = member.unwrap();
// //         if member != ']' {
// //             group.push(member);
// //             continue;
// //         }
// //         break;
// //     }
// //     (positive, group)
// // }
// // fn build_patterns(pattern: &str) -> Vec<Pattern> {
// //     let mut iter = pattern.chars();
// //     let mut patterns = Vec::new();
// //     loop {
// //         let current = iter.next();
// //         if current.is_none() {
// //             break;
// //         }
// //         patterns.push(match current.unwrap() {
// //             '\\' => {
// //                 let special = iter.next();
// //                 if special.is_none() {
// //                     panic!("Incomplete special character")
// //                 }
// //                 match special.unwrap() {
// //                     'd' => Pattern::Digit,
// //                     'w' => Pattern::Alphanumeric,
// //                     '\\' => Pattern::Literal('\\'),
// //                     _ => panic!("Invalid special character"),
// //                 }
// //             }
// //             '[' => {
// //                 let (positive, group) = build_group_pattern(&mut iter);
// //                 Pattern::Group(positive, group)
// //             }
// //             l => Pattern::Literal(l),
// //         })
// //     }
// //     patterns
// // }

// // // Usage: echo <input_text> | your_program.sh -E <pattern>
// // fn main() {
// //     // You can use print statements as follows for debugging, they'll be visible when running tests.
// //     //println!("Logs from your program will appear here!");

// //     if env::args().nth(1).unwrap() != "-E" {
// //         println!("Expected first argument to be '-E'");
// //         process::exit(1);
// //     }

// //     let pattern = env::args().nth(2).unwrap();
// //     let mut input_line = String::new();

// //     io::stdin().read_line(&mut input_line).unwrap();

// //     // Uncomment this block to pass the first stage
// //     // if match_pattern(&input_line, &pattern) {
// //     //     process::exit(0)
// //     // } else {
// //     //     process::exit(1)
// //     // }
// //     if match_pattern(&input_line, &pattern) {
// //         process::exit(0)
// //     } else {
// //         process::exit(1)
// //     }
// // }


use std::env;
use std::io;
use std::process;

mod r#match;
mod pattern;

use colored::Colorize;
use pattern::parse;
use r#match::match_substring;

fn match_regex(input_line: &str, regex: &str) -> Option<String> {
    let mut input_line = input_line.trim().chars().peekable();
    let (patterns, start, end) = parse(regex);
    let mut groups = vec![];
    let mut current_group = String::new();

    loop {
        let mut input_start = input_line.clone();
        if patterns
            .iter()
            .all(|p| match_substring(&mut input_start, p, &mut groups, &mut current_group))
        {
            if !end || input_start.peek().is_none() {
                return Some(current_group);
            } else {
                return None;
            }
        }
        if start {
            // first and only match failed
            return None;
        }
        if input_line.next().is_none() {
            return None;
        }
        current_group.clear();
        groups.clear();
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    let first_arg = env::args().nth(1);
    if first_arg.is_none() || first_arg.unwrap() != "-E" {
        eprintln!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2);
    if pattern.is_none() {
        eprintln!("Expected second argument to be a pattern");
        process::exit(1);
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    if let Some(group) = match_regex(&input_line, &pattern.unwrap()) {
        let i = input_line.find(&group).unwrap();
        let j = i + group.len();
        print!(
            "{}{}{}",
            input_line[..i].normal(),
            input_line[i..j].bright_red().bold(),
            input_line[j..].normal()
        );
        process::exit(0)
    } else {
        process::exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_backreferences() {
        assert_eq!(
            match_regex(
                "'cat and cat' is the same as 'cat and cat'",
                r"('(cat) and \2') is the same as \1"
            ),
            Some("'cat and cat' is the same as 'cat and cat'".to_string())
        );
        assert!(match_regex(
            "'cat and cat' is the same as 'cat and dog'",
            r"('(cat) and \2') is the same as \1"
        )
        .is_none());
        assert_eq!(
            match_regex(
                "grep 101 is doing grep 101 times, and again grep 101 times",
                r"((\w\w\w\w) (\d\d\d)) is doing \2 \3 times, and again \1 times"
            ),
            Some("grep 101 is doing grep 101 times, and again grep 101 times".to_string())
        );
        assert!(match_regex(
            "$?! 101 is doing $?! 101 times, and again $?! 101 times",
            r"((\w\w\w) (\d\d\d)) is doing \2 \3 times, and again \1 times"
        )
        .is_none());
        assert!(match_regex(
            "grep yes is doing grep yes times, and again grep yes times",
            r"((\w\w\w\w) (\d\d\d)) is doing \2 \3 times, and again \1 times"
        )
        .is_none());
        assert_eq!(
            match_regex(
                "abc-def is abc-def, not efg, abc, or def",
                r"(([abc]+)-([def]+)) is \1, not ([^xyz]+), \2, or \3"
            ),
            Some("abc-def is abc-def, not efg, abc, or def".to_string())
        );
        assert!(match_regex(
            "efg-hij is efg-hij, not klm, efg, or hij",
            r"(([abc]+)-([def]+)) is \1, not ([^xyz]+), \2, or \3"
        )
        .is_none());
        assert!(match_regex(
            "abc-def is abc-def, not xyz, abc, or def",
            r"(([abc]+)-([def]+)) is \1, not ([^xyz]+), \2, or \3"
        )
        .is_none());
        assert_eq!(
            match_regex(
                "apple pie is made of apple and pie. love apple pie",
                r"^((\w+) (\w+)) is made of \2 and \3. love \1$"
            ),
            Some("apple pie is made of apple and pie. love apple pie".to_string())
        );
        assert!(match_regex(
            "pineapple pie is made of apple and pie. love apple pie",
            r"^((apple) (\w+)) is made of \2 and \3. love \1$"
        )
        .is_none());
        assert!(match_regex(
            "apple pie is made of apple and pie. love apple pies",
            r"^((\w+) (pie)) is made of \2 and \3. love \1$"
        )
        .is_none());
        assert_eq!(
            match_regex(
                "'howwdy hey there' is made up of 'howwdy' and 'hey'. howwdy hey there",
                r"'((how+dy) (he?y) there)' is made up of '\2' and '\3'. \1"
            ),
            Some(
                "'howwdy hey there' is made up of 'howwdy' and 'hey'. howwdy hey there".to_string()
            )
        );
        assert!(match_regex(
            "'howwdy heeey there' is made up of 'howwdy' and 'heeey'. howwdy heeey there",
            r"'((how+dy) (he?y) there)' is made up of '\2' and '\3'. \1"
        )
        .is_none());
        assert_eq!(
            match_regex(
                "cat and fish, cat with fish, cat and fish",
                r"((c.t|d.g) and (f..h|b..d)), \2 with \3, \1"
            ),
            Some("cat and fish, cat with fish, cat and fish".to_string())
        );
        assert!(match_regex(
            "bat and fish, bat with fish, bat and fish",
            r"((c.t|d.g) and (f..h|b..d)), \2 with \3, \1"
        )
        .is_none());
    }
}