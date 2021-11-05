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

        let lexer = Lexer::for_str(line);
        let mut parser = Parser::for_lexer(lexer);
        // FIXME: This is a bit of a hack really... In the current impl the parser uses the constant
        //  pool of the VM to seed its _own_ constant pool, which it uses during parsing, and then
        //  once evaluation begins `Program::eval` sets the VM's constant pool to the constant pool
        //  from the parser (which now contains the VM's original constants, plus any new ones added
        //  during `Parser::execute()`). Not confusing at all, right? Right.
        parser.extend_constant_pool(self.it.vm().constant_pool());

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
