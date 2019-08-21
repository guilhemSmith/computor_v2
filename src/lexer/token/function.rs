/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:22:09 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/21 12:31:15 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{ComputorError as CError, ComputorResult as CResult};
use crate::memory::{Extension, Memory};
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

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> CResult {
        CResult::Err(CError::unparsed_token(self))
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
        mut ext: Option<&mut Extension>,
        first: Imaginary,
        mut iter: Iter<Box<TokenTree>>,
    ) -> CResult {
        let mut lst = vec![first];

        let mut loop_iter =
            |extend: Option<&mut Extension>| -> Result<bool, CResult> {
                match iter.next() {
                    Some(tree) => match tree.compute(mem, extend) {
                        CResult::Val(val) => {
                            lst.push(val);
                            Ok(true)
                        }
                        _ => {
                            return Err(CResult::Err(CError::fun_arg_inv(
                                &self.id,
                            )))
                        }
                    },
                    None => return Ok(false),
                }
            };

        match &mut ext {
            Some(extend) => loop {
                let mut ext_clone = extend.clone();
                match loop_iter(Some(&mut ext_clone)) {
                    Ok(true) => continue,
                    Ok(false) => break,
                    Err(error) => return error,
                }
            },
            None => loop {
                match loop_iter(None) {
                    Ok(true) => continue,
                    Ok(false) => break,
                    Err(error) => return error,
                }
            },
        }
        let fun_mem = mem.get_fun(&self.id);
        match fun_mem {
            Some(fun) => fun.compute(mem, lst),
            None => CResult::Err(CError::fun_undef(&self.id)),
        }
    }

    fn setup_fun(
        &self,
        mem: &Memory,
        mut ext: Option<&mut Extension>,
        first: String,
        mut iter: Iter<Box<TokenTree>>,
    ) -> CResult {
        let mut lst = vec![first];

        let mut loop_iter =
            |extend: Option<&mut Extension>| -> Result<bool, CResult> {
                match iter.next() {
                    Some(tree) => match tree.compute(mem, extend) {
                        CResult::Var(name, coef, pow) => {
                            if valid_var(coef, pow) {
                                lst.push(name);
                                return Ok(true);
                            } else {
                                return Err(CResult::Err(CError::fun_arg_inv(
                                    &self.id,
                                )));
                            }
                        }
                        _ => {
                            return Err(CResult::Err(CError::fun_arg_inv(
                                &self.id,
                            )))
                        }
                    },
                    None => return Ok(false),
                }
            };

        match &mut ext {
            Some(extend) => loop {
                let mut ext_clone = extend.clone();
                match loop_iter(Some(&mut ext_clone)) {
                    Ok(true) => continue,
                    Ok(false) => break,
                    Err(error) => return error,
                }
            },
            None => loop {
                match loop_iter(None) {
                    Ok(true) => continue,
                    Ok(false) => break,
                    Err(error) => return error,
                }
            },
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

    fn get_result(
        &self,
        mem: &Memory,
        mut ext: Option<&mut Extension>,
    ) -> CResult {
        let mut iter_param = self.param.iter();

        let check_first =
            |extend: Option<&mut Extension>, clone: Option<&mut Extension>| {
                match iter_param.next() {
                    Some(param) => match param.compute(mem, clone) {
                        CResult::Val(val) => {
                            self.exec_fun(mem, extend, val, iter_param)
                        }
                        CResult::Var(name, coef, pow) => {
                            if valid_var(coef, pow) {
                                self.setup_fun(mem, extend, name, iter_param)
                            } else {
                                CResult::Err(CError::fun_arg_inv(&self.id))
                            }
                        }
                        _ => CResult::Err(CError::fun_arg_inv(&self.id)),
                    },
                    None => CResult::Err(CError::fun_arg_inv(&self.id)),
                }
            };

        match &mut ext {
            Some(extend) => {
                let mut ext_clone = extend.clone();
                check_first(ext, Some(&mut ext_clone))
            }
            None => check_first(ext, None),
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
