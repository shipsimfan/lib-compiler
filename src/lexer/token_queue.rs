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
}
