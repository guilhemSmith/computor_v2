/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 11:31:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/28 17:33:35 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod error;
mod result;

pub use error::{ComputorError, ErrorKind};
pub use result::{Computed, ComputorResult, TreeResult};

use crate::arg_parse::Param;
use crate::lexer::token;
use crate::memory::{Memory, Value};
use crate::parser::{TokenTree, TreeBranch};
use crate::timer::Timer;
use crate::types::{Imaginary, Matrix, Rational};
use Computed as Comp;

use std::collections::HashMap;
use std::i32::{MAX as I32_MAX, MIN as I32_MIN};

extern crate colored;
use colored::Colorize;

type TTree = Box<dyn TokenTree>;
type Equ = HashMap<i32, Im>;
type Im = Imaginary;
type CErr = ComputorError;

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

    pub fn update_param(&mut self, verbose: bool, benchmark: bool) {
        self.verbose = verbose;
        self.bench = benchmark;
    }

    pub fn read_tokens(&mut self, tree: TTree) -> ComputorResult {
        if self.verbose {
            println!(
                "{}",
                format!(
                    "{} - tree received: {:?}",
                    "[v:Computor]".purple().bold(),
                    tree
                )
                .dimmed()
            )
        }
        if !self.bench {
            self.compute(tree)
        } else {
            let display = format!("Computor({})", tree);
            let _timer = Timer::new(&display[..]);
            self.compute(tree)
        }
    }

    fn compute(&mut self, mut tree: TTree) -> ComputorResult {
        let n = tree.count(token::count_error);
        if n > 0 {
            return Err(CErr::invalid_tokens(n));
        }

        let left: Option<TTree>;
        let right: Option<TTree>;
        match tree.as_any_mut().downcast_mut::<TreeBranch>() {
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
        return Err(CErr::bad_use_op('='));
    }

    fn single_part(&mut self, tree: TTree) -> ComputorResult {
        let comp = tree.compute(&mut self.memory, None)?;
        self.single_part_comp(comp)
    }

    fn single_part_comp(&mut self, comp: Comp) -> ComputorResult {
        Ok(match comp {
            Comp::None => return Err(CErr::empty_instr()),
            Comp::Res => self.mem_dump(),
            Comp::ValMat(mat) => {
                println!("{}", mat.to_string().replace(" ; ", "\n"))
            }
            Comp::ValIm(val) => println!("{}", val),
            Comp::VarCall(_, val) => println!("{}", val),
            Comp::VarSet(v) => return Err(CErr::unknown_id(v, true)),
            Comp::FunId(f, param) => {
                self.single_part_comp(self.memory.solve_fun(f, param)?)?
            }
            Comp::Equ(_, eq) => self.eq_one_sided(eq)?,
        })
    }

    fn dual_part(&mut self, left: TTree, right: TTree) -> ComputorResult {
        let comp = left.compute(&mut self.memory, None)?;
        self.dual_part_comp(comp, right)
    }

    fn dual_part_comp(&mut self, left: Comp, right: TTree) -> ComputorResult {
        Ok(match left {
            Comp::None => return Err(CErr::bad_use_op('=')),
            Comp::Res => return Err(CErr::bad_resolve()),
            Comp::ValMat(mat) => {
                self.dual_matr(mat, right.compute(&self.memory, None)?)?
            }
            Comp::ValIm(val) => {
                self.left_val(val, right.compute(&self.memory, None)?)?
            }
            Comp::VarCall(id, val) => {
                self.call_var(id, val, right.compute(&self.memory, None)?)?
            }
            Comp::VarSet(id) => {
                self.set_var(id, right.compute(&self.memory, None)?)?
            }
            Comp::FunId(id, param) => self.set_fun(id, param, right)?,
            Comp::Equ(id, eq) => {
                self.eq_two_sided(id, eq, right.compute(&self.memory, None)?)?
            }
        })
    }

    fn eq_one_sided(&self, eq: Equ) -> ComputorResult {
        let zero: i32 = 0;
        for (pow, coef) in eq.iter() {
            if *pow > 0 && *coef != Im::new(0.0, 0.0) {
                return Err(CErr::uncomplete_eq());
            }
        }
        if let Some(coef) = eq.get(&zero) {
            println!("{}", *coef);
        } else {
            return Err(CErr::uncomplete_eq());
        }
        Ok(())
    }

    fn eq_two_sided(
        &self,
        id: String,
        mut left: Equ,
        right: Comp,
    ) -> ComputorResult {
        Ok(match right {
            Comp::None => return Err(CErr::bad_use_op('=')),
            Comp::Res => return Err(CErr::uncomplete_eq()),
            Comp::ValMat(_) => return Err(CErr::matrix_in_eq()),
            Comp::ValIm(val) => {
                val_into_eq(&mut left, val)?;
                self.solve_eq(left, id)?;
            }
            Comp::VarCall(id_v, val) => match val {
                Value::Im(val) => {
                    var_into_eq(&mut left, &id, id_v, val)?;
                    self.solve_eq(left, id)?;
                }
                Value::Mat(_) => return Err(CErr::matrix_in_eq()),
            },
            Comp::VarSet(v) => {
                unknow_into_eq(&mut left, &id, v)?;
                self.solve_eq(left, id)?;
            }
            Comp::FunId(f, arg) => {
                self.eq_two_sided(id, left, self.memory.solve_fun(f, arg)?)?
            }
            Comp::Equ(id_r, eq) => {
                fuse_eq(&mut left, &id, eq, id_r)?;
                self.solve_eq(left, id)?;
            }
        })
    }

    fn left_val(&self, val: Im, right: Comp) -> ComputorResult {
        Ok(match right {
            Comp::None => return Err(CErr::bad_use_op('=')),
            Comp::Res => println!("{}", val),
            Comp::ValMat(_) => println!("false"),
            Comp::ValIm(r_val) => solve_two_val(val, Value::Im(r_val)),
            Comp::VarCall(_, r_val) => solve_two_val(val, r_val),
            Comp::VarSet(v) => println!("{} = {} is a solution.", v, val),
            Comp::FunId(f, arg) => {
                self.left_val(val, self.memory.solve_fun(f, arg)?)?
            }
            Comp::Equ(id, eq) => {
                let mut eq = eq;
                val_into_eq(&mut eq, val)?;
                self.solve_eq(eq, id)?;
            }
        })
    }

    fn call_var(
        &mut self,
        var: String,
        val: Value,
        right: Comp,
    ) -> ComputorResult {
        Ok(match right {
            Comp::None => return Err(CErr::bad_use_op('=')),
            Comp::Res => println!("{}", val),
            Comp::ValMat(mat) => {
                self.memory.set_var(var, Value::Mat(mat));
            }
            Comp::ValIm(nval) => {
                self.memory.set_var(var, Value::Im(nval));
            }
            Comp::VarCall(_, nval) => {
                self.memory.set_var(var, nval);
            }
            Comp::VarSet(v) => return Err(CErr::unknown_id(v, true)),
            Comp::FunId(f, arg) => {
                self.call_var(var, val, self.memory.solve_fun(f, arg)?)?
            }
            Comp::Equ(id, eq) => match val {
                Value::Im(val) => {
                    let mut eq = eq;
                    var_into_eq(&mut eq, &id, var, val)?;
                    self.solve_eq(eq, id)?;
                }
                Value::Mat(_) => return Err(CErr::matrix_in_eq()),
            },
        })
    }

    fn set_var(&mut self, var: String, right: Comp) -> ComputorResult {
        Ok(match right {
            Comp::None => return Err(CErr::bad_use_op('=')),
            Comp::Res => return Err(CErr::unknown_id(var, true)),
            Comp::ValMat(mat) => {
                self.memory.set_var(var, Value::Mat(mat));
            }
            Comp::ValIm(val) => {
                self.memory.set_var(var, Value::Im(val));
            }
            Comp::VarCall(_, val) => {
                self.memory.set_var(var, val);
            }
            Comp::VarSet(id) => {
                if id != var {
                    return Err(CErr::unknown_id(id, true));
                } else {
                    println!("Any value for {} is a solution.", id);
                }
            }
            Comp::FunId(f, arg) => {
                self.set_var(var, self.memory.solve_fun(f, arg)?)?
            }
            Comp::Equ(id, eq) => {
                let mut eq = eq;
                unknow_into_eq(&mut eq, &id, var)?;
                self.solve_eq(eq, id)?;
            }
        })
    }

    fn set_fun(
        &mut self,
        id: String,
        param: Vec<Comp>,
        exp: TTree,
    ) -> ComputorResult {
        match exp.token().as_any().downcast_ref::<token::Resolve>() {
            None => {
                self.memory.set_fun(id, param, exp)?;
                Ok(())
            }
            Some(_) => self.set_or_print_fn(id, param),
        }
    }

    fn dual_matr(&self, mat: Matrix, right: Comp) -> ComputorResult {
        Ok(match right {
            Comp::None => return Err(CErr::bad_use_op('=')),
            Comp::Res => println!("{}", mat),
            Comp::ValMat(other) => {
                println!("{}", if mat == other { "True" } else { "False" });
            }
            Comp::ValIm(_) => println!("False"),
            Comp::VarCall(_, _) => println!("False"),
            Comp::VarSet(v) => return Err(CErr::unknown_id(v, true)),
            Comp::FunId(f, arg) => {
                self.dual_matr(mat, self.memory.solve_fun(f, arg)?)?
            }
            Comp::Equ(_, _) => return Err(CErr::matrix_in_eq()),
        })
    }

    fn set_or_print_fn(
        &mut self,
        id: String,
        param: Vec<Comp>,
    ) -> ComputorResult {
        match self.memory.get_fun(&id) {
            None => Err(CErr::unknown_id(id, false)),
            Some(fun) => {
                if self.memory.valid_args(&param) {
                    self.single_part_comp(self.memory.solve_fun(id, param)?)?;
                } else {
                    fun.print(self.memory.param_to_string(param)?);
                }
                Ok(())
            }
        }
    }

    fn mem_dump(&self) {
        println!("{}", self.memory);
    }

    fn solve_eq(&self, mut eq: Equ, id: String) -> ComputorResult {
        filter_eq(&mut eq);
        if !valid_eq(&eq) {
            return Ok(());
        }
        match eq.keys().max() {
            None => println!("Any value for {} is a solution.", id),
            Some(max) => match *max {
                0 => println!("False."),
                1 => eq_degree_one(eq, id)?,
                2 => eq_degree_two(eq, id, self.verbose)?,
                2..=I32_MAX => {
                    println!("Can't solve equation with degree above 2.")
                }
                I32_MIN..=0 => {
                    println!("Can't solve equation with degree below 0.")
                }
            },
        };
        Ok(())
    }
}

