/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 16:50:34 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/06 12:56:49 by gsmith           ###   ########.fr       */
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

use crate::error::{ComputorError, IOError};
use std::io::{self, prelude::Write};

const PROMPT: &str = "> ";

pub struct Lexer {
    verbose: bool,
}

impl Lexer {
    pub fn new(verbose: bool) -> Self {
        Lexer { verbose: verbose }
    }

    pub fn read_input(&mut self) -> Result<Expression, ComputorError> {
        let mut input = String::new();

        match print_prompt() {
            Err(err) => return Err(IOError::new(err)),
            _ => {}
        };
        match io::stdin().read_line(&mut input) {
            Err(err) => return Err(IOError::new(err)),
            Ok(len) => {
                if len < 1 {
                    println!("");
                }
            }
        };
        if self.verbose {
            println!("[V:Lexer] - input read: '{}'", input.trim());
        }
        Expression::new(self.clear_input(input))
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

fn print_prompt() -> Result<(), io::Error> {
    let mut stdout = io::stdout();

    write!(&mut stdout, "{}", PROMPT)?;
    stdout.flush()?;
    Ok(())
}
