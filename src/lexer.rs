/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 16:50:34 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/14 16:25:10 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod expression;
mod function;
mod operand;
mod operator;
pub mod token;
mod variable;

pub use expression::Expression;
pub use function::Function;
pub use operand::Operand;
pub use operator::Operator;
pub use token::Token;
pub use variable::Variable;

extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::error::ComputorError;
use crate::Timer;

use std::collections::LinkedList;
use std::str::CharIndices;

const PROMPT: &str = "> ";

pub struct Lexer {
    verbose: bool,
    bench: bool,
    line: Editor<()>,
    last_ch: Option<(usize, char)>,
    depth: i32,
}

impl Lexer {
    pub fn new(verbose: bool, bench: bool) -> Self {
        Lexer {
            verbose: verbose,
            bench: bench,
            line: Editor::new(),
            last_ch: None,
            depth: 0,
        }
    }

    pub fn read_input(&mut self) -> Result<LinkedList<Token>, ComputorError> {
        let readline = self.line.readline(PROMPT);
        match readline {
            Ok(line) => {
                self.line.add_history_entry(line.as_str());
                if self.verbose {
                    println!("[V:Lexer] - input read: '{}'", line);
                }
                let cleared = self.clear_input(line);
                let mut iter = cleared.char_indices();
                let tokens;
                self.depth = 0;
                self.last_ch = None;
                if !self.bench {
                    tokens = self.tokenize(&mut iter);
                } else {
                    let _timer = Timer::new("Lexing");
                    tokens = self.tokenize(&mut iter);
                }
                if self.depth == 0 && iter.next() == None {
                    Ok(tokens)
                } else {
                    Err(ComputorError::invalid_expr())
                }
            }
            Err(ReadlineError::Interrupted) => Err(ComputorError::io_stop()),
            Err(ReadlineError::Eof) => Err(ComputorError::io_stop()),
            Err(err) => Err(ComputorError::io(&format!("{:?}", err))),
        }
    }

    fn tokenize(&mut self, chars: &mut CharIndices) -> LinkedList<Token> {
        let mut tokens: LinkedList<Token> = LinkedList::new();
        loop {
            match if self.last_ch != None {
                self.last_ch
            } else {
                chars.next()
            } {
                Some((i, ch)) if ch.is_digit(10) => {
                    self.last_ch = Some((i, ch));
                    tokens.push_back(self.read_val(chars));
                }
                Some((i, ch)) if ch.is_alphabetic() => {
                    self.last_ch = Some((i, ch));
                    tokens.push_back(self.read_id(chars));
                }
                Some((_, ch)) if ch == '=' => tokens.push_back(Token::Equal),
                Some((_, ch)) if ch == '?' => tokens.push_back(Token::Resolve),
                Some((_, ch)) if ch == '(' => {
                    self.depth += 1;
                    tokens.push_back(Token::Expr(Expression::new(
                        self.tokenize(chars),
                    )));
                    self.last_ch = None;
                }
                Some((_, ch)) if ch == ')' => {
                    self.depth -= 1;
                    break;
                }
                Some((_, ch)) => {
                    tokens.push_back(match Operator::new(ch) {
                        Ok(orator) => Token::Orator(orator),
                        Err(err) => Token::Invalid(err),
                    });
                    self.last_ch = None;
                },
                None => break,
            }
        }
        return tokens;
    }

    fn read_val(&mut self, chars: &mut CharIndices) -> Token {
        let mut raw = String::new();
        let is_real: bool;

        raw.push(self.last_ch.unwrap().1);
        loop {
            match chars.next() {
                Some((_, ch)) if ch == '.' || ch.is_digit(10) => raw.push(ch),
                Some((_, ch)) if ch == 'i' => {
                    is_real = false;
                    self.last_ch = chars.next();
                    break;
                }
                Some((i, ch)) => {
                    is_real = true;
                    self.last_ch = Some((i, ch));
                    break;
                }
                None => {
                    is_real = true;
                    self.last_ch = None;
                    break;
                }
            }
        }
        match Operand::new(raw, is_real) {
            Ok(orand) => Token::Orand(orand),
            Err(err) => Token::Invalid(err),
        }
    }

    fn read_id(&mut self, chars: &mut CharIndices) -> Token {
        let mut raw = String::new();

        raw.push(self.last_ch.unwrap().1);
        loop {
            match chars.next() {
                Some((i, ch))
                    if ch == '+'
                        || ch == '-'
                        || ch == '*'
                        || ch == '/'
                        || ch == '='
                        || ch == '?' =>
                {
                    self.last_ch = Some((i, ch));
                    break;
                }
                Some((_, ch)) if ch == '(' => {
                    let mut param = String::new();
                    loop {
                        match chars.next() {
                            Some((_, ch)) if ch == ')' => {
                                self.last_ch = chars.next();
                                return match Function::new(raw, param) {
                                    Ok(fun) => Token::Fun(fun),
                                    Err(err) => Token::Invalid(err),
                                };
                            }
                            Some((_, ch)) => param.push(ch),
                            None => {
                                self.last_ch = None;
                                raw.push('(');
                                raw.push_str(&param);
                                return Token::Invalid(
                                    ComputorError::invalid_token(raw),
                                );
                            }
                        }
                    }
                }
                Some((_, ch)) => raw.push(ch),
                None => {
                    self.last_ch = None;
                    break;
                }
            }
        }
        match Variable::new(raw) {
            Ok(var) => Token::Var(var),
            Err(err) => Token::Invalid(err),
        }
    }

    fn clear_input(&self, raw_input: String) -> String {
        let mut cleared = String::new();
        let mut iter = raw_input.trim().split_whitespace();

        loop {
            match iter.next() {
                Some(word) => cleared.push_str(word),
                None => {
                    if self.verbose {
                        println!("[V:Lexer] - input cleared: '{}'", cleared);
                    }
                    return cleared;
                }
            };
        }
    }
}
