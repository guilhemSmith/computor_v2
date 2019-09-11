/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/11 14:05:50 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{ComputorError as CErr, ComputorResult as CRes};
use crate::memory::{Extension, Memory};
use crate::types::Imaginary as Im;

use std::any::Any;
use std::collections::HashMap;
use std::fmt;

type Equ = HashMap<i32, Im>;

pub trait Operator: Token + fmt::Display {
    fn priority(&self) -> i32;
    fn is_prior(&self, other: &dyn Operator) -> bool {
        self.priority() > other.priority()
    }
    fn set_prior_as_exp(&mut self);
    fn symbol(&self) -> char;
    fn dual_var(&self, var_a: String, var_b: String) -> CRes;
    fn new_eq(&self, var: String, val: Im, var_left: bool) -> CRes;
    fn op(&self, val_a: Im, val_b: Im) -> CRes;
    fn fus_eq(&self, id_a: String, id_b: String, eq_a: Equ, eq_b: Equ) -> CRes;
    fn var_eq(&self, id_var: String, id_eq: String, eq: Equ, eq_left: bool) -> CRes;
    fn val_eq(&self, val: Im, id: String, eq: Equ, eq_left: bool) -> CRes;
    fn exec(&self, _mem: &Memory, left: CRes, right: CRes) -> CRes {
        match (left, right) {
            (CRes::Err(err), _) => CRes::Err(err),
            (_, CRes::Err(err)) => CRes::Err(err),
            (CRes::Res, _) => CRes::Err(CErr::bad_resolve()),
            (_, CRes::Res) => CRes::Err(CErr::bad_resolve()),
            (CRes::FunSet(id, _), _) => CRes::Err(CErr::fun_undef(&id)),
            (_, CRes::FunSet(id, _)) => CRes::Err(CErr::fun_undef(&id)),
            (CRes::None, right) => none_on_left(right, self.symbol()),
            (_, CRes::None) => CRes::Err(CErr::bad_use_op(self.symbol())),
            (CRes::VarSet(v_a), CRes::VarSet(v_b)) => self.dual_var(v_a, v_b),
            (CRes::VarSet(var), CRes::VarCall(_, val)) => self.new_eq(var, val, true),
            (CRes::VarCall(_, val), CRes::VarSet(var)) => self.new_eq(var, val, false),
            (CRes::VarSet(var), CRes::Val(val)) => self.new_eq(var, val, true),
            (CRes::Val(val), CRes::VarSet(var)) => self.new_eq(var, val, false),
            (CRes::Val(v_a), CRes::Val(v_b)) => self.op(v_a, v_b),
            (CRes::Val(v_a), CRes::VarCall(_, v_b)) => self.op(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::Val(v_b)) => self.op(v_a, v_b),
            (CRes::VarCall(_, v_a), CRes::VarCall(_, v_b)) => self.op(v_a, v_b),
            (CRes::Equ(id_a, eq_a), CRes::Equ(id_b, eq_b)) => self.fus_eq(id_a, id_b, eq_a, eq_b),
            (CRes::Equ(id_eq, eq), CRes::VarSet(id_v)) => self.var_eq(id_v, id_eq, eq, true),
            (CRes::VarSet(id_v), CRes::Equ(id_eq, eq)) => self.var_eq(id_v, id_eq, eq, false),
            (CRes::Equ(id, eq), CRes::Val(v)) => self.val_eq(v, id, eq, true),
            (CRes::Val(v), CRes::Equ(id, eq)) => self.val_eq(v, id, eq, false),
            (CRes::Equ(id, eq), CRes::VarCall(_, v)) => self.val_eq(v, id, eq, true),
            (CRes::VarCall(_, v), CRes::Equ(id, eq)) => self.val_eq(v, id, eq, false),
        }
    }
}

pub fn new_operator(symbol: char) -> Result<Box<dyn Token>, LexerError> {
    match symbol {
        '=' => Ok(Box::new(OpEqual::new())),
        '+' => Ok(Box::new(OpAdd::new())),
        '-' => Ok(Box::new(OpSub::new())),
        '*' => Ok(Box::new(OpMul::new())),
        '/' => Ok(Box::new(OpDiv::new())),
        '^' => Ok(Box::new(OpPow::new())),
        _ => Err(LexerError::InvalidOp(symbol)),
    }
}

struct OpEqual {
    priority: i32
}

impl OpEqual {
    fn new() -> Self {
        OpEqual { priority: 0 }
    }
}

impl fmt::Display for OpEqual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=")
    }
}

impl fmt::Debug for OpEqual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[op:=]")
    }
}

