#![feature(derive_default_enum)]
#![feature(stdio_locked)]

use enumerate::{Enumerate, EnumerateStr};
use jakescript::ast::Program;
use jakescript::interpreter::{self, Eval, Interpreter, Vm};
use jakescript::lexer::{self, Element, Lexer};
use jakescript::parser::{self, Parser};
use repl::Repl;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::{env, fmt, fs, io};
use utf8_chars::BufReadCharsExt;

mod repl;

static PROGRAM_NAME: &str = "jakescript-cli";

fn main() -> Result<(), Error> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().ok();

    match Options::try_from(env::args())? {
        Options(Mode::Eval, None, Some(ref source_path)) => {
            let source_file = fs::File::open(source_path)?;
            let mut buf = io::BufReader::new(source_file);
            let lexer = Lexer::for_chars_fallible(buf.chars());

            let (ast, parse_runtime) = parse(lexer)?;
            println!("Parsed in {:?}", parse_runtime);

            let (value, eval_runtime) = eval(&ast)?;
            println!(
                "Evaluated in {:?} (total: {:?})",
                eval_runtime,
                parse_runtime + eval_runtime
            );
            println!("{:?}", value);
        }
        Options(Mode::Parse, Some(format), Some(ref source_path)) => {
            let source_file = fs::File::open(source_path)?;
            let mut buf = io::BufReader::new(source_file);
            let lexer = Lexer::for_chars_fallible(buf.chars());

            let (ast, parse_runtime) = parse(lexer)?;
            println!("Parsed in {:?}", parse_runtime);

            let stdout = io::stdout_locked();
            match format {
                Format::Json => serde_json::to_writer_pretty(stdout, &ast).unwrap(),
                Format::Yaml => serde_yaml::to_writer(stdout, &ast).unwrap(),
            }
        }
        Options(Mode::Lex, None, Some(ref source_path)) => {
            let source_file = fs::File::open(source_path)?;
            let mut buf = io::BufReader::new(source_file);
            let lexer = Lexer::for_chars_fallible(buf.chars());

            let (elements, lex_runtime) = lex_and_print(lexer)?;
            println!("Lexed in {:?}", lex_runtime);
            println!(
                "{}",
                elements.iter().map(Element::to_string).collect::<String>()
            );

            println!("Lexed in {:?}", lex_runtime);
            println!(
                "{}",
                elements.iter().map(Element::to_string).collect::<String>()
            );
        }
        Options(Mode::Repl, None, None) => {
            let mut stdin = io::stdin_locked();
            let lexer = Lexer::for_chars_fallible(stdin.chars());
            let mut it = Interpreter::new(Vm::new().unwrap());
            Repl::new(lexer).execute(&mut it);
        }
        Options(_, _, _) => unreachable!(),
    }
    Ok(())
}

fn lex_and_print<I: Iterator<Item = io::Result<char>>>(
    lexer: Lexer<I>,
) -> lexer::Result<(Vec<Element>, Duration)> {
    let start_time = Instant::now();
    let mut elements = Vec::new();
    for element in lexer {
        elements.push(element?);
    }
    Ok((elements, start_time.elapsed()))
}

fn parse<I: Iterator<Item = io::Result<char>>>(
    lexer: Lexer<I>,
) -> parser::Result<(Program, Duration)> {
    let start_time = Instant::now();
    let parser = Parser::for_lexer(lexer);
    parser.execute().map(|ast| (ast, start_time.elapsed()))
}

fn eval(ast: &Program) -> interpreter::Result<(interpreter::Value, Duration)> {
    let start_time = Instant::now();
    let mut it = Interpreter::new(Vm::new().unwrap());
    ast.eval(&mut it).map(|value| (value, start_time.elapsed()))
}

#[derive(Clone, Debug)]
struct Options(Mode, Option<Format>, Option<PathBuf>);

impl TryFrom<env::Args> for Options {
    type Error = ParseOptionsError;

