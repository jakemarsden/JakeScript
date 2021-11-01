#![feature(process_exitcode_placeholder)]
#![feature(termination_trait_lib)]

use harness::{TestSuiteReport, TestSuiteSummary};
use walkdir::WalkDir;

pub mod harness;

#[test]
fn js_tests() -> TestSuiteSummary {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().ok();

    let mut test_cases = Vec::new();
    for dir_entry in WalkDir::new("tests-js") {
        let source_file = dir_entry.unwrap();
        if !source_file.file_type().is_file()
            || !matches!(source_file.file_name().to_str(), Some(name) if name.ends_with(".js"))
        {
            continue;
        }

        let test_case = harness::exec_source_file(source_file.path());
        test_case.print_report();
        test_cases.push(test_case);
    }
    TestSuiteReport::from(test_cases).into_summary()
}
