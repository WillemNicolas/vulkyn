use std::path::PathBuf;

use super::vm::Vulkyn;


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