impl Token for OpEqual {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_op_ref(&self) -> Option<&dyn Operator> {
        Some(self as &dyn Operator)
    }

    fn as_op_mut(&mut self) -> Option<&mut dyn Operator> {
        Some(self as &mut dyn Operator)
    }

    fn get_result(&self, _mem: &Memory, _ext: Option<&mut Extension>) -> CRes {
        CRes::Err(CErr::unparsed_token(self))
    }
}

impl Operator for OpEqual {
    fn priority(&self) -> i32 {
        self.priority
    }

    fn set_prior_as_exp(&mut self) {
        self.priority = 4;
    }

    fn symbol(&self) -> char {
        '='
    }

    fn op(&self, _: Im, _: Im) -> CRes {
        CRes::Err(CErr::too_many_equal())
    }

    fn dual_var(&self, _: String, _: String) -> CRes {
        CRes::Err(CErr::too_many_equal())
    }

    fn fus_eq(&self, _: String, _: String, _: Equ, _: Equ) -> CRes {
        CRes::Err(CErr::too_many_equal())
    }

    fn var_eq(&self, _: String, _: String, _: Equ, _: bool) -> CRes {
        CRes::Err(CErr::too_many_equal())
    }

    fn val_eq(&self, _: Im, _: String, _: Equ, _: bool) -> CRes {
        CRes::Err(CErr::too_many_equal())
    }

    fn new_eq(&self, _: String, _: Im, _: bool) -> CRes {
        CRes::Err(CErr::too_many_equal())
    }
    
    fn exec(&self, _: &Memory, _: CRes, _: CRes) -> CRes {
        CRes::Err(CErr::too_many_equal())
    }
}

struct OpMul {
    priority: i32,
}

impl OpMul {
    fn new() -> Self {
        OpMul { priority: 2 }
    }
}

impl fmt::Display for OpMul {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "*")
    }
}

impl fmt::Debug for OpMul {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[op:*]")
    }
}

impl Token for OpMul {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_op_ref(&self) -> Option<&dyn Operator> {
        Some(self as &dyn Operator)
    }

    fn as_op_mut(&mut self) -> Option<&mut dyn Operator> {
        Some(self as &mut dyn Operator)
    }

    fn get_result(&self, _mem: &Memory, _ext: Option<&mut Extension>) -> CRes {
        CRes::Err(CErr::unparsed_token(self))
    }
}

impl Operator for OpMul {
    fn priority(&self) -> i32 {
        self.priority
    }

    fn set_prior_as_exp(&mut self) {
        self.priority = 4;
    }

    fn symbol(&self) -> char {
        '*'
    }

    fn op(&self, val_a: Im, val_b: Im) -> CRes {
        // TODO: overflow protection here
        CRes::Val(val_a * val_b)
    }

    fn dual_var(&self, var_a: String, var_b: String) -> CRes {
        if var_a != var_b {
            return CRes::Err(CErr::too_many_unknown());
        }
        let mut eq: Equ = HashMap::new();
        eq.insert(2, Im::new(1.0, 0.0));
        return CRes::Equ(var_a, eq);
    }

    fn fus_eq(&self, id_a: String, id_b: String, eq_a: Equ, eq_b: Equ) -> CRes {
        if id_a != id_b {
            return CRes::Err(CErr::too_many_unknown());
        }
        let mut res: Equ = HashMap::new();

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

    fn var_eq(&self, id_var: String, id_eq: String, eq: Equ, _: bool) -> CRes {
        if id_var != id_eq {
            return CRes::Err(CErr::too_many_unknown());
        }
        let mut res: Equ = HashMap::new();

        for (pow, coef) in eq.into_iter() {
            res.insert(pow + 1, coef);
        }
        return CRes::Equ(id_var, res);
    }

    fn val_eq(&self, val: Im, id: String, mut eq: Equ, _: bool) -> CRes {
        for (_, coef) in eq.iter_mut() {
            *coef = *coef * val;
        }
        return CRes::Equ(id, eq);
    }

    fn new_eq(&self, var: String, val: Im, _: bool) -> CRes {
        let mut eq: Equ = HashMap::new();
        eq.insert(1, val);
        return CRes::Equ(var, eq);
    }
}

struct OpAdd {
    priority: i32,
}

impl OpAdd {
    fn new() -> Self {
        OpAdd { priority: 1 }
    }
}

impl fmt::Display for OpAdd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+")
    }
}

impl fmt::Debug for OpAdd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[op:+]")
    }
}

impl Token for OpAdd {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_op_ref(&self) -> Option<&dyn Operator> {
        Some(self as &dyn Operator)
    }

