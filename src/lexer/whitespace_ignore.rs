use super::{CharIter, Token, TokenClass, TokenQueue};

pub enum WhitespaceIgnore<C: TokenClass> {
    None,
    AllExceptNewline(C),
    All,
}

impl<C: TokenClass> WhitespaceIgnore<C> {
    pub fn ignore(&self, iter: &mut CharIter, token_queue: &mut TokenQueue<C>) -> bool {
        match self {
            WhitespaceIgnore::All => Self::ignore_all(iter),
            WhitespaceIgnore::AllExceptNewline(newline_token) => {
                Self::ignore_all_except_newline(iter, token_queue, newline_token)
            }
            WhitespaceIgnore::None => true,
        }
    }

    fn ignore_all_except_newline(
        iter: &mut CharIter,
        token_queue: &mut TokenQueue<C>,
        newline_token: &C,
    ) -> bool {
        loop {
            match iter.next() {
                Some(c) => {
                    if !c.is_whitespace() {
                        iter.unget(c);
                        return true;
                    } else {
                        if c == '\n' {
                            let (line, column) = iter.last_pos();
                            token_queue.insert(Token::new(newline_token.clone(), line, column))
                        }
                    }
                }
                None => return false,
            }
        }
    }

    fn ignore_all(iter: &mut CharIter) -> bool {
        loop {
            match iter.next() {
                Some(c) => {
                    if !c.is_whitespace() {
                        iter.unget(c);
                        return true;
                    }
                }
                None => return false,
            }
        }
    }
}
