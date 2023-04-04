use std::io::stdin;
use string_finder::Strings;

fn main() {
    for string in stdin_lines() {
        println!("{}", string);
    }
}

fn stdin_lines() -> impl Iterator<Item = String> {
    stdin()
        .lines()
        .flat_map(|line| {
            let mut chars = line.unwrap().chars().collect::<Vec<_>>();
            chars.push('\n');
            chars
        })
        .strings()
}