    fn as_op_mut(&mut self) -> Option<&mut dyn Operator> {
        Some(self as &mut dyn Operator)
    }

    fn get_result(&self, _mem: &Memory, _ext: Option<&mut Extension>) -> CRes {
        CRes::Err(CErr::unparsed_token(self))
    }
}

impl Operator for OpAdd {
    fn priority(&self) -> i32 {
        self.priority
    }

    fn set_prior_as_exp(&mut self) {
        self.priority = 4;
    }

    fn symbol(&self) -> char {
        '+'
    }

    fn op(&self, val_a: Im, val_b: Im) -> CRes {
        // TODO: overflow protection here
        CRes::Val(val_a + val_b)
    }

    fn dual_var(&self, var_a: String, var_b: String) -> CRes {
        if var_a != var_b {
            return CRes::Err(CErr::too_many_unknown());
        }
        let mut eq: Equ = HashMap::new();
        eq.insert(1, Im::new(2.0, 0.0));
        return CRes::Equ(var_a, eq);
    }

    fn fus_eq(
        &self,
        id_a: String,
        id_b: String,
        mut eq_a: Equ,
        eq_b: Equ,
    ) -> CRes {
        if id_a != id_b {
            return CRes::Err(CErr::too_many_unknown());
        }

        for (pow_b, coef_b) in eq_b.into_iter() {
            match eq_a.get_mut(&pow_b) {
                None => {
                    eq_a.insert(pow_b, coef_b);
                }
                Some(coef_a) => *coef_a = *coef_a + coef_b,
            }
        }
        return CRes::Equ(id_a, eq_a);
    }

    fn var_eq(&self, id_var: String, id_eq: String, mut eq: Equ, _: bool) -> CRes {
        if id_var != id_eq {
            return CRes::Err(CErr::too_many_unknown());
        }
        let one: i32 = 1;

        match eq.get_mut(&one) {
            None => {
                eq.insert(one, Im::new(1.0, 0.0));
            }
            Some(coef) => *coef = *coef + Im::new(1.0, 0.0),
        };
        return CRes::Equ(id_var, eq);
    }

    fn val_eq(&self, val: Im, id: String, mut eq: Equ, _: bool) -> CRes {
        let zero: i32 = 0;
        match eq.get_mut(&zero) {
            None => {
                eq.insert(zero, val);
            }
            Some(coef) => *coef = *coef + val,
        }
        return CRes::Equ(id, eq);
    }

    fn new_eq(&self, var: String, val: Im, _: bool) -> CRes {
        let mut eq: Equ = HashMap::new();
        eq.insert(1, Im::new(1.0, 0.0));
        eq.insert(0, val);
        return CRes::Equ(var, eq);
    }
}

struct OpSub {
    priority: i32,
}

impl OpSub {
    pub fn new() -> Self {
        OpSub { priority: 1 }
    }
}

impl fmt::Display for OpSub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-")
    }
}

impl fmt::Debug for OpSub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[op:-]")
    }
}

impl Token for OpSub {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_op_ref(&self) -> Option<&dyn Operator> {
        Some(self as &dyn Operator)
    }

    fn as_op_mut(&mut self) -> Option<&mut dyn Operator> {
        Some(self as &mut dyn Operator)
    }

    fn get_result(&self, _mem: &Memory, _ext: Option<&mut Extension>) -> CRes {
        CRes::Err(CErr::unparsed_token(self))
    }
}

impl Operator for OpSub {
    fn priority(&self) -> i32 {
        self.priority
    }

    fn set_prior_as_exp(&mut self) {
        self.priority = 4;
    }

    fn symbol(&self) -> char {
        '-'
    }

    fn op(&self, val_a: Im, val_b: Im) -> CRes {
        // TODO: overflow protection here
        CRes::Val(val_a - val_b)
    }

    fn dual_var(&self, var_a: String, var_b: String) -> CRes {
        if var_a != var_b {
            return CRes::Err(CErr::too_many_unknown());
        }
        return CRes::Val(Im::new(0.0, 0.0));
    }

    fn fus_eq(
        &self,
        id_a: String,
        id_b: String,
        mut eq_a: Equ,
        eq_b: Equ,
    ) -> CRes {
        if id_a != id_b {
            return CRes::Err(CErr::too_many_unknown());
        }

        for (pow_b, coef_b) in eq_b.into_iter() {
            match eq_a.get_mut(&pow_b) {
                None => {
                    eq_a.insert(pow_b, coef_b);
                }
                Some(coef_a) => *coef_a = *coef_a - coef_b,
            }
        }
        return CRes::Equ(id_a, eq_a);
    }

