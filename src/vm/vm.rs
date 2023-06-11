use std::{path::PathBuf, fs::File};

use serde::{Serialize, Deserialize};
use crate::vm::memory::{Memory};

use super::{word::Word, register::{Register}};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Program{
    pub instructions : Vec<Instruction>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}



#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Instruction {
    /* MEMORY ACCESS */
    PUSH(Either<Word,Register>),
    POP,
    SCOPY(Register),
    SMOVE(Register),
    RCOPY(Register,Register),
    RMOVE(Register,Register),
    WRITE(Word,Register),
    SLOAD((Register,isize)),
    SLOADB((Register,isize),usize),

    /* OPERATOR */
    // +
    ADD,
    RADD(Either<Word,Register>,Either<Word,Register>),
    // -
    MINUS,
    RMINUS(Either<Word,Register>,Either<Word,Register>),
    // *
    MUL,
    RMUL(Either<Word,Register>,Either<Word,Register>),
    // /
    DIV,
    RDIV(Either<Word,Register>,Either<Word,Register>),
    // % 
    MOD,
    RMOD(Either<Word,Register>,Either<Word,Register>),

    // &
    BAND,
    RBAND(Either<Word,Register>,Either<Word,Register>),
    // |
    BOR,
    RBOR(Either<Word,Register>,Either<Word,Register>),
    // ^
    BXOR,
    RBXOR(Either<Word,Register>,Either<Word,Register>),
    // >>
    RSHIFT,
    RRSHIFT(Either<Word,Register>,Either<Word,Register>),
    // <<
    LSHIFT,
    RLSHIFT(Either<Word,Register>,Either<Word,Register>),


    // == 
    EQUAL,
    REQUAL(Either<Word,Register>,Either<Word,Register>),
    // !=
    DIFF,
    RDIFF(Either<Word,Register>,Either<Word,Register>),
    // ! 
    NOT,
    RNOT(Either<Word,Register>),
    // &&
    AND,
    RAND(Either<Word,Register>,Either<Word,Register>),
    // ||
    OR,
    ROR(Either<Word,Register>,Either<Word,Register>),

    // <
    LESS,
    RLESS(Either<Word,Register>,Either<Word,Register>),
    // <=
    ELESS,
    RELESS(Either<Word,Register>,Either<Word,Register>),
    // >
    GREAT,
    RGREAT(Either<Word,Register>,Either<Word,Register>),
    // >=
    EGREAT,
    REGREAT(Either<Word,Register>,Either<Word,Register>),


    /* FLOW */
    EXIT,
    NOP,
    LABEL,
    GO(usize),
    GOIF(usize),
    RGOIF(usize,Register),
    CALL(usize),
    SCALL,
    CALLP(usize,usize),
    SCALLP(usize),
    RCALL(Register),
    RCALLP(Register,usize),
    RET(usize),
}

pub enum State {
    OK,
    StackOverflow,
    StackUnderflow,
    IllegalInstruction,
    SegmentationFault,
    DivisionZero,
}

const FLAG_OK: Word = Word::U64(0x1 << 0);
const FLAG_ST_OF: Word =  Word::U64(0x1  << 1);
const FLAG_ST_UF: Word =   Word::U64(0x1 << 2);
const FLAG_I_I: Word =  Word::U64(0x1  << 3);
const FLAG_SF: Word =  Word::U64(0x1  << 4);
const FLAG_DZ: Word =  Word::U64(0x1  << 5);

impl State {
    fn flag(self) -> Word{
        match self {
            State::OK => FLAG_OK,
            State::StackOverflow => FLAG_ST_OF,
            State::StackUnderflow => FLAG_ST_UF,
            State::IllegalInstruction => FLAG_I_I,
            State::SegmentationFault => FLAG_SF,
            State::DivisionZero => FLAG_DZ,
        }
    }
}

pub struct Vulkyn {
    memory : Memory,
    program : Program,
}

impl Vulkyn {

    pub fn build(program : &PathBuf) -> Result<Self,()>{
        let program = bincode::deserialize_from::<File,Program>
            (File::open(program).unwrap());
        if program.is_err(){
            return Err(());
        }
        let program = program.unwrap();

        Ok(Self {
            memory:Memory::build(),
            program : program,
        })
    }

