/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/08 18:14:20 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/28 17:29:18 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Extension, Memory, Value};

use crate::computor::{ComputorError, TreeResult};
use crate::parser::TokenTree;

use std::{fmt, vec::Vec};

pub struct Function {
    name: String,
    var: Vec<String>,
    expr: Option<Box<dyn TokenTree>>,
}

impl Function {
    pub fn new(name: String) -> Self {
        Function {
            name: name,
            var: Vec::new(),
            expr: None,
        }
    }

    pub fn set(&mut self, mut vars: Vec<String>, expr: Box<dyn TokenTree>) {
        vars.reverse();
        self.var = Vec::new();
        loop {
            match vars.pop() {
                Some(var) => self.var.push(var),
                None => break,
            };
        }
        self.expr = Some(expr);
    }

    pub fn compute(&self, mem: &Memory, arg: Vec<Value>) -> TreeResult {
        if arg.len() != self.var.len() {
            return Err(ComputorError::fun_arg_inv(&self.name));
        }
        let mut extended = Extension::new();
        for i in 0..arg.len() {
            extended.add(&self.var[i], arg[i].clone());
        }
        let res = match &self.expr {
            Some(tree) => tree.compute(mem, Some(&mut extended)),
            None => Err(ComputorError::fun_undef(&self.name)),
        };
        return res;
    }

    fn var_to_string(&self) -> String {
        let mut var_str = String::new();
        let mut var_iter = self.var.iter();

        loop {
            match var_iter.next() {
                Some(var) => var_str = format!("{}, {}", var_str, var),
                None => break,
            }
        }
        return String::from(var_str.trim_start_matches(", "));
    }

    pub fn print(&self, alias: Vec<String>) {
        let var = &self.var;
        let len = alias.len();
        if len != var.len() {
            return;
        }
        println!("{}", self);
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.expr {
            Some(expr) => {
                write!(f, "{}({}): {}", self.name, self.var_to_string(), expr)
            }
            None => write!(f, "{}(): not defined.", self.name),
        }
    }
}