    fn try_from(mut args: env::Args) -> Result<Self, Self::Error> {
        let executable_path = args
            .next()
            .filter(|it| !it.is_empty())
            .ok_or_else(ParseOptionsError::default)?;
        let mode = args
            .next()
            .and_then(|arg| Mode::from_str(&arg).ok())
            .ok_or_else(|| ParseOptionsError::new(executable_path.clone()))?;
        let format = match mode {
            Mode::Parse => Some(
                args.next()
                    .and_then(|arg| Format::from_str(&arg).ok())
                    .ok_or_else(|| ParseOptionsError::new(executable_path.clone()))?,
            ),
            Mode::Eval | Mode::Lex | Mode::Repl => None,
        };
        let source_path = match mode {
            Mode::Eval | Mode::Parse | Mode::Lex => Some(
                args.next()
                    .and_then(|arg| PathBuf::from_str(&arg).ok())
                    .ok_or_else(|| ParseOptionsError::new(executable_path.clone()))?,
            ),
            Mode::Repl => None,
        };
        if args.next().is_some() {
            return Err(ParseOptionsError::new(executable_path));
        }
        Ok(Self(mode, format, source_path))
    }
}

#[derive(Enumerate, EnumerateStr, Copy, Clone, Default, Eq, PartialEq, Debug)]
enum Mode {
    #[default]
    #[enumerate_str(rename = "--eval")]
    Eval,
    #[enumerate_str(rename = "--parse")]
    Parse,
    #[enumerate_str(rename = "--lex")]
    Lex,
    #[enumerate_str(rename = "--repl")]
    Repl,
}

#[derive(Enumerate, EnumerateStr, Copy, Clone, Eq, PartialEq, Debug)]
enum Format {
    #[enumerate_str(rename = "--json")]
    Json,
    #[enumerate_str(rename = "--yaml")]
    Yaml,
}

enum Error {
    Options(ParseOptionsError),
    Lex(lexer::Error),
    Parse(parser::Error),
    Eval(interpreter::Error),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Options(source) => write!(f, "{}", source),
            Self::Lex(source) => write!(f, "{}", source),
            Self::Parse(source) => write!(f, "{}", source),
            Self::Eval(source) => write!(f, "{}", source),
            Self::Io(source) => write!(f, "{}", source),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::Options(source) => source,
            Self::Lex(source) => source,
            Self::Parse(source) => source,
            Self::Eval(source) => source,
            Self::Io(source) => source,
        })
    }
}

impl From<lexer::Error> for Error {
    fn from(source: lexer::Error) -> Self {
        Self::Lex(source)
    }
}

impl From<ParseOptionsError> for Error {
    fn from(source: ParseOptionsError) -> Self {
        Self::Options(source)
    }
}

impl From<parser::Error> for Error {
    fn from(source: parser::Error) -> Self {
        Self::Parse(source)
    }
}

impl From<interpreter::Error> for Error {
    fn from(source: interpreter::Error) -> Self {
        Self::Eval(source)
    }
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        Self::Io(source)
    }
}

#[derive(Debug)]
struct ParseOptionsError {
    executable_path: String,
}

impl Default for ParseOptionsError {
    fn default() -> Self {
        Self::new(PROGRAM_NAME.to_owned())
    }
}

impl ParseOptionsError {
    fn new(executable_path: String) -> Self {
        Self { executable_path }
    }
}

impl fmt::Display for ParseOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let exec_path = self.executable_path.as_str();
        write!(
            f,
            r#"Usage:
    {}  --eval                      <source-path>  # Evaluate a file
    {}  --parse  [--json | --yaml]  <source-path>  # Parse a file and output as JSON or YAML
    {}  --lex                       <source-path>  # Lex (tokenise) a file
    {}  --repl                                     # Enter the interactive REPL"#,
            exec_path, exec_path, exec_path, exec_path,
        )
    }
}

impl std::error::Error for ParseOptionsError {}
