/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 11:31:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/30 11:55:15 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod error;
mod result;

pub use error::{ComputorError, ErrorKind};
pub use result::ComputorResult;

use crate::arg_parse::Param;
use crate::lexer::token;
use crate::memory::Memory;
use crate::parser::TokenTree;
use crate::timer::Timer;
use crate::types::Imaginary;

const LOG: &str = "[err:Computor] -> ";

pub struct Computor {
    verbose: bool,
    bench: bool,
    memory: Memory,
}

impl Computor {
    pub fn new(param: &Param) -> Self {
        Computor {
            verbose: param.verbose(),
            bench: param.bench(),
            memory: Memory::new(),
        }
    }

    pub fn read_tokens(&mut self, tree: Box<TokenTree>) {
        if self.verbose {
            println!("[v:Computor] -> tree received: {:?}", tree)
        }
        if !self.bench {
            self.compute(tree);
        } else {
            let display = format!("Computor({})", tree);
            let _timer = Timer::new(&display[..]);
            self.compute(tree);
        }
    }

    fn compute(&mut self, tree: Box<TokenTree>) {
        match &tree.count(token::count_error) {
            0 => match tree.compute(&mut self.memory, None) {
                ComputorResult::Val(val) => println!("{}", val),
                ComputorResult::Var(var, coef, pow) => {
                    self.catch_var(var, coef, pow)
                }
                ComputorResult::Set(var, val) => self.set_var(var, val),
                ComputorResult::Equ(id, coefs, true) => self.solve(id, coefs),
                ComputorResult::Equ(_, _, false) => self.uncomplete(),
                ComputorResult::Fun(id, param, exp) => {
                    self.set_fun(id, param, exp)
                }
                ComputorResult::Err(error) => self.print_err(error),
                _ => {}
            },
            n => eprintln!("{}{} invalid tokens. Abort.", LOG, n),
        }
    }

    fn catch_var(&self, id: String, coef: Imaginary, pow: Imaginary) {
        match self.memory.get_var(&id, None) {
            Some(var) => match var.get() {
                Some(val) => println!("{}", coef * val),
                None => eprintln!("{}Unknown variable: '{}'.", LOG, id),
            },
            None => eprintln!("{}Unknown variable: '{}'.", LOG, id),
        };
    }

    fn set_var(&mut self, var: String, val: Imaginary) {
        self.memory.set_var(var, Some(val));
        println!("{}", val);
    }

    fn solve(&self, _id: String, _coefs: Vec<Imaginary>) {}

    fn uncomplete(&self) {
        eprintln!("{}Can't compute uncompleted expression.", LOG);
    }

    fn set_fun(
        &mut self,
        id: String,
        param: Vec<String>,
        exp: Option<Box<TokenTree>>,
    ) {
        match exp {
            Some(fun) => self.memory.set_fun(id, param, fun),
            None => eprintln!("{}'{}' need an expression to be set.", LOG, id),
        }
    }

    fn print_err(&self, err: ComputorError) {
        eprintln!("{}", err);
    }
}
