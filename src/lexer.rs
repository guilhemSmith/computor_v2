/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 16:50:34 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/26 16:27:31 by gsmith           ###   ########.fr       */
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

const PROMPT: &str = "> ";

pub enum Token {
    Expr(Expression),
    Orand(Operand),
    Orator(Operator),
    InvalidOperand(InvalidOperandError),
    InvalidOperator(InvalidOperatorError),
}

pub fn read_input() -> Expression {
    let mut res = Expression::new();
    let mut input = String::new();
    let len;
    let mut stdout = io::stdout();

    write!(&mut stdout, "{}", PROMPT);
    stdout.flush();
    match io::stdin().read_line(&mut input) {
        Ok(size) => len = size,
        Err(err) => log_error(err, 0),
    };
    writeln!(&mut stdout, "input: {}", input);

    let tik = match Operand::new("125", true) {
        Ok(op) => Token::Orand(op),
        Err(err) => Token::InvalidOperand(err),
    };
    let tok = match Operator::new('+') {
        Ok(op) => Token::Orator(op),
        Err(err) => Token::InvalidOperator(err),
    };
    res.push(tik);
    res.push(tok);
    return res;
}
