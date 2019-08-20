/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   memory.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/08 18:14:00 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/20 13:51:08 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod function;
mod variable;

pub use function::Function;
pub use variable::Variable;

use crate::parser::TokenTree;
use crate::types::Imaginary;
use std::{collections::HashMap, fmt};

pub struct Memory {
    var: HashMap<String, Variable>,
    fun: HashMap<String, Function>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            var: HashMap::new(),
            fun: HashMap::new(),
        }
    }

    pub fn set_var(&mut self, name: String, val: Option<Imaginary>) {
        match self.var.get_mut(&name) {
            Some(var) => var.set(val),
            None => {
                let mut var = Variable::new(name.to_lowercase());
                var.set(val);
                self.var.insert(name, var);
            }
        };
    }

    pub fn get_var(&self, name: &String) -> Option<&Variable> {
        self.var.get(name)
    }

    pub fn set_fun(
        &mut self,
        name: String,
        var: Vec<String>,
        exp: Box<TokenTree>,
    ) {
        let mut var_vec: Vec<Variable> = Vec::new();
        let mut name_iter = var.iter();

        loop {
            match name_iter.next() {
                Some(var_name) => var_vec.push(Variable::new(var_name.clone())),
                None => break,
            }
        }
        match self.fun.get_mut(&name) {
            Some(fun) => fun.set(var_vec, exp),
            None => {
                let mut fun = Function::new(name.clone());
                fun.set(var_vec, exp);
                self.fun.insert(name, fun);
            }
        };
    }

    pub fn get_fun(&self, name: String) -> Option<&Function> {
        self.fun.get(&name)
    }

    fn var_to_string(&self) -> String {
        let mut var_str = String::from("Variables:");
        let mut var_iter = self.var.iter();

        loop {
            match var_iter.next() {
                Some(var) => var_str = format!("{}\n{}", var_str, var.1),
                None => break,
            };
        }
        return var_str;
    }

    fn fun_to_string(&self) -> String {
        let mut fun_str = String::from("Functions:");
        let mut fun_iter = self.fun.iter();

        loop {
            match fun_iter.next() {
                Some(fun) => fun_str = format!("{}\n{}", fun_str, fun.1),
                None => break,
            };
        }
        return fun_str;
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n ---\n{}\n ---",
            self.var_to_string(),
            self.fun_to_string()
        )
    }
}
