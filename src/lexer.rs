/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 16:50:34 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/13 14:12:18 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod expression;
mod operand;
mod operator;
mod token;

pub use expression::Expression;
pub use operand::Operand;
pub use operator::Operator;
pub use token::Token;

extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::error::{ComputorError, ErrorPosition};
use crate::Timer;

use std::collections::LinkedList;

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
                if !self.bench {
                    self.lexing_input(self.clear_input(line), 0)
                } else {
                    let _timer = Timer::new("Lexing");
                    self.lexing_input(self.clear_input(line), 0)
                }
            }
            Err(ReadlineError::Interrupted) => Err(ComputorError::io_stop()),
            Err(ReadlineError::Eof) => Err(ComputorError::io_stop()),
            Err(err) => Err(ComputorError::io(&format!("{:?}", err))),
        }
    }

    fn lexing_input(&self, input: String, start: usize) -> Result<LinkedList<Token>, ComputorError> {
        if self.verbose {
            println!("[V:Lexer] - input read: '{}'", input.trim());
        }
        let mut tokens: LinkedList::<Token> = LinkedList::new();
        let mut op_index: i64 = -1;
        let last = input.len();
        let mut iter_char = input.char_indices();

        loop {
            match iter_char.next() {
                Some((i, ch))
                    if ch == '+' || ch == '-' || ch == '*' || ch == '/' =>
                {
                    if op_index >= 0 {
                        let u = op_index as usize;
                        tokens.push_back(read_operand(&input[u..i], u + start));
                        op_index = -1;
                    }
                    
                    let orator = Operator::new(ch)?;
                    tokens.push_back(Token::Orator(orator));
                }
                Some((i, ch)) if ch == '(' => {
                    if op_index >= 0 {
                        let u = op_index as usize;
                        tokens.push_back(read_operand(&input[u..i], u + start));
                        op_index = -1;
                    }
                    let mut end_exp = i + 1;
                    let mut opening = 1;
                    while opening > 0 {
                        match iter_char.next() {
                            Some((_, ch)) => {
                                end_exp += 1;
                                if ch == ')' {
                                    opening -= 1;
                                    continue;
                                }
                                if ch == '(' {
                                    opening += 1;
                                    continue;
                                }
                            }
                            None => {
                                return Err(ComputorError::incomplete_expr(
                                    &input[i + 1 - 1..],
                                ))
                            }
                        }
                    }
                    let tok = Token::Expr(Expression::new(self.lexing_input(String::from(&input[i + 1..end_exp - 1]), start + i + 1 + 1)?));
                    tokens.push_back(tok);
                }
                Some((i, _)) => {
                    if op_index < 0 {
                        op_index = i as i64;
                    }
                }
                None => {
                    if op_index >= 0 {
                        let u = op_index as usize;
                        tokens.push_back(read_operand(&input[u..last], u + start));
                        op_index = -1;
                    }
                    break;
                }
            }
        }
        return Ok(tokens);
        // Expression::new(self.clear_input(input), 0)
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

