/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   variable.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:16:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/30 17:38:41 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::ComputorResult;
use crate::memory::{Extension, Memory};
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
    ) -> ComputorResult {
        match ext {
            Some(extension) => match extension.get(&self.id) {
                Some(val) => return ComputorResult::Val(val),
                None => {}
            },
            None => {}
        }
        match mem.get_var_val(&self.id) {
            Some(val) => ComputorResult::VarCall(self.id.clone(), val),
            None => ComputorResult::VarSet(self.id.clone()),
        }
    }
}
