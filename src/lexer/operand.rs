/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operand.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:49 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/13 10:26:41 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Token;
use crate::error::ComputorError;
use crate::types::Imaginary;
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
                return Err(ComputorError::invalid_operand(raw_str, is_real));
            }
        };

        if is_real {
            Ok(Operand {
                value: Imaginary::new(fl_value, 0.0),
            })
        } else {
            Ok(Operand {
                value: Imaginary::new(0.0, fl_value),
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
            Err(ComputorError::div_by_zero(
                format!("{}", lhs.value),
                format!("{}", rhs.value),
                '/',
            ))
        }
    }
}
