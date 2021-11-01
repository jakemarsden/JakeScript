use ansi_term::Color::*;
use std::time::Duration;
use walkdir::WalkDir;

pub mod harness;

#[test]
fn js_tests() {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().ok();

    let mut success_count = 0_usize;
    let mut failure_count = 0_usize;
    let mut total_runtime = Duration::ZERO;

    for dir_entry in WalkDir::new("tests-js") {
        let source_file = dir_entry.unwrap();
        if !matches!(source_file.file_name().to_str(), Some(name) if name.ends_with(".js")) {
            continue;
        }

        let result = harness::exec_source_file(source_file.path());
        if result.is_pass() {
            success_count += 1;
            println!("    {}", result);
        } else {
            failure_count += 1;
            eprintln!("    {}", result);
        }
        total_runtime += result.runtime();
    }

    let msg = format!(
        "JavaScript tests: {}, {} in {:?}",
        Green.paint(format!("{} passed", success_count)),
        Red.paint(format!("{} failed", failure_count)),
        total_runtime
    );
    if failure_count == 0 {
        println!("    {}", msg);
    } else {
        eprintln!("    {}", msg);
        panic!("At least one JavaScript test has failed.");
    }
}
