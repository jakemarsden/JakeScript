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
                eprintln!("|{test_number}| description invalid: {}", path.display());
                continue;
            }

            Ok(..) | Err(Error::DescriptionInvalid(..)) => continue,
            Err(err) => panic!("{}", err),
        };

        let expected = Expected::from(test.desc.negative.as_ref());
        let test_path = &test.path.display();

        // TODO: Time out to protect against tests which fail by entering an infinite
        // loop.
        let test_result = exec_test_suppressing_panic(&test);

        // TODO: For negative test cases, check that the name of the thrown exception
        // matches the `Negative::type` (as well as the `Negative::phase`) in the test
        // description.
        match (test_result, expected) {
            (Ok(()), Expected::Pass)
            | (Err(FailureReason::Parse(..)), Expected::ParserFail)
            | (Err(FailureReason::Eval(..)), Expected::RuntimeFail) => {
                pass_count += 1;
                eprintln!("|{test_number}| PASS {test_path}",);
            }

            (Ok(()), expected @ (Expected::ParserFail | Expected::RuntimeFail)) => {
                fail_count += 1;
                eprintln!("|{test_number}| FAIL {test_path}: Expected to {expected} but passed",);
            }

            (Err(err @ FailureReason::Parse(..)), expected @ Expected::RuntimeFail)
            | (Err(err @ FailureReason::Eval(..)), expected @ Expected::ParserFail) => {
                fail_count += 1;
                eprintln!(
                    "|{}| FAIL {}: Expected to {} but failed {}",
                    test_number, test_path, expected, err
                );
            }

            (Err(err @ (FailureReason::Parse(..) | FailureReason::Eval(..))), Expected::Pass) => {
                fail_count += 1;
                eprintln!("|{test_number}| FAIL {test_path}: {err}",);
            }

            (Err(FailureReason::Panic(..)), _) => {
                panic_count += 1;
                eprintln!("|{test_number}| PANIC {test_path}");
            }
        }
    }

    let total_count = pass_count
        .saturating_add(fail_count)
        .saturating_add(panic_count);
    eprintln!("-- TEST262 COMPLETED --");
    eprintln!("Passed:   {pass_count} of {total_count}",);
    eprintln!("Failed:   {fail_count}",);
    eprintln!("Panicked: {panic_count}",);
}

fn is_test_we_care_about(path: impl AsRef<Path>) -> bool {
    path.as_ref().starts_with(TEST262_TEST_DIR)
}

fn exec_test_suppressing_panic(test: &Test) -> Result<(), FailureReason> {
    thread::scope(|s| {
        s.spawn(|| exec_test(test))
            .join()
            .unwrap_or_else(|payload| Err(FailureReason::from_panic_payload(payload)))
    })
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Expected {
    Pass,
    ParserFail,
    RuntimeFail,
}

impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Pass => "pass",
            Self::ParserFail => "fail at parse time",
            Self::RuntimeFail => "fail at runtime",
        })
    }
}

impl From<Option<&Negative>> for Expected {
    fn from(negative: Option<&Negative>) -> Self {
        match negative {
            None => Self::Pass,
            Some(Negative { phase: Parse, .. }) => Self::ParserFail,
            Some(Negative {
                phase: Early | Resolution | Runtime,
                ..
            }) => Self::RuntimeFail,
        }
    }
}

// TODO: Using `String` as a workaround for the fact that `parser::Error` and
// `interpreter::Error` aren't currently `Send`.
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
            Self::Parse(source) => write!(f, "parse: {source}"),
            Self::Eval(source) => write!(f, "eval: {source}"),
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
