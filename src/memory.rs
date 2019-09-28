/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   memory.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/08 18:14:00 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/28 17:53:24 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod extension;
mod function;
mod variable;

pub use extension::Extension;
pub use function::Function;
pub use variable::Value;
pub use variable::Variable;

use crate::computor::{Computed, ComputorError, ComputorResult, TreeResult};
use crate::parser::TokenTree;
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

    pub fn set_var(&mut self, name: String, val: Value) {
        println!("{}", val);
        self.var.insert(name.clone(), Variable::new(name, val));
    }

    pub fn get_var<'ext, 'mem: 'ext>(
        &'mem self,
        name: &String,
    ) -> Option<&Variable> {
        self.var.get(name)
    }

    pub fn set_fun(
        &mut self,
        name: String,
        args: Vec<Computed>,
        mut exp: Box<dyn TokenTree>,
    ) -> ComputorResult {
        let mut vars: Vec<String> = Vec::new();
        let mut iter = args.into_iter();

        loop {
            match iter.next() {
                Some(comp) => match comp {
                    Computed::VarSet(id) => {
                        if vars.contains(&id) {
                            return Err(ComputorError::fun_arg_inv(&name));
                        }
                        vars.push(id);
                    }
                    Computed::VarCall(id, _) => {
                        if vars.contains(&id) {
                            return Err(ComputorError::fun_arg_inv(&name));
                        }
                        vars.push(id);
                    }
                    _ => return Err(ComputorError::fun_arg_inv(&name)),
                },
                None => break,
            }
        }
        exp.fix_exp(self, &vars)?;
        println!("{}", exp);
        match self.fun.get_mut(&name) {
            Some(fun) => fun.set(vars, exp),
            None => {
                let mut fun = Function::new(name.clone());
                fun.set(vars, exp);
                self.fun.insert(name, fun);
            }
        };
        Ok(())
    }

    pub fn get_fun(&self, name: &String) -> Option<&Function> {
        self.fun.get(name)
    }

    fn solve_arg(
        &self,
        id: String,
        arg: Computed,
    ) -> Result<Value, ComputorError> {
        match arg {
            Computed::ValIm(val) => Ok(Value::Im(val)),
            Computed::VarCall(_, val) => Ok(val),
            Computed::ValMat(val) => Ok(Value::Mat(val)),
            Computed::FunId(id, sub_args) => {
                self.solve_arg(id.clone(), self.solve_fun(id, sub_args)?)
            }
            _ => Err(ComputorError::fun_arg_inv(&id)),
        }
    }

    pub fn solve_fun(&self, name: String, args: Vec<Computed>) -> TreeResult {
        let mut lst: Vec<Value> = Vec::new();
        let mut iter = args.into_iter();

        loop {
            match iter.next() {
                Some(comp) => lst.push(self.solve_arg(name.clone(), comp)?),
                None => break,
            }
        }
        let fun_mem = self.get_fun(&name);
        match fun_mem {
            Some(fun) => fun.compute(self, lst),
            None => Err(ComputorError::fun_undef(&name)),
        }
    }

    pub fn valid_args(&self, args: &Vec<Computed>) -> bool {
        for arg in args.iter() {
            match arg {
                Computed::ValMat(_) => continue,
                Computed::ValIm(_) => continue,
                Computed::VarCall(_, _) => continue,
                Computed::FunId(_, sub_args) => {
                    if !self.valid_args(sub_args) {
                        return false;
                    }
                }
                _ => return false,
            };
        }
        return true;
    }

    pub fn param_to_string(
        &self,
        args: Vec<Computed>,
    ) -> Result<Vec<String>, ComputorError> {
        let mut param: Vec<String> = Vec::new();
        for arg in args.iter() {
            param.push(arg.to_string());
        }
        return Ok(param);
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
