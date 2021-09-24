use std::collections::VecDeque;

pub struct CharIter {
    iter: VecDeque<char>,
    line: usize,
    column: usize,
    last_columns: Vec<usize>,
}

impl CharIter {
    pub fn new(string: String) -> Self {
        CharIter {
            iter: string.chars().collect(),
            line: 1,
            column: 1,
            last_columns: Vec::new(),
        }
    }

    pub fn next(&mut self) -> Option<char> {
        match self.iter.pop_front() {
            Some(c) => {
                if c == '\n' {
                    self.line += 1;
                    self.last_columns.push(self.column);
                    self.column = 1;
                } else {
                    self.column += 1;
                }
                Some(c)
            }
            None => None,
        }
    }

    pub fn unget(&mut self, c: char) {
        if c == '\n' {
            if self.last_columns.len() == 0 {
                panic!("Ungetting \\n while on line 1!");
            }

            self.line -= 1;
            self.column = self.last_columns.pop().unwrap();
        } else {
            self.column -= 1;
        }

        self.iter.push_front(c);
    }

    pub fn last_pos(&mut self) -> (usize, usize) {
        if self.column == 1 {
            if self.last_columns.len() == 0 {
                panic!("Getting last position with no last!");
            }

            let column = self.last_columns.pop().unwrap();
            self.last_columns.push(column);
            (self.line - 1, column)
        } else {
            (self.line, self.column - 1)
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn has_next(&self) -> bool {
        self.iter.front().is_some()
    }
}
