/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operand.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:49 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/30 13:08:17 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::error::{ComputorError, DivByZeroError, InvalidOperandError};
use crate::types::{Imaginary, Raw};
use std::fmt;

pub struct Operand {
    value: Imaginary,
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.value)
    }
}

impl Operand {
    pub fn new(raw_str: &str, is_real: bool) -> Result<Operand, ComputorError> {
        let fl_value = match raw_str.parse::<f64>() {
            Ok(val) => val,
            Err(_err) => {
                return Err(InvalidOperandError::new(raw_str, is_real));
            }
        };

        if is_real {
            Ok(Operand {
                value: Imaginary::new(Raw::Float(fl_value), Raw::Zero),
            })
        } else {
            Ok(Operand {
                value: Imaginary::new(Raw::Zero, Raw::Float(fl_value)),
            })
        }
    }

    pub fn add(&mut self, rhs: Operand) -> &Operand {
        self.value = self.value + rhs.value;
        self
    }

    pub fn sub(&mut self, rhs: Operand) -> &Operand {
        self.value = self.value - rhs.value;
        self
    }

    pub fn mul(&mut self, rhs: Operand) -> &Operand {
        self.value = self.value * rhs.value;
        self
    }

    pub fn div(&mut self, rhs: Operand) -> Result<&Operand, ComputorError> {
        if rhs.value != Imaginary::zero() {
            self.value = self.value / rhs.value;
            Ok(self)
        } else {
            Err(DivByZeroError::new(
                format!("{}", self.value),
                format!("{}", rhs.value),
                '/',
            ))
        }
    }
}