fn val_into_eq(eq: &mut Equ, val: Im) -> ComputorResult {
    let zero: i32 = 0;
    match eq.get_mut(&zero) {
        None => {
            eq.insert(zero, -val);
        }
        Some(coef) => *coef = coef.sub(&val)?,
    };
    Ok(())
}

fn var_into_eq(
    eq: &mut Equ,
    id: &String,
    id_v: String,
    val: Im,
) -> ComputorResult {
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
            eq.insert(pow, -n_coef);
        }
        Some(coef) => *coef = coef.sub(&n_coef)?,
    };
    Ok(())
}

fn unknow_into_eq(eq: &mut Equ, id: &String, unk: String) -> ComputorResult {
    if *id == unk {
        let one: i32 = 1;
        let one_im = Im::new(1.0, 0.0);
        match eq.get_mut(&one) {
            None => {
                eq.insert(one, -one_im);
            }
            Some(coef) => *coef = coef.sub(&one_im)?,
        }
        Ok(())
    } else {
        Err(CErr::too_many_unknown())
    }
}

fn fuse_eq(
    eq_l: &mut Equ,
    id_l: &String,
    eq_r: Equ,
    id_r: String,
) -> ComputorResult {
    if *id_l == id_r {
        for (pow, val) in eq_r {
            match eq_l.get_mut(&pow) {
                None => {
                    eq_l.insert(pow, -val);
                }
                Some(coef) => *coef = coef.sub(&val)?,
            };
        }
        Ok(())
    } else {
        Err(CErr::too_many_unknown())
    }
}