    fn get_instruction(&self) -> Option<Instruction>{
        let word = {
            match self.memory.registers.Ni {
                Word::U64(w) => w as usize,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
                Word::BOOL(w) => w as usize,
            }
        };
        let some_instruction = self.program.instructions.get(word);
        if some_instruction.is_none() {
            return None;
        }            
        let instruction = some_instruction.unwrap();
        if let Instruction::EXIT = instruction {
            return None;
        }
        return Some(instruction.clone());
    }
    pub fn next_instruction(&mut self) {
        let word =  self.memory.registers.Ni + Word::U64(1);
        self.memory.registers.Ni = word;
    }
    pub fn exec(&mut self) {
        loop {
            if let Some(instruction) = self.get_instruction(){
                let state = self.run(instruction);
                self.next_instruction();
                let flag = state.flag();
                self.memory.registers.Fl = flag;
                if flag & FLAG_OK != FLAG_OK {
                    break;
                }
            }else {
                break;
            }
        }
        self.exit()
    }

    fn run(&mut self,instruction : Instruction) -> State {
        match instruction {
            Instruction::ADD | Instruction::MINUS | Instruction::MUL | Instruction::DIV | Instruction::MOD=> {
                return self.operation(instruction);
            }
            Instruction::BAND | Instruction::BOR | Instruction::BXOR | Instruction::LSHIFT | Instruction::RSHIFT => {
                return self.bitewise_operation(instruction);
            }
            Instruction::RBAND(_,_) | Instruction::RBOR(_,_) | Instruction::RBXOR(_,_) | Instruction::RLSHIFT(_,_) | Instruction::RRSHIFT(_,_) => {
                return self.r_bitewise_operation(instruction);
            }
            Instruction::RADD(_,_) 
                | Instruction::RMINUS(_,_) 
                | Instruction::RMUL(_,_) 
                | Instruction::RDIV(_,_)
                | Instruction::RMOD(_,_) => {
                return self.r_operation(instruction);
            }
            Instruction::AND 
            | Instruction::OR 
            | Instruction::EQUAL 
            | Instruction::NOT 
            | Instruction::DIFF
            | Instruction::LESS
            | Instruction::GREAT
            | Instruction::ELESS
            | Instruction::EGREAT
             => {
                return self.boolean_operation(instruction);
            }
            Instruction::RAND(_,_) 
                | Instruction::ROR(_,_) 
                | Instruction::RLESS(_,_) 
                | Instruction::RGREAT(_,_)
                | Instruction::RELESS(_,_) 
                | Instruction::REGREAT(_,_)
                | Instruction::REQUAL(_,_)
                | Instruction::RDIFF(_,_)
                | Instruction::RNOT(_)
                => {
                return self.r_boolean_operation(instruction);
            },
            Instruction::CALL(_)
                | Instruction::CALLP(_, _)
                | Instruction::SCALL
                | Instruction::SCALLP(_)
                | Instruction::RCALL(_)
                | Instruction::RCALLP(_, _)
                | Instruction::RET(_)
                => {
                return self.function_operation(instruction);
            },
            Instruction::PUSH(either) => {
                self.memory.push(self.get_either(either));
                return State::OK
            },
            Instruction::POP => {
                let Ok(_) = self.memory.pop() else {
                    return State::StackUnderflow
                };
            },
            Instruction::SCOPY(reg) => {
                let some_word = self.memory.peek();
                if some_word.is_err() {
                    return State::StackUnderflow
                }
                self.memory.registers.set(reg, some_word.unwrap());
                return State::OK;
            },
            Instruction::SMOVE(reg) => {
                let Ok(word) = self.memory.pop() else {
                    return State::StackUnderflow
                };
                self.memory.registers.set(reg, word);
                return State::OK;
            },
            Instruction::RCOPY(from, to) => {
                let word = self.memory.registers.get(from.clone());
                self.memory.registers.set(to, word);
                return State::OK;
            },
            Instruction::RMOVE(from, to) => {
                let word = self.memory.registers.get(from.clone());
                self.memory.registers.set(to, word);
                self.memory.registers.set(from, Word::init());
                return State::OK;
            },
            Instruction::WRITE(word, reg) => {
                self.memory.registers.set(reg, word);
                return State::OK;
            },
            Instruction::SLOAD((reg,offset)) => {
                let idx = self.memory.registers.get(reg) + Word::I64(offset);
                let Ok(word) = self.memory.stack_read(idx) else {
                    return State::SegmentationFault;
                };
                self.memory.push(word);
                return State::OK;
            },
            Instruction::SLOADB((reg,offset),size ) => {
                let idx = self.memory.registers.get(reg) + Word::I64(offset);
                let Ok(words) = self.memory.stack_read_range(idx,size) else {
                    return State::SegmentationFault;
                };
                self.memory.extend(words);
                return State::OK;
            }
            /* FLOW */
            Instruction::NOP => {},
            Instruction::EXIT => {},
            Instruction::LABEL => {},
            Instruction::GO(label)=> {
                self.memory.registers.Ni = Word::U64(label);
            },
            Instruction::GOIF(label) => {
                let Ok(word) = self.memory.pop() else {
                    return State::StackUnderflow
                };
                if !word.is_zero() {
                    self.memory.registers.Ni = Word::U64(label);
                }
            },
            Instruction::RGOIF(label,reg) => {
                let word = self.memory.registers.get(reg);
                if !word.is_zero() {
                    self.memory.registers.Ni = Word::U64(label);
                }
            },
        }
        return State::OK;
    }

