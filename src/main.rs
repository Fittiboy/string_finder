use std::io::{self, stdin};
use string_finder::StringFinder;

fn main() {
    let lines = stdin_lines().unwrap().join("\n");
    let strings = StringFinder::from(lines.chars());
    for string in strings {
        println!("{}\n", string);
    }
}

fn stdin_lines() -> Result<Vec<String>, io::Error> {
    stdin().lines().collect()
}
