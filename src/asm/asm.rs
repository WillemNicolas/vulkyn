use std::{path::{PathBuf}, fs::{self, File}};

use bincode::{de, Options};

use crate::{asm::{lexer::Lexer, parser::Parser}, vm::vm::{Instruction, Program}};




pub struct Vasm {
    src_path : PathBuf, 
    src : String,
    out_path : PathBuf,
}

impl Vasm {

    pub fn build(path : PathBuf) -> Result<Self,()>{
        let mut vk_path = path.clone();
        let src = fs::read_to_string(&path);
        if src.is_err() {
            return Err(());
        }
        vk_path.set_extension("vk");
        Ok(Self {
            src_path : path, 
            src      : src.unwrap().to_string(),
            out_path : vk_path,
        })
    }

    pub fn dissamble(&self) {
        let instructions = bincode::deserialize_from
            ::<File,Vec<Instruction>>(File::open(&self.out_path).unwrap());
        dbg!(&instructions);
    }


    pub fn assemble(&self) -> Result<(),()>{
        let mut lexer = Lexer::new();

        let error = lexer.run(&self.src);
        if error.is_some() {
            panic!("{:#?}", error.unwrap());
        }

        let mut parser = Parser::init(lexer.lexems);
        let instructions = parser.run();
        if instructions.is_err() {
            panic!("{:#?}", instructions);
        }

        let instructions = instructions.unwrap();

        let program = Program{
            instructions : instructions,
        };
        dbg!(&program);
        let file = File::create(&self.out_path).unwrap();
        let res = bincode::serialize_into(file, &program);
        if res.is_err(){
            return Err(());
        }
        return Ok(());
    }
}