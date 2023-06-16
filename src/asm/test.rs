use std::{fs, path::{PathBuf}};
use crate::asm::{parser::Parser, lexer};

use super::{asm::Vasm};

fn test_file(file : &str) -> PathBuf{
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test/");
    d.push(file);
    d.clone()
}


#[test]
fn test_lexer() {
    let src = fs::read_to_string(test_file("test.vasm").as_path()).unwrap();
    let res = lexer::tokenize(&src);
    dbg!(&res);
    assert!(res.is_ok());
}


#[test]
fn test_parser() {
    let src = fs::read_to_string(test_file("test.vasm").as_path()).unwrap();
    let res = lexer::tokenize(&src);
    
    assert!(res.is_ok());
    let mut parser = Parser::init(res.unwrap());
    let instructions = parser.run();
    assert!(instructions.is_ok());
    let instructions = instructions.unwrap();

    dbg!(instructions);
}
#[test]
fn test_asm_compile() {
    let res = Vasm::build(test_file("test.vasm")).unwrap().assemble();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn test_asm_decompile() {
    Vasm::build(test_file("test.vasm")).unwrap().dissamble();
}
