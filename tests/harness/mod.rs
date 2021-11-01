use ansi_term::{Color, Style};
use jakescript::interpreter::{self, Eval, Interpreter};
use jakescript::lexer::Lexer;
use jakescript::parser::{self, Parser};
use std::path::Path;
use std::time::{Duration, Instant};
use std::{fmt, fs, io, process};
use utf8_chars::BufReadCharsExt;

pub fn exec_source_file(source_path: &Path) -> TestCaseReport {
    let source_name = source_path.display().to_string();
    let mut buf = match fs::File::open(source_path) {
        Ok(file) => io::BufReader::new(file),
        Err(err) => return TestCaseReport::read_error(source_name, err),
    };
    return exec(source_name, Lexer::for_chars_fallible(buf.chars()));
}

pub fn exec_source_code(source_code: &str) -> TestCaseReport {
    exec("untitled".to_owned(), Lexer::for_str(source_code))
}

fn exec<I: Iterator<Item = io::Result<char>>>(
    source_name: String,
    source: Lexer<I>,
) -> TestCaseReport {
    let parser = Parser::for_lexer(source);
    let mut interpreter = Interpreter::default();

    let started_at = Instant::now();

    let ast = match parser.execute() {
        Ok(ast) => ast,
        Err(reason) => {
            return TestCaseReport::parser_error(source_name, started_at.elapsed(), reason);
        }
    };

    let result = match ast.eval(&mut interpreter) {
        Ok(result) => result,
        Err(err) => {
            return TestCaseReport::interpreter_error(source_name, started_at.elapsed(), err)
        }
    };

    TestCaseReport::pass(source_name, started_at.elapsed(), result)
}

#[derive(Debug)]
pub enum TestCaseResult {
    Pass(interpreter::Value),
    ParserError(parser::ParseError),
    InterpreterError(interpreter::Error),
    ReadError(io::Error),
}

impl TestCaseResult {
    pub fn is_pass(&self) -> bool {
        match self {
            Self::Pass(..) => true,
            Self::ParserError(..) | Self::InterpreterError(..) | Self::ReadError(..) => false,
        }
    }

    pub fn success_value(&self) -> Option<&interpreter::Value> {
        match self {
            Self::Pass(value) => Some(value),
            Self::ParserError(..) | Self::InterpreterError(..) | Self::ReadError(..) => None,
        }
    }

    pub fn failure_reason(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::Pass(..) => None,
            Self::ParserError(reason) => Some(reason),
            Self::InterpreterError(reason) => Some(reason),
            Self::ReadError(reason) => Some(reason),
        }
    }
}

#[derive(Debug)]
pub struct TestCaseReport {
    source_name: String,
    runtime: Duration,
    result: TestCaseResult,
}

impl TestCaseReport {
    pub fn pass(source_name: String, runtime: Duration, value: interpreter::Value) -> Self {
        Self {
            source_name,
            runtime,
            result: TestCaseResult::Pass(value),
        }
    }

    pub fn parser_error(
        source_name: String,
        runtime: Duration,
        reason: parser::ParseError,
    ) -> Self {
        Self {
            source_name,
            runtime,
            result: TestCaseResult::ParserError(reason),
        }
    }

    pub fn interpreter_error(
        source_name: String,
        runtime: Duration,
        reason: interpreter::Error,
    ) -> Self {
        Self {
            source_name,
            runtime,
            result: TestCaseResult::InterpreterError(reason),
        }
    }

    pub fn read_error(source_name: String, reason: io::Error) -> Self {
        Self {
            source_name,
            runtime: Duration::ZERO,
            result: TestCaseResult::ReadError(reason),
        }
    }

    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    pub fn runtime(&self) -> Duration {
        self.runtime
    }

    pub fn is_pass(&self) -> bool {
        self.result().is_pass()
    }

    pub fn success_value(&self) -> Option<&interpreter::Value> {
        self.result().success_value()
    }

    pub fn failure_reason(&self) -> Option<&dyn std::error::Error> {
        self.result().failure_reason()
    }

    pub fn result(&self) -> &TestCaseResult {
        &self.result
    }

    pub fn into_result(self) -> TestCaseResult {
        self.result
    }

    /// See also: [`Self::report()`].
    pub fn print_report(&self) {
        if self.is_pass() {
            println!("     {}", self);
        } else {
            eprintln!("     {}", self);
        }
    }
}

