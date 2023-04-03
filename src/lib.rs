pub struct StringFinder<T>
where
    T: Iterator<Item = char>,
{
    state: State,
    chars: T,
    ignoring: bool,
    running_count: u32,
    target_count: u32,
    buffer: String,
    result: String,
}

enum State {
    Searching,
    CountingStart,
    InsideString,
    CountingEnd,
}

pub trait Strings<T>
where
    T: Iterator<Item = char>,
{
    fn words(self) -> StringFinder<T>;
}

impl<T> Strings<T> for T
where
    T: Iterator<Item = char>,
{
    fn words(self) -> StringFinder<T> {
        StringFinder::from(self)
    }
}

impl<T> Iterator for StringFinder<T>
where
    T: Iterator<Item = char>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while self.result.is_empty() {
            match self.chars.next() {
                Some(c) => self.process_char(c),
                None => return None,
            }
        }
        let mut result = String::new();
        std::mem::swap(&mut result, &mut self.result);
        Some(result)
    }
}

impl<T> From<T> for StringFinder<T>
where
    T: Iterator<Item = char>,
{
    fn from(chars: T) -> Self {
        Self::from(chars)
    }
}

impl<T> StringFinder<T>
where
    T: Iterator<Item = char>,
{
    pub fn from(chars: T) -> Self {
        Self {
            state: State::Searching,
            chars,
            ignoring: false,
            running_count: 0,
            target_count: 0,
            buffer: String::new(),
            result: String::new(),
        }
    }

    fn process_char(&mut self, c: char) {
        match self.state {
            State::Searching => self.search(c),
            State::CountingStart => self.count_start(c),
            State::InsideString => self.inside_string(c),
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
                self.state = State::InsideString;
                self.inside_string(c);
            }
        }
    }

    fn inside_string(&mut self, c: char) {
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
                    std::mem::swap(&mut self.buffer, &mut self.result);
                    self.state = State::Searching;
                }
            }
            _ => {
                for _ in 0..(self.target_count - self.running_count) {
                    self.buffer.push('"');
                }
                self.running_count = self.target_count;
                self.state = State::InsideString;
                self.inside_string(c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn simple_string() {
        let chars = r#"This is a "test" string!"#.chars();
        assert_eq!(
            vec!["test"],
            StringFinder::from(chars).collect::<Vec<String>>()
        );
    }

    #[test]
    fn multiple_strings_per_line() {
        let chars = r#"This "is" a "test" string!"#.chars();
        assert_eq!(
            vec!["is", "test"],
            StringFinder::from(chars).collect::<Vec<String>>()
        );
    }

    #[test]
    fn escaped_quotes_before_string() {
        let chars = r#"This \" is a "test""#.chars();
        assert_eq!(
            vec!["test"],
            StringFinder::from(chars).collect::<Vec<String>>()
        );
    }

    #[test]
    fn escaped_quotes_inside_string() {
        let chars = r#"This is a "huge \"test\"""#.chars();
        assert_eq!(
            vec![r#"huge \"test\""#],
            StringFinder::from(chars).collect::<Vec<String>>()
        );
    }

    #[test]
    fn tripe_quote_string() {
        let chars = r#"This is a """triple "super" test""""#.chars();
        assert_eq!(
            vec![r#"triple "super" test"#],
            StringFinder::from(chars).collect::<Vec<String>>()
        );
    }

    #[test]
    fn leading_escaped_string() {
        let chars = r#"There is a little \"trick "going on" here"#.chars();
        assert_eq!(
            vec!["going on"],
            StringFinder::from(chars).collect::<Vec<String>>()
        );
    }

    #[test]
    fn multi_line_string() {
        let chars = "There's a \"multi\nline\" string in this one!".chars();
        assert_eq!(
            vec!["multi\nline"],
            StringFinder::from(chars).collect::<Vec<String>>()
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
