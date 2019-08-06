/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operand.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:49 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/06 12:49:53 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::error::{ComputorError, DivByZeroError, InvalidOperandError};
use crate::types::{Imaginary, Raw};
use std::fmt;

#[derive(Clone)]
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

    pub fn add(lhs: &Operand, rhs: &Operand) -> Operand {
        Operand {
            value: lhs.value + rhs.value,
        }
    }

    pub fn sub(lhs: &Operand, rhs: &Operand) -> Operand {
        Operand {
            value: lhs.value - rhs.value,
        }
    }

    pub fn mul(lhs: &Operand, rhs: &Operand) -> Operand {
        Operand {
            value: lhs.value * rhs.value,
        }
    }

    pub fn div(
        lhs: &Operand,
        rhs: &Operand,
    ) -> Result<(Operand), ComputorError> {
        if rhs.value != Imaginary::zero() {
            Ok(Operand {
                value: lhs.value / rhs.value,
            })
        } else {
            Err(DivByZeroError::new(
                format!("{}", lhs.value),
                format!("{}", rhs.value),
                '/',
            ))
        }
    }
}
