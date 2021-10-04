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

pub type GetNextTokenFn<C> =
    fn(iter: &mut CharIter) -> Result<Option<Token<C>>, Box<dyn std::error::Error>>;

pub fn tokenize<C: TokenClass>(
    input: String,
    get_next_token: GetNextTokenFn<C>,
    end_of_file_token: C,
    whitespace_ignore: WhitespaceIgnore<C>,
) -> Result<TokenQueue<C>, Box<dyn std::error::Error>> {
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
