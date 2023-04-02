use std::io::{self, stdin};
use string_finder::StringFinder;

fn main() {
    let lines = stdin_lines();
    let strings: StringFinder = lines.chars().into();
    for string in strings {
        println!("{}\n", string);
    }
}

fn stdin_lines() -> String {
    stdin()
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()
        .unwrap()
        .join("\n")
}
