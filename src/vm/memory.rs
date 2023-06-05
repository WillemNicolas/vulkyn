use std::ops::{BitAnd, Add, Sub, Mul, Div, Rem};

use serde::{Serialize, Deserialize};


#[derive(Debug,Clone,Copy,Serialize,Deserialize)]
pub enum Word{
    U64(usize),
    I64(isize),
    F64(f64),
    CHAR(char),
}
impl Word {
    pub fn init() -> Self{
        return Word::U64(0x0);
    }
    pub fn is_zero(self) -> bool {
        match self {
            Word::U64(w) => w == 0,
            Word::I64(w) => w == 0,
            Word::F64(w) => w == 0.0,
            Word::CHAR(w) => w as u8 == 0,
        }
    }
}
impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::U64(l0), Self::U64(r0)) => *l0 == *r0,
            (Self::I64(l0), Self::I64(r0)) => *l0 == *r0,
            (Self::F64(l0), Self::F64(r0)) => *l0 == *r0,
            (Self::CHAR(l0), Self::CHAR(r0)) => *l0 == *r0,
            _ => false,
        }
    }
}

impl Add for Word{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Word::U64(x) => match rhs {
                Word::U64(y) => Word::U64(x+y),
                Word::I64(y) => Word::I64(x as  isize + y),
                Word::F64(y) => Word::F64(x as f64 + y),
                Word::CHAR(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
            },
            Word::I64(x) =>  match rhs {
                Word::U64(y) => Word::I64(x+y as isize),
                Word::I64(y) => Word::I64(x + y),
                Word::F64(y) => Word::F64(x as f64 + y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
            },
            Word::F64(x) => match rhs {
                Word::U64(y) => Word::F64(x + y as f64),
                Word::I64(y) => Word::F64(x + y as f64),
                Word::F64(y) => Word::F64(x + y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
            },
            Word::CHAR(x) => match rhs {
                Word::U64(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
                Word::I64(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
                Word::F64(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
                Word::CHAR(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
            },
        }
    }
}

impl Sub for Word{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Word::U64(x) => match rhs {
                Word::U64(y) => Word::U64(x-y),
                Word::I64(y) => Word::I64(x as  isize - y),
                Word::F64(y) => Word::F64(x as f64 - y),
                Word::CHAR(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
            },
            Word::I64(x) =>  match rhs {
                Word::U64(y) => Word::I64(x-y as isize),
                Word::I64(y) => Word::I64(x - y),
                Word::F64(y) => Word::F64(x as f64 - y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
            },
            Word::F64(x) => match rhs {
                Word::U64(y) => Word::F64(x - y as f64),
                Word::I64(y) => Word::F64(x - y as f64),
                Word::F64(y) => Word::F64(x - y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
            },
            Word::CHAR(x) => match rhs {
                Word::U64(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
                Word::I64(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
                Word::F64(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
                Word::CHAR(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
            },
        }
    }
}

impl Mul for Word{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Word::U64(x) => match rhs {
                Word::U64(y) => Word::U64(x*y),
                Word::I64(y) => Word::I64(x as  isize * y),
                Word::F64(y) => Word::F64(x as f64 * y),
                Word::CHAR(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
            },
            Word::I64(x) =>  match rhs {
                Word::U64(y) => Word::I64(x*y as isize),
                Word::I64(y) => Word::I64(x * y),
                Word::F64(y) => Word::F64(x as f64 * y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
            },
            Word::F64(x) => match rhs {
                Word::U64(y) => Word::F64(x * y as f64),
                Word::I64(y) => Word::F64(x * y as f64),
                Word::F64(y) => Word::F64(x * y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
            },
            Word::CHAR(x) => match rhs {
                Word::U64(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
                Word::I64(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
                Word::F64(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
                Word::CHAR(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
            },
        }
    }
}

impl Div for Word{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Word::U64(x) => match rhs {
                Word::U64(y) => Word::U64(x/y),
                Word::I64(y) => Word::I64(x as  isize / y),
                Word::F64(y) => Word::F64(x as f64 / y),
                Word::CHAR(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
            },
            Word::I64(x) =>  match rhs {
                Word::U64(y) => Word::I64(x/y as isize),
                Word::I64(y) => Word::I64(x / y),
                Word::F64(y) => Word::F64(x as f64 / y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
            },
            Word::F64(x) => match rhs {
                Word::U64(y) => Word::F64(x / y as f64),
                Word::I64(y) => Word::F64(x / y as f64),
                Word::F64(y) => Word::F64(x / y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
            },
            Word::CHAR(x) => match rhs {
                Word::U64(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
                Word::I64(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
                Word::F64(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
                Word::CHAR(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
            },
        }
    }
}

impl Rem for Word{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Word::U64(x) => match rhs {
                Word::U64(y) => Word::U64(x%y),
                Word::I64(y) => Word::I64(x as  isize % y),
                Word::F64(y) => Word::F64(x as f64 % y),
                Word::CHAR(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
            },
            Word::I64(x) =>  match rhs {
                Word::U64(y) => Word::I64(x%y as isize),
                Word::I64(y) => Word::I64(x % y),
                Word::F64(y) => Word::F64(x as f64 % y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
            },
            Word::F64(x) => match rhs {
                Word::U64(y) => Word::F64(x % y as f64),
                Word::I64(y) => Word::F64(x % y as f64),
                Word::F64(y) => Word::F64(x % y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
            },
            Word::CHAR(x) => match rhs {
                Word::U64(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
                Word::I64(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
                Word::F64(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
                Word::CHAR(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
            },
        }
    }
}

impl BitAnd for Word {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let cmp_bytes = {
            match rhs {
                Word::U64(w) => w,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
            }
        };
        let self_bytes = {
            match self {
                Word::U64(w) => w,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
            }
        };
        let res = self_bytes & cmp_bytes;
        return Word::U64(res);
    }
}

#[derive(Debug)]
pub enum MemoryError {
    StackUnderflow,
    StackOverflow,
    StackSegmentationFault,
    HeapSegmentationFault,
}

#[derive(Debug)]
pub struct Memory {
    stack : Vec<Word>,
    heap : Vec<Word>,
    stack_size : usize,
    heap_size : usize
}

impl Memory {
    pub fn build() -> Self {
        Self {
            stack : Vec::new(),
            heap : Vec::new(),
            stack_size : 0,
            heap_size : 0,
        }
    }
    /* STACK ACCESS */
    pub fn push(&mut self,word : Word) {
        self.stack.push(word);
        self.stack_size += 1;
    }

    pub fn pop(&mut self) -> Result<Word,MemoryError> {
        let some_word = self.stack.pop();
        match some_word {
            Some(word) => {
                self.stack_size -= 1;
                return Ok(word);
            }
            None => {
                return Err(MemoryError::StackUnderflow);
            }
        }
    }
    pub fn peek(&mut self) -> Result<Word,MemoryError>{
        let some_word = self.stack.get(self.stack_size-1);
        match some_word {
            Some(word) => {
                self.stack_size -= 1;
                return Ok(*word);
            }
            None => {
                return Err(MemoryError::StackSegmentationFault);
            }
        }
    }

    /* HEAP ACCESS */
    pub fn read(&mut self) -> Result<Word,MemoryError> {
        Err(MemoryError::HeapSegmentationFault)
    }

    pub fn write(&mut self,word : Word) -> Result<Word,MemoryError> {
        Err(MemoryError::HeapSegmentationFault)
    }

    pub fn alloc(&mut self,size:usize)  -> Option<MemoryError>{
        Some(MemoryError::HeapSegmentationFault)
    }

    pub fn free(&mut self,pointer:usize,size:usize)  -> Option<MemoryError>{
        Some(MemoryError::HeapSegmentationFault)
    }
}