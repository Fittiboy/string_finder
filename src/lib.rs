use std::io::{self, stdin};

pub fn strings_in_stdin() -> std::vec::IntoIter<String> {
    let lines = stdin_lines().unwrap();
    strings_from_lines(lines)
}

fn stdin_lines() -> Result<Vec<String>, io::Error> {
    stdin().lines().collect()
}

fn strings_from_lines(lines: Vec<String>) -> std::vec::IntoIter<String> {
    strings_from_line(lines.join("\n"))
}

fn strings_from_line(line: String) -> std::vec::IntoIter<String> {
    StringFinder::new(&line).find()
}

struct StringFinder<'a> {
    state: State,
    line: &'a str,
    ignoring: bool,
    running_count: u32,
    target_count: u32,
    buffer: Vec<char>,
    result: Vec<String>,
}

enum State {
    Searching,
    CountingStart,
    Adding,
    CountingEnd,
}

impl<'a> StringFinder<'a> {
    fn new(line: &'a str) -> Self {
        Self {
            state: State::Searching,
            line,
            ignoring: false,
            running_count: 0,
            target_count: 0,
            buffer: Vec::new(),
            result: Vec::new(),
        }
    }

    fn find(mut self) -> std::vec::IntoIter<String> {
        self.line.chars().for_each(|c| self.process_char(c));
        self.result.into_iter()
    }

    fn process_char(&mut self, c: char) {
        match self.state {
            State::Searching => self.search(c),
            State::CountingStart => self.count_start(c),
            State::Adding => self.add(c),
            State::CountingEnd => self.count_end(c),
        }
    }

    fn search(&mut self, c: char) {
        if !self.ignoring {
            match c {
                '"' => {
                    self.state = State::CountingStart;
                    self.count_start(c);
                }
                '\\' => self.ignoring = true,
                _ => {}
            }
        } else {
            self.ignoring = false;
        }
    }

    fn count_start(&mut self, c: char) {
        match c {
            '"' => self.running_count += 1,
            _ => {
                self.target_count = self.running_count;
                self.state = State::Adding;
                self.add(c);
            }
        }
    }

    fn add(&mut self, c: char) {
        if self.ignoring {
            self.buffer.push(c);
            self.ignoring = false;
        } else {
            match c {
                '"' => {
                    self.state = State::CountingEnd;
                    self.count_end(c);
                }
                '\\' => {
                    self.buffer.push(c);
                    self.ignoring = true;
                }
                _ => self.buffer.push(c),
            }
        }
    }

    fn count_end(&mut self, c: char) {
        match c {
            '"' => {
                self.running_count -= 1;
                if self.running_count == 0 {
                    self.target_count = 0;
                    self.result.push(self.buffer.iter().collect());
                    self.buffer = Vec::new();
                    self.state = State::Searching;
                }
            }
            _ => {
                for _ in 0..(self.target_count - self.running_count) {
                    self.buffer.push('"');
                }
                self.running_count = self.target_count;
                self.state = State::Adding;
                self.add(c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn simple_string() {
        let line = r#"This is a "test" string!"#.into();
        assert_eq!(
            vec!["test"],
            strings_from_line(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn simple_strings() {
        let lines = vec![r#"This is a "test" string!"#.into()];
        assert_eq!(
            vec!["test"],
            strings_from_lines(lines).collect::<Vec<String>>()
        );
    }

    #[test]
    fn multiple_strings_per_line() {
        let line = r#"This "is" a "test" string!"#.into();
        assert_eq!(
            vec!["is", "test"],
            strings_from_line(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn escaped_quotes_before_string() {
        let line = r#"This \" is a "test""#.into();
        assert_eq!(
            vec!["test"],
            strings_from_line(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn escaped_quotes_inside_string() {
        let line = r#"This is a "huge \"test\"""#.into();
        assert_eq!(
            vec![r#"huge \"test\""#],
            strings_from_line(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn tripe_quote_string() {
        let line = r#"This is a """triple "super" test""""#.into();
        assert_eq!(
            vec![r#"triple "super" test"#],
            strings_from_line(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn leading_escaped_string() {
        let line = r#"There is a little \"trick "going on" here"#.into();
        assert_eq!(
            vec!["going on"],
            strings_from_line(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn multi_line_string() {
        let line = "There's a \"multi\nline\" string in this one!".into();
        assert_eq!(
            vec!["multi\nline"],
            strings_from_line(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn multiple_lines() {
        let lines = vec![
            r#"This is a "simple" one!"#.into(),
            r#"This is a \""tougher" one!"#.into(),
            r#"There are """triple quotes""" in ""this"" one!"#.into(),
            "There is a \"multi\nline\" string in this one!".into(),
        ];
        assert_eq!(
            vec!["simple", "tougher", "triple quotes", "this", "multi\nline"],
            strings_from_lines(lines).collect::<Vec<String>>()
        );
    }
}
