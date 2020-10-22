use std::convert::{
    TryFrom,
    TryInto,
};

use super::{
    Node,
    Token,
    Register,
    SymbolTable,
    ParserError,
    Instruction,
};

pub struct Inc<E>(pub E);

impl TryFrom<(Vec<Node<Token>>, &SymbolTable)> for Inc<Instruction> {
    type Error = ParserError;

    fn try_from(value: (Vec<Node<Token>>, &SymbolTable)) -> Result<Self, Self::Error> {
        let (args, _st) = value;
        if args.len() != 1 {
            return Err(ParserError::ArgumentCountMismatch { expected: 1, got: args.len() });
        }


        let reg: Register = (&args[0]).try_into()?;

        Ok(Inc(Instruction::INC { r: reg.0 }))
    }
}
