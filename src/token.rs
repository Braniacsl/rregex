
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Literal(char),
    Union,
    Star,
    LParen,
    RParen,
    Unknown(char)
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Token::Literal(c) => write!(f, "{c}"),
            Token::Union => write!(f, "|"),
            Token::Star => write!(f, "*"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Unknown(c) => write!(f, "{c}"),
        }
    }
}