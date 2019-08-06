/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/06 12:50:14 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Operand;
use crate::error::{ComputorError, InvalidOperatorError};
use std::fmt;

#[derive(Clone)]
enum Operation {
    Basic(fn(&Operand, &Operand) -> Operand),
    Divide(fn(&Operand, &Operand) -> Result<(Operand), ComputorError>),
}

#[derive(Clone)]
pub struct Operator {
    symbol: char,
    op: Operation,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.symbol)
    }
}

impl Operator {
    pub fn new(symbol: char) -> Result<Operator, ComputorError> {
        let op = match symbol {
            '+' => Operation::Basic(add),
            '-' => Operation::Basic(sub),
            '*' => Operation::Basic(mul),
            '/' => Operation::Divide(div),
            _ => return Err(InvalidOperatorError::new(symbol)),
        };
        Ok(Operator {
            symbol: symbol,
            op: op,
        })
    }

    pub fn exec(
        &self,
        val_a: &Operand,
        val_b: &Operand,
    ) -> Result<(Operand), ComputorError> {
        match self.op {
            Operation::Basic(operation) => Ok(operation(val_a, val_b)),
            Operation::Divide(operation) => operation(val_a, val_b),
        }
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }

    pub fn prior(&self) -> bool {
        self.symbol == '*' || self.symbol == '/' || self.symbol == '%'
    }
}

fn add(val_a: &Operand, val_b: &Operand) -> Operand {
    Operand::add(val_a, val_b)
}

fn sub(val_a: &Operand, val_b: &Operand) -> Operand {
    Operand::sub(val_a, val_b)
}

fn mul(val_a: &Operand, val_b: &Operand) -> Operand {
    Operand::mul(val_a, val_b)
}

fn div(val_a: &Operand, val_b: &Operand) -> Result<(Operand), ComputorError> {
    Operand::div(val_a, val_b)
}