    fn var_eq(&self, id_var: String, id_eq: String, mut eq: Equ, eq_left: bool) -> CRes {
        if id_var != id_eq {
            return CRes::Err(CErr::too_many_unknown());
        }
        let one: i32 = 1;

        match eq.get_mut(&one) {
            None => {
                eq.insert(one, Im::new(1.0, 0.0));
            }
            Some(coef) => *coef = if eq_left {
                *coef - Im::new(1.0, 0.0)
            } else {
                Im::new(1.0, 0.0) - *coef
            },
        };
        return CRes::Equ(id_var, eq);
    }

    fn val_eq(&self, val: Im, id: String, mut eq: Equ, eq_left: bool) -> CRes {
        let zero: i32 = 0;
        match eq.get_mut(&zero) {
            None => {
                eq.insert(zero, val);
            }
            Some(coef) => *coef = if eq_left {
                *coef - val
            } else {
                val - *coef
            },
        }
        return CRes::Equ(id, eq);
    }

    fn new_eq(&self, var: String, val: Im, var_left: bool) -> CRes {
        let mut eq: Equ = HashMap::new();
        if var_left {
            eq.insert(1, Im::new(1.0, 0.0));
            eq.insert(0, Im::new(0.0, 0.0) - val);
        } else {
            eq.insert(1, Im::new(-1.0, 0.0));
            eq.insert(0, val);
        }
        return CRes::Equ(var, eq);
    }
}

struct OpDiv {
    priority: i32,
}

impl OpDiv {
    pub fn new() -> Self {
        OpDiv { priority: 2 }
    }
}

impl fmt::Display for OpDiv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/")
    }
}

impl fmt::Debug for OpDiv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[op:/]")
    }
}

impl Token for OpDiv {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_op_ref(&self) -> Option<&dyn Operator> {
        Some(self as &dyn Operator)
    }

    fn as_op_mut(&mut self) -> Option<&mut dyn Operator> {
        Some(self as &mut dyn Operator)
    }

    fn get_result(&self, _mem: &Memory, _ext: Option<&mut Extension>) -> CRes {
        CRes::Err(CErr::unparsed_token(self))
    }
}

impl Operator for OpDiv {
    fn priority(&self) -> i32 {
        self.priority
    }

    fn set_prior_as_exp(&mut self) {
        self.priority = 4;
    }

    fn symbol(&self) -> char {
        '/'
    }

    fn op(&self, val_a: Im, val_b: Im) -> CRes {
        if val_b == Im::new(0.0, 0.0) {
            return CRes::Err(CErr::div_by_zero());
        }
        // TODO: overflow protection here
        CRes::Val(val_a / val_b)
    }

    fn dual_var(&self, var_a: String, var_b: String) -> CRes {
        if var_a != var_b {
            return CRes::Err(CErr::too_many_unknown());
        }
        return CRes::Val(Im::new(1.0, 0.0));
    }

    fn fus_eq(&self, id_a: String, id_b: String, eq_a: Equ, eq_b: Equ) -> CRes {
        if id_a != id_b {
            return CRes::Err(CErr::too_many_unknown());
        }
        let mut res: Equ = HashMap::new();
        let zero = Im::new(0.0, 0.0);

        for (pow_a, coef_a) in eq_a.iter() {
            for (pow_b, coef_b) in eq_b.iter() {
                let mut pow = *pow_a;
                let mut coef = *coef_a;
                if *coef_b != zero {
                    pow = pow - *pow_b;
                    coef = coef / *coef_b;
                }
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

    fn var_eq(&self, id_var: String, id_eq: String, eq: Equ, eq_left: bool) -> CRes {
        if id_var != id_eq {
            return CRes::Err(CErr::too_many_unknown());
        }
        let mut res: Equ = HashMap::new();
        if eq_left {
            for (pow, coef) in eq.into_iter() {
                res.insert(pow - 1, coef);
            }
            return CRes::Equ(id_var, res);
        }
        let zero = Im::new(0.0, 0.0);
        for (pow, coef) in eq.into_iter() {
            if coef != zero {
                res.insert(1 - pow, zero - coef);
            }
        }
        if res.len() == 0 {
            return CRes::Err(CErr::div_by_zero());
        }
        return CRes::Equ(id_var, res);
    }

    fn val_eq(&self, val: Im, id: String, mut eq: Equ, eq_left: bool) -> CRes {
        if eq_left {
            if val == Im::new(0.0, 0.0) {
                return CRes::Err(CErr::div_by_zero());
            }
            for (_, coef) in eq.iter_mut() {
                *coef = *coef / val;
            }
            return CRes::Equ(id, eq);
        }
        let mut res: Equ = HashMap::new();
        let zero = Im::new(0.0, 0.0);
        for (pow, coef) in eq.into_iter() {
            if coef != zero {
                res.insert(0 - pow, val / coef);
            }
        }
        if res.len() == 0 {
            return CRes::Err(CErr::div_by_zero());
        }
        return CRes::Equ(id, res);
    }

    fn new_eq(&self, var: String, val: Im, var_left: bool) -> CRes {
        if  var_left && val == Im::new(0.0, 0.0) {
            return CRes::Err(CErr::div_by_zero());
        }
        let mut eq: Equ = HashMap::new();
        if var_left {
            eq.insert(1, Im::new(1.0, 0.0) / val);
        } else {
            eq.insert(-1, val);
        }
        return CRes::Equ(var, eq);
    }
}

struct OpPow {
    priority: i32
}

impl OpPow {
    pub fn new() ->  Self {
        OpPow { priority: 3 }
    }
}

impl fmt::Display for OpPow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "^")
    }
}

