/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:22:09 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/19 15:48:35 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::ComputorResult;
use crate::computor_error::ComputorError;
use crate::memory::Memory;
use crate::parser::TokenTree;
// use crate::types::Imaginary;
use std::any::Any;
use std::fmt;
use std::rc::Rc;

pub struct FunctionToken {
    id: String,
    param: Vec<Vec<Rc<Token>>>,
}

impl FunctionToken {
    pub fn new(
        id: String,
        vars: Vec<Vec<Rc<Token>>>,
    ) -> Result<Self, LexerError> {
        let mut chars = id.chars();

        if !chars.next().unwrap().is_alphabetic() {
            return Err(LexerError::InvalidFun(id, vars));
        }
        for ch in chars {
            if !ch.is_alphanumeric() {
                return Err(LexerError::InvalidFun(id, vars));
            }
        }
        Ok(FunctionToken {
            id: id,
            param: vars,
        })
    }

    pub fn consume_param(self) -> Vec<Vec<Rc<Token>>> {
        self.param
    }
}

impl fmt::Display for FunctionToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut param = String::new();
        for p in &self.param {
            param = format!("{}, {}", param, super::display_token(&p));
        }
        write!(f, "{}({})", self.id, param.trim_start_matches(", "))
    }
}

impl fmt::Debug for FunctionToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut param = String::new();
        for p in &self.param {
            param = format!("{},{}", param, super::debug_token(&p));
        }
        write!(f, "[fun:{}({})]", self.id, param.trim_start_matches(","))
    }
}

impl Token for FunctionToken {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(&self, mem: &Memory) -> ComputorResult {
        panic!("Function left behind by Parser: {}", self);
        // let mut iter_param = self.param.iter();

        // match iter_param.next() {
        //     None => return ComputorResult::Err(ComputorError::invalid_fun()),
        //     Some(token) => match token.get_result(mem) {
        //         ComputorResult::Resolve => {
        //             return ComputorResult::Err(ComputorError::invalid_fun())
        //         }
        //         ComputorResult::Value(_val) => {}
        //         ComputorResult::Unknown(_id, _coef, _pow) => {}
        //         ComputorResult::SolveVar(_id, _coefs) => {
        //             return ComputorResult::Err(ComputorError::invalid_fun())
        //         }
        //         ComputorResult::AssignFun(_id, _param, _exec) => {
        //             return ComputorResult::Err(ComputorError::invalid_fun())
        //         }
        //         ComputorResult::Err(error) => {
        //             return ComputorResult::Err(error)
        //         }
        //     },
        // };
    }
}

pub struct FunctionTree {
    id: String,
    param: Vec<Rc<TokenTree>>,
}

impl FunctionTree {
    pub fn new(
        id: String,
        vars: Vec<Rc<TokenTree>>,
    ) -> Result<Self, LexerError> {
        Ok(FunctionTree {
            id: id,
            param: vars,
        })
    }

    pub fn param(&self) -> &Vec<Rc<TokenTree>> {
        &self.param
    }
}
