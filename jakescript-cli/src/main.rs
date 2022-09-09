use fallible_iterator::FallibleIterator;
use jakescript::ast::Script;
use jakescript::interpreter::{self, Eval, Interpreter, Vm};
use jakescript::lexer::{self, Lexer};
use jakescript::parser::{self, Parser};
use jakescript::token::{Element, SourceLocation};
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

    let start_loc = SourceLocation::at_start_of("stdin");
    match Options::try_from(env::args())? {
        Options(Mode::Eval, None, Some(ref source_path)) => {
            let source_file = fs::File::open(source_path)?;
            let mut buf = io::BufReader::new(source_file);
            let lexer =
                Lexer::for_chars_fallible(fallible_iterator::convert(buf.chars()), start_loc);

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
            let lexer =
                Lexer::for_chars_fallible(fallible_iterator::convert(buf.chars()), start_loc);

            let (ast, parse_runtime) = parse(lexer)?;
            println!("Parsed in {:?}", parse_runtime);

            let stdout = io::stdout().lock();
            match format {
                Format::Json => serde_json::to_writer_pretty(stdout, &ast).unwrap(),
                Format::Yaml => serde_yaml::to_writer(stdout, &ast).unwrap(),
            }
        }
        Options(Mode::Lex, None, Some(ref source_path)) => {
            let source_file = fs::File::open(source_path)?;
            let mut buf = io::BufReader::new(source_file);
            let lexer =
                Lexer::for_chars_fallible(fallible_iterator::convert(buf.chars()), start_loc);

            let (elements, lex_runtime) = lex_and_print(lexer)?;
            println!("Lexed in {:?}", lex_runtime);
            println!(
                "{}",
                elements.iter().map(Element::to_string).collect::<String>()
            );
        }
        Options(Mode::Repl, None, None) => {
            let mut stdin = io::stdin().lock();
            let lexer =
                Lexer::for_chars_fallible(fallible_iterator::convert(stdin.chars()), start_loc);
            let mut it = Interpreter::new(Vm::new().unwrap());
            Repl::new(lexer).execute(&mut it);
        }
        Options(_, _, _) => unreachable!(),
    }
    Ok(())
}

fn lex_and_print<I: FallibleIterator<Item = char, Error = io::Error>>(
    lexer: Lexer<I>,
) -> lexer::Result<(Vec<Element>, Duration)> {
    let start_time = Instant::now();
    let mut elements = Vec::new();
    for element in lexer.iterator() {
        elements.push(element?);
    }
    Ok((elements, start_time.elapsed()))
}

fn parse<I: FallibleIterator<Item = char, Error = io::Error>>(
    lexer: Lexer<I>,
) -> parser::Result<(Script, Duration)> {
    let start_time = Instant::now();
    let parser = Parser::for_lexer(lexer);
    parser.execute().map(|ast| (ast, start_time.elapsed()))
}

fn eval(ast: &Script) -> interpreter::Result<(interpreter::Value, Duration)> {
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

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
enum Mode {
    #[default]
    Eval,
    Parse,
    Lex,
    Repl,
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "--eval" => Ok(Self::Eval),
            "--parse" => Ok(Self::Parse),
            "--lex" => Ok(Self::Lex),
            "--repl" => Ok(Self::Repl),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Format {
    Json,
    Yaml,
}

impl FromStr for Format {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "--json" => Ok(Self::Json),
            "--yaml" => Ok(Self::Yaml),
            _ => Err(()),
        }
    }
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
