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

pub struct Cmp<E>(pub E);

impl TryFrom<(&str, Vec<Node<Token>>)> for Cmp<Instruction> {
    type Error = ParserError;

    fn try_from(value: (&str, Vec<Node<Token>>)) -> Result<Self, Self::Error> {
        let (op, args) = value;

        if args.len() != 2 {
            return Err(ParserError::ArgumentCountMismatch { expected: 2, got: args.len() });
        }

        let r0: Register = (&args[0]).try_into()?;
        let r1: Register = (&args[1]).try_into()?;

        Ok(Cmp(match op {
            "eq" => Instruction::EQ { rl: r0.0, rh: r1.0 },
            "neq" => Instruction::NEQ { rl: r0.0, rh: r1.0 },
            "gte" => Instruction::GTE { rl: r0.0, rh: r1.0 },
            "lte" => Instruction::LTE { rl: r0.0, rh: r1.0 },
            "lt" => Instruction::LT { rl: r0.0, rh: r1.0 },
            "gt" => Instruction::GT { rl: r0.0, rh: r1.0 },
            _ => return Err(ParserError::OpUnknown(op.to_string()))
        }))
    }
}
