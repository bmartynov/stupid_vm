use std::convert::{
    TryFrom,
    TryInto,
};

use super::{
    Node,
    Token,
    Register,
    ParserError,
    Instruction,
};

pub struct Math<E>(pub E);

impl TryFrom<(&str, Vec<Node<Token>>)> for Math<Instruction> {
    type Error = ParserError;

    fn try_from(value: (&str, Vec<Node<Token>>)) -> Result<Self, Self::Error> {
        let (op, args) = value;

        if args.len() != 3 {
            return Err(ParserError::ArgumentCountMismatch { got: args.len(), expected: 3 });
        }
        let r0: Register = (&args[0]).try_into()?;
        let r1: Register = (&args[1]).try_into()?;
        let r2: Register = (&args[2]).try_into()?;


        Ok(Math(match op {
            "add" => Instruction::ADD { rd: r0.0, rl: r1.0, rh: r2.0 },
            "sub" => Instruction::SUB { rd: r0.0, rl: r1.0, rh: r2.0 },
            "mul" => Instruction::MUL { rd: r0.0, rl: r1.0, rh: r2.0 },
            "div" => Instruction::DIV { rd: r0.0, rl: r1.0, rh: r2.0 },
            _ => return Err(ParserError::OpUnknown(op.to_string()))
        }))
    }
}