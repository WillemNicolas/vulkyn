
use std::{collections::{HashSet, HashMap}, borrow::BorrowMut};

use serde::{Serialize, Deserialize};

use super::{word::Word, register::{Registers, Register}};

#[derive(Debug)]
pub enum MemoryError {
    StackUnderflow,
    StackOverflow,
    StackSegmentationFault,
    HeapSegmentationFault,
}

#[derive(Debug)]
pub struct Memory{
    stack : Vec<Word>,
    heap : HashMap<usize, Word>,
    pub stack_size : usize,
    pub registers : Registers,
}

impl Memory{
    pub fn build() -> Self {
        Self {
            stack : Vec::new(),
            heap : HashMap::new(),
            stack_size : 0,
            registers : Registers::init(),
        }
    }
    /* STACK ACCESS */
    pub fn push(&mut self,word : Word) {
        self.stack.push(word);
        self.registers.set(Register::Ts, Word::U64(self.stack_size));
        self.stack_size += 1;
    }

    pub fn extend(&mut self,words : Vec<Word>) {
        self.stack_size += words.len();
        self.stack.extend(words.iter());
        self.registers.set(Register::Ts, Word::U64(self.stack_size-1));
    }

    pub fn insert(&mut self, word : Word,idx : usize) -> Result<Word,MemoryError> {
        if self.stack_size < idx {
            return Err(MemoryError::StackUnderflow);
        }
        self.stack.insert(self.stack_size - idx, word);
        return Ok(Word::U64(self.stack_size - idx))
    }

    pub fn pop(&mut self) -> Result<Word,MemoryError> {
        let some_word = self.stack.pop();
        match some_word {
            Some(word) => {
                self.stack_size -= 1;
                if self.stack_size == 0 {
                    self.registers.set(Register::Ts, Word::U64(0));
                }else {
                    self.registers.set(Register::Ts, Word::U64(self.stack_size-1));
                }
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

    pub fn stack_read(&self, addr : Word) -> Result<Word,MemoryError>{
        let idx = addr.as_usize();
        let Some(word) = self.stack.get(idx) else {
            return Err(MemoryError::StackSegmentationFault);
        };
        return Ok(*word);
    }
    pub fn stack_read_range(&self, addr : Word,size : usize) -> Result<Vec<Word>,MemoryError>{
        let idx = addr.as_usize();
        if idx + size > self.stack_size {
            return Err(MemoryError::StackSegmentationFault);
        }
        let res = self.stack[idx..size].to_vec();
        return Ok(res);
    }
    pub fn stack_clean(&mut self,start : usize,end : usize) -> Result<(),MemoryError> {
        if end > self.stack_size {
            return Err(MemoryError::StackOverflow);
        }
        self.stack.drain(start..end);
        self.stack_size = self.stack.len();
        return Ok(())
    }

    /* HEAP ACCESS */
    pub fn read(&mut self,idx:Word,size:usize,offset:usize) -> Result<Vec<Word>,MemoryError> {
        let mut res : Vec<Word> = Vec::with_capacity(size);
        for i in (0..size*16).step_by(16) {        
            let Some(word) = self.heap.get(&(idx.as_usize() + offset*16 + i)) else {
                return Err(MemoryError::HeapSegmentationFault);
            };
            res.push(*word);
        }
        return Ok(res);
    }

    pub fn write(&mut self,word : Word,idx : Word,offset:isize) -> Result<Word,MemoryError> {
        let addr = &((idx.as_usize() as isize + offset*16) as usize);
        if !self.heap.contains_key(addr) {
            return Err(MemoryError::HeapSegmentationFault);
        }
        let Some(addr) = self.heap.insert(*addr,word) else {
            return Err(MemoryError::HeapSegmentationFault);
        };
        return Ok(word);
    }

    pub fn alloc(&mut self,size:usize)  -> Result<Word,MemoryError>{
        let mut words: Vec<Word> = vec![Word::init();size];
        for i in 0..size {
            let addr = {
                let a = (&words[i] as *const Word);
                a.clone() as usize
            };
            self.heap.insert( addr as usize, words[i]);
        }
        return Ok(Word::U64((&words[0] as *const Word) as usize));
    }

    pub fn free(&mut self,idx : Word)  -> Result<(),MemoryError> {
        let Some(word) = self.heap.get(&idx.as_usize()) else {
            return Err(MemoryError::HeapSegmentationFault);
        };
        drop(word);
        self.heap.remove(&idx.as_usize());
        return Ok(()) 
    }
}