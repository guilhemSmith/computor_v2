/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   token.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/30 14:43:15 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/15 12:55:55 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod expression;
mod function;
mod operator;
mod value;
mod variable;

pub use expression::Expression;
pub use function::Function;
pub use operator::Operator;
pub use value::Value;
pub use variable::Variable;

use crate::error::ComputorError;
use std::collections::LinkedList;
use std::fmt::{self, Debug, Display};

#[derive(Clone)]
pub enum OldToken {
    Equal,
    Expr(Expression),
    Fun(Function),
    Val(Value),
    Orator(Operator),
    Invalid(ComputorError),
    Resolve,
    Var(Variable),
}

impl fmt::Display for OldToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OldToken::Expr(exp) => write!(f, "({})", exp),
            OldToken::Val(val) => write!(f, "{}", val),
            OldToken::Orator(orator) => write!(f, "{}", orator),
            OldToken::Invalid(err) => write!(f, "{{{}}}", err.kind()),
            OldToken::Equal => write!(f, "="),
            OldToken::Resolve => write!(f, "?"),
            OldToken::Var(var) => write!(f, "{}", var),
            OldToken::Fun(fun) => write!(f, "{}", fun),
        }
    }
}

impl fmt::Debug for OldToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OldToken::Expr(exp) => write!(f, "[exp:({:?})]", exp),
            OldToken::Val(val) => write!(f, "[val:{}]", val),
            OldToken::Orator(orator) => write!(f, "[op:{}]", orator),
            OldToken::Invalid(err) => write!(f, "[err:{{{}}}]", err.kind()),
            OldToken::Equal => write!(f, "[=]"),
            OldToken::Resolve => write!(f, "[?]"),
            OldToken::Var(var) => write!(f, "[var:{}]", var),
            OldToken::Fun(fun) => write!(f, "[fun:{:?}]", fun),
        }
    }
}

pub fn tokens_to_string(lst: &LinkedList<OldToken>) -> String {
    let mut tokens_str = String::new();
    let mut iter_token = lst.iter();

    loop {
        match iter_token.next() {
            Some(tok) => tokens_str = format!("{} {}", tokens_str, tok),
            None => break,
        };
    }
    return tokens_str.trim_start().replace("- -", "+");
}

pub fn tokens_to_debug(lst: &LinkedList<OldToken>) -> String {
    let mut tokens_str = String::new();
    let mut iter_token = lst.iter();

    loop {
        match iter_token.next() {
            Some(tok) => tokens_str = format!("{} {:?}", tokens_str, tok),
            None => break,
        };
    }
    return String::from(tokens_str.trim_start());
}

pub trait Token {
    fn tokens_to_string<T: Token + Display>(lst: &LinkedList<T>) -> String {
        let mut tokens_str = String::new();
        let mut iter_token = lst.iter();

        loop {
            match iter_token.next() {
                Some(tok) => tokens_str = format!("{} {}", tokens_str, tok),
                None => break,
            };
        }
        return tokens_str.trim_start().replace("- -", "+");
    }

    fn tokens_to_debug<T: Token + Debug>(lst: &LinkedList<T>) -> String {
        let mut tokens_str = String::new();
        let mut iter_token = lst.iter();

        loop {
            match iter_token.next() {
                Some(tok) => tokens_str = format!("{} {:?}", tokens_str, tok),
                None => break,
            };
        }
        return String::from(tokens_str.trim_start());
    }
}
