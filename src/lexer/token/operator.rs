/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/10 11:37:30 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{ComputorError as CErr, ComputorResult as CRes};
use crate::memory::{Extension, Memory};
use crate::types::Imaginary;

use std::any::Any;
use std::collections::HashMap;
use std::fmt;

type Equation = HashMap<u32, Imaginary>;

#[derive(Clone)]
pub struct Operator {
    symbol: char,
    priority: i32,
    op: fn(&Self, CRes, CRes) -> CRes,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[op:{}]", self.symbol)
    }
}

impl Token for Operator {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(&self, _mem: &Memory, _ext: Option<&mut Extension>) -> CRes {
        CRes::Err(CErr::unparsed_token(self))
    }
}

impl Operator {
    pub fn new(symbol: char) -> Result<Self, LexerError> {
        let priority: i32;
        let op: fn(&Self, CRes, CRes) -> CRes;
        match symbol {
            '=' => {
                priority = 0;
                op = Operator::equal;
            }
            '+' => {
                priority = 1;
                op = Operator::add;
            }
            '-' => {
                priority = 1;
                op = Operator::sub;
            }
            '*' => {
                priority = 2;
                op = Operator::mul;
            }
            '/' => {
                priority = 2;
                op = Operator::div;
            }
            '^' => {
                priority = 3;
                op = Operator::pow;
            }
            _ => return Err(LexerError::InvalidOp(symbol)),
        };
        Ok(Operator {
            symbol: symbol,
            priority: priority,
            op: op,
        })
    }

    pub fn is_prior(&self, other: &Self) -> bool {
        self.priority > other.priority
    }

    pub fn set_prior_as_exp(&mut self) {
        self.priority = 4;
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }

    pub fn exec(&self, _mem: &Memory, orand_l: CRes, orand_r: CRes) -> CRes {
        (self.op)(self, orand_l, orand_r)
    }

    fn equal(&self, _orand_l: CRes, _orand_r: CRes) -> CRes {
        CRes::Err(CErr::too_many_equal())
    }

