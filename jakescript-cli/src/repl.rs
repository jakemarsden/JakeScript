use jakescript::interpreter::{Eval, ExecutionState, Interpreter};
use jakescript::lexer::{self, Element, Lexer, Punctuator, Token};
use jakescript::parser::Parser;
use std::assert_matches::assert_matches;
use std::{io, mem};

/// Read Evaluate Print Loop (REPL).
pub(crate) struct Repl<I>
where
    I: Iterator<Item = io::Result<char>>,
{
    input: Lexer<I>,
    brace_depth: usize,
    token_buf: Vec<Token>,
}

impl<I> Repl<I>
where
    I: Iterator<Item = io::Result<char>>,
{
    pub(crate) fn new(input: Lexer<I>) -> Self {
        Self {
            input,
            brace_depth: 0,
            token_buf: Vec::new(),
        }
    }

    pub(crate) fn execute(&mut self, it: &mut Interpreter) -> Result {
        loop {
            if matches!(it.vm().execution_state(), ExecutionState::Exit) {
                eprintln!("Exit");
                self.token_buf.clear();
                return Result::ExitNormally;
            }
            assert_matches!(it.vm().execution_state(), ExecutionState::Advance);

            eprint!("> {}", "  ".repeat(self.brace_depth));
            match self.read_next_tokens() {
                BufferState::Execute => {}
                BufferState::KeepBuffering => {
                    continue;
                }
                BufferState::Err(lex_err) => {
                    eprintln!("Lex error: {}", lex_err);
                    self.token_buf.clear();
                    continue;
                }
                BufferState::EndOfInput => {
                    eprintln!("Exit");
                    self.token_buf.clear();
                    return Result::EndOfInput;
                }
            }

            // TODO: Optimise: Don't recreate the buffer every time.
            let buf_size = self.token_buf.len();
            let stolen_tokens = mem::replace(&mut self.token_buf, Vec::with_capacity(buf_size));

            let parser = Parser::for_tokens(stolen_tokens.into_iter());
            let ast = match parser.execute() {
                Ok(ast) => ast,
                Err(err) => {
                    eprintln!("Parse error: {}", err);
                    self.token_buf.clear();
                    continue;
                }
            };

            let value = match ast.eval(it) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Runtime error: {}", err);
                    self.token_buf.clear();
                    continue;
                }
            };
            eprintln!("{}", value);
        }
    }

    fn read_next_tokens(&mut self) -> BufferState {
        for element in &mut self.input {
            let element = match element {
                Ok(element) => element,
                Err(lex_err) => return BufferState::Err(lex_err),
            };
            match element {
                Element::Token(t @ Token::Punctuator(Punctuator::OpenBrace)) => {
                    self.brace_depth = self.brace_depth.checked_add(1).unwrap();
                    self.token_buf.push(t);
                }
                Element::Token(t @ Token::Punctuator(Punctuator::CloseBrace)) => {
                    // Leave it to the parser to deal with mismatched braces
                    self.brace_depth = self.brace_depth.saturating_sub(1);
                    self.token_buf.push(t);
                }
                Element::Token(t) => {
                    self.token_buf.push(t);
                }
                Element::LineTerminator(..) => {
                    return if self.brace_depth == 0 {
                        BufferState::Execute
                    } else {
                        BufferState::KeepBuffering
                    };
                }
                Element::Comment(..) | Element::Whitespace(..) => {}
            }
        }
        BufferState::EndOfInput
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum Result {
    ExitNormally,
    EndOfInput,
}

enum BufferState {
    Execute,
    KeepBuffering,
    EndOfInput,
    Err(lexer::Error),
}
