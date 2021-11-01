use jakescript::interpreter::{Eval, Interpreter};
use jakescript::lexer::Lexer;
use jakescript::parser::Parser;
use std::time::Instant;
use std::{env, fs, io};
use utf8_chars::BufReadCharsExt;

fn main() {
    let path = env::args().nth(1).expect("Expected path as CLI argument");

    let mut buf = io::BufReader::new(fs::File::open(&path).expect("Failed to open source file"));
    let lexer = Lexer::for_chars_fallible(buf.chars());

    let parser = Parser::for_lexer(lexer);
    let mut it = Interpreter::default();

    let parse_start_time = Instant::now();
    let ast = parser.execute().expect("Failed to parse program");
    let parse_runtime = parse_start_time.elapsed();
    println!("Parsed in {:?}", parse_runtime);

    let eval_start_time = Instant::now();
    let result = ast.eval(&mut it);
    let eval_runtime = eval_start_time.elapsed();
    println!(
        "Evaluated in {:?} (total: {:?})",
        eval_runtime,
        parse_runtime + eval_runtime
    );

    let result = result.expect("Failed to evaluate program");
    println!("{:?}", result);
}
