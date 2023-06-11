use serde::{Serialize, Deserialize};

use super::word::Word;

#[derive(Debug, Copy, Clone,Deserialize,Serialize)]
pub enum Register{
    R1,
    R2,
    R3,
    R4,
    Ts,// Top of the stack pointer
    Bs,// Bottom of the stack pointer
    He,// Hermes : runtime flag (ex : negatif ...)
    Fl,// Flag
    Li,// Link
    Ni,// Next instruction pointer
}

#[derive(Debug, Copy, Clone,Deserialize,Serialize)]
pub struct Registers {
    pub R1 : Word,
    pub R2 : Word,
    pub R3 : Word,
    pub R4 : Word,
    pub Ts : Word,// Top of the stack pointer
    pub Bs : Word,// Bottom of the stack pointer
    pub He : Word,// Hermes : runtime flag (ex : negatif ...)
    pub Fl : Word,// Flag
    pub Li : Word,// Link
    pub Ni : Word,// Next instruction pointer
}

impl Registers {
    pub fn set(&mut self,register : Register,word : Word){
        match register {
            Register::R1 => self.R1 = word,
            Register::R2 => self.R2 = word,
            Register::R3 => self.R3 = word,
            Register::R4 => self.R4 = word,
            Register::Ts => self.Ts = word,
            Register::Bs => self.Bs = word,
            Register::He => self.He = word,
            Register::Fl => self.Fl = word,
            Register::Li => self.Li = word,
            Register::Ni => self.Ni = word,
        }
    }
    pub fn get(&self,register : Register) -> Word{
        match register {
            Register::R1 => self.R1,
            Register::R2 => self.R2,
            Register::R3 => self.R3,
            Register::R4 => self.R4,
            Register::Ts => self.Ts,
            Register::Bs => self.Bs,
            Register::He => self.He,
            Register::Fl => self.Fl,
            Register::Li => self.Li,
            Register::Ni => self.Ni,
        }
    }
    pub fn init() -> Self{
        Self { 
            R1: Word::init(),
            R2: Word::init(),
            R3: Word::init(),
            R4: Word::init(),
            Ts: Word::init(),
            Bs: Word::init(),
            He: Word::init(),
            Fl: Word::init(),
            Li: Word::init(),
            Ni: Word::init(),
        }
    }
}
