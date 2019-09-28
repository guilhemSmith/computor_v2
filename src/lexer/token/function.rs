/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:22:09 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/28 16:30:40 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{Computed as Comp, ComputorError as CError, TreeResult};
use crate::memory::{Extension, Memory};
use crate::parser::TokenTree;

use std::any::Any;
use std::fmt;

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

    pub fn param_mut(&mut self) -> &mut Vec<Box<dyn TokenTree>> {
        &mut self.param
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
        ext: Option<&mut Extension>,
    ) -> TreeResult {
        let mut args: Vec<Comp> = Vec::new();
        match ext {
            Some(extend) => {
                for arg in self.param.iter() {
                    let mut cloned = extend.clone();
                    args.push(arg.compute(mem, Some(&mut cloned))?);
                }
            }
            None => {
                for arg in self.param.iter() {
                    args.push(arg.compute(mem, None)?);
                }
            }
        }
        return Ok(Comp::FunId(self.id.clone(), args));
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
