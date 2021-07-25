use std::fs;

mod common;

#[test]
fn js_tests() {
    let mut success_count = 0_usize;
    let mut failure_count = 0_usize;
    for dir_entry in fs::read_dir("tests-js").unwrap() {
        let source_file = dir_entry.unwrap().path();
        let ast = common::parse_from_source_file(&source_file).expect("Failed to read source file");
        match common::eval(&ast) {
            Ok(..) => {
                success_count += 1;
                println!("JS :: [passed] {:?}", source_file);
            }
            Err(err) => {
                failure_count += 1;
                println!("JS :: [failed] {:?}: {}", source_file, err);
                println!("{:#?}", ast);
            }
        }
    }
    println!("JS :: {} passed, {} failed", success_count, failure_count);
    if failure_count != 0 {
        panic!("JS :: Tests failed");
    }
}
