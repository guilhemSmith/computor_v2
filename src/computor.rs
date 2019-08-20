/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 11:31:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/20 13:40:59 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod result;
pub use result::ComputorResult;

use crate::arg_parse::Param;
use crate::computor_error::ComputorError;
use crate::lexer::token;
use crate::parser::TokenTree;
use crate::timer::Timer;
use crate::types::Imaginary;
use crate::{memory::Variable, Memory};

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
            println!("[v:Computor] - tree received: {:?}", tree)
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
            0 => match tree.compute(&self.memory) {
                ComputorResult::Val(val) => println!("{}", val),
                ComputorResult::Var(var, _, _) => self.catch_var(var),
                ComputorResult::Equ(id, coefs) => self.solve(id, coefs),
                ComputorResult::Fun(id, param, exp) => {
                    self.set_fun(id, param, exp)
                }
                ComputorResult::Err(error) => self.print_err(error),
                _ => {}
            },
            n => eprintln!("[err:Computor] - {} invalid tokens. Abort.", n),
        }
    }

    fn catch_var(&self, id: String) {
        eprintln!("Unknown variable: '{}'.", id);
    }

    fn solve(&mut self, _id: String, _coefs: Vec<Imaginary>) {}

    fn set_fun(&mut self, id: String, param: Vec<String>, exp: Box<TokenTree>) {
        self.memory.set_fun(id, param, exp);
    }

    fn print_err(&self, err: ComputorError) {
        eprintln!("[err:Computor] - {}", err);
    }
}
