/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   value.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:49 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/15 13:00:12 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::OldToken as Token;
use crate::error::ComputorError;
use crate::types::Imaginary;
use std::fmt;

#[derive(Clone)]
pub struct Value {
    value: Imaginary,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Value {
    pub fn new(raw: String) -> Result<Value, ComputorError> {
        if raw.ends_with('i') {
            if raw.len() > 1 {
                match raw[..raw.len() - 1].parse::<f64>() {
                    Ok(val) => Ok(Value {
                        value: Imaginary::new(0.0, val),
                    }),
                    Err(_err) => Err(ComputorError::invalid_value(raw)),
                }
            } else {
                Ok(Value {
                    value: Imaginary::new(0.0, 1.0),
                })
            }
        } else {
            match raw.parse::<f64>() {
                Ok(val) => Ok(Value {
                    value: Imaginary::new(val, 0.0),
                }),
                Err(_err) => Err(ComputorError::invalid_value(raw)),
            }
        }
    }

    pub fn add_val(lhs: &Value, rhs: &Value) -> Token {
        Token::Val(Value {
            value: lhs.value + rhs.value,
        })
    }

    pub fn sub_val(lhs: &Value, rhs: &Value) -> Token {
        Token::Val(Value {
            value: lhs.value - rhs.value,
        })
    }

    pub fn mul_val(lhs: &Value, rhs: &Value) -> Token {
        Token::Val(Value {
            value: lhs.value * rhs.value,
        })
    }

    pub fn div_val(lhs: &Value, rhs: &Value) -> Result<(Token), ComputorError> {
        if rhs.value != Imaginary::zero() {
            Ok(Token::Val(Value {
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
