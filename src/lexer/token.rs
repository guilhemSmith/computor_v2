/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   token.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/30 14:43:15 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/19 15:52:34 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod expression;
mod function;
mod lexer_error;
mod operator;
mod resolve;
mod value;
mod variable;

pub use expression::Expression;
pub use function::FunctionToken;
pub use function::FunctionTree;
pub use lexer_error::LexerError;
pub use operator::Operator;
pub use resolve::Resolve;
pub use value::Value;
pub use variable::Variable;

use crate::computor::ComputorResult;
use crate::memory::Memory;

use std::any::Any;
use std::fmt;
use std::rc::Rc;

pub trait Token: fmt::Display + fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_result(&self, mem: &Memory) -> ComputorResult;
}

pub fn count_error(token: &Rc<Token>) -> i32 {
    match token.as_any().downcast_ref::<LexerError>() {
        None => match token.as_any().downcast_ref::<FunctionTree>() {
            None => 0,
            Some(fun) => {
                let mut sub_count = 1;
                for param in fun.param() {
                    sub_count += param.count(count_error);
                }
                sub_count
            }
        },
        Some(err) => {
            eprintln!("[err:Token] -> {}", err);
            1
        }
    }
}

pub fn display_token(tokens: &Vec<Rc<Token>>) -> String {
    let mut vec = String::new();
    for tok in tokens {
        vec = format!("{}{}", vec, tok);
    }
    vec
}

pub fn debug_token(tokens: &Vec<Rc<Token>>) -> String {
    let mut vec = String::new();
    for tok in tokens {
        vec = format!("{}{:?}", vec, tok);
    }
    vec
}

// pub fn debug_token(tokens: &Vec<Rc<Token>>) -> String {
//     let mut debug = String::new();

//     for token in tokens {
//         debug.push_str(&format!("{}{:?}", sep, token)[..]);
//     }
//     return String::from(debug.trim_start_matches(sep));
// }

// pub fn display_token(tokens: &Vec<Rc<Token>>, sep: &str) -> String {
//     let mut display = String::new();

//     for token in tokens {
//         display.push_str(&format!("{}{}", sep, token)[..]);
//     }
//     return String::from(display.trim_start_matches(sep));
// }
