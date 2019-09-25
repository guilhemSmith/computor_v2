/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:22:09 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/25 17:38:48 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{Computed as Comp, ComputorError as CError, TreeResult};
use crate::memory::{Extension, Memory, Value};
use crate::parser::TokenTree;

use std::any::Any;
use std::fmt;
use std::slice::Iter;

pub struct FunctionToken {
    id: String,
    param: Vec<Vec<Box<dyn Token>>>,
}

impl FunctionToken {
    pub fn new(
        id: String,
        vars: Vec<Vec<Box<dyn Token>>>,
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

    pub fn consume_param(&mut self) -> Vec<Vec<Box<dyn Token>>> {
        let mut extractor: Vec<Vec<Box<dyn Token>>> = Vec::new();
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
    ) -> TreeResult {
        Err(CError::unparsed_token(self))
    }
}

pub struct FunctionTree {
    id: String,
    param: Vec<Box<dyn TokenTree>>,
}

impl FunctionTree {
    pub fn new(id: String, vars: Vec<Box<dyn TokenTree>>) -> Self {
        FunctionTree {
            id: id,
            param: vars,
        }
    }

    pub fn param(&self) -> &Vec<Box<dyn TokenTree>> {
        &self.param
    }

    fn exec_fun(
        &self,
        mem: &Memory,
        mut ext: Option<&mut Extension>,
        first: Value,
        mut iter: Iter<Box<dyn TokenTree>>,
    ) -> TreeResult {
        let mut lst = vec![first];

        let mut loop_iter =
            |extend: Option<&mut Extension>| -> Result<bool, CError> {
                match iter.next() {
                    Some(tree) => match tree.compute(mem, extend)? {
                        Comp::Val(val) => {
                            lst.push(Value::Im(val));
                            Ok(true)
                        }
                        Comp::VarCall(_, val) => {
                            lst.push(Value::Im(val));
                            Ok(true)
                        }
                        Comp::Mat(val) => {
                            lst.push(Value::Mat(val));
                            Ok(true)
                        }
                        _ => {
                            return Err(CError::fun_arg_inv(&self.id));
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
                    Err(error) => return Err(error),
                }
            },
            None => loop {
                match loop_iter(None) {
                    Ok(true) => continue,
                    Ok(false) => break,
                    Err(error) => return Err(error),
                }
            },
        }
        let fun_mem = mem.get_fun(&self.id);
        match fun_mem {
            Some(fun) => fun.compute(mem, lst),
            None => Err(CError::fun_undef(&self.id)),
        }
    }

    fn setup_fun(
        &self,
        mem: &Memory,
        mut ext: Option<&mut Extension>,
        first: String,
        mut iter: Iter<Box<dyn TokenTree>>,
    ) -> TreeResult {
        let mut lst = vec![first];

        let mut loop_iter =
            |extend: Option<&mut Extension>| -> Result<bool, CError> {
                match iter.next() {
                    Some(tree) => match tree.compute(mem, extend)? {
                        Comp::VarSet(name) => {
                            lst.push(name);
                            return Ok(true);
                        }
                        _ => return Err(CError::fun_arg_inv(&self.id)),
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
                    Err(error) => return Err(error),
                }
            },
            None => loop {
                match loop_iter(None) {
                    Ok(true) => continue,
                    Ok(false) => break,
                    Err(error) => return Err(error),
                }
            },
        }
        Ok(Comp::FunSet(self.id.clone(), lst))
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
    ) -> TreeResult {
        let mut iter_param = self.param.iter();

        let check_first = |extend: Option<&mut Extension>,
                           clone: Option<&mut Extension>|
         -> TreeResult {
            match iter_param.next() {
                Some(param) => match param.compute(mem, clone)? {
                    Comp::Mat(val) => {
                        self.exec_fun(mem, extend, Value::Mat(val), iter_param)
                    }
                    Comp::Val(val) => {
                        self.exec_fun(mem, extend, Value::Im(val), iter_param)
                    }
                    Comp::VarCall(name, val) => {
                        let fun = mem.get_fun(&self.id);
                        if let None = fun {
                            self.setup_fun(mem, extend, name, iter_param)
                        } else {
                            self.exec_fun(
                                mem,
                                extend,
                                Value::Im(val),
                                iter_param,
                            )
                        }
                    }
                    Comp::VarSet(name) => {
                        self.setup_fun(mem, extend, name, iter_param)
                    }
                    _ => Err(CError::fun_arg_inv(&self.id)),
                },
                None => Err(CError::fun_arg_inv(&self.id)),
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
