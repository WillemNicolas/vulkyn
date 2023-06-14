use std::path::PathBuf;

use crate::vm::word::Word;

use super::{vm::Vulkyn, memory::{self, Memory}};


fn test_file(file : &str) -> PathBuf{
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test/");
    d.push(file);
    d.clone()
}


#[test]
fn test_addition() {
    let mut Vulkyn = Vulkyn::build(&test_file("test.vk")).unwrap();
    Vulkyn.exec();
}


#[test]
fn test_memory_heap() {
    let mut memory = Memory::build();
    let idx = memory.alloc(3);
    assert!(idx.is_ok());
    let idx = idx.unwrap();
    dbg!(idx);
    let read1 = memory.read(idx,2,0);
    assert!(read1.is_ok());
    dbg!(&read1);
    let write = memory.write(Word::CHAR('a'), idx,0);
    assert!(write.is_ok());
    let read2 = memory.read(idx,2,0);
    assert!(read2.is_ok());
    dbg!(&read2);

}