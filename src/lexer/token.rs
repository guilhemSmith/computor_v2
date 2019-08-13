/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   token.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/30 14:43:15 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/13 13:43:00 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Expression, Operand, Operator};
use crate::error::ComputorError;
use std::{fmt, collections::LinkedList};

#[derive(Clone)]
pub enum Token {
    Equal,
    Expr(Expression),
    Orand(Operand),
    Orator(Operator),
    Invalid(ComputorError),
    Resolve
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Expr(exp) => write!(f, "({})", exp),
            Token::Orand(orand) => write!(f, "{}", orand),
            Token::Orator(orator) => write!(f, "{}", orator),
            Token::Invalid(err) => write!(f, "{{{}}}", err.kind()),
            Token::Equal => write!(f, "="),
            Token::Resolve => write!(f, "?"),
        }
    }
}

pub fn tokens_to_string(lst: &LinkedList<Token>) -> String {
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