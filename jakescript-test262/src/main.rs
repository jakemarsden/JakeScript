//! It is expected that the _[test262](https://github.com/tc39/test262)_ repository is checked out
//! next to this repository.

use jakescript::interpreter::{Eval, Interpreter, Vm};
use jakescript::lexer::Lexer;
use jakescript::parser::Parser;
use jakescript::token::SourceLocation;
use jakescript::{interpreter, parser};
use std::any::Any;
use std::path::Path;
use std::{fmt, thread};
use test262_harness::Phase::{Early, Parse, Resolution, Runtime};
use test262_harness::{Error, Harness, Negative, Test};

static TEST262_ROOT_DIR: &str = "../test262";
static TEST262_TEST_DIR: &str = "../test262/test";

fn main() {
    let harness = Harness::new(TEST262_ROOT_DIR).expect("failed to initialize harness");

    let mut pass_count = 0_usize;
    let mut fail_count = 0_usize;
    let mut panic_count = 0_usize;
    for (idx, test) in harness.enumerate() {
        let test_number = idx + 1;
        let test = match test {
            Ok(test) if is_test_we_care_about(&test.path) => test,
            Err(Error::DescriptionInvalid(path)) if is_test_we_care_about(&path) => {
                eprintln!("|{}| description invalid: {}", test_number, path.display());
                continue;
            }

            Ok(..) | Err(Error::DescriptionInvalid(..)) => continue,
            Err(err) => panic!("{}", err),
        };

        let test_path = test.path.clone();
        let negative = test.desc.negative.clone();

        // TODO: Timeout after awhile, for tests which fail by entering an infinite loop.
        let test_result = exec_test_suppressing_panic(test);

        match (test_result, &negative) {
            (Ok(()), None)
            | (Err(FailureReason::Parse(..)), Some(Negative { phase: Parse, .. }))
            | (
                Err(FailureReason::Eval(..)),
                Some(Negative {
                    phase: Early | Resolution | Runtime,
                    ..
                }),
            ) => {
                pass_count += 1;
                eprintln!("|{}| PASS {}", test_number, test_path.display());
            }

            (Ok(()), Some(Negative { phase, .. })) => {
                fail_count += 1;
                eprintln!(
                    "|{}| FAIL {}: Expected to fail {:?} but passed",
                    test_number,
                    test_path.display(),
                    phase
                );
            }

            (
                Err(err @ FailureReason::Parse(..)),
                Some(Negative {
                    phase: phase @ (Early | Resolution | Runtime),
                    ..
                }),
            )
            | (
                Err(err @ FailureReason::Eval(..)),
                Some(Negative {
                    phase: phase @ Parse,
                    ..
                }),
            ) => {
                fail_count += 1;
                eprintln!(
                    "|{}| FAIL {}: Expected to fail {:?} but failed {}",
                    test_number,
                    test_path.display(),
                    phase,
                    err
                );
            }

            (Err(err @ (FailureReason::Parse(..) | FailureReason::Eval(..))), None) => {
                fail_count += 1;
                eprintln!("|{}| FAIL {}: {}", test_number, test_path.display(), err);
            }

            (Err(FailureReason::Panic(..)), _) => {
                panic_count += 1;
                eprintln!("|{}| PANIC {}", test_number, test_path.display());
            }
        }
    }

    let total_count = pass_count
        .saturating_add(fail_count)
        .saturating_add(panic_count);
    eprintln!(" -- TEST262 COMPLETED --");
    eprintln!("Passed:   {} of {}", pass_count, total_count);
    eprintln!("Failed:   {}", fail_count);
    eprintln!("Panicked: {}", panic_count);
}

fn is_test_we_care_about(path: impl AsRef<Path>) -> bool {
    path.as_ref().starts_with(TEST262_TEST_DIR)
}

fn exec_test_suppressing_panic(test: Test) -> Result<(), FailureReason> {
    let t = thread::spawn(move || exec_test(&test));
    t.join()
        .unwrap_or_else(|payload| Err(FailureReason::from_panic_payload(payload)))
}

fn exec_test(test: &Test) -> Result<(), FailureReason> {
    let lexer = Lexer::for_str(&test.source, SourceLocation::at_start_of(&test.path));
    let parser = Parser::for_lexer(lexer);

    let vm = Vm::new().expect("failed to initialise a virtual machine");
    let mut it = Interpreter::new(vm);

    let script = parser.execute()?;
    script.eval(&mut it)?;

    Ok(())
}

// TODO: Using `String` as a workaround for the fact that `parser::Error` and `interpreter::Error`
//  aren't currently `Send`.
#[derive(Debug)]
enum FailureReason {
    Parse(String),
    Eval(String),
    Panic(Box<dyn Any + Send + 'static>),
}

impl std::error::Error for FailureReason {}

impl fmt::Display for FailureReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parse(source) => write!(f, "parse: {}", source),
            Self::Eval(source) => write!(f, "eval: {}", source),
            Self::Panic(_) => f.write_str("panic"),
        }
    }
}

impl From<parser::Error> for FailureReason {
    fn from(source: parser::Error) -> Self {
        Self::Parse(source.to_string())
    }
}

impl From<interpreter::Error> for FailureReason {
    fn from(source: interpreter::Error) -> Self {
        Self::Eval(source.to_string())
    }
}

impl FailureReason {
    fn from_panic_payload(payload: Box<dyn Any + Send + 'static>) -> Self {
        Self::Panic(payload)
    }
}
