/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   value.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:49 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/20 10:02:51 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::ComputorResult;
use crate::memory::Memory;
use crate::types::Imaginary;
use std::any::Any;
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

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[val:{}]", self)
    }
}

impl Value {
    pub fn new(raw: String) -> Result<Value, LexerError> {
        if raw.ends_with('i') {
            if raw.len() > 1 {
                match raw[..raw.len() - 1].parse::<f64>() {
                    Ok(val) => Ok(Value {
                        value: Imaginary::new(0.0, val),
                    }),
                    Err(_err) => Err(LexerError::InvalidVal(raw)),
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
                Err(_err) => Err(LexerError::InvalidVal(raw)),
            }
        }
    }
}

impl Token for Value {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(&self, _mem: &Memory) -> ComputorResult {
        ComputorResult::Value(self.value)
    }
}
