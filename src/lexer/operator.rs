/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/30 13:07:51 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Operand;
use crate::error::{ComputorError, InvalidOperatorError};
use std::fmt;

enum Operation {
    Basic(fn(&mut Operand, Operand) -> &Operand),
    Divide(fn(&mut Operand, Operand) -> Result<&Operand, ComputorError>),
}

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

    pub fn exec<'a>(
        &self,
        val_a: &'a mut Operand,
        val_b: Operand,
    ) -> Result<&'a Operand, ComputorError> {
        match self.op {
            Operation::Basic(operation) => Ok(operation(val_a, val_b)),
            Operation::Divide(operation) => operation(val_a, val_b),
        }
    }
}

fn add(val_a: &mut Operand, val_b: Operand) -> &Operand {
    val_a.add(val_b)
}

fn sub(val_a: &mut Operand, val_b: Operand) -> &Operand {
    val_a.sub(val_b)
}

fn mul(val_a: &mut Operand, val_b: Operand) -> &Operand {
    val_a.mul(val_b)
}

fn div(val_a: &mut Operand, val_b: Operand) -> Result<&Operand, ComputorError> {
    val_a.div(val_b)
}
