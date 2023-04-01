use string_finder::strings_in_stdin;

fn main() {
    let strings = strings_in_stdin();
    for string in strings {
        println!("{}\n", string);
    }
}
