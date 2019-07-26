/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 16:50:34 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/26 12:06:10 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod expression;
mod operand;
mod operator;

pub use expression::Expression;
pub use operand::Operand;
pub use operator::Operator;

use crate::error::{InvalidOperandError, InvalidOperatorError};

pub enum Token {
    Expr(Expression),
    Orand(Operand),
    Orator(Operator),
    InvalidOperand(InvalidOperandError),
    InvalidOperator(InvalidOperatorError),
}

pub fn read_input() -> Expression {
    let tik = match Operand::new("125", true) {
        Ok(op) => Token::Orand(op),
        Err(err) => Token::InvalidOperand(err),
    };
    let tok = match Operator::new('+') {
        Ok(op) => Token::Orator(op),
        Err(err) => Token::InvalidOperator(err),
    };
    let mut res = Expression::new();
    res.push(tik);
    res.push(tok);
    return res;
}
