/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 11:31:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/13 14:04:53 by gsmith           ###   ########.fr       */
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
use crate::types::{Imaginary, Rational};
use ComputorResult as CRes;

use std::collections::HashMap;
use std::i32::{MAX as I32_MAX, MIN as I32_MIN};

const LOG: &str = "[err:Computor] -> ";

type TTree = Box<dyn TokenTree>;
type Equ = HashMap<i32, Im>;
type Im = Imaginary;

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
            CRes::Equ(id, eq) => self.eq_two_sided(id, eq, right),
        }
    }

    fn eq_one_sided(&self, eq: Equ) {
        let zero: i32 = 0;
        for (pow, coef) in eq.iter() {
            if *pow > 0 && *coef != Im::new(0.0, 0.0) {
                return log_err("Equation not complete");
            }
        }
        if let Some(coef) = eq.get(&zero) {
            println!("{}", *coef);
        } else {
            log_err("Equation not complete");
        }
    }

    fn eq_two_sided(&self, id: String, mut left: Equ, right: TTree) {
        match right.compute(&self.memory, None) {
            CRes::None => eprintln!("{}", ComputorError::bad_use_op('=')),
            CRes::Res => log_err("Equation not complete"),
            CRes::Err(err) => self.print_err(err),
            CRes::Val(val) => {
                val_into_eq(&mut left, val);
                self.solve_eq(left, id);
            }
            CRes::VarCall(id_v, val) => {
                var_into_eq(&mut left, &id, id_v, val);
                self.solve_eq(left, id);
            }
            CRes::VarSet(v) => match unknow_into_eq(&mut left, &id, v) {
                Err(err) => self.print_err(err),
                Ok(()) => self.solve_eq(left, id),
            },
            CRes::FunSet(f, _) => log_err(&format!("Unknown function '{}'", f)),
            CRes::Equ(id_r, eq) => match fuse_eq(&mut left, &id, eq, id_r) {
                Ok(()) => self.solve_eq(left, id),
                Err(err) => self.print_err(err),
            },
        }
    }

    fn left_val(&self, val: Im, right: TTree) {
        match right.compute(&self.memory, None) {
            CRes::None => eprintln!("{}", ComputorError::bad_use_op('=')),
            CRes::Res => println!("{}", val),
            CRes::Err(err) => self.print_err(err),
            CRes::Val(r_val) => solve_two_val(val, r_val),
            CRes::VarCall(_, r_val) => solve_two_val(val, r_val),
            CRes::VarSet(v) => println!("{} = {} is a solution.", v, val),
            CRes::FunSet(f, _) => log_err(&format!("Unknown function '{}'", f)),
            CRes::Equ(id, eq) => {
                let mut eq = eq;
                val_into_eq(&mut eq, val);
                self.solve_eq(eq, id);
            }
        };
    }

    fn call_var(&mut self, var: String, val: Im, right: TTree) {
        match right.compute(&self.memory, None) {
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
            CRes::VarSet(v) => println!("{} = {} is a solution.", v, val),
            CRes::FunSet(f, _) => log_err(&format!("Unknown function '{}'", f)),
            CRes::Equ(id, eq) => {
                let mut eq = eq;
                var_into_eq(&mut eq, &id, var, val);
                self.solve_eq(eq, id);
            }
        }
    }

    fn set_var(&mut self, var: String, right: TTree) {
        match right.compute(&self.memory, None) {
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
                    println!("Any value for {} is a solution.", id);
                }
            }
            CRes::FunSet(f, _) => log_err(&format!("Unknown function '{}'", f)),
            CRes::Equ(id, eq) => {
                let mut eq = eq;
                match unknow_into_eq(&mut eq, &id, var) {
                    Err(err) => self.print_err(err),
                    Ok(()) => self.solve_eq(eq, id),
                }
            }
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

    fn solve_eq(&self, mut eq: Equ, id: String) {
        filter_eq(&mut eq);
        if !valid_eq(&eq) {
            return;
        }
        match eq.keys().max() {
            None => println!("Any value for {} is a solution.", id),
            Some(max) => match *max {
                0 => println!("False."),
                1 => eq_degree_one(eq, id),
                2 => eq_degree_two(eq, id),
                2..=I32_MAX => {
                    println!("Can't solve equation with degree above 2.")
                }
                I32_MIN..=0 => {
                    println!("Can't solve equation with degree below 0.")
                }
            },
        };
    }
}

