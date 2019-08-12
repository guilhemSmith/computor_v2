/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 16:50:34 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/12 18:50:43 by gsmith           ###   ########.fr       */
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

use crate::error::ComputorError;
use crate::Timer;

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

    pub fn read_input(&mut self) -> Result<Expression, ComputorError> {
        let readline = self.line.readline(PROMPT);
        match readline {
            Ok(line) => {
                self.line.add_history_entry(line.as_str());
                if !self.bench {
                    self.lexing_input(line)
                } else {
                    let _timer = Timer::new("Lexing");
                    self.lexing_input(line)
                }
            }
            Err(ReadlineError::Interrupted) => Err(ComputorError::io_stop()),
            Err(ReadlineError::Eof) => Err(ComputorError::io_stop()),
            Err(err) => Err(ComputorError::io(&format!("{:?}", err))),
        }
    }

    fn lexing_input(&self, input: String) -> Result<Expression, ComputorError> {
        if self.verbose {
            println!("[V:Lexer] - input read: '{}'", input.trim());
        }
        Expression::new(self.clear_input(input), 0)
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
