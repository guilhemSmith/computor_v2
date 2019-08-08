/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operand.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:49 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/08 15:05:09 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Token;
use crate::error::{ComputorError, DivByZeroError, InvalidOperandError};
use crate::types::{Imaginary, Raw};
use std::fmt;

#[derive(Clone)]
pub struct Operand {
    value: Imaginary,
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
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

    pub fn add_orand(lhs: &Operand, rhs: &Operand) -> Token {
        Token::Orand(Operand {
            value: lhs.value + rhs.value,
        })
    }

    pub fn sub_orand(lhs: &Operand, rhs: &Operand) -> Token {
        Token::Orand(Operand {
            value: lhs.value - rhs.value,
        })
    }

    pub fn mul_orand(lhs: &Operand, rhs: &Operand) -> Token {
        Token::Orand(Operand {
            value: lhs.value * rhs.value,
        })
    }

    pub fn div_orand(
        lhs: &Operand,
        rhs: &Operand,
    ) -> Result<(Token), ComputorError> {
        if rhs.value != Imaginary::zero() {
            Ok(Token::Orand(Operand {
                value: lhs.value / rhs.value,
            }))
        } else {
            Err(DivByZeroError::new(
                format!("{}", lhs.value),
                format!("{}", rhs.value),
                '/',
            ))
        }
    }
}
