/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer_error.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 15:47:12 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/23 17:21:46 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Token;
use crate::computor::{ComputorError, TreeResult};
use crate::memory::{Extension, Memory};
use crate::types::MatrixError;

use std::any::Any;
use std::{error::Error, fmt};

pub enum LexerError {
    InvalidOp(char),
    InvalidMat(MatrixError),
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
            LexerError::InvalidMat(err) => write!(f, "!{{{}}}", err),
            LexerError::InvalidVal(word) => write!(f, "!{{{}}}", word),
            LexerError::InvalidVar(word) => write!(f, "!{{{}}}", word),
            LexerError::InvalidPar(word) => write!(f, "!{{{}}}", word),
            LexerError::InvalidFun(fun, _) => write!(f, "!{{{}}}", fun),
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let txt = match self {
            LexerError::InvalidOp(ch) => {
                format!("'{}' isn't a valid operator.", ch)
            }
            LexerError::InvalidMat(err) => format!("{}", err),
            LexerError::InvalidVal(word) => {
                format!("'{}' isn't a valid value.", word)
            }
            LexerError::InvalidVar(word) => {
                format!("'{}' isn't a valid variable name.", word)
            }
            LexerError::InvalidPar(word) => {
                format!("'{}' isn't a valid function parameter.", word)
            }
            LexerError::InvalidFun(word, _) => {
                format!("'{}' isn't a valid function name.", word)
            }
        };
        write!(f, "{}", txt)
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
