use ansi_term::{Color, Style};
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
        if !source_file.file_type().is_file()
            || !matches!(source_file.file_name().to_str(), Some(name) if name.ends_with(".js"))
        {
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

    let success_count_style = Color::Green.bold();
    let failure_count_style = match failure_count {
        0 => Style::default().bold(),
        _ => Color::Red.bold(),
    };
    let msg = format!(
        "JavaScript test suite: {} and {} in {:?}",
        success_count_style.paint(format!("{} passed", success_count)),
        failure_count_style.paint(format!("{} failed", failure_count)),
        total_runtime,
    );
    match failure_count {
        0 => println!("    {}", msg),
        _ => {
            eprintln!("    {}", msg);
            panic!("At least one JavaScript test has failed.");
        }
    }
}
