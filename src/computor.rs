/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 11:31:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/12 17:06:24 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod error;
mod result;

pub use error::{ComputorError, ErrorKind};
pub use result::ComputorResult;

use crate::arg_parse::Param;
use crate::lexer::token;
use crate::memory::Memory;
use crate::parser::{TokenTree, TreeBranch};
use crate::timer::Timer;
use crate::types::Imaginary;
use ComputorResult as CRes;

use std::collections::HashMap;

const LOG: &str = "[err:Computor] -> ";

type TTree = Box<dyn TokenTree>;
type Equ = HashMap<i32, Imaginary>;

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

    pub fn read_tokens(&mut self, tree: TTree) {
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

    fn compute(&mut self, mut tree: TTree) {
        let n = tree.count(token::count_error);
        if n > 0 {
            eprintln!("{}{} invalid tokens. Abort.", LOG, n);
            return;
        }

        let left: Option<TTree>;
        let right: Option<TTree>;
        match tree.as_any().downcast_mut::<TreeBranch>() {
            None => return self.single_part(tree),
            Some(branch) => {
                if branch.op_ref().symbol() != '=' {
                    return self.single_part(tree);
                } else {
                    left = branch.extract(true);
                    right = branch.extract(false);
                }
            }
        };
        if let (Some(br_left), Some(br_right)) = (left, right) {
            return self.dual_part(br_left, br_right);
        };
        eprintln!("{}", ComputorError::bad_use_op('='));
    }

    fn single_part(&mut self, tree: TTree) {
        match tree.compute(&mut self.memory, None) {
            CRes::None => log_err("Empty instruction given"),
            CRes::Res => self.mem_dump(),
            CRes::Err(err) => self.print_err(err),
            CRes::Val(val) => println!("{}", val),
            CRes::VarCall(_, val) => println!("{}", val),
            CRes::VarSet(v) => log_err(&format!("Unknown variable '{}'", v)),
            CRes::FunSet(f, _) => log_err(&format!("Unknown function '{}'", f)),
            CRes::Equ(_, eq) => self.eq_one_sided(eq),
        };
    }

    fn dual_part(&mut self, left: TTree, right: TTree) {
        match left.compute(&mut self.memory, None) {
            CRes::None => eprintln!("{}", ComputorError::bad_use_op('=')),
            CRes::Res => eprintln!("{}", ComputorError::bad_resolve()),
            CRes::Err(err) => self.print_err(err),
            CRes::Val(val) => self.left_val(val, right),
            CRes::VarCall(id, val) => self.call_var(id, val, right),
            CRes::VarSet(id) => self.set_var(id, right),
            CRes::FunSet(id, param) => self.set_fun(id, param, right),
            CRes::Equ(_, _) => log_err("Can't compute equation for now"),
        }
    }

    fn eq_one_sided(&mut self, eq: Equ) {
        let zero: i32 = 0;
        for (pow, coef) in eq.iter() {
            if *pow > 0 && *coef != Imaginary::new(0.0, 0.0) {
                return log_err("Equation not complete");
            }
        }
        if let Some(coef) = eq.get(&zero) {
            println!("{}", *coef);
        } else {
            log_err("Equation not complete");
        }
    }

    fn left_val(&mut self, val: Imaginary, right: TTree) {
        match right.compute(&mut self.memory, None) {
            CRes::None => eprintln!("{}", ComputorError::bad_use_op('=')),
            CRes::Res => println!("{}", val),
            CRes::Err(err) => self.print_err(err),
            CRes::Val(_) => log_err("This is an equation"),
            CRes::VarCall(_, _) => log_err("This is an equation"),
            CRes::VarSet(v) => log_err(&format!("Unknown variable '{}'", v)),
            CRes::FunSet(f, _) => log_err(&format!("Unknown function '{}'", f)),
            CRes::Equ(_, _) => log_err("Can't compute equation for now"),
        };
    }

    fn call_var(&mut self, var: String, val: Imaginary, right: TTree) {
        match right.compute(&mut self.memory, None) {
            CRes::None => eprintln!("{}", ComputorError::bad_use_op('=')),
            CRes::Res => println!("{}", val),
            CRes::Err(err) => self.print_err(err),
            CRes::Val(nval) => {
                self.memory.set_var(var, Some(nval));
                println!("{}", nval);
            }
            CRes::VarCall(_, nval) => {
                self.memory.set_var(var, Some(nval));
                println!("{}", nval);
            }
            CRes::VarSet(v) => log_err(&format!("Unknown variable '{}'", v)),
            CRes::FunSet(f, _) => log_err(&format!("Unknown function '{}'", f)),
            CRes::Equ(_, _) => log_err("Can't compute equation for now"),
        }
    }

    fn set_var(&mut self, var: String, right: TTree) {
        match right.compute(&mut self.memory, None) {
            CRes::None => eprintln!("{}", ComputorError::bad_use_op('=')),
            CRes::Res => log_err(&format!("Unknown variable '{}'", var)),
            CRes::Err(err) => self.print_err(err),
            CRes::Val(val) => {
                self.memory.set_var(var, Some(val));
                println!("{}", val);
            }
            CRes::VarCall(_, val) => {
                self.memory.set_var(var, Some(val));
                println!("{}", val);
            }
            CRes::VarSet(id) => {
                if id != var {
                    log_err(&format!("Unknown variable '{}'", id))
                } else {
                    log_err("This is an equation")
                }
            }
            CRes::FunSet(f, _) => log_err(&format!("Unknown function '{}'", f)),
            CRes::Equ(_, _) => log_err("Can't compute equation for now"),
        };
    }

    fn set_fun(&mut self, id: String, param: Vec<String>, exp: TTree) {
        self.memory.set_fun(id, param, exp);
    }

    fn print_err(&self, err: ComputorError) {
        eprintln!("{}", err);
    }

    fn mem_dump(&self) {
        println!("{}", self.memory);
    }
}

fn log_err(msg: &str) {
    eprintln!("{}{}.", LOG, msg);
}
