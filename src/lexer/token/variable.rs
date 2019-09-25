/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   variable.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:16:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/25 17:32:53 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{Computed, TreeResult};
use crate::memory::{Extension, Memory, Value};
use std::any::Any;
use std::fmt;

#[derive(Clone)]
pub struct Variable {
    id: String,
}

impl Variable {
    pub fn new(id: String) -> Result<Self, LexerError> {
        let mut chars = id.chars();

        if !chars.next().unwrap().is_alphabetic() {
            return Err(LexerError::InvalidVar(id));
        }
        for ch in chars {
            if !ch.is_alphanumeric() {
                return Err(LexerError::InvalidVar(id));
            }
        }
        Ok(Variable { id: id })
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[var:{}]", self)
    }
}

impl Token for Variable {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(
        &self,
        mem: &Memory,
        ext: Option<&mut Extension>,
    ) -> TreeResult {
        if let Some(extension) = ext {
            let query = extension.get(&self.id);
            if let Some(var) = query {
                match var.val() {
                    Value::Im(val) => return Ok(Computed::Val(val)),
                    Value::Mat(val) => return Ok(Computed::Mat(val)),
                }
            }
        }
        match mem.get_var(&self.id) {
            Some(var) => match var.val() {
                Value::Im(val) => Ok(Computed::VarCall(var.name(), val)),
                Value::Mat(val) => Ok(Computed::Mat(val)),
            },
            None => Ok(Computed::VarSet(self.id.clone())),
        }
    }
}