impl fmt::Debug for OpPow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[op:^]")
    }
}

impl Token for OpPow {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_op_ref(&self) -> Option<&dyn Operator> {
        Some(self as &dyn Operator)
    }

    fn as_op_mut(&mut self) -> Option<&mut dyn Operator> {
        Some(self as &mut dyn Operator)
    }

    fn get_result(&self, _mem: &Memory, _ext: Option<&mut Extension>) -> CRes {
        CRes::Err(CErr::unparsed_token(self))
    }
}

impl Operator for OpPow {
    fn priority(&self) -> i32 {
        self.priority
    }

    fn set_prior_as_exp(&mut self) {
        self.priority = 4;
    }

    fn symbol(&self) -> char {
        '^'
    }

    fn op(&self, val_a: Im, val_b: Im) -> CRes {
        if !val_b.is_real() || !val_b.is_int() {
            return CRes::Err(CErr::bad_pow());
        }
        let res = val_a.pow(val_b.get_real());
        return CRes::Val(res);
    }

    fn dual_var(&self, _: String, _: String) -> CRes {
        CRes::Err(CErr::bad_pow())
    }

    fn fus_eq(
        &self,
        id_a: String,
        id_b: String,
        _: Equ,
        _: Equ,
    ) -> CRes {
        if id_a != id_b {
            return CRes::Err(CErr::too_many_unknown());
        }
        CRes::Err(CErr::bad_pow())
    }

    fn var_eq(&self, id_var: String, id_eq: String, _: Equ, _: bool) -> CRes {
        if id_var != id_eq {
            return CRes::Err(CErr::too_many_unknown());
        }
        CRes::Err(CErr::bad_pow())
    }

    fn val_eq(&self, val: Im, id: String, eq: Equ, eq_left: bool) -> CRes {
        if !eq_left || !val.is_real() || !val.is_int() {
            return CRes::Err(CErr::bad_pow());
        }
        let mut res: Equ = HashMap::new();
        let power = val.get_real() as i32;
        
        run_power(power, (0, Im::new(1.0, 0.0)), &mut res, &eq);
        return CRes::Equ(id, res);
    }

    fn new_eq(&self, var: String, val: Im, var_left: bool) -> CRes {
        if !var_left || !val.is_real() || !val.is_int() {
            return CRes::Err(CErr::bad_pow());
        }
        let mut eq: Equ = HashMap::new();
        eq.insert(val.get_real() as i32, Im::new(1.0, 0.0));
        return CRes::Equ(var, eq);
    }
}

fn run_power(i: i32, field: (i32, Im), res: &mut Equ, eq: &Equ) {
    if i == 0 {
        match res.get_mut(&field.0) {
            None => {
                res.insert(field.0, field.1);
            }
            Some(prev_coef) => *prev_coef = *prev_coef + field.1,
        }
    }
    for (old_pow, old_coef) in eq.iter() {
        run_power(i - 1, (field.0 + *old_pow, field.1 * *old_coef), res, eq);
    }
}

fn none_on_left(right: CRes, op: char) -> CRes {
    let zero = Im::new(0.0, 0.0);
    match op {
        '-' => match right {
            CRes::Val(val) => CRes::Val(zero - val),
            CRes::VarCall(_, val) => CRes::Val(zero - val),
            CRes::Equ(id, eq) => {
                let mut res = eq;
                for (_, coef) in res.iter_mut() {
                    *coef = zero - *coef;
                }
                CRes::Equ(id, res)
            }
            _ => CRes::Err(CErr::bad_use_op(op))
        },
        '+' => right,
        _ => CRes::Err(CErr::bad_use_op(op))
    }
}