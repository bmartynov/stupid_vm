mod assembler;
mod instruction;

use instruction::Instruction;

enum Step {
    Halt,
    PCNext,
    PCSet(usize),
}

struct VM {
    pub ir: [i32; 32],
    pub pc: usize,
    pub sp: usize,
    pub bp: usize,
    pub running: bool,
    pub remainder: i32,
    pub compare_flag: bool,
    pub loop_counter: usize,
    pub stack: Vec<usize>,
    pub instructions: Vec<Instruction>,
}

impl VM {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            pc: 0,
            sp: 0,
            bp: 0,
            ir: [0; 32],
            stack: Vec::new(),
            running: true,
            remainder: 0,
            loop_counter: 0,
            compare_flag: false,
        }
    }

    fn run(&mut self) {
        loop {
            match self.execute_instruction() {
                Step::Halt => break,
                Step::PCNext => self.pc += 1,
                Step::PCSet(pc) => {
                    self.pc = pc
                }
            }
        }
    }

    #[inline]
    fn execute_instruction(&mut self) -> Step {
        match self.instructions[self.pc] {
            Instruction::IGL => {
                return Step::Halt;
            }
            Instruction::HLT => {
                return Step::Halt;
            }
            Instruction::JMP { dst: r } => {
                return Step::PCSet(r);
            }
            Instruction::JMPE { dst: r } => {
                if self.compare_flag {
                    return Step::PCSet(r);
                }
            }
            Instruction::JMPNE { dst: r } => {
                if !self.compare_flag {
                    return Step::PCSet(r);
                }
            }
            Instruction::LOAD { rd, value } => {
                self.ir[rd] = value;
            }
            Instruction::RET => {
                self.sp = self.bp;
                self.bp = self.stack.pop().unwrap();

                return Step::PCSet(self.stack.pop().unwrap());
            }
            Instruction::CALL { dst: r } => {
                self.stack.push(self.pc + 1);
                self.stack.push(self.bp);
                self.bp = self.sp;

                return Step::PCSet(r);
            }
            Instruction::LOOP { dst: r } => {
                if self.loop_counter == 0 {
                    return Step::PCNext;
                }
                self.loop_counter -= 1;
                return Step::PCSet(r);
            }
            Instruction::CLOOP { count } => {
                self.loop_counter = count
            }
            Instruction::INC { r } => {
                self.ir[r] += 1;
            }
            Instruction::ADD { rd, rl, rh } => {
                self.ir[rd] = self.ir[rl] + self.ir[rh]
            }
            Instruction::SUB { rd, rl, rh } => {
                self.ir[rd] = self.ir[rl] - self.ir[rh]
            }
            Instruction::MUL { rd, rl, rh } => {
                self.ir[rd] = self.ir[rl] * self.ir[rh]
            }
            Instruction::DIV { rd, rl, rh } => {
                self.ir[rd] = self.ir[rl] / self.ir[rh];
                self.remainder = self.ir[rl] % self.ir[rh];
            }
            Instruction::EQ { rl, rh } => {
                self.compare_flag = self.ir[rl] == self.ir[rh];
            }
            Instruction::NEQ { rl, rh } => {
                self.compare_flag = self.ir[rl] != self.ir[rh];
            }
            Instruction::GTE { rl, rh } => {
                self.compare_flag = self.ir[rl] >= self.ir[rh];
            }
            Instruction::LTE { rl, rh } => {
                self.compare_flag = self.ir[rl] <= self.ir[rh];
            }
            Instruction::LT { rl, rh } => {
                self.compare_flag = self.ir[rl] < self.ir[rh];
            }
            Instruction::GT { rl, rh } => {
                self.compare_flag = self.ir[rl] > self.ir[rh];
            }
        }

        Step::PCNext
    }
}


const CODE: &str = r"
.data
.code
load $0 #10000000
load $2 #0
zaloop:
inc $2
eq $0 $2
jmpne @zaloop
hlt
";

fn main() {
    let parser = assembler::Parser::new();

    let instructions = parser.process(CODE);

    let mut vm = VM::new(instructions);

    vm.run();

    println!("{:?}", vm.ir);
}