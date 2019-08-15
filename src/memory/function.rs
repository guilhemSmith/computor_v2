/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/08 18:14:20 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/15 12:57:28 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Variable;
use crate::lexer::token::Expression;
use std::{fmt, vec::Vec};

pub struct Function {
    name: String,
    var: Vec<Variable>,
    expr: Option<Expression>,
}

impl Function {
    pub fn new(name: String) -> Self {
        Function {
            name: name,
            var: Vec::new(),
            expr: None,
        }
    }

    pub fn set(&mut self, mut vars: Vec<Variable>, expr: Expression) {
        vars.reverse();
        loop {
            match vars.pop() {
                Some(var) => self.var.push(var),
                None => break,
            };
        }
        self.expr = Some(expr);
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    fn reset_var_val(&mut self) {
        let mut var_iter = self.var.iter_mut();

        loop {
            match var_iter.next() {
                Some(var) => var.set(None),
                None => break,
            };
        }
    }

    fn var_to_string(&self) -> String {
        let mut var_str = String::new();
        let mut var_iter = self.var.iter();

        loop {
            match var_iter.next() {
                Some(var) => var_str = format!("{}, {}", var_str, var.name()),
                None => break,
            }
        }
        return String::from(var_str.trim_start_matches(", "));
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.expr.clone() {
            Some(expr) => {
                write!(f, "{}({}): {}", self.name, self.var_to_string(), expr)
            }
            None => write!(f, "{}(): not defined.", self.name),
        }
    }
}
