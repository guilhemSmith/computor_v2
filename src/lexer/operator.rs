/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/26 11:45:24 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::error::{DivByZeroError, InvalidOperatorError};
use crate::lexer::Operand;

enum Operation {
	Basic(fn(&mut Operand, Operand) -> &Operand),
	Divide(fn(&mut Operand, Operand) -> Result<&Operand, DivByZeroError>),
}

pub struct Operator {
	op: Operation,
}

impl Operator {
	pub fn new(symbol: char) -> Result<Operator, InvalidOperatorError> {
		let op = match symbol {
			'+' => Operation::Basic(add),
			'-' => Operation::Basic(sub),
			'*' => Operation::Basic(mul),
			'/' => Operation::Divide(div),
			_ => return Err(InvalidOperatorError::new(symbol)),
		};
		Ok(Operator { op: op })
	}

	pub fn exec(&self, val_a: &mut Operand, val_b: Operand) {}
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

fn div(
	val_a: &mut Operand,
	val_b: Operand,
) -> Result<&Operand, DivByZeroError> {
	val_a.div(val_b)
}
