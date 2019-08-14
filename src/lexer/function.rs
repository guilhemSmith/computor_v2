/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:22:09 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/14 16:55:58 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Token, Variable, Operand};
use crate::error::ComputorError;
use std::fmt;

#[derive(Clone)]
pub struct Function {
    id: String,
    param: Vec<Token>,
}

impl Function {
    pub fn new(id: String, vars: String) -> Result<Self, ComputorError> {
        let mut new = Function {
            id: id,
            param: Vec::new(),
        };
        let mut var_name = vars.split_terminator(",");

        loop {
            match var_name.next() {
                Some(name) => {
                    new.param.push(Token::Var(Variable::new(String::from(name))?))
                }
                None => break,
            };
        }
        return Ok(new);
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut param = String::new();
        let mut iter = self.param.iter();

        loop {
            match iter.next() {
                Some(var) => param = format!("{}, {}", param, var),
                None => break,
            }
        }
        write!(f, "{}({})", self.id, param.trim_start_matches(", "))
    }
}
