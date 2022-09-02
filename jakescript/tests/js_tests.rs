use harness::{TestCaseReport, TestSuiteReport};
use std::process;
use walkdir::{DirEntry, WalkDir};

pub mod harness;

#[test]
fn js_tests() -> impl process::Termination {
    harness::init();
    WalkDir::new("tests-js")
        .into_iter()
        .map(Result::unwrap)
        .filter(is_normal_file)
        .filter(has_js_extension)
        .map(|dir_entry| harness::exec_source_file(dir_entry.path()))
        .inspect(TestCaseReport::print_report)
        .collect::<TestSuiteReport>()
        .into_summary()
}

fn is_normal_file(dir_entry: &DirEntry) -> bool {
    dir_entry.file_type().is_file()
}

fn has_js_extension(dir_entry: &DirEntry) -> bool {
    let file_name = dir_entry.file_name().to_string_lossy();
    file_name.ends_with(".js") && !file_name.ends_with(".ignore.js")
}
