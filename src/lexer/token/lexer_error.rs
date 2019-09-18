/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer_error.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 15:47:12 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/18 16:55:01 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Token;
use crate::computor::{ComputorError, TreeResult};
use crate::memory::{Extension, Memory};

use std::any::Any;
use std::{error::Error, fmt};

pub enum LexerError {
    InvalidOp(char),
    InvalidVal(String),
    InvalidVar(String),
    InvalidPar(String),
    InvalidFun(String, Vec<Vec<Box<dyn Token>>>),
}

impl Error for LexerError {}

impl fmt::Debug for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::InvalidOp(ch) => write!(f, "!{{{}}}", ch),
            LexerError::InvalidVal(word) => write!(f, "!{{{}}}", word),
            LexerError::InvalidVar(word) => write!(f, "!{{{}}}", word),
            LexerError::InvalidPar(word) => write!(f, "!{{{}}}", word),
            LexerError::InvalidFun(fun, _) => write!(f, "!{{{}}}", fun),
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info: (String, &str) = match self {
            LexerError::InvalidOp(ch) => {
                let mut symbol = String::new();
                symbol.push(*ch);
                (symbol, "operator")
            }
            LexerError::InvalidVal(word) => (word.clone(), "value"),
            LexerError::InvalidVar(word) => (word.clone(), "variable name"),
            LexerError::InvalidPar(w) => (w.clone(), "function parameter"),
            LexerError::InvalidFun(word, _) => (word.clone(), "function name"),
        };
        write!(f, "'{}' isn't a valid {}.", info.0, info.1)
    }
}

impl Token for LexerError {
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
        Err(ComputorError::unparsed_token(self))
    }
}
