/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   value.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:49 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/28 13:08:41 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{Computed, TreeResult};
use crate::memory::{Extension, Memory};
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
        match Imaginary::parse(&raw) {
            None => Err(LexerError::InvalidVal(raw)),
            Some(val) => Ok(Value { value: val }),
        }
    }

    pub fn from(value: Imaginary) -> Self {
        Value { value }
    }
}

impl Token for Value {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> TreeResult {
        Ok(Computed::ValIm(self.value))
    }
}
