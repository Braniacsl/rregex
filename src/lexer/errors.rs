use crate::lexer::token::Token;


#[derive(Debug)]
pub enum ParseError{
    UnexpectedToken(Token),
    MismatchedParentheses,
    UnexpectedEOF,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ParseError::UnexpectedToken(t) => write!(f, "Unexpected Token: {}", t),
            ParseError::MismatchedParentheses => write!(f, "Uneven number of parentheses."),
            ParseError::UnexpectedEOF => write!(f, "Unexpected EOF."),
        }
    }
}