/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 16:50:34 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/26 17:20:27 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod expression;
mod operand;
mod operator;

pub use expression::Expression;
pub use operand::Operand;
pub use operator::Operator;

use crate::error::{log_error, InvalidOperandError, InvalidOperatorError};
use std::io::{self, prelude::Write};
use std::vec::Vec;

const PROMPT: &str = "> ";

pub enum Token {
    Expr(Expression),
    Orand(Operand),
    Orator(Operator),
    InvalidOperand(InvalidOperandError),
    InvalidOperator(InvalidOperatorError),
}

pub struct Lexer {
    verbose: bool,
    hist: Vec<String>,
}

impl Lexer {
    pub fn new(verbose: bool) -> Self {
        Lexer {
            verbose: verbose,
            hist: Vec::new(),
        }
    }

    pub fn read_input(&mut self) -> Option<Expression> {
        let mut input = String::new();

        if !print_prompt() {
            return None;
        }
        match io::stdin().read_line(&mut input) {
            Ok(size) => {
                if size == 0 {
                    return None;
                }
            }
            Err(err) => log_error(err, 0),
        };
        self.hist.push(input.clone());
        Some(Expression::new(self.clear_input(input)))
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

fn print_prompt() -> bool {
    let mut stdout = io::stdout();

    match write!(&mut stdout, "{}", PROMPT) {
        Ok(_) => {}
        Err(err) => {
            log_error(err, 0);
            return false;
        }
    };
    match stdout.flush() {
        Ok(_) => true,
        Err(err) => {
            log_error(err, 0);
            false
        }
    }
}
