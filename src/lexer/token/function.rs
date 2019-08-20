/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:22:09 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/20 15:31:13 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{ComputorError as CError, ComputorResult as CResult};
use crate::memory::Memory;
use crate::parser::TokenTree;
use crate::types::Imaginary;

use std::any::Any;
use std::fmt;
use std::slice::Iter;

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

    fn get_result(&self, _mem: &Memory) -> CResult {
        panic!("Function left behind by Parser: {:?}", self);
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

    fn exec_fun(
        &self,
        mem: &Memory,
        first: Imaginary,
        mut iter: Iter<Box<TokenTree>>,
    ) -> CResult {
        let mut lst = vec![first];

        loop {
            match iter.next() {
                Some(tree) => match tree.compute(mem) {
                    CResult::Val(val) => lst.push(val),
                    _ => return CResult::Err(CError::fun_arg_inv(&self.id)),
                },
                None => break,
            }
        }
        match mem.get_fun(self.id.to_lowercase()) {
            Some(fun) => fun.compute(lst),
            None => CResult::Err(CError::fun_undefined(&self.id)),
        }
    }

    fn setup_fun(
        &self,
        mem: &Memory,
        first: String,
        mut iter: Iter<Box<TokenTree>>,
    ) -> CResult {
        let mut lst = vec![first];

        loop {
            match iter.next() {
                Some(tree) => match tree.compute(mem) {
                    CResult::Var(name, coef, pow) => {
                        if valid_var(coef, pow) {
                            lst.push(name);
                        } else {
                            return CResult::Err(CError::fun_arg_inv(&self.id));
                        }
                    }
                    _ => return CResult::Err(CError::fun_arg_inv(&self.id)),
                },
                None => break,
            }
        }
        CResult::Fun(self.id.to_lowercase(), lst, None)
    }
}

impl Token for FunctionTree {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(&self, mem: &Memory) -> CResult {
        let mut iter_param = self.param.iter();

        match iter_param.next() {
            Some(param) => match param.compute(mem) {
                CResult::Val(val) => self.exec_fun(mem, val, iter_param),
                CResult::Var(name, coef, pow) => {
                    if valid_var(coef, pow) {
                        self.setup_fun(mem, name, iter_param)
                    } else {
                        CResult::Err(CError::fun_arg_inv(&self.id))
                    }
                }
                _ => CResult::Err(CError::fun_arg_inv(&self.id)),
            },
            None => CResult::Err(CError::fun_arg_inv(&self.id)),
        }
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

fn valid_var(coef: Imaginary, pow: Imaginary) -> bool {
    coef == Imaginary::new(1.0, 0.0) && pow == Imaginary::new(1.0, 0.0)
}
