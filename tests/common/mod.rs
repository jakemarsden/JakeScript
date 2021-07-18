use jakescript::interpreter::{Eval, Interpreter, Value};
use jakescript::lexer::Lexer;
use jakescript::parser::Parser;

pub(crate) fn eval_from_source(source_code: &str) -> Value {
    let lexer = Lexer::for_str(source_code);
    let parser = Parser::for_lexer(lexer);
    let program = parser.execute();

    let mut it = Interpreter::default();
    program.eval(&mut it)
}
