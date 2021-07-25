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
                println!("    [passed] {}", source_file.display());
            }
            Err(err) => {
                failure_count += 1;
                eprintln!("    [failed] {}: {}", source_file.display(), err);
                eprintln!("{:#?}", ast);
            }
        }
    }
    if failure_count == 0 {
        println!(
            "    JavaScript tests: {} passed, {} failed",
            success_count, failure_count
        );
    } else {
        eprintln!(
            "    JavaScript tests: {} passed, {} failed",
            success_count, failure_count
        );
        panic!("At least one JavaScript test failed");
    }
}
