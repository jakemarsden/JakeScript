use jakescript::ast::Program;
use jakescript::interpreter::{self, Eval, Interpreter};
use jakescript::lexer::Lexer;
use jakescript::parser::{self, Parser};

// dead_code: See https://github.com/rust-lang/rust/issues/46379
#[allow(dead_code)]
pub fn eval(ast: &Program) -> interpreter::Result {
    let mut it = Interpreter::default();
    ast.eval(&mut it)
}

// dead_code: See https://github.com/rust-lang/rust/issues/46379
#[allow(dead_code)]
pub fn parse_from_source_code(source_code: &str) -> parser::ParseResult {
    let lexer = Lexer::for_str(source_code);
    let parser = Parser::for_lexer(lexer);
    parser.execute()
}
