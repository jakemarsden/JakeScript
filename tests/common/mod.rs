use jakescript::ast::Program;
use jakescript::interpreter::{Error, Eval, Interpreter, Value};
use jakescript::lexer::Lexer;
use jakescript::parser::Parser;

pub fn eval_from_source_code(source_code: &str) -> Result<Value, (Error, Program)> {
    let lexer = Lexer::for_str(source_code);
    let parser = Parser::for_lexer(lexer);
    let program = parser.execute();

    let mut it = Interpreter::default();
    program.eval(&mut it).map_err(|err| (err, program))
}
