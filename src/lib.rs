use core::str::Chars;

pub struct StringFinder<'a> {
    state: State,
    line: Chars<'a>,
    ignoring: bool,
    running_count: u32,
    target_count: u32,
    buffer: Vec<char>,
    result: String,
}

enum State {
    Searching,
    CountingStart,
    Adding,
    CountingEnd,
}

impl<'a> Iterator for StringFinder<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while self.result.is_empty() {
            match self.line.next() {
                Some(c) => self.process_char(c),
                None => return None,
            }
        }
        let mut result = String::new();
        std::mem::swap(&mut result, &mut self.result);
        Some(result)
    }
}

impl<'a> StringFinder<'a> {
    pub fn from(line: Chars<'a>) -> Self {
        Self {
            state: State::Searching,
            line,
            ignoring: false,
            running_count: 0,
            target_count: 0,
            buffer: Vec::new(),
            result: String::new(),
        }
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
            self.buffer.push(c.clone());
            self.ignoring = false;
        } else {
            match c {
                '"' => {
                    self.state = State::CountingEnd;
                    self.count_end(c);
                }
                '\\' => {
                    self.buffer.push(c.clone());
                    self.ignoring = true;
                }
                _ => self.buffer.push(c.clone()),
            }
        }
    }

    fn count_end(&mut self, c: char) {
        match c {
            '"' => {
                self.running_count -= 1;
                if self.running_count == 0 {
                    self.target_count = 0;
                    self.result = self.buffer.iter().collect();
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
        let line = r#"This is a "test" string!"#.chars();
        assert_eq!(
            vec!["test"],
            StringFinder::from(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn multiple_strings_per_line() {
        let line = r#"This "is" a "test" string!"#.chars();
        assert_eq!(
            vec!["is", "test"],
            StringFinder::from(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn escaped_quotes_before_string() {
        let line = r#"This \" is a "test""#.chars();
        assert_eq!(
            vec!["test"],
            StringFinder::from(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn escaped_quotes_inside_string() {
        let line = r#"This is a "huge \"test\"""#.chars();
        assert_eq!(
            vec![r#"huge \"test\""#],
            StringFinder::from(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn tripe_quote_string() {
        let line = r#"This is a """triple "super" test""""#.chars();
        assert_eq!(
            vec![r#"triple "super" test"#],
            StringFinder::from(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn leading_escaped_string() {
        let line = r#"There is a little \"trick "going on" here"#.chars();
        assert_eq!(
            vec!["going on"],
            StringFinder::from(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn multi_line_string() {
        let line = "There's a \"multi\nline\" string in this one!".chars();
        assert_eq!(
            vec!["multi\nline"],
            StringFinder::from(line).collect::<Vec<String>>()
        );
    }

    #[test]
    fn multiple_lines() {
        let lines: String = vec![
            r#"This is a "simple" one!"#.to_string(),
            r#"This is a \""tougher" one!"#.to_string(),
            r#"There are """triple quotes""" in ""this"" one!"#.to_string(),
            "There is a \"multi\nline\" string in this one!".to_string(),
        ]
        .join("\n");
        assert_eq!(
            vec!["simple", "tougher", "triple quotes", "this", "multi\nline"],
            StringFinder::from(lines.chars()).collect::<Vec<String>>()
        );
    }
}
