#![feature(bool_to_option)]
#![feature(derive_default_enum)]
#![feature(iter_intersperse)]

use jakescript::ast::Program;
use jakescript::interpreter::{self, Eval, Interpreter};
use jakescript::lexer::{Element, Lexer, LexicalError, LexicalResult};
use jakescript::parser::{ParseError, ParseResult, Parser};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::{env, fmt, fs, io};
use utf8_chars::BufReadCharsExt;

fn main() -> Result<(), Error> {
    let Options { mode, source_path } = Options::try_from(env::args())?;

    let source_file = fs::File::open(&source_path)?;
    let mut buf = io::BufReader::new(source_file);

    let lexer = Lexer::for_chars_fallible(buf.chars());
    match mode {
        Mode::Eval => {
            let (ast, parse_runtime) = parse(lexer)?;
            println!("Parsed in {:?}", parse_runtime);

            let (value, eval_runtime) = eval(ast)?;
            println!(
                "Evaluated in {:?} (total: {:?})",
                eval_runtime,
                parse_runtime + eval_runtime
            );
            println!("{:?}", value);
        }
        Mode::Parse => {
            let (ast, parse_runtime) = parse(lexer)?;
            println!("Parsed in {:?}", parse_runtime);
            println!("{:#?}", ast);
        }
        Mode::Lex => {
            let (elements, lex_runtime) = lex_and_print(lexer)?;
            println!("Lexed in {:?}", lex_runtime);
            println!(
                "{}",
                elements.iter().map(Element::to_string).collect::<String>()
            );
        }
    }
    Ok(())
}

fn lex_and_print<I: Iterator<Item = io::Result<char>>>(
    lexer: Lexer<I>,
) -> LexicalResult<(Vec<Element>, Duration)> {
    let start_time = Instant::now();
    let mut elements = Vec::new();
    for element in lexer {
        elements.push(element?);
    }
    Ok((elements, start_time.elapsed()))
}

fn parse<I: Iterator<Item = io::Result<char>>>(
    lexer: Lexer<I>,
) -> ParseResult<(Program, Duration)> {
    let start_time = Instant::now();
    let parser = Parser::for_lexer(lexer);
    parser.execute().map(|ast| (ast, start_time.elapsed()))
}

fn eval(ast: Program) -> interpreter::Result<(interpreter::Value, Duration)> {
    let start_time = Instant::now();
    let mut it = Interpreter::default();
    ast.eval(&mut it).map(|value| (value, start_time.elapsed()))
}

#[derive(Clone, Debug)]
struct Options {
    mode: Mode,
    source_path: PathBuf,
}

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
            .and_then(|arg| Mode::from_str(&arg))
            .map_err(|()| ParseOptionsError {
                executable_path: Some(executable_path.to_owned()),
            })?;
        let source_path = args
            .next()
            .ok_or(())
            .and_then(|arg| PathBuf::from_str(&arg).map_err(|_| ()))
            .map_err(|()| ParseOptionsError {
                executable_path: Some(executable_path.to_owned()),
            })?;
        Ok(Self { mode, source_path })
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
enum Mode {
    #[default]
    Eval,
    Parse,
    Lex,
}

impl Mode {
    fn variants() -> &'static [Self] {
        static ALL: &[Mode] = &[Mode::Eval, Mode::Parse, Mode::Lex];
        ALL
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Eval => "--eval",
            Self::Parse => "--parse",
            Self::Lex => "--lex",
        }
    }
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::variants()
            .iter()
            .find_map(|it| (s == it.as_str()).then_some(*it))
            .ok_or(())
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

enum Error {
    Options(ParseOptionsError),
    Lex(LexicalError),
    Parse(ParseError),
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

impl From<LexicalError> for Error {
    fn from(source: LexicalError) -> Self {
        Self::Lex(source)
    }
}

impl From<ParseOptionsError> for Error {
    fn from(source: ParseOptionsError) -> Self {
        Self::Options(source)
    }
}

impl From<ParseError> for Error {
    fn from(source: ParseError) -> Self {
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
        write!(
            f,
            "Usage: {} [{}] <source-path>",
            self.executable_path.as_deref().unwrap_or("jakescript"),
            Mode::variants()
                .iter()
                .map(Mode::to_string)
                .intersperse_with(|| " | ".to_owned())
                .collect::<String>()
        )
    }
}

impl std::error::Error for ParseOptionsError {}
