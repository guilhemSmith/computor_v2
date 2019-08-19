/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:22:09 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/19 18:02:00 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::ComputorResult;
use crate::memory::Memory;
use crate::parser::TokenTree;
// use crate::types::Imaginary;
use std::any::Any;
use std::fmt;

pub struct FunctionToken {
    id: String,
    param: Vec<Vec<Box<Token>>>,
}

impl FunctionToken {
    pub fn new(
        id: String,
        vars: Vec<Vec<Box<Token>>>,
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

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn consume_param(&mut self) -> Vec<Vec<Box<Token>>> {
        let mut extractor: Vec<Vec<Box<Token>>> = Vec::new();
        std::mem::swap(&mut self.param, &mut extractor);
        return extractor;
    }
}

impl Token for FunctionToken {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(&self, _mem: &Memory) -> ComputorResult {
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
    param: Vec<Box<TokenTree>>,
}

impl FunctionTree {
    pub fn new(id: String, vars: Vec<Box<TokenTree>>) -> Self {
        FunctionTree {
            id: id,
            param: vars,
        }
    }

    pub fn param(&self) -> &Vec<Box<TokenTree>> {
        &self.param
    }
}

impl Token for FunctionTree {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(&self, _mem: &Memory) -> ComputorResult {
        ComputorResult::Resolve
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

impl fmt::Display for FunctionTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut param = String::new();
        for p in &self.param {
            param = format!("{}, {}", param, p);
        }
        write!(f, "{}({})", self.id, param.trim_start_matches(", "))
    }
}

impl fmt::Debug for FunctionTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut param = String::new();
        for p in &self.param {
            param = format!("{},{:?}", param, p);
        }
        write!(f, "[fun:{}({})]", self.id, param.trim_start_matches(","))
    }
}
