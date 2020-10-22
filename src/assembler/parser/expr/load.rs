use std::convert::TryFrom;

use super::{
    Node,
    Token,
    SymbolTable,
    ParserError,
    Instruction,
};

pub struct Load<E>(pub E);

impl TryFrom<(Vec<Node<Token>>, &SymbolTable)> for Load<Instruction> {
    type Error = ParserError;

    fn try_from(value: (Vec<Node<Token>>, &SymbolTable)) -> Result<Self, Self::Error> {
        let (args, st) = value;
        if args.len() != 2 {
            return Err(ParserError::ArgumentCountMismatch { expected: 2, got: args.len() });
        }

        Ok(Load(match (&args[0].expr, &args[1].expr) {
            (Token::Register(r0), Token::Ident(ident)) => {
                let value = st.get_integer(ident).expect("should be an integer");

                Instruction::LOAD { rd: *r0, value }
            }
            (Token::Register(r0), Token::Int(i)) => {
                Instruction::LOAD { rd: *r0, value: *i }
            }
            tok => return Err(ParserError::ArgumentInvalid { token: tok.1.clone() })
        }))
    }
}