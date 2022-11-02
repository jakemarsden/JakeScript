use fallible_iterator::FallibleIterator;
use jakescript::interpreter::{Eval, ExecutionState, Interpreter};
use jakescript::lexer::{self, Lexer};
use jakescript::parser::Parser;
use jakescript::token::{Element, ElementKind, Punctuator, Token};
use std::{io, mem};

/// Read Evaluate Print Loop (REPL).
pub(crate) struct Repl<I>
where
    I: FallibleIterator<Item = char, Error = io::Error>,
{
    input: Lexer<I>,
    input_buf: Vec<Element>,
    brace_depth: usize,
}

impl<I> Repl<I>
where
    I: FallibleIterator<Item = char, Error = io::Error>,
{
    pub(crate) fn new(input: Lexer<I>) -> Self {
        Self {
            input,
            input_buf: Vec::new(),
            brace_depth: 0,
        }
    }

    pub(crate) fn execute(&mut self, it: &mut Interpreter) -> Result {
        loop {
            match it.vm().execution_state() {
                ExecutionState::Advance => {}
                ExecutionState::Exception(ex) => {
                    eprintln!("Exception: {ex}");
                    self.input_buf.clear();
                    return Result::ExitWithRuntimeError;
                }
                ExecutionState::Exit => {
                    eprintln!("Exit");
                    self.input_buf.clear();
                    return Result::ExitNormally;
                }
                ExecutionState::Break | ExecutionState::Continue | ExecutionState::Return(..) => {
                    unreachable!()
                }
            }

            eprint!("> {}", "  ".repeat(self.brace_depth));
            match self.read_next_elements() {
                BufferState::Execute => {}
                BufferState::KeepBuffering => {
                    continue;
                }
                BufferState::Err(lex_err) => {
                    eprintln!("Lex error: {lex_err}");
                    self.input_buf.clear();
                    continue;
                }
                BufferState::EndOfInput => {
                    eprintln!("Exit");
                    self.input_buf.clear();
                    return Result::EndOfInput;
                }
            }

            // TODO: Optimise: Don't recreate the buffer every time.
            let buf_size = self.input_buf.len();
            let stolen_tokens = mem::replace(&mut self.input_buf, Vec::with_capacity(buf_size));

            let parser = Parser::for_elements(stolen_tokens.into_iter());
            let ast = match parser.execute() {
                Ok(ast) => ast,
                Err(err) => {
                    eprintln!("Parse error: {err}");
                    self.input_buf.clear();
                    continue;
                }
            };

            let value = match ast.eval(it) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Runtime error: {err}");
                    self.input_buf.clear();
                    continue;
                }
            };
            eprintln!("{value}");
        }
    }

    fn read_next_elements(&mut self) -> BufferState {
        loop {
            let element = match self.input.next() {
                Ok(Some(element)) => element,
                Ok(None) => break BufferState::EndOfInput,
                Err(lex_err) => break BufferState::Err(lex_err),
            };
            match element.kind() {
                ElementKind::LineTerminator(..) => {
                    self.input_buf.push(element);
                    break if self.brace_depth == 0 {
                        BufferState::Execute
                    } else {
                        BufferState::KeepBuffering
                    };
                }
                ElementKind::Token(Token::Punctuator(Punctuator::OpenBrace)) => {
                    self.brace_depth = self.brace_depth.checked_add(1).unwrap();
                    self.input_buf.push(element);
                }
                ElementKind::Token(Token::Punctuator(Punctuator::CloseBrace)) => {
                    // Leave it to the parser to deal with mismatched braces
                    self.brace_depth = self.brace_depth.saturating_sub(1);
                    self.input_buf.push(element);
                }
                ElementKind::Token(..) | ElementKind::Comment(..) | ElementKind::Whitespace(..) => {
                    self.input_buf.push(element);
                }
            };
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Result {
    ExitNormally,
    ExitWithRuntimeError,
    EndOfInput,
}

enum BufferState {
    Execute,
    KeepBuffering,
    EndOfInput,
    Err(lexer::Error),
}
