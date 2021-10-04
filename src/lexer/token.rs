use super::TokenClass;

pub struct Token<C: TokenClass> {
    class: C,
    line: usize,
    column: usize,
}

impl<C: TokenClass> Token<C> {
    pub fn new(class: C, line: usize, column: usize) -> Self {
        Token {
            class,
            line,
            column,
        }
    }

    pub fn class(&self) -> &C {
        &self.class
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

impl<C: TokenClass> std::fmt::Display for Token<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}:{}", self.class, self.line, self.column)
    }
}
