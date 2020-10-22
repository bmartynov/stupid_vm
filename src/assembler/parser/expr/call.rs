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

pub struct Call<E>(pub E);

impl TryFrom<(Vec<Node<Token>>, &SymbolTable)> for Call<Instruction> {
    type Error = ParserError;

    fn try_from(value: (Vec<Node<Token>>, &SymbolTable)) -> Result<Self, Self::Error> {
        let (args, st) = value;
        if args.len() != 1 {
            return Err(ParserError::ArgumentCountMismatch { got: args.len(), expected: 1 });
        }

        let ident: Ident = (&args[0]).try_into()?;

        let value = st.get_offset(&ident.0).expect("should be an integer");

        Ok(Call(Instruction::CALL { dst: value }))
    }
}