    fn mul(&self, left: CRes, right: CRes) -> CRes {
        match (left, right) {
            (CRes::None, _) => CRes::Err(CErr::bad_use_op('*')),
            (_, CRes::None) => CRes::Err(CErr::bad_use_op('*')),
            (CRes::Err(err), _) => CRes::Err(err),
            (_, CRes::Err(err)) => CRes::Err(err),
            (CRes::Res, _) => CRes::Err(CErr::bad_resolve()),
            (_, CRes::Res) => CRes::Err(CErr::bad_resolve()),
            (CRes::FunSet(id, _), _) => CRes::Err(CErr::fun_undef(&id)),
            (_, CRes::FunSet(id, _)) => CRes::Err(CErr::fun_undef(&id)),
            (CRes::VarSet(v_a), CRes::VarSet(v_b)) => dual_var_mul(v_a, v_b),
            (CRes::VarSet(var), CRes::VarCall(_, val)) => new_eq_mul(var, val),
            (CRes::VarCall(_, val), CRes::VarSet(var)) => new_eq_mul(var, val),
            (CRes::VarSet(var), CRes::Val(val)) => new_eq_mul(var, val),
            (CRes::Val(val), CRes::VarSet(var)) => new_eq_mul(var, val),
            (CRes::Val(v_a), CRes::Val(v_b)) => mul_ex(v_a, v_b),
            (CRes::Val(v_a), CRes::VarCall(_, v_b)) => mul_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::Val(v_b)) => mul_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::VarCall(_, v_b)) => mul_ex(v_a, v_b),
            (CRes::Equ(id_a, eq_a), CRes::Equ(id_b, eq_b)) => {
                comb_eq_mul(id_a, id_b, eq_a, eq_b)
            }
            (CRes::Equ(id_eq, eq), CRes::VarSet(id_v)) => {
                var_eq_mul(id_v, id_eq, eq)
            }
            (CRes::VarSet(id_v), CRes::Equ(id_eq, eq)) => {
                var_eq_mul(id_v, id_eq, eq)
            }
            (CRes::Equ(id, eq), CRes::Val(v)) => val_eq_mul(v, id, eq),
            (CRes::Val(v), CRes::Equ(id, eq)) => val_eq_mul(v, id, eq),
            (CRes::Equ(id, eq), CRes::VarCall(_, v)) => val_eq_mul(v, id, eq),
            (CRes::VarCall(_, v), CRes::Equ(id, eq)) => val_eq_mul(v, id, eq),
        }
    }

    fn div(&self, left: CRes, right: CRes) -> CRes {
        match (left, right) {
            (CRes::None, _) => CRes::Err(CErr::bad_use_op('/')),
            (_, CRes::None) => CRes::Err(CErr::bad_use_op('/')),
            (CRes::Err(err), _) => CRes::Err(err),
            (_, CRes::Err(err)) => CRes::Err(err),
            (CRes::Res, _) => CRes::Err(CErr::bad_resolve()),
            (_, CRes::Res) => CRes::Err(CErr::bad_resolve()),
            (CRes::FunSet(id, _), _) => CRes::Err(CErr::fun_undef(&id)),
            (_, CRes::FunSet(id, _)) => CRes::Err(CErr::fun_undef(&id)),
            (CRes::VarSet(var), CRes::Val(val)) => new_eq_div(var, val),
            (CRes::VarSet(var), CRes::VarCall(_, val)) => new_eq_div(var, val),
            (CRes::Val(v_a), CRes::Val(v_b)) => div_ex(v_a, v_b),
            (CRes::Val(v_a), CRes::VarCall(_, v_b)) => div_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::Val(v_b)) => div_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::VarCall(_, v_b)) => div_ex(v_a, v_b),
        }
    }

    fn add(&self, left: CRes, right: CRes) -> CRes {
        match (left, right) {
            (CRes::None, _) => CRes::Err(CErr::bad_use_op('+')),
            (_, CRes::None) => CRes::Err(CErr::bad_use_op('+')),
            (CRes::Err(err), _) => CRes::Err(err),
            (_, CRes::Err(err)) => CRes::Err(err),
            (CRes::Res, _) => CRes::Err(CErr::bad_resolve()),
            (_, CRes::Res) => CRes::Err(CErr::bad_resolve()),
            (CRes::FunSet(id, _), _) => CRes::Err(CErr::fun_undef(&id)),
            (_, CRes::FunSet(id, _)) => CRes::Err(CErr::fun_undef(&id)),
            (CRes::VarSet(var), CRes::Val(val)) => new_eq_add(var, val),
            (CRes::VarSet(var), CRes::VarCall(_, val)) => new_eq_add(var, val),
            (CRes::Val(v_a), CRes::Val(v_b)) => add_ex(v_a, v_b),
            (CRes::Val(v_a), CRes::VarCall(_, v_b)) => add_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::Val(v_b)) => add_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::VarCall(_, v_b)) => add_ex(v_a, v_b),
        }
    }

    fn sub(&self, left: CRes, right: CRes) -> CRes {
        match (left, right) {
            (CRes::None, _) => CRes::Err(CErr::bad_use_op('-')),
            (_, CRes::None) => CRes::Err(CErr::bad_use_op('-')),
            (CRes::Err(err), _) => CRes::Err(err),
            (_, CRes::Err(err)) => CRes::Err(err),
            (CRes::Res, _) => CRes::Err(CErr::bad_resolve()),
            (_, CRes::Res) => CRes::Err(CErr::bad_resolve()),
            (CRes::FunSet(id, _), _) => CRes::Err(CErr::fun_undef(&id)),
            (_, CRes::FunSet(id, _)) => CRes::Err(CErr::fun_undef(&id)),
            (CRes::VarSet(var), CRes::Val(val)) => new_eq_sub(var, val),
            (CRes::VarSet(var), CRes::VarCall(_, val)) => new_eq_sub(var, val),
            (CRes::Val(v_a), CRes::Val(v_b)) => sub_ex(v_a, v_b),
            (CRes::Val(v_a), CRes::VarCall(_, v_b)) => sub_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::Val(v_b)) => sub_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::VarCall(_, v_b)) => sub_ex(v_a, v_b),
        }
    }

    fn pow(&self, left: CRes, right: CRes) -> CRes {
        match (left, right) {
            (CRes::None, _) => CRes::Err(CErr::bad_use_op('^')),
            (_, CRes::None) => CRes::Err(CErr::bad_use_op('^')),
            (CRes::Err(err), _) => CRes::Err(err),
            (_, CRes::Err(err)) => CRes::Err(err),
            (CRes::Res, _) => CRes::Err(CErr::bad_resolve()),
            (_, CRes::Res) => CRes::Err(CErr::bad_resolve()),
            (CRes::FunSet(id, _), _) => CRes::Err(CErr::fun_undef(&id)),
            (_, CRes::FunSet(id, _)) => CRes::Err(CErr::fun_undef(&id)),
            (CRes::VarSet(var), CRes::Val(val)) => new_eq_pow(var, val),
            (CRes::VarSet(var), CRes::VarCall(_, val)) => new_eq_pow(var, val),
            (CRes::Val(v_a), CRes::Val(v_b)) => pow_ex(v_a, v_b),
            (CRes::Val(v_a), CRes::VarCall(_, v_b)) => pow_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::Val(v_b)) => pow_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::VarCall(_, v_b)) => pow_ex(v_a, v_b),
        }
    }
}

