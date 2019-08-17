/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lexer_error.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 15:47:12 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 15:49:21 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Token;
use std::any::Any;
use std::rc::Rc;
use std::{error::Error, fmt};

pub enum LexerError {
    InvalidOp(char),
    InvalidVal(String),
    InvalidVar(String),
    InvalidFun(String, Vec<Rc<Token>>),
}

impl Error for LexerError {}

impl fmt::Debug for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::InvalidOp(ch) => write!(f, "!{{{}}}", ch),
            LexerError::InvalidVal(word) => write!(f, "!{{{}}}", word),
            LexerError::InvalidVar(word) => write!(f, "!{{{}}}", word),
            LexerError::InvalidFun(fun, v) => {
                write!(f, "!{{{}{}}}", fun, super::debug_token(v, ""))
            }
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
            LexerError::InvalidFun(word, _) => (word.clone(), "function name"),
        };
        write!(f, "'{}' isn't a valid {}.", info.0, info.1)
    }
}

impl Token for LexerError {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
