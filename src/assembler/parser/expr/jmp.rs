use std::convert::{
    TryFrom,
    TryInto,
};

use super::{
    Node,
    Token,
    Ident,
    SymbolTable,
    ParserError,
    Instruction,
};

pub struct Jmp<E>(pub E);

impl TryFrom<(&str, Vec<Node<Token>>, &SymbolTable)> for Jmp<Instruction> {
    type Error = ParserError;

    fn try_from(value: (&str, Vec<Node<Token>>, &SymbolTable)) -> Result<Self, Self::Error> {
        let (op, args, st) = value;
        if args.len() != 1 {
            return Err(ParserError::ArgumentCountMismatch { got: args.len(), expected: 1 });
        }

        let ident: Ident = (&args[0]).try_into()?;

        let dst = st.get_offset(&ident.0).expect("should be an integer");

        Ok(Jmp(match op {
            "jmp" => Instruction::JMP { dst },
            "jmpe" => Instruction::JMPE { dst },
            "jmpne" => Instruction::JMPNE { dst },
            _ => return Err(ParserError::OpUnknown(op.to_string())),
        }))
    }
}
