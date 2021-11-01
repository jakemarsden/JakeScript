use ansi_term::Color;
use jakescript::interpreter::{self, Eval, Interpreter};
use jakescript::lexer::Lexer;
use jakescript::parser::{self, Parser};
use std::path::Path;
use std::time::{Duration, Instant};
use std::{fmt, fs, io};
use utf8_chars::BufReadCharsExt;

pub fn exec_source_file(source_path: &Path) -> TestResult {
    let source_name = source_path.display().to_string();
    let mut buf = match fs::File::open(source_path) {
        Ok(file) => io::BufReader::new(file),
        Err(err) => return TestResult::read_error(source_name, err),
    };
    return exec(source_name, Lexer::for_chars_fallible(buf.chars()));
}

pub fn exec_source_code(source_code: &str) -> TestResult {
    exec("untitled".to_owned(), Lexer::for_str(source_code))
}

fn exec<I: Iterator<Item = io::Result<char>>>(source_name: String, source: Lexer<I>) -> TestResult {
    let parser = Parser::for_lexer(source);
    let mut interpreter = Interpreter::default();

    let started_at = Instant::now();

    let ast = match parser.execute() {
        Ok(ast) => ast,
        Err(reason) => return TestResult::parser_error(source_name, started_at.elapsed(), reason),
    };

    let result = match ast.eval(&mut interpreter) {
        Ok(result) => result,
        Err(err) => return TestResult::interpreter_error(source_name, started_at.elapsed(), err),
    };

    TestResult::pass(source_name, started_at.elapsed(), result)
}

#[derive(Debug)]
pub enum TestOutput {
    Pass(interpreter::Value),
    ParserError(parser::ParseError),
    InterpreterError(interpreter::Error),
    ReadError(io::Error),
}

#[derive(Debug)]
pub struct TestResult {
    source_name: String,
    runtime: Duration,
    output: TestOutput,
}

impl TestResult {
    pub fn pass(source_name: String, runtime: Duration, value: interpreter::Value) -> Self {
        Self {
            source_name,
            runtime,
            output: TestOutput::Pass(value),
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
            output: TestOutput::ParserError(reason),
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
            output: TestOutput::InterpreterError(reason),
        }
    }

    pub fn read_error(source_name: String, reason: io::Error) -> Self {
        Self {
            source_name,
            runtime: Duration::ZERO,
            output: TestOutput::ReadError(reason),
        }
    }

    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    pub fn runtime(&self) -> Duration {
        self.runtime
    }

    pub fn output(&self) -> &TestOutput {
        &self.output
    }

    pub fn is_pass(&self) -> bool {
        match self.output() {
            TestOutput::Pass(..) => true,
            TestOutput::ParserError(..)
            | TestOutput::InterpreterError(..)
            | TestOutput::ReadError(..) => false,
        }
    }

    pub fn success_value(&self) -> Option<&interpreter::Value> {
        match self.output() {
            TestOutput::Pass(value) => Some(value),
            TestOutput::ParserError(..)
            | TestOutput::InterpreterError(..)
            | TestOutput::ReadError(..) => None,
        }
    }

    pub fn failure_reason(&self) -> Option<&dyn std::error::Error> {
        match self.output() {
            TestOutput::Pass(..) => None,
            TestOutput::ParserError(reason) => Some(reason),
            TestOutput::InterpreterError(reason) => Some(reason),
            TestOutput::ReadError(reason) => Some(reason),
        }
    }
}

impl fmt::Display for TestResult {
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
