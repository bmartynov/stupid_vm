use super::lexer::{
    Token,
    Node,
    Declare,
    TokenNode,
    Expression,
    assembler::parse,
};

use std::convert::TryInto;

mod expr;
mod token;
mod symbol;

use symbol::{
    SymbolType,
    SymbolTable,
};
use crate::instruction::Instruction;

#[derive(Debug)]
pub enum ParserError {
    OpUnknown(String),
    ArgumentInvalid { token: Token },
    ArgumentCountMismatch { expected: usize, got: usize },
}

pub struct Parser {
    st: SymbolTable,
    instructions: Vec<Result<Instruction, ParserError>>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            st: SymbolTable::new(),
            instructions: Vec::new(),
        }
    }

    pub fn process(mut self, code: &str) -> Vec<Instruction> {
        let (data_segment, code_segment) = parse(code).expect("ok");

        let data_segment = data_segment.unwrap_or_default();

        self.process_data_segment(data_segment);
        self.process_code_segment(code_segment);


        self.instructions.into_iter().map(|instr| {
            println!("{:?}", instr);

            instr.unwrap()
        }).collect::<Vec<_>>()
    }

    fn process_data_segment(&mut self, data_segment: Vec<Node<Declare>>) {
        for decl in data_segment {
            self.process_data_decl(decl.expr);
        }
    }

    fn process_data_decl(&mut self, decl: Declare) {
        match decl {
            Declare::ConstI64(
                Node { expr: Token::Ident(ident), .. },
                Node { expr: Token::Int(i), .. },
            ) => {
                self.st.add(ident, SymbolType::Integer(i));
            }
            Declare::ConstString(
                Node { expr: Token::Ident(ident), .. },
                Node { expr: Token::String(s), .. },
            ) => {
                self.st.add(ident, SymbolType::String(s));
            }
            _ => panic!("unexpected form: {:?}", decl)
        }
    }

    fn process_code_segment(&mut self, code_segment: Vec<Node<Expression>>) {
        let prepared = code_segment.into_iter().
            enumerate().
            map(|(idx, expr)| {
                let start = expr.start;
                let end = expr.end;

                match expr.expr {
                    Expression::Call(_, _) => (None, expr),
                    Expression::Label(label, expr) => {
                        (Some((label, SymbolType::Label(idx))), Node {
                            start,
                            end,
                            expr: *expr,
                        })
                    }
                }
            }).collect::<Vec<_>>();

        for (sm, _) in &prepared {
            if let Some((label, symbol)) = sm {
                self.st.add(label.clone(), symbol.clone());
            }
        }

        for (_, expr) in prepared {
            if let Expression::Call(op, args) = expr.expr {
                let instruction = self.process_op_expression(op, args);

                self.instructions.push(instruction);
            }
        }
    }

    fn process_op_expression(&mut self, op: String, args: Vec<TokenNode>) -> Result<Instruction, ParserError> {
        match op.as_str() {
            "ret" => Ok(Instruction::RET),
            "hlt" => Ok(Instruction::HLT),
            "load" => {
                let instruction: expr::Load<Instruction> = (args, &self.st).try_into()?;

                Ok(instruction.0)
            }
            "cloop" => {
                let instruction: expr::CLoop<Instruction> = (args, &self.st).try_into()?;

                Ok(instruction.0)
            }
            "loop" => {
                let instruction: expr::Loop<Instruction> = (args, &self.st).try_into()?;

                Ok(instruction.0)
            }
            "inc" => {
                let instruction: expr::Inc<Instruction> = (args, &self.st).try_into()?;

                Ok(instruction.0)
            }
            "jmp" | "jmpe" | "jmpne" => {
                let instruction: expr::Jmp<Instruction> = (op.as_str(), args, &self.st).try_into()?;

                Ok(instruction.0)
            }
            "call" => {
                let instruction: expr::Call<Instruction> = (args, &self.st).try_into()?;

                Ok(instruction.0)
            }
            "add" | "sub" | "mul" | "div" => {
                let instruction: expr::Math<Instruction> = (op.as_str(), args).try_into()?;

                Ok(instruction.0)
            }
            "eq" | "neq" | "gte" | "lte" | "lt" | "gt" => {
                let instruction: expr::Cmp<Instruction> = (op.as_str(), args).try_into()?;

                Ok(instruction.0)
            }
            _ => panic!("unknown token: {:?}", op)
        }
    }
}


#[test]
fn test_lex_line_instruction() {
    let code = "
.data
.code
load $0 #400
call @test
hlt
test:
load $0 #500
ret
";


    let (data_segment, code_segment) = parse(code).expect("ok");
    let mut p = Parser::new();

    let data_segment = data_segment.unwrap_or_default();

    p.process_data_segment(data_segment);
    p.process_code_segment(code_segment);


    for instruction in p.instructions {
        println!("{:?}", instruction);
    }

    println!("{:?}", p.st);
}

// load $0 @label   0   0   -
// load $1 #3       1   1   -
// label1:          2   -   2
// mul $1 $0 $2     3   2   -
// load $0 #1       4   3   -
// label2:          5   -   4
// sub $2 $0 $1     6   4