fn solve_two_val(val_l: Im, val_r: Value) {
    match val_r {
        Value::Im(val_r) => {
            if val_l == val_r {
                println!("True");
            } else {
                println!("False");
            }
        }
        Value::Mat(_) => println!("False"),
    }
}

fn eq_degree_one(eq: Equ, id: String) -> ComputorResult {
    let mut index: i32 = 0;
    let zero = match eq.get(&index) {
        None => Im::new(0.0, 0.0),
        Some(val) => *val,
    };
    index += 1;
    let one = *eq.get(&index).unwrap();
    print_eq(&eq, &id, 1);
    println!("Solution: {} = {}", id, -zero.div(&one)?);
    Ok(())
}

fn eq_degree_two(eq: Equ, id: String, verb: bool) -> ComputorResult {
    let mut index: i32 = 0;
    let deg_zero = match eq.get(&index) {
        None => Imaginary::new(0.0, 0.0),
        Some(val) => *val,
    };
    index += 1;
    let deg_one = match eq.get(&index) {
        None => Imaginary::new(0.0, 0.0),
        Some(val) => *val,
    };
    index += 1;
    let deg_two = *eq.get(&index).unwrap();
    print_eq(&eq, &id, 2);
    let right = Im::new(4.0, 0.0).mul(&deg_two)?.mul(&deg_zero)?;
    let delta = deg_one.pow(2)?.sub(&right)?.get_real();
    if verb {
        println!(
            "{}",
            format!(
                "{} - Delta = {}",
                "[v:Computor]".purple().bold(),
                delta.to_string()
            )
            .dimmed()
        );
    }
    let two_re = Im::new(2.0, 0.0);
    let div = deg_two.mul(&two_re)?;
    if delta > Rational::zero() {
        let root = Im::new((delta.get_val()).sqrt(), 0.0);
        let sol_a = (-deg_one).add(&root)?.div(&div)?;
        let sol_b = (-deg_one).sub(&root)?.div(&div)?;
        println!(
            "Delta is positive, 2 real solutions:\n{} = {}\n{} = {}",
            id, sol_a, id, sol_b
        );
    } else if delta < Rational::zero() {
        let root = Im::new(0.0, (-delta.get_val()).sqrt());
        let sol_a = (-deg_one).add(&root)?.div(&div)?;
        let sol_b = (-deg_one).sub(&root)?.div(&div)?;
        println!(
            "Delta is negative, 2 imaginary solutions:\n{} = {}\n{} = {}",
            id, sol_a, id, sol_b
        );
    } else {
        let sol = -deg_one.div(&div)?;
        println!("Delta is null, 1 real solution:\n{} = {}", id, sol);
    }
    Ok(())
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

fn print_eq(eq: &Equ, id: &String, degree: i32) {
    let mut pow: i32 = degree;
    let mut to_print = eq.len();
    println!("Equation of degree {}:", degree);
    while to_print > 0 {
        if let Some(coef) = eq.get(&pow) {
            to_print -= 1;
            let val = *coef;
            print!(
                "{}{}{}{}",
                if val.get_real().get_val() < 0.0 || pow == degree {
                    " "
                } else {
                    " + "
                },
                val,
                if pow != 0 {
                    format!(" * {}", id)
                } else {
                    String::new()
                },
                if pow != 0 && pow != 1 {
                    format!("^{}", pow)
                } else {
                    String::new()
                },
            );
        }
        pow -= 1;
    }
    println!(" = 0");
}
