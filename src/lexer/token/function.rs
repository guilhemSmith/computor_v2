/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:22:09 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/19 12:06:31 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::ComputorResult;
use crate::computor_error::ComputorError;
use crate::memory::Memory;
// use crate::types::Imaginary;
use std::any::Any;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Function {
    id: String,
    param: Vec<Rc<Token>>,
}

impl Function {
    pub fn new(id: String, vars: Vec<Rc<Token>>) -> Result<Self, LexerError> {
        let mut chars = id.chars();

        if !chars.next().unwrap().is_alphabetic() {
            return Err(LexerError::InvalidFun(id, vars));
        }
        for ch in chars {
            if !ch.is_alphanumeric() {
                return Err(LexerError::InvalidFun(id, vars));
            }
        }
        Ok(Function {
            id: id,
            param: vars,
        })
    }

    pub fn param(&self) -> &Vec<Rc<Token>> {
        &self.param
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let param = super::display_token(&self.param, ", ");
        write!(f, "{}({})", self.id, param)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let param = super::debug_token(&self.param, ", ");
        write!(f, "[fun:{}({})]", self.id, param)
    }
}

impl Token for Function {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(&self, mem: &Memory) -> ComputorResult {
        let param_known: bool;
        let mut iter_param = self.param.iter();

        match iter_param.next() {
            None => return ComputorResult::Err(ComputorError::invalid_fun()),
            Some(token) => match token.get_result(mem) {
                ComputorResult::Resolve => {
                    return ComputorResult::Err(ComputorError::invalid_fun())
                }
                ComputorResult::Value(_val) => {}
                ComputorResult::Unknown(_id, _coef, _pow) => {}
                ComputorResult::SolveVar(_id, _coefs) => {
                    return ComputorResult::Err(ComputorError::invalid_fun())
                }
                ComputorResult::AssignFun(_id, _param, _exec) => {
                    return ComputorResult::Err(ComputorError::invalid_fun())
                }
                ComputorResult::Err(error) => {
                    return ComputorResult::Err(error)
                }
            },
        };
        ComputorResult::Resolve
    }
}