fn pow_ex(val_a: Imaginary, val_b: Imaginary) -> CRes {
    if !val_b.is_real() || !val_b.is_int() {
        return CRes::Err(CErr::bad_pow());
    }
    let res = val_a.pow(val_b.get_real());
    return CRes::Val(res);
}

fn mul_ex(val_a: Imaginary, val_b: Imaginary) -> CRes {
    CRes::Val(val_a * val_b)
}

fn div_ex(val_a: Imaginary, val_b: Imaginary) -> CRes {
    if val_b == Imaginary::new(0.0, 0.0) {
        return CRes::Err(CErr::div_by_zero());
    }
    CRes::Val(val_a / val_b)
}

fn add_ex(val_a: Imaginary, val_b: Imaginary) -> CRes {
    CRes::Val(val_a + val_b)
}

fn sub_ex(val_a: Imaginary, val_b: Imaginary) -> CRes {
    CRes::Val(val_a - val_b)
}

fn dual_var_mul(var_a: String, var_b: String) -> CRes {
    if var_a != var_b {
        return CRes::Err(CErr::too_many_unknown());
    }
    let mut eq: Equation = HashMap::new();
    eq.insert(2, Imaginary::new(1.0, 0.0));
    return CRes::Equ(var_a, eq);
}

fn comb_eq_mul(
    id_a: String,
    id_b: String,
    eq_a: Equation,
    eq_b: Equation,
) -> CRes {
    if id_a != id_b {
        return CRes::Err(CErr::too_many_unknown());
    }
    let mut res: Equation = HashMap::new();

    for (pow_a, coef_a) in eq_a.iter() {
        for (pow_b, coef_b) in eq_b.iter() {
            let pow = pow_a + pow_b;
            let coef = *coef_a * *coef_b;
            match res.get_mut(&pow) {
                None => {
                    res.insert(pow, coef);
                }
                Some(prev_coef) => *prev_coef = *prev_coef + coef,
            }
        }
    }
    return CRes::Equ(id_a, res);
}

fn var_eq_mul(id_var: String, id_eq: String, eq: Equation) -> CRes {
    if id_var != id_eq {
        return CRes::Err(CErr::too_many_unknown());
    }
    let mut res: Equation = HashMap::new();

    for (pow, coef) in eq.into_iter() {
        res.insert(pow + 1, coef);
    }
    return CRes::Equ(id_var, res);
}

fn val_eq_mul(val: Imaginary, id: String, mut eq: Equation) -> CRes {
    for (_, coef) in eq.iter_mut() {
        *coef = *coef * val;
    }
    return CRes::Equ(id, eq);
}

fn new_eq_pow(var: String, val: Imaginary) -> CRes {
    if !val.is_real() || !val.is_int() {
        return CRes::Err(CErr::bad_pow());
    }
    let mut eq: Equation = HashMap::new();
    eq.insert(val.get_real(), Imaginary::new(1.0, 0.0));
    return CRes::Equ(var, eq);
}

fn new_eq_mul(var: String, val: Imaginary) -> CRes {
    let mut eq: Equation = HashMap::new();
    eq.insert(1, val);
    return CRes::Equ(var, eq);
}

fn new_eq_div(var: String, val: Imaginary) -> CRes {
    if val == Imaginary::new(0.0, 0.0) {
        return CRes::Err(CErr::div_by_zero());
    }
    let mut eq: Equation = HashMap::new();
    eq.insert(1, Imaginary::new(1.0, 0.0) / val);
    return CRes::Equ(var, eq);
}

fn new_eq_add(var: String, val: Imaginary) -> CRes {
    let mut eq: Equation = HashMap::new();
    eq.insert(1, Imaginary::new(1.0, 0.0));
    eq.insert(0, val);
    return CRes::Equ(var, eq);
}

fn new_eq_sub(var: String, val: Imaginary) -> CRes {
    let mut eq: Equation = HashMap::new();
    eq.insert(1, Imaginary::new(1.0, 0.0));
    eq.insert(0, Imaginary::new(0.0, 0.0) - val);
    return CRes::Equ(var, eq);
}
