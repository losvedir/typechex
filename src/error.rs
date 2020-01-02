use crate::parser::lexer;
use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum Error {
    Utf8Error,
    StdIOError(String),
    ParseError(String),
}

impl From<lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>> for Error {
    fn from(err: lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>) -> Self {
        Error::ParseError(format!("{:?}", err))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_err: std::string::FromUtf8Error) -> Self {
        Error::Utf8Error
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::StdIOError(format!("{:?}", err.kind()))
    }
}
