/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/25 18:52:04 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use crate::types::Imaginary;
use std::{error::Error, fmt};

enum Operation {
	Simple(fn(Imaginary, Imaginary) -> Imaginary),
	CanFail(fn(Imaginary, Imaginary) -> Imaginary),
}

#[derive(Debug)]
pub struct InvalidOperatorError {
	symbol: char,
}

impl fmt::Display for InvalidOperatorError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Invalid operator symbol caught : {}", self.symbol)
	}
}

impl Error for InvalidOperatorError {}

pub struct Operator {
	op: Operation,
}

impl Operator {
	pub fn new(symbol: char) -> Result<Operator, InvalidOperatorError> {
		let op = match symbol {
			'+' => Operation::Simple(add),
			'-' => Operation::Simple(sub),
			'*' => Operation::Simple(mul),
			'/' => Operation::CanFail(div),
			_ => return Err(InvalidOperatorError { symbol: symbol }),
		};
		Ok(Operator { op: op })
	}
}

fn add(val_a: Imaginary, val_b: Imaginary) -> Imaginary {
	Imaginary::zero()
}

fn sub(val_a: Imaginary, val_b: Imaginary) -> Imaginary {
	Imaginary::zero()
}

fn mul(val_a: Imaginary, val_b: Imaginary) -> Imaginary {
	Imaginary::zero()
}

fn div(val_a: Imaginary, val_b: Imaginary) -> Imaginary {
	Imaginary::zero()
}