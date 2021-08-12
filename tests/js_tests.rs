use std::fs;
use std::time::{Duration, Instant};

mod common;

#[test]
fn js_tests() {
    let mut success_count = 0_usize;
    let mut failure_count = 0_usize;
    let mut total_runtime = Duration::ZERO;

    for dir_entry in fs::read_dir("tests-js").unwrap() {
        let source_file = dir_entry.unwrap().path();
        let source_code = fs::read_to_string(&source_file).expect("Failed to read source file");

        let started_at = Instant::now();
        let ast = common::parse_from_source_code(&source_code);
        let result = common::eval(&ast);

        let elapsed_runtime = started_at.elapsed();
        total_runtime += elapsed_runtime;

        match result {
            Ok(..) => {
                success_count += 1;
                println!(
                    "    [passed] {} ({:?})",
                    source_file.display(),
                    elapsed_runtime
                );
            }
            Err(err) => {
                failure_count += 1;
                eprintln!(
                    "    [failed] {} ({:?}): {}",
                    source_file.display(),
                    elapsed_runtime,
                    err
                );
                eprintln!("{:#?}", ast);
            }
        }
    }

    if failure_count == 0 {
        println!(
            "    JavaScript tests: {} passed, {} failed ({:?})",
            success_count, failure_count, total_runtime
        );
    } else {
        eprintln!(
            "    JavaScript tests: {} passed, {} failed ({:?})",
            success_count, failure_count, total_runtime
        );
        panic!("At least one JavaScript test failed");
    }
}
