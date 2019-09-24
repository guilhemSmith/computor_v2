/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   token.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/30 14:43:15 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/24 11:15:24 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod expression;
mod function;
mod lexer_error;
mod matrix;
mod operator;
mod resolve;
mod value;
mod variable;

pub use expression::Expression;
pub use function::FunctionToken;
pub use function::FunctionTree;
pub use lexer_error::LexerError;
pub use matrix::MatrixUnparsed;
pub use operator::new_operator;
pub use operator::Operator;
pub use resolve::Resolve;
pub use value::Value;
pub use variable::Variable;

use crate::computor::TreeResult;
use crate::memory::{Extension, Memory};

use std::any::Any;
use std::fmt;

extern crate colored;
use colored::Colorize;

pub trait Token: fmt::Display + fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_op_ref(&self) -> Option<&dyn Operator> {
        None
    }
    fn as_op_mut(&mut self) -> Option<&mut dyn Operator> {
        None
    }
    fn get_result(
        &self,
        mem: &Memory,
        ext: Option<&mut Extension>,
    ) -> TreeResult;
}

pub fn count_error(token: &Box<dyn Token>) -> i32 {
    match token.as_any().downcast_ref::<LexerError>() {
        None => match token.as_any().downcast_ref::<FunctionTree>() {
            None => 0,
            Some(fun) => {
                let mut sub_count = 0;
                for param in fun.param() {
                    sub_count += param.count(count_error);
                }
                sub_count
            }
        },
        Some(err) => {
            eprintln!("{} - {}", String::from("[err:token]").red(), err);
            1
        }
    }
}

pub fn display_token(tokens: &Vec<Box<dyn Token>>) -> String {
    let mut vec = String::new();
    for tok in tokens {
        vec = format!("{}{}", vec, tok);
    }
    vec
}

pub fn debug_token(tokens: &Vec<Box<dyn Token>>) -> String {
    let mut vec = String::new();
    for tok in tokens {
        vec = format!("{}{:?}", vec, tok);
    }
    vec
}
