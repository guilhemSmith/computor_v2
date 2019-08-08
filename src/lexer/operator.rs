/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/08 12:52:49 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Operand, Token};
use crate::error::{BadUseOperatorError, ComputorError, InvalidOperatorError};
use std::{collections::LinkedList as LList, fmt};

// #[derive(Clone)]
// enum Operation {
//     Basic(fn(&Token, &Token) -> Result<(LList<Token>), ComputorError>),
//     Divide(fn(&Token, &Token) -> Result<(LList<Token>), ComputorError>),
// }

#[derive(Clone)]
pub struct Operator {
    symbol: char,
    op: fn(&Token, &Token) -> Result<(LList<Token>), ComputorError>,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.symbol)
    }
}

impl Operator {
    pub fn new(symbol: char) -> Result<Operator, ComputorError> {
        let op = match symbol {
            '+' => add,
            '-' => sub,
            '*' => mul,
            '/' => div,
            _ => return Err(InvalidOperatorError::new(symbol)),
        };
        Ok(Operator {
            symbol: symbol,
            op: op,
        })
    }

    pub fn exec(
        &self,
        val_a: &Token,
        val_b: &Token,
    ) -> Result<(LList<Token>), ComputorError> {
        (self.op)(val_a, val_b)
        // match self.op {
        //     Operation::Basic(operation) => Ok(operation(val_a, val_b)),
        //     Operation::Divide(operation) => operation(val_a, val_b),
        // }
    }

    // pub fn symbol(&self) -> char {
    //     self.symbol
    // }

    pub fn prior(&self) -> bool {
        self.symbol == '*' || self.symbol == '/' || self.symbol == '%'
    }
}

fn add(val_a: &Token, val_b: &Token) -> Result<(LList<Token>), ComputorError> {
    let mut result: LList<Token> = LList::new();
    match (val_a, val_b) {
        (Token::Orand(op_a), Token::Orand(op_b)) => {
            result.push_back(Token::Orand(Operand::add(op_a, op_b)))
        }
        _ => return Err(BadUseOperatorError::new('+')),
    };
    return Ok(result);
}

fn sub(val_a: &Token, val_b: &Token) -> Result<(LList<Token>), ComputorError> {
    let mut result: LList<Token> = LList::new();
    match (val_a, val_b) {
        (Token::Orand(op_a), Token::Orand(op_b)) => {
            result.push_back(Token::Orand(Operand::sub(op_a, op_b)))
        }
        _ => return Err(BadUseOperatorError::new('-')),
    };
    return Ok(result);
}

fn mul(val_a: &Token, val_b: &Token) -> Result<(LList<Token>), ComputorError> {
    let mut result: LList<Token> = LList::new();
    match (val_a, val_b) {
        (Token::Orand(op_a), Token::Orand(op_b)) => {
            result.push_back(Token::Orand(Operand::mul(op_a, op_b)))
        }
        _ => return Err(BadUseOperatorError::new('*')),
    };
    return Ok(result);
}

fn div(val_a: &Token, val_b: &Token) -> Result<(LList<Token>), ComputorError> {
    let mut result: LList<Token> = LList::new();
    match (val_a, val_b) {
        (Token::Orand(op_a), Token::Orand(op_b)) => {
            result.push_back(Token::Orand(Operand::div(op_a, op_b)?))
        }
        _ => return Err(BadUseOperatorError::new('/')),
    };
    return Ok(result);
}
