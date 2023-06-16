use std::path::PathBuf;

use kwargs::utils::arg_parser::{Parser, Value};
use kwargs::utils::arg_parser::Value::Flag;
use vulkyn::asm::asm::Vasm;
use vulkyn::vm::vm::Vulkyn;

#[derive(Debug)]
enum Arg{
   VasmFile(String),
   VkFile(String)
}
#[derive(Debug)]
enum ErrorArg{
    NotValidVasmFile,
    NotValidVkFile
}

fn main() {
    let mut args:Parser<Arg, ErrorArg> = Parser::build("This project is a simple implementation of a stack-based virtual machine (VM) written in Rust.\nThe VM includes a stack, a heap, and several registers, and it interprets instructions written in its own assembly language.");
    args.arg("assemble", "vasm", "assemble the given vasm file",
    |s| {
        if s.ends_with(".vasm") {
            return Ok(Arg::VasmFile(s))
        }
        Err(ErrorArg::NotValidVasmFile)
    });
    args.arg("disassemble", "dvasm", "dassemble the given vk file",
    |s| {
        if s.ends_with(".vk") {
            return Ok(Arg::VkFile(s))
        }
        Err(ErrorArg::NotValidVkFile)
    });
    args.arg("run", "r", "run the given vk file",
    |s| {
        if s.ends_with(".vk") {
            return Ok(Arg::VkFile(s))
        }
        Err(ErrorArg::NotValidVkFile)
    });
    args.flag("debug", "d", "debug mode",false);
    
    let parsed_args = args.parse();
    if parsed_args.is_err() {
        dbg!(parsed_args);
        return;
    }
    let parsed_args = parsed_args.unwrap();

    if let Some(Flag(true)) = parsed_args.get("help") {
        args.print_help();
        return;
    }
    if let Some(Value::Value(Arg::VasmFile(file))) = parsed_args.get("assemble") {

        let mut path = PathBuf::from(file);        
        let mut copy = PathBuf::from(file);
        let Ok(vasm) = Vasm::build(path) else {
            panic!("Something went wrong when opening file {:?}",copy);
        };
        let res = vasm.assemble();
        if res.is_err() {
            panic!("Something went wrong when assemble the file\n\terror : {:?}",res.unwrap_err())
        }

    }
    if let Some(Value::Value(Arg::VkFile(file))) = parsed_args.get("run") {

        let mut path = PathBuf::from(file);        
        let mut copy = PathBuf::from(file);
        let Ok(mut vulkyn) = Vulkyn::build(&path) else {
            panic!("Something went wrong when opening file {:?}",copy);
        };
        vulkyn.exec();
    }
}