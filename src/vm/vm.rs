use std::{path::PathBuf, fs::File, collections::HashMap};

use serde::{Serialize, Deserialize};
use crate::vm::memory::{Word,Memory};

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
    PUSH(Word),
    POP,
    SCOPY(Register),
    SMOVE(Register),
    RCOPY(Register,Register),
    RMOVE(Register,Register),
    WRITE(Word,Register),

    /* OPERATOR */
    ADD,
    MINUS,
    MUL,
    DIV,
    MOD,

    RADD(Either<Word,Register>,Either<Word,Register>),
    RMINUS(Either<Word,Register>,Either<Word,Register>),
    RMUL(Either<Word,Register>,Either<Word,Register>),
    RDIV(Either<Word,Register>,Either<Word,Register>),
    RMOD(Either<Word,Register>,Either<Word,Register>),


    /* FLOW */
    EXIT,
    NOP,
    LABEL,
    GO(usize),
    GOIF(usize),
    RGOIF(usize,Register),
    //CALL(usize),
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

#[derive(Debug, Copy, Clone,Deserialize,Serialize)]
pub enum Register{
    R1,
    R2,
    R3,
    R4,
    He,//Hermes : runtime flag (ex : negatif ...)
    Fl,//Flag
    Li,//Link
    Ni,//Next instruction pointer
}

pub struct Registers {
    R1 : Word,
    R2 : Word,
    R3 : Word,
    R4 : Word,
    He : Word,//Hermes : runtime flag (ex : negatif ...)
    Fl : Word,//Flag
    Li : Word,//Link
    Ni : Word,//Next instruction pointer
}
impl Registers {
    fn set(&mut self,register : Register,word : Word){
        match register {
            Register::R1 => self.R1 = word,
            Register::R2 => self.R2 = word,
            Register::R3 => self.R3 = word,
            Register::R4 => self.R4 = word,
            Register::He => self.He = word,
            Register::Fl => self.Fl = word,
            Register::Li => self.Li = word,
            Register::Ni => self.Ni = word,
        }
    }
    fn get(&self,register : Register) -> Word{
        match register {
            Register::R1 => self.R1,
            Register::R2 => self.R2,
            Register::R3 => self.R3,
            Register::R4 => self.R4,
            Register::He => self.He,
            Register::Fl => self.Fl,
            Register::Li => self.Li,
            Register::Ni => self.Ni,
        }
    }
}

impl Registers {
    fn init() -> Self{
        Self { 
            R1: Word::init(),
            R2: Word::init(),
            R3: Word::init(),
            R4: Word::init(),
            He: Word::init(),
            Fl: Word::init(),
            Li: Word::init(),
            Ni: Word::init(),
        }
    }
}



