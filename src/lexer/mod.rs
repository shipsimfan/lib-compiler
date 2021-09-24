mod char_iter;
mod token;
mod token_class;
mod token_queue;
mod whitespace_ignore;

pub use char_iter::CharIter;
pub use token::Token;
pub use token_class::TokenClass;
pub use token_queue::TokenQueue;
pub use whitespace_ignore::WhitespaceIgnore;

pub type GetNextTokenFn<C, E> = fn(iter: &mut CharIter) -> Result<Option<Token<C>>, E>;

pub fn tokenize<C: TokenClass, E: std::error::Error>(
    input: String,
    get_next_token: GetNextTokenFn<C, E>,
    end_of_file_token: C,
    whitespace_ignore: WhitespaceIgnore<C>,
) -> Result<TokenQueue<C>, E> {
    let mut tokens = TokenQueue::new();
    let mut iter = CharIter::new(input);

    while iter.has_next() {
        // Ignore whitespace
        if !whitespace_ignore.ignore(&mut iter, &mut tokens) {
            break;
        }

        match get_next_token(&mut iter)? {
            Some(token) => tokens.insert(token),
            None => {}
        }
    }

    tokens.insert(Token::new(end_of_file_token, iter.line(), iter.column()));

    Ok(tokens)
}