    fn operation(&mut self,instruction : Instruction) -> State{
        let Ok(x) = self.memory.pop() else {
            return State::StackUnderflow
        };
        let Ok(y) = self.memory.pop() else {
            return State::StackUnderflow
        };
        match instruction {
            Instruction::ADD => {
                let result = x  + y ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::MINUS => {
                let result = x  - y ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::MUL => {
                let result = x  * y ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::MOD => {
                let result = x  % y ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::DIV => {
                if y.is_zero() {
                    return State::DivisionZero;
                }
                let result = x / y;
                self.memory.push(result);
                return State::OK
            }
            _ => {}
        }
        return State::OK
    }
    fn get_either(&self,e: Either<Word, Register>) -> Word{
        match e {
            Either::Left(word) => word,
            Either::Right(reg) => self.memory.registers.get(reg)
        }
    }
    fn bitewise_operation(&mut self,instruction : Instruction) -> State{
        let Ok(x) = self.memory.pop() else {
            return State::StackUnderflow
        };
        let Ok(y) = self.memory.pop() else {
            return State::StackUnderflow
        };
        match instruction {
            Instruction::BAND  => {
                let result = x & y ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::BOR  => {
                let result = x | y ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::BXOR  => {
                let result = x ^ y ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::LSHIFT  => {
                let result = x << y ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::RSHIFT => {
                let result = x >> y ;
                self.memory.push(result);
                return State::OK
            }
            _ => {}
        }
        return State::OK;
    }
    fn r_bitewise_operation(&mut self,instruction : Instruction) -> State{
        match instruction {
            Instruction::RBAND(e1,e2)  => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x & y ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::RBOR(e1,e2)  => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x | y ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::RBXOR(e1,e2)  => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x ^ y ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::RLSHIFT(e1,e2)  => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x << y ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::RRSHIFT(e1,e2) => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x >> y ;
                self.memory.push(result);
                return State::OK
            }
            _ => {}
        }
        return State::OK;
    }
    fn boolean_operation(&mut self,instruction : Instruction) -> State{
        let Ok(x) = self.memory.pop() else {
            return State::StackUnderflow
        };
        match instruction {
            Instruction::NOT => {
                self.memory.push(x.neg());
                return State::OK;
            }
            _ => {}
        }
        let Ok(y) = self.memory.pop() else {
            return State::StackUnderflow
        };
        match instruction {
            Instruction::AND => {
                let result = x.and(&y);
                self.memory.push(result);
                return State::OK;
            }
            Instruction::OR  => {
                let result = x.or(&y);
                self.memory.push(result);
                return State::OK;
            }
            Instruction::LESS  => {
                let result = x < y;
                self.memory.push(Word::BOOL(result));
                return State::OK;
            }
            Instruction::GREAT => {
                let result = x > y;
                self.memory.push(Word::BOOL(result));
                return State::OK;
            }
            Instruction::ELESS  => {
                let result = x <= y;
                self.memory.push(Word::BOOL(result));
                return State::OK;
            }
            Instruction::EGREAT => {
                let result = x >= y;
                self.memory.push(Word::BOOL(result));
                return State::OK;
            }
            Instruction::EQUAL => {
                let result = x == y;
                self.memory.push(Word::BOOL(result));
                return State::OK;
            }
            Instruction::DIFF => {
                let result = x != y;
                self.memory.push(Word::BOOL(result));
                return State::OK;
            }
            _ => {}
        }
        return State::OK;
    }
    fn r_boolean_operation(&mut self,instruction : Instruction) -> State{
        match instruction {
            Instruction::RAND(e1,e2) => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x.and(&y);
                self.memory.push(result);
                return State::OK;
            }
            | Instruction::ROR(e1,e2)  => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x.or(&y);
                self.memory.push(result);
                return State::OK;
            }
            | Instruction::RLESS(e1,e2)  => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x < y;
                self.memory.push(Word::BOOL(result));
                return State::OK;
            }
            | Instruction::RGREAT(e1,e2) => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x > y;
                self.memory.push(Word::BOOL(result));
                return State::OK;
            }
            | Instruction::RELESS(e1,e2)  => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x <= y;
                self.memory.push(Word::BOOL(result));
                return State::OK;
            }
            | Instruction::REGREAT(e1,e2) => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x >= y;
                self.memory.push(Word::BOOL(result));
                return State::OK;
            }
            | Instruction::REQUAL(e1,e2) => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x == y;
                self.memory.push(Word::BOOL(result));
                return State::OK;
            }
            | Instruction::RDIFF(e1,e2) => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x != y;
                self.memory.push(Word::BOOL(result));
                return State::OK;
            }
            | Instruction::RNOT(e) => {
                let x = self.get_either(e);
                self.memory.push(x.neg());
                return State::OK;
            }
            _ => {}
        }
        return State::OK;
    }
    fn r_operation(&mut self,instruction : Instruction) -> State{
        match instruction {
            Instruction::RADD(e1,e2) => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x + y;
                self.memory.push(result);
                return State::OK;
            }
            Instruction::RMINUS(e1,e2) => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x - y;
                self.memory.push(result);
                return State::OK;
            }
            Instruction::RMUL(e1,e2) => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x * y;
                self.memory.push(result);
                return State::OK;
            }
            Instruction::RMOD(e1,e2) => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                let result = x % y;
                self.memory.push(result);
                return State::OK;
            }
            Instruction::RDIV(e1,e2) => {
                let x = self.get_either(e1);
                let y = self.get_either(e2);
                if y.is_zero() {
                    return State::DivisionZero;
                }
                let result = x / y;
                self.memory.push(result);
                return State::OK;
            }
            _ => {}
        }
        return State::OK
    }
    fn function_operation(&mut self,instruction : Instruction) -> State{
        match instruction {
            Instruction::CALL(label) => {
                self.memory.push(self.memory.registers.Ni);
                self.memory.registers.Li = self.memory.registers.Ts;
                self.memory.registers.Ni = Word::U64(label);
            },

            | Instruction::CALLP(label, size) => {
                let Ok(addr) = self.memory.insert(
                    self.memory.registers.Ni,size) 
                else {
                    return State::StackUnderflow
                };
                self.memory.registers.Li = addr;
                self.memory.registers.Ni = Word::U64(label);
            },
            | Instruction::SCALL => {
                self.memory.push(self.memory.registers.Ni);
                self.memory.registers.Li = self.memory.registers.Ts;
                let Ok(word) = self.memory.pop() else {
                    return State::StackUnderflow
                };
                self.memory.registers.Ni = word;
            },
            | Instruction::SCALLP(size) => {
                let Ok(addr) = self.memory.insert(
                    self.memory.registers.Ni,size) 
                else {
                    return State::StackUnderflow
                };
                let Ok(word) = self.memory.pop() else {
                    return State::StackUnderflow
                };
                self.memory.registers.Li = addr;
                self.memory.registers.Ni = word;
            },
            | Instruction::RCALL(reg) => {
                let word = self.memory.registers.get(reg);
                self.memory.registers.Li = self.memory.registers.Ts;
                self.memory.push(self.memory.registers.Ni);
                self.memory.registers.Ni = word;
            },
            | Instruction::RCALLP(reg, size) => {
                self.memory.push(self.memory.registers.Ni);
                let word = self.memory.registers.get(reg);
                let Ok(addr) = self.memory.insert(
                    self.memory.registers.Ni,size) 
                else {
                    return State::StackUnderflow
                };
                self.memory.registers.Li = addr;
                self.memory.registers.Ni = word;
            },
            Instruction::RET(size) => {
                let start = self.memory.registers.Li.as_usize();
                let end = self.memory.stack_size - size;
                let Ok(word) = self.memory.stack_read(self.memory.registers.Li) else {
                    return State::StackOverflow;
                };
                self.memory.registers.Ni = word;
                if let Err(_) = self.memory.stack_clean(start,end){
                    return State::SegmentationFault
                }
            }
            _ => {}
        }
        return State::OK
    }
    fn exit(&self) {
        dbg!(&self.memory);
        if (self.memory.registers.Fl & FLAG_OK) == FLAG_OK{
            println!("Successfuly exited program !")
        }
        if (self.memory.registers.Fl & FLAG_ST_OF) == FLAG_ST_OF{
            println!("Error : state overflow")
        }
        if (self.memory.registers.Fl & FLAG_ST_UF) == FLAG_ST_UF{
            println!("Error : state overflow")
        }
        if (self.memory.registers.Fl & FLAG_I_I) == FLAG_I_I{
            println!("Error : illegal instruction")
        }
        if (self.memory.registers.Fl & FLAG_DZ) == FLAG_DZ{
            println!("Error : Divizion per zero")
        }
    }
}