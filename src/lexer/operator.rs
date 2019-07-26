/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/26 10:55:19 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::error::InvalidOperatorError;
use crate::types::Imaginary;

enum Operation {
    Simple(fn(Imaginary, Imaginary) -> Imaginary),
    CanFail(fn(Imaginary, Imaginary) -> Imaginary),
}

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
            _ => return Err(InvalidOperatorError::new(symbol)),
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
