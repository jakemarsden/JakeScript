#![feature(bool_to_option)]
#![feature(derive_default_enum)]
#![feature(iter_intersperse)]
#![feature(stdio_locked)]

use enumerate::{Enumerate, EnumerateStr};
use jakescript::ast::Program;
use jakescript::interpreter::{self, Eval, Interpreter};
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
        Options(Mode::Eval, Some(ref source_path)) => {
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
        Options(Mode::Parse, Some(ref source_path)) => {
            let source_file = fs::File::open(source_path)?;
            let mut buf = io::BufReader::new(source_file);
            let lexer = Lexer::for_chars_fallible(buf.chars());

            let (ast, parse_runtime) = parse(lexer)?;
            println!("Parsed in {:?}", parse_runtime);
            println!("{:#?}", ast);
        }
        Options(Mode::Lex, Some(ref source_path)) => {
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
        Options(Mode::Repl, None) => {
            let mut stdin = io::stdin_locked();
            let lexer = Lexer::for_chars_fallible(stdin.chars());
            let mut it = Interpreter::default();
            Repl::new(lexer).execute(&mut it);
        }
        Options(_, _) => unreachable!(),
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
    let mut it = Interpreter::default();
    ast.eval(&mut it).map(|value| (value, start_time.elapsed()))
}

#[derive(Clone, Debug)]
struct Options(Mode, Option<PathBuf>);

impl TryFrom<env::Args> for Options {
    type Error = ParseOptionsError;

    fn try_from(mut args: env::Args) -> Result<Self, Self::Error> {
        let executable_path = args
            .next()
            .filter(|it| !it.is_empty())
            .ok_or(ParseOptionsError {
                executable_path: None,
            })?;
        let mode = args
            .next()
            .ok_or(())
            .and_then(|arg| Mode::from_str(&arg).map_err(|_| ()))
            .map_err(|()| ParseOptionsError {
                executable_path: Some(executable_path.clone()),
            })?;
        let source_path = match mode {
            Mode::Eval | Mode::Parse | Mode::Lex => Some(
                args.next()
                    .ok_or(())
                    .and_then(|arg| PathBuf::from_str(&arg).map_err(|_| ()))
                    .map_err(|_| ParseOptionsError {
                        executable_path: Some(executable_path.clone()),
                    })?,
            ),
            Mode::Repl => None,
        };
        if args.next().is_some() {
            return Err(ParseOptionsError {
                executable_path: Some(executable_path),
            });
        }
        Ok(Self(mode, source_path))
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
    executable_path: Option<String>,
}

impl fmt::Display for ParseOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let exec_path = self.executable_path.as_deref().unwrap_or(PROGRAM_NAME);
        write!(
            f,
            r#"Usage:
    {}  --eval   <source-path>  # To evaluate a file
    {}  --parse  <source-path>  # To parse a file
    {}  --lex    <source-path>  # To lex (tokenise) a file
    {}  --repl                  # To enter an interactive REPL"#,
            exec_path, exec_path, exec_path, exec_path,
        )
    }
}

impl std::error::Error for ParseOptionsError {}
