use std::convert::TryFrom;

use super::{
    Node,
    Token,
    ParserError,
};

pub struct Register(pub usize);

impl TryFrom<&Node<Token>> for Register {
    type Error = ParserError;

    fn try_from(value: &Node<Token>) -> Result<Self, Self::Error> {
        match &value.expr {
            Token::Register(reg) => Ok(Register(*reg)),
            tok => Err(ParserError::ArgumentInvalid { token: tok.clone() })
        }
    }
}

pub struct Ident(pub String);

impl TryFrom<&Node<Token>> for Ident {
    type Error = ParserError;

    fn try_from(value: &Node<Token>) -> Result<Self, Self::Error> {
        match &value.expr {
            Token::Ident(reg) => Ok(Ident(reg.clone())),
            tok => Err(ParserError::ArgumentInvalid { token: tok.clone() })
        }
    }
}

pub struct Int(pub i32);

impl TryFrom<&Node<Token>> for Int {
    type Error = ParserError;

    fn try_from(value: &Node<Token>) -> Result<Self, Self::Error> {
        match &value.expr {
            Token::Int(reg) => Ok(Int(*reg)),
            tok => Err(ParserError::ArgumentInvalid { token: tok.clone() })
        }
    }
}