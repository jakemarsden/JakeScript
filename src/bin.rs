use jakescript::interpreter::{Eval, Interpreter};
use jakescript::lexer::Lexer;
use jakescript::parser::Parser;
use std::path::PathBuf;
use std::time::Instant;
use std::{env, fs};

fn main() {
    let path = env::args().nth(1).expect("Expected path as CLI argument");
    let path = PathBuf::from(path);

    let source_code = fs::read_to_string(&path).expect("Failed to read source file");

    let lexer = Lexer::for_str(&source_code);
    let parser = Parser::for_lexer(lexer);
    let mut it = Interpreter::default();

    let parse_start_time = Instant::now();
    let ast = parser.execute();
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
