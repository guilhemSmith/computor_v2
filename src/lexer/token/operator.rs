/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/10 10:06:38 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{ComputorError as CErr, ComputorResult as CRes};
use crate::memory::{Extension, Memory};
use crate::types::Imaginary;

use std::any::Any;
use std::fmt;

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
            // (CRes::VarSet(var), CRes::Val(val)) => CRes::default(),
            // (CRes::VarSet(var), CRes::VarCall(_, val)) => CRes::default(),
            (CRes::Val(v_a), CRes::Val(v_b)) => mul_ex(v_a, v_b),
            (CRes::Val(v_a), CRes::VarCall(_, v_b)) => mul_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::Val(v_b)) => mul_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::VarCall(_, v_b)) => mul_ex(v_a, v_b),
            _ => CRes::default(),
        }
    }

    fn div(&self, left: CRes, right: CRes) -> CRes {
        match (left, right) {
            // (CRes::VarSet(var), CRes::Val(val)) => CRes::default(),
            // (CRes::VarSet(var), CRes::VarCall(_, val)) => CRes::default(),
            (CRes::Val(v_a), CRes::Val(v_b)) => div_ex(v_a, v_b),
            (CRes::Val(v_a), CRes::VarCall(_, v_b)) => div_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::Val(v_b)) => div_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::VarCall(_, v_b)) => div_ex(v_a, v_b),
            _ => CRes::default(),
        }
    }

    fn add(&self, left: CRes, right: CRes) -> CRes {
        match (left, right) {
            // (CRes::VarSet(var), CRes::Val(val)) => CRes::default(),
            // (CRes::VarSet(var), CRes::VarCall(_, val)) => CRes::default(),
            (CRes::Val(v_a), CRes::Val(v_b)) => add_ex(v_a, v_b),
            (CRes::Val(v_a), CRes::VarCall(_, v_b)) => add_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::Val(v_b)) => add_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::VarCall(_, v_b)) => add_ex(v_a, v_b),
            _ => CRes::default(),
        }
    }

    fn sub(&self, left: CRes, right: CRes) -> CRes {
        match (left, right) {
            // (CRes::VarSet(var), CRes::Val(val)) => CRes::default(),
            // (CRes::VarSet(var), CRes::VarCall(_, val)) => CRes::default(),
            (CRes::Val(v_a), CRes::Val(v_b)) => sub_ex(v_a, v_b),
            (CRes::Val(v_a), CRes::VarCall(_, v_b)) => sub_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::Val(v_b)) => sub_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::VarCall(_, v_b)) => sub_ex(v_a, v_b),
            _ => CRes::default(),
        }
    }

    fn pow(&self, left: CRes, right: CRes) -> CRes {
        match (left, right) {
            // (CRes::VarSet(var), CRes::Val(val)) => CRes::default(),
            // (CRes::VarSet(var), CRes::VarCall(_, val)) => CRes::default(),
            (CRes::Val(v_a), CRes::Val(v_b)) => pow_ex(v_a, v_b),
            (CRes::Val(v_a), CRes::VarCall(_, v_b)) => pow_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::Val(v_b)) => pow_ex(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::VarCall(_, v_b)) => pow_ex(v_a, v_b),
            _ => CRes::default(),
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