fn val_into_eq(eq: &mut Equ, val: Im) {
    let zero: i32 = 0;
    match eq.get_mut(&zero) {
        None => {
            eq.insert(zero, Im::new(0.0, 0.0) - val);
        }
        Some(coef) => *coef = *coef - val,
    };
}

fn var_into_eq(eq: &mut Equ, id: &String, id_v: String, val: Im) {
    let pow: i32;
    let n_coef: Im;
    if *id == id_v {
        pow = 1;
        n_coef = Im::new(1.0, 0.0);
    } else {
        pow = 0;
        n_coef = val;
    };
    match eq.get_mut(&pow) {
        None => {
            eq.insert(pow, Im::new(0.0, 0.0) - n_coef);
        }
        Some(coef) => *coef = *coef - n_coef,
    };
}

fn unknow_into_eq(
    eq: &mut Equ,
    id: &String,
    unk: String,
) -> Result<(), ComputorError> {
    if *id == unk {
        let one: i32 = 1;
        match eq.get_mut(&one) {
            None => {
                eq.insert(one, Im::new(-1.0, 0.0));
            }
            Some(coef) => *coef = *coef - Im::new(1.0, 0.0),
        }
        Ok(())
    } else {
        Err(ComputorError::too_many_unknown())
    }
}

fn fuse_eq(
    eq_l: &mut Equ,
    id_l: &String,
    eq_r: Equ,
    id_r: String,
) -> Result<(), ComputorError> {
    if *id_l == id_r {
        for (pow, val) in eq_r {
            match eq_l.get_mut(&pow) {
                None => {
                    eq_l.insert(pow, Im::new(0.0, 0.0) - val);
                }
                Some(coef) => *coef = *coef - val,
            };
        }
        Ok(())
    } else {
        Err(ComputorError::too_many_unknown())
    }
}

fn solve_two_val(val_l: Im, val_r: Im) {
    if val_l == val_r {
        println!("True");
    } else {
        println!("False");
    }
}

fn eq_degree_one(eq: Equ, id: String) {
    let mut index: i32 = 0;
    let zero = match eq.get(&index) {
        None => Im::new(0.0, 0.0),
        Some(val) => *val,
    };
    index += 1;
    let one = *eq.get(&index).unwrap();
    let sign = if zero.get_real() < Rational::zero() {
        " "
    } else {
        " + "
    };
    println!(
        "Equation of degree 1:\n{} * {}{}{} = 0",
        one, id, sign, zero
    );
    println!("Solution: {} = {}", id, (Im::new(0.0, 0.0) - zero) / one);
}

fn eq_degree_two(eq: Equ, id: String) {}

fn log_err(msg: &str) {
    eprintln!("{}{}.", LOG, msg);
}

pub fn filter_eq(eq: &mut Equ) {
    let zero = Im::new(0.0, 0.0);
    let mut dead_key: Vec<i32> = Vec::new();
    for (pow, coef) in eq.iter() {
        if *coef == zero {
            dead_key.push(*pow);
        }
    }
    for key in dead_key.iter() {
        eq.remove(key);
    }
}

fn valid_eq(eq: &Equ) -> bool {
    for (pow, coef) in eq.iter() {
        if *pow < 0 {
            println!("Can't compute equation with negative pow.");
            return false;
        }
        if !coef.is_real() {
            println!("Can't compute equation with complex coeficient.");
            return false;
        }
    }
    return true;
}
