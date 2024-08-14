use crate::parser::ParsingError;

pub fn check_next_token<'a, I: Iterator<Item = &'a str>>(
    tokens: &mut I,
    expected: &'static str,
) -> Result<(), ParsingError> {
    match tokens.next() {
        Some(token) => {
            if token != expected {
                return Err(ParsingError::UnexpectedToken {
                    expected: expected,
                    found: token.to_string(),
                });
            } else {
                return Ok(());
            }
        }
        None => Err(ParsingError::UnexpectedEndOfTokens),
    }
}

