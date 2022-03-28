use ansi_term::{Color, Style};
use fallible_iterator::FallibleIterator;
use jakescript::interpreter::{self, Eval, ExecutionState, Interpreter, Vm};
use jakescript::lexer::Lexer;
use jakescript::parser::{self, Parser};
use std::path::Path;
use std::time::{Duration, Instant};
use std::{fmt, fs, io, process, sync};
use utf8_chars::BufReadCharsExt;

pub fn init() {
    static INIT: sync::Once = sync::Once::new();
    INIT.call_once(|| {
        #[cfg(windows)]
        ansi_term::enable_ansi_support().ok();
    });
}

pub fn exec_source_file(source_path: &Path) -> TestCaseReport {
    let source_name = source_path.display().to_string();
    let mut buf = match fs::File::open(source_path) {
        Ok(file) => io::BufReader::new(file),
        Err(err) => return TestCaseReport::fail(source_name, Duration::ZERO, err.into()),
    };
    exec(
        source_name,
        Lexer::for_chars_fallible(fallible_iterator::convert(buf.chars())),
    )
}

pub fn exec_source_code(source_code: &str) -> TestCaseReport {
    exec("untitled".to_owned(), Lexer::for_str(source_code))
}

fn exec<I: FallibleIterator<Item = char, Error = io::Error>>(
    source_name: String,
    source: Lexer<I>,
) -> TestCaseReport {
    let parser = Parser::for_lexer(source);
    let mut interpreter = Interpreter::new(Vm::new().unwrap());

    let started_at = Instant::now();

    let ast = match parser.execute() {
        Ok(ast) => ast,
        Err(err) => return TestCaseReport::fail(source_name, started_at.elapsed(), err.into()),
    };

    let result = match ast.eval(&mut interpreter) {
        Ok(result) => result,
        Err(err) => return TestCaseReport::fail(source_name, started_at.elapsed(), err.into()),
    };

    let vm_state = interpreter.vm().execution_state().clone();
    TestCaseReport::pass(source_name, started_at.elapsed(), result, vm_state)
}

#[derive(Debug)]
pub enum TestCaseResult {
    Pass(interpreter::Value, ExecutionState),
    Fail(FailureReason),
}

#[derive(Debug)]
pub struct TestCaseReport {
    source_name: String,
    runtime: Duration,
    result: TestCaseResult,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TestSuiteSummary {
    Success(usize, Duration),
    Failure(usize, usize, Duration),
}

#[derive(Debug)]
pub struct TestSuiteReport {
    test_cases: Vec<TestCaseReport>,
    summary: TestSuiteSummary,
}

#[derive(Debug)]
pub enum FailureReason {
    Read(io::Error),
    Parse(parser::Error),
    Runtime(interpreter::Error),
}

impl TestCaseResult {
    pub fn is_pass(&self) -> bool {
        match self {
            Self::Pass(..) => true,
            Self::Fail(..) => false,
        }
    }

    pub fn is_fail(&self) -> bool {
        match self {
            Self::Pass(..) => false,
            Self::Fail(..) => true,
        }
    }

    pub fn success_value(&self) -> Option<&interpreter::Value> {
        match self {
            Self::Pass(value, _) => Some(value),
            Self::Fail(..) => None,
        }
    }

    pub fn vm_state(&self) -> Option<&ExecutionState> {
        match self {
            Self::Pass(_, vm_state) => Some(vm_state),
            Self::Fail(..) => None,
        }
    }

    pub fn failure_reason(&self) -> Option<&FailureReason> {
        match self {
            Self::Pass(..) => None,
            Self::Fail(reason) => Some(reason),
        }
    }
}

impl TestCaseReport {
    pub fn pass(
        source_name: String,
        runtime: Duration,
        value: interpreter::Value,
        vm_state: ExecutionState,
    ) -> Self {
        Self {
            source_name,
            runtime,
            result: TestCaseResult::Pass(value, vm_state),
        }
    }

    pub fn fail(source_name: String, runtime: Duration, reason: FailureReason) -> Self {
        Self {
            source_name,
            runtime,
            result: TestCaseResult::Fail(reason),
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

    pub fn is_fail(&self) -> bool {
        self.result().is_fail()
    }

    pub fn success_value(&self) -> Option<&interpreter::Value> {
        self.result().success_value()
    }

    pub fn vm_state(&self) -> Option<&ExecutionState> {
        self.result().vm_state()
    }

    pub fn failure_reason(&self) -> Option<&FailureReason> {
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
    fn report(self) -> process::ExitCode {
        self.print_report();
        if self.is_pass() {
            process::ExitCode::SUCCESS
        } else {
            process::ExitCode::FAILURE
        }
    }
}

impl fmt::Display for TestCaseReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const TICK: char = '\u{2714}';
        const CROSS: char = '\u{274C}';

        let (symbol, status, status_style) = if self.is_pass() {
            (TICK, "pass", Color::Green.normal())
        } else {
            (CROSS, "fail", Color::Red.normal())
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
    fn report(self) -> process::ExitCode {
        self.print_report();
        if self.is_success() {
            process::ExitCode::SUCCESS
        } else {
            process::ExitCode::FAILURE
        }
    }
}

impl fmt::Display for TestSuiteSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let success_count_style = Color::Green.bold();
        let failure_count_style = if self.is_success() {
            Style::default().bold()
        } else {
            Color::Red.bold()
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
        let failure_count = test_cases.iter().filter(|case| case.is_fail()).count();
        let total_runtime = test_cases.iter().map(TestCaseReport::runtime).sum();
        match failure_count {
            0 => Self::Success(success_count, total_runtime),
            _ => Self::Failure(success_count, failure_count, total_runtime),
        }
    }
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

impl fmt::Display for FailureReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Read(source) => write!(f, "{}", source),
            Self::Parse(source) => write!(f, "{}", source),
            Self::Runtime(source) => write!(f, "{}", source),
        }
    }
}

impl std::error::Error for FailureReason {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::Read(source) => source,
            Self::Parse(source) => source,
            Self::Runtime(source) => source,
        })
    }
}

impl From<io::Error> for FailureReason {
    fn from(source: io::Error) -> Self {
        Self::Read(source)
    }
}

impl From<parser::Error> for FailureReason {
    fn from(source: parser::Error) -> Self {
        Self::Parse(source)
    }
}

impl From<interpreter::Error> for FailureReason {
    fn from(source: interpreter::Error) -> Self {
        Self::Runtime(source)
    }
}
