use std::{fs, path::{PathBuf}};
use crate::asm::parser::Parser;

use super::{lexer::Lexer, asm::Vasm};

fn test_file(file : &str) -> PathBuf{
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test/");
    d.push(file);
    d.clone()
}


#[test]
fn test_lexer() {
    let mut lexer = Lexer::new();
    let src = fs::read_to_string(test_file("test.vasm").as_path()).unwrap();
    let error = lexer.run(&src);
    assert!(error.is_none());
}
#[test]
fn test_parser() {
    let mut lexer = Lexer::new();
    let src = fs::read_to_string(test_file("test.vasm").as_path()).unwrap();
    let error = lexer.run(&src);
    assert!(error.is_none());
    
    let mut parser = Parser::init(lexer.lexems);
    let instructions = parser.run();
    assert!(instructions.is_ok());
    let instructions = instructions.unwrap();

    dbg!(instructions);
}
#[test]
fn test_asm_compile() {
    Vasm::build(test_file("test.vasm")).unwrap().assemble();
}

#[test]
fn test_asm_decompile() {
    Vasm::build(test_file("test.vasm")).unwrap().dissamble();
}