impl process::Termination for TestCaseReport {
    fn report(self) -> i32 {
        self.print_report();
        if self.is_pass() {
            process::ExitCode::SUCCESS.report()
        } else {
            process::ExitCode::FAILURE.report()
        }
    }
}

impl fmt::Display for TestCaseReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const TICK: char = '\u{2714}';
        const CROSS: char = '\u{274C}';

        let (symbol, status, status_style) = match self.is_pass() {
            true => (TICK, "pass", Color::Green.normal()),
            false => (CROSS, "fail", Color::Red.normal()),
        };
        let mut msg = format!(
            "[{} {}] {} ({:?})",
            symbol,
            status_style.paint(status),
            self.source_name(),
            self.runtime()
        );
        if let Some(failure_reason) = self.failure_reason() {
            msg.push_str(&format!(": {}", failure_reason));
        }
        f.write_str(&msg)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TestSuiteSummary {
    Success(usize, Duration),
    Failure(usize, usize, Duration),
}

impl TestSuiteSummary {
    pub fn is_success(&self) -> bool {
        match self {
            Self::Success(..) => true,
            Self::Failure(..) => false,
        }
    }

    pub fn success_count(&self) -> usize {
        match self {
            Self::Success(success_count, _) | Self::Failure(success_count, _, _) => *success_count,
        }
    }

    pub fn failure_count(&self) -> usize {
        match self {
            Self::Success(_, _) => 0,
            Self::Failure(_, failure_count, _) => *failure_count,
        }
    }

    pub fn total_count(&self) -> usize {
        self.success_count() + self.failure_count()
    }

    pub fn total_runtime(&self) -> Duration {
        match self {
            Self::Success(_, total_runtime) | Self::Failure(_, _, total_runtime) => *total_runtime,
        }
    }

    /// See also: [`Self::report()`].
    pub fn print_report(&self) {
        if self.is_success() {
            println!("     {}", self);
        } else {
            eprintln!("     {}", self);
        }
    }
}

impl process::Termination for TestSuiteSummary {
    fn report(self) -> i32 {
        self.print_report();
        if self.is_success() {
            process::ExitCode::SUCCESS.report()
        } else {
            process::ExitCode::FAILURE.report()
        }
    }
}

impl fmt::Display for TestSuiteSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let success_count_style = Color::Green.bold();
        let failure_count_style = match self.is_success() {
            true => Style::default().bold(),
            false => Color::Red.bold(),
        };
        write!(
            f,
            "JavaScript test suite: {} and {} in {:?}",
            success_count_style.paint(format!("{} passed", self.success_count())),
            failure_count_style.paint(format!("{} failed", self.failure_count())),
            self.total_runtime(),
        )
    }
}

impl From<&[TestCaseReport]> for TestSuiteSummary {
    fn from(test_cases: &[TestCaseReport]) -> Self {
        let success_count = test_cases.iter().filter(|case| case.is_pass()).count();
        let failure_count = test_cases.iter().filter(|case| !case.is_pass()).count();
        let total_runtime = test_cases.iter().map(TestCaseReport::runtime).sum();
        match failure_count {
            0 => Self::Success(success_count, total_runtime),
            _ => Self::Failure(success_count, failure_count, total_runtime),
        }
    }
}

#[derive(Debug)]
pub struct TestSuiteReport {
    test_cases: Vec<TestCaseReport>,
    summary: TestSuiteSummary,
}

impl TestSuiteReport {
    pub fn cases(&self) -> &[TestCaseReport] {
        &self.test_cases
    }

    pub fn summary(&self) -> &TestSuiteSummary {
        &self.summary
    }

    pub fn into_summary(self) -> TestSuiteSummary {
        self.summary
    }
}

impl From<Vec<TestCaseReport>> for TestSuiteReport {
    fn from(test_cases: Vec<TestCaseReport>) -> Self {
        let summary = TestSuiteSummary::from(test_cases.as_slice());
        Self {
            test_cases,
            summary,
        }
    }
}

impl FromIterator<TestCaseReport> for TestSuiteReport {
    fn from_iter<T: IntoIterator<Item = TestCaseReport>>(iter: T) -> Self {
        let test_cases: Vec<_> = iter.into_iter().collect();
        Self::from(test_cases)
    }
}
