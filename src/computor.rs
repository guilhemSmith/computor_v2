/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 11:31:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/09 12:25:32 by gsmith           ###   ########.fr       */
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
use ComputorResult as CRes;

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

    pub fn read_tokens(&mut self, tree: Box<dyn TokenTree>) {
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

    fn compute(&mut self, mut tree: Box<dyn TokenTree>) {
        let n = tree.count(token::count_error);
        if n > 0 {
            eprintln!("{}{} invalid tokens. Abort.", LOG, n);
            return;
        }

        let left: Option<Box<dyn TokenTree>>;
        let right: Option<Box<dyn TokenTree>>;
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

    fn single_part(&mut self, tree: Box<dyn TokenTree>) {
        match tree.compute(&mut self.memory, None) {
            CRes::None => self.log_err("Empty instruction given"),
            CRes::Res => self.mem_dump(),
            CRes::Err(err) => self.print_err(err),
            CRes::Val(val) => println!("{}", val),
            CRes::VarCall(_, val) => println!("{}", val),
            CRes::VarSet(_) => self.log_err("Unknown variable"),
            CRes::FunSet(_, _) => self.log_err("Unknown function"),
            CRes::Equ(_, _) => self.log_err("One sided equation"),
        };
    }

    fn dual_part(&mut self, left: Box<dyn TokenTree>, right: Box<dyn TokenTree>) {
        match left.compute(&mut self.memory, None) {
            CRes::None => eprintln!("{}", ComputorError::bad_use_op('=')),
            CRes::Res => eprintln!("{}", ComputorError::bad_resolve()),
            CRes::Err(err) => self.print_err(err),
            CRes::Val(val) => println!("{}", val),
            CRes::VarCall(_, val) => println!("{}", val),
            CRes::VarSet(_) => self.log_err("Unknown variable"),
            CRes::FunSet(_, _) => self.log_err("Unknown function"),
            CRes::Equ(_, _) => self.log_err("One sided equation"),
        }
    }

    // fn var_pr(&self, id: String, coef: Imaginary, pow: Imaginary) {
    //     match self.memory.get_var_val(&id) {
    //         Some(val) => println!("{}", coef * val),
    //         None => eprintln!("{}Unknown variable: '{}'.", LOG, id),
    //     };
    // }

    // fn set_var(&mut self, var: String, val: Imaginary) {
    //     self.memory.set_var(var, Some(val));
    //     println!("{}", val);
    // }

    // fn solve(&self, _id: String, _coefs: Vec<Imaginary>) {}

    // fn set_fun(
    //     &mut self,
    //     id: String,
    //     param: Vec<String>,
    //     exp: Option<Box<dyn TokenTree>>,
    // ) {
    //     match exp {
    //         Some(fun) => self.memory.set_fun(id, param, fun),
    //         None => eprintln!("{}'{}' need an expression to be set.", LOG, id),
    //     }
    // }

    fn print_err(&self, err: ComputorError) {
        eprintln!("{}", err);
    }

    fn log_err(&self, msg: &str) {
        eprintln!("{}{}.", LOG, msg);
    }

    fn mem_dump(&self) {
        println!("{}", self.memory);
    }
}
