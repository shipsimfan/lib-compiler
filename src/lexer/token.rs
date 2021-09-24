use super::TokenClass;

pub struct Token<C: TokenClass> {
    class: C,
    line: usize,
    column: usize,
}

impl<C: TokenClass> Token<C> {
    pub fn new(class: C, line: usize, column: usize) -> Self {
        Token {
            class, line, column
        }
    }
}