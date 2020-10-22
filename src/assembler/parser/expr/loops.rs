use std::convert::{
    TryFrom,
    TryInto,
};

use super::{
    Node,
    Token,
    Ident,
    Int,
    SymbolTable,
    ParserError,
    Instruction,
};

pub struct Loop<E>(pub E);

impl TryFrom<(Vec<Node<Token>>, &SymbolTable)> for Loop<Instruction> {
    type Error = ParserError;

    fn try_from(value: (Vec<Node<Token>>, &SymbolTable)) -> Result<Self, Self::Error> {
        let (args, st) = value;
        if args.len() != 1 {
            return Err(ParserError::ArgumentCountMismatch { expected: 1, got: args.len() });
        }


        let ident: Ident = (&args[0]).try_into()?;

        let value = st.get_offset(&ident.0).expect("should be an integer");

        Ok(Loop(Instruction::LOOP { dst: value }))
    }
}


pub struct CLoop<E>(pub E);

impl TryFrom<(Vec<Node<Token>>, &SymbolTable)> for CLoop<Instruction> {
    type Error = ParserError;

    fn try_from(value: (Vec<Node<Token>>, &SymbolTable)) -> Result<Self, Self::Error> {
        let (args, _st) = value;
        if args.len() != 1 {
            return Err(ParserError::ArgumentCountMismatch { got: args.len(), expected: 1 });
        }

        let int: Int = (&args[0]).try_into()?;

        Ok(CLoop(Instruction::CLOOP { count: int.0 as usize }))
    }
}