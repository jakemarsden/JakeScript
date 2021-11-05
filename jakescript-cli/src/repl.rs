use jakescript::interpreter::{self, Eval, Interpreter};
use jakescript::lexer::Lexer;
use jakescript::parser::{self, Parser};

/// Read Evaluate Print Loop (REPL).
#[derive(Default)]
pub(crate) struct Repl {
    it: Interpreter,
}

impl Repl {
    pub(crate) fn new(it: Interpreter) -> Self {
        Self { it }
    }

    pub(crate) fn handle_input_str(&mut self, line: &str) -> Value {
        assert!(self.it.vm().execution_state().is_advance());
        if line.trim() == "exit" {
            return Value::Exit;
        }

        // FIXME: Make the constant pool work with the REPL, because at the moment it's hideously
        //  broken! Maybe it would be possible to give the parser some initial state?
        let lexer = Lexer::for_str(line);
        let parser = Parser::for_lexer(lexer);
        let ast = match parser.execute() {
            Ok(ast) => ast,
            Err(err) => return Value::ParseErr(err),
        };

        let value = match ast.eval(&mut self.it) {
            Ok(value) => value,
            Err(err) => return Value::RuntimeErr(err),
        };
        Value::Ok(value)
    }
}

#[derive(Debug)]
pub(crate) enum Value {
    Ok(interpreter::Value),
    ParseErr(parser::Error),
    RuntimeErr(interpreter::Error),
    Exit,
}