pub struct Vulkyn {
    memory : Memory,
    program : Program,
    registers : Registers,
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
            registers : Registers::init(),
        })
    }

    fn get_instruction(&self) -> Option<Instruction>{
        let word = {
            match self.registers.Ni{
                Word::U64(w) => w as usize,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
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
        let word = {
            match self.registers.Ni{
                Word::U64(w) => Word::U64(w+1),
                Word::I64(w) => Word::I64(w+1),
                Word::F64(w) => Word::F64(w+1.0),
                Word::CHAR(w) => Word::CHAR((w as u8 +1) as char),
            }
        };
        self.registers.Ni = word;
    }
    pub fn exec(&mut self) {
        loop {
            if let Some(instruction) = self.get_instruction(){
                let state = self.run(instruction);
                self.next_instruction();
                let flag = state.flag();
                self.registers.Fl = flag;
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
            Instruction::RADD(_,_) 
                | Instruction::RMINUS(_,_) 
                | Instruction::RMUL(_,_) 
                | Instruction::RDIV(_,_)
                | Instruction::RMOD(_,_) => {
                return self.r_operation(instruction);
            }
            Instruction::PUSH(word) => {
                self.memory.push(word);
                return State::OK
            },
            Instruction::POP => {
                let some_word = self.memory.pop();
                if some_word.is_err() {
                    return State::StackUnderflow
                }
            },
            Instruction::SCOPY(reg) => {
                let some_word = self.memory.peek();
                if some_word.is_err() {
                    return State::StackUnderflow
                }
                self.registers.set(reg, some_word.unwrap());
                return State::OK;
            },
            Instruction::SMOVE(reg) => {
                let some_word: Result<Word, super::memory::MemoryError> = self.memory.pop();
                if some_word.is_err() {
                    return State::StackUnderflow
                }
                self.registers.set(reg, some_word.unwrap());
                return State::OK;
            },
            Instruction::RCOPY(from, to) => {
                let word = self.registers.get(from);
                self.registers.set(to, word);
                return State::OK;
            },
            Instruction::RMOVE(from, to) => {
                let word = self.registers.get(from.clone());
                self.registers.set(to, word);
                self.registers.set(from, Word::init());
                return State::OK;
            },
            Instruction::WRITE(word, reg) => {
                self.registers.set(reg, word);
                return State::OK;
            },
            Instruction::NOP => {},
            Instruction::EXIT => {},
            Instruction::LABEL => {},
            Instruction::GO(label)=> {
                self.registers.Ni = Word::U64(label);
            },
            Instruction::GOIF(label) => {
                let some_word: Result<Word, super::memory::MemoryError> = self.memory.pop();
                if some_word.is_err() {
                    return State::StackUnderflow
                }
                let word = some_word.unwrap();
                if !word.is_zero() {
                    self.registers.Ni = Word::U64(label);
                }
            },
            Instruction::RGOIF(label,reg) => {
                let word = self.registers.get(reg);
                if !word.is_zero() {
                    self.registers.Ni = Word::U64(label);
                }
            },
        }
        return State::OK;
    }

    fn operation(&mut self,instruction : Instruction) -> State{
        match instruction {
            Instruction::ADD => {
                let some_x = self.memory.pop();
                if some_x.is_err() {
                    return State::StackUnderflow
                }
                let some_y = self.memory.pop();
                if some_y.is_err() {
                    return State::StackUnderflow
                }
                let result = some_x.unwrap()  + some_y.unwrap() ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::MINUS => {
                let some_x = self.memory.pop();
                if some_x.is_err() {
                    return State::StackUnderflow
                }
                let some_y = self.memory.pop();
                if some_y.is_err() {
                    return State::StackUnderflow
                }
                let result = some_x.unwrap()  - some_y.unwrap() ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::MUL => {
                let some_x = self.memory.pop();
                if some_x.is_err() {
                    return State::StackUnderflow
                }
                let some_y = self.memory.pop();
                if some_y.is_err() {
                    return State::StackUnderflow
                }
                let result = some_x.unwrap()  * some_y.unwrap() ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::MOD => {
                let some_x = self.memory.pop();
                if some_x.is_err() {
                    return State::StackUnderflow
                }
                let some_y = self.memory.pop();
                if some_y.is_err() {
                    return State::StackUnderflow
                }
                let result = some_x.unwrap()  % some_y.unwrap() ;
                self.memory.push(result);
                return State::OK
            }
            Instruction::DIV => {
                let some_x = self.memory.pop();
                if some_x.is_err() {
                    return State::StackUnderflow
                }
                let some_y = self.memory.pop();
                if some_y.is_err() {
                    return State::StackUnderflow
                }
                let y = some_y.unwrap() ;
                if y.is_zero() {
                    return State::DivisionZero;
                }
                let result = some_x.unwrap() / y;
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
            Either::Right(reg) => self.registers.get(reg)
        }
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
    fn exit(&self) {
        dbg!(&self.memory);
        if (self.registers.Fl & FLAG_OK) == FLAG_OK{
            println!("Successfuly exited program !")
        }
        if (self.registers.Fl & FLAG_ST_OF) == FLAG_ST_OF{
            println!("Error : state overflow")
        }
        if (self.registers.Fl & FLAG_ST_UF) == FLAG_ST_UF{
            println!("Error : state overflow")
        }
        if (self.registers.Fl & FLAG_I_I) == FLAG_I_I{
            println!("Error : illegal instruction")
        }
        if (self.registers.Fl & FLAG_DZ) == FLAG_DZ{
            println!("Error : Divizion per zero")
        }
    }
}