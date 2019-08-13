/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 16:50:34 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/13 17:21:58 by gsmith           ###   ########.fr       */
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

use crate::error::{ComputorError, ErrorPosition};
use crate::Timer;

use std::collections::LinkedList;
use std::str::CharIndices;

const PROMPT: &str = "> ";

pub struct Lexer {
    verbose: bool,
    bench: bool,
    line: Editor<()>,
}

impl Lexer {
    pub fn new(verbose: bool, bench: bool) -> Self {
        Lexer {
            verbose: verbose,
            bench: bench,
            line: Editor::new(),
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
                let cloned = cleared.clone();
                let mut iter = cloned.char_indices();
                if !self.bench {
                    self.tokenize(cleared, &mut iter, 0, false)
                } else {
                    let _timer = Timer::new("Lexing");
                    self.tokenize(cleared, &mut iter, 0, false)
                }
            }
            Err(ReadlineError::Interrupted) => Err(ComputorError::io_stop()),
            Err(ReadlineError::Eof) => Err(ComputorError::io_stop()),
            Err(err) => Err(ComputorError::io(&format!("{:?}", err))),
        }
    }

    fn tokenize(
        &self,
        input: String,
        chars: &mut CharIndices,
        start: usize,
        stop_with_closing: bool,
    ) -> Result<LinkedList<Token>, ComputorError> {
        let mut toks: LinkedList<Token> = LinkedList::new();
        let mut ind: i64 = -1;

        if self.verbose {
            println!("[V:Lexer] - tokenize: '{}'", input);
        }
        loop {
            match chars.next() {
                Some((i, ch))
                    if ch == '+' || ch == '-' || ch == '*' || ch == '/' =>
                {
                    push_orand(&mut toks, &mut ind, i - start, &input, start);
                    let orator = Operator::new(ch)?;
                    toks.push_back(Token::Orator(orator));
                }
                Some((i, ch)) if ch == '(' => {
                    push_orand(&mut toks, &mut ind, i - start, &input, start);
                    let sub_str = String::from(&input[i - start + 1 - start..]);
                    let sub_tok =
                        self.tokenize(sub_str, chars, i - start + 1, true)?;
                    toks.push_back(Token::Expr(Expression::new(sub_tok)));
                }
                Some((i, ch)) if ch == ')' && stop_with_closing => {
                    push_orand(&mut toks, &mut ind, i - start, &input, start);
                    break;
                }
                Some((i, eq)) if eq == '=' => {
                    push_orand(&mut toks, &mut ind, i - start, &input, start);
                    toks.push_back(Token::Equal);
                }
                Some((i, res)) if res == '?' => {
                    push_orand(&mut toks, &mut ind, i - start, &input, start);
                    toks.push_back(Token::Resolve);
                }
                Some((i, _)) => {
                    if ind < 0 {
                        ind = (i - start) as i64;
                    }
                }
                None => {
                    push_orand(&mut toks, &mut ind, input.len(), &input, start);
                    break;
                }
            }
        }
        return Ok(toks);
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

fn read_operand(raw_operand: &str, pos: usize) -> Token {
    let is_real = match raw_operand.chars().rev().next() {
        Some(c) => c != 'i',
        None => true,
    };

    match Operand::new(
        if is_real {
            raw_operand
        } else {
            let len = raw_operand.len();
            if len > 1 {
                &raw_operand[..raw_operand.len() - 1]
            } else {
                "1"
            }
        },
        is_real,
    ) {
        Ok(orand) => Token::Orand(orand),
        Err(mut err) => {
            err.set_pos(ErrorPosition::Char(pos));
            Token::Invalid(err)
        }
    }
}

fn push_orand(
    toks: &mut LinkedList<Token>,
    start: &mut i64,
    stop: usize,
    chain: &String,
    pos: usize,
) {
    let i = *start;
    if i >= 0 {
        let u = i as usize;
        toks.push_back(read_operand(&chain[u..stop], u + pos));
        *start = -1;
    }
}
