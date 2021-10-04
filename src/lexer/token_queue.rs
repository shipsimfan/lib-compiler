use super::{Token, TokenClass};
use std::collections::VecDeque;

pub struct TokenQueue<C: TokenClass> {
    tokens: VecDeque<Token<C>>,
}

impl<C: TokenClass> TokenQueue<C> {
    pub fn new() -> Self {
        TokenQueue {
            tokens: VecDeque::new(),
        }
    }

    pub fn insert(&mut self, token: Token<C>) {
        self.tokens.push_back(token);
    }

    pub fn next(&mut self) -> Token<C> {
        self.tokens.pop_front().unwrap()
    }

    pub fn unget(&mut self, token: Token<C>) {
        self.tokens.push_front(token);
    }
}

impl<C: TokenClass> std::fmt::Display for TokenQueue<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.tokens {
            writeln!(f, "{}", token)?;
        }

        Ok(())
    }
}
