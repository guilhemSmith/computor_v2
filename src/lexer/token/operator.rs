/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/25 11:13:41 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{
    filter_eq, Computed as Comp, ComputorError as CErr, TreeResult,
};
use crate::memory::{Extension, Memory};
use crate::types::{Imaginary as Im, Matrix};

use std::any::Any;
use std::collections::HashMap;
use std::fmt;

type Equ = HashMap<i32, Im>;

pub trait Operator: Token + fmt::Display {
    fn priority(&self) -> i32;
    fn is_prior(&self, other: &dyn Operator) -> bool {
        self.priority() >= other.priority()
    }
    fn set_prior_as_exp(&mut self);
    fn symbol(&self) -> char;
    fn dual_var(&self, var_a: String, var_b: String) -> TreeResult;
    fn dual_mat(&self, mat_a: Matrix, mat_b: Matrix) -> TreeResult;
    fn new_eq(&self, var: String, val: Im, var_left: bool) -> TreeResult;
    fn op(&self, val_a: Im, val_b: Im) -> TreeResult;
    fn op_mat(&self, mat: Matrix, val: Im) -> TreeResult;
    fn fus_eq(
        &self,
        id_a: String,
        id_b: String,
        eq_a: Equ,
        eq_b: Equ,
    ) -> TreeResult;
    fn var_eq(
        &self,
        id_var: String,
        id_eq: String,
        eq: Equ,
        eq_left: bool,
    ) -> TreeResult;
    fn val_eq(&self, val: Im, id: String, eq: Equ, eq_left: bool)
        -> TreeResult;
    fn exec(&self, _mem: &Memory, left: Comp, right: Comp) -> TreeResult {
        match (left, right) {
            (Comp::Res, _) => Err(CErr::bad_resolve()),
            (_, Comp::Res) => Err(CErr::bad_resolve()),
            (Comp::FunSet(id, _), _) => Err(CErr::fun_undef(&id)),
            (_, Comp::FunSet(id, _)) => Err(CErr::fun_undef(&id)),
            (Comp::None, right) => none_on_left(right, self.symbol()),
            (_, Comp::None) => Err(CErr::bad_use_op(self.symbol())),
            (Comp::Mat(mat_a), Comp::Mat(mat_b)) => self.dual_mat(mat_a, mat_b),
            (Comp::Val(val), Comp::Mat(mat)) => self.op_mat(mat, val),
            (Comp::VarCall(_, val), Comp::Mat(mat)) => self.op_mat(mat, val),
            (Comp::Mat(mat), Comp::Val(val)) => self.op_mat(mat, val),
            (Comp::Mat(mat), Comp::VarCall(_, val)) => self.op_mat(mat, val),
            (Comp::Mat(_), Comp::VarSet(_)) => {
                Err(CErr::bad_use_op(self.symbol()))
            }
            (Comp::VarSet(_), Comp::Mat(_)) => {
                Err(CErr::bad_use_op(self.symbol()))
            }
            (Comp::Mat(_), Comp::Equ(_, _)) => {
                Err(CErr::bad_use_op(self.symbol()))
            }
            (Comp::Equ(_, _), Comp::Mat(_)) => {
                Err(CErr::bad_use_op(self.symbol()))
            }
            (Comp::VarSet(v_a), Comp::VarSet(v_b)) => self.dual_var(v_a, v_b),
            (Comp::VarSet(var), Comp::VarCall(_, val)) => {
                self.new_eq(var, val, true)
            }
            (Comp::VarCall(_, val), Comp::VarSet(var)) => {
                self.new_eq(var, val, false)
            }
            (Comp::VarSet(var), Comp::Val(val)) => self.new_eq(var, val, true),
            (Comp::Val(val), Comp::VarSet(var)) => self.new_eq(var, val, false),
            (Comp::Val(v_a), Comp::Val(v_b)) => self.op(v_a, v_b),
            (Comp::Val(v_a), Comp::VarCall(_, v_b)) => self.op(v_a, v_b),
            (Comp::VarCall(_, v_a), Comp::Val(v_b)) => self.op(v_a, v_b),
            (Comp::VarCall(_, v_a), Comp::VarCall(_, v_b)) => self.op(v_a, v_b),
            (Comp::Equ(id_a, eq_a), Comp::Equ(id_b, eq_b)) => {
                self.fus_eq(id_a, id_b, eq_a, eq_b)
            }
            (Comp::Equ(id_eq, eq), Comp::VarSet(id_v)) => {
                self.var_eq(id_v, id_eq, eq, true)
            }
            (Comp::VarSet(id_v), Comp::Equ(id_eq, eq)) => {
                self.var_eq(id_v, id_eq, eq, false)
            }
            (Comp::Equ(id, eq), Comp::Val(v)) => self.val_eq(v, id, eq, true),
            (Comp::Val(v), Comp::Equ(id, eq)) => self.val_eq(v, id, eq, false),
            (Comp::Equ(id, eq), Comp::VarCall(_, v)) => {
                self.val_eq(v, id, eq, true)
            }
            (Comp::VarCall(_, v), Comp::Equ(id, eq)) => {
                self.val_eq(v, id, eq, false)
            }
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
        '%' => Ok(Box::new(OpMod::new())),
        '^' => Ok(Box::new(OpPow::new())),
        _ => Err(LexerError::InvalidOp(symbol)),
    }
}

struct OpEqual {
    priority: i32,
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

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> TreeResult {
        Err(CErr::unparsed_token(self))
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

    fn op(&self, _: Im, _: Im) -> TreeResult {
        Err(CErr::too_many_equal())
    }

    fn dual_var(&self, _: String, _: String) -> TreeResult {
        Err(CErr::too_many_equal())
    }

    fn dual_mat(&self, _: Matrix, _: Matrix) -> TreeResult {
        Err(CErr::too_many_equal())
    }

    fn op_mat(&self, _: Matrix, _: Im) -> TreeResult {
        Err(CErr::too_many_equal())
    }

    fn fus_eq(&self, _: String, _: String, _: Equ, _: Equ) -> TreeResult {
        Err(CErr::too_many_equal())
    }

    fn var_eq(&self, _: String, _: String, _: Equ, _: bool) -> TreeResult {
        Err(CErr::too_many_equal())
    }

    fn val_eq(&self, _: Im, _: String, _: Equ, _: bool) -> TreeResult {
        Err(CErr::too_many_equal())
    }

    fn new_eq(&self, _: String, _: Im, _: bool) -> TreeResult {
        Err(CErr::too_many_equal())
    }

    fn exec(&self, _: &Memory, _: Comp, _: Comp) -> TreeResult {
        Err(CErr::too_many_equal())
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

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> TreeResult {
        Err(CErr::unparsed_token(self))
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

    fn op(&self, val_a: Im, val_b: Im) -> TreeResult {
        match val_a.mul(&val_b) {
            Ok(res) => Ok(Comp::Val(res)),
            Err(err) => Err(err),
        }
    }

    fn dual_var(&self, var_a: String, var_b: String) -> TreeResult {
        if var_a != var_b {
            return Err(CErr::too_many_unknown());
        }
        let mut eq: Equ = HashMap::new();
        eq.insert(2, Im::new(1.0, 0.0));
        return Ok(Comp::Equ(var_a, eq));
    }

    fn dual_mat(&self, _: Matrix, _: Matrix) -> TreeResult {
        Err(CErr::op_matrix('*'))
    }

    fn op_mat(&self, mut mat: Matrix, val: Im) -> TreeResult {
        match mat.apply_mul(val) {
            Ok(n_mat) => Ok(Comp::Mat(n_mat)),
            Err(err) => Err(err),
        }
    }

    fn fus_eq(
        &self,
        id_a: String,
        id_b: String,
        eq_a: Equ,
        eq_b: Equ,
    ) -> TreeResult {
        if id_a != id_b {
            return Err(CErr::too_many_unknown());
        }
        let mut res: Equ = HashMap::new();

        for (pow_a, coef_a) in eq_a.iter() {
            for (pow_b, coef_b) in eq_b.iter() {
                let pow = pow_a + pow_b;
                let coef = coef_a.mul(coef_b)?;
                match res.get_mut(&pow) {
                    None => {
                        res.insert(pow, coef);
                    }
                    Some(prev_coef) => *prev_coef = prev_coef.add(&coef)?,
                }
            }
        }
        return Ok(Comp::Equ(id_a, res));
    }

    fn var_eq(
        &self,
        id_var: String,
        id_eq: String,
        eq: Equ,
        _: bool,
    ) -> TreeResult {
        if id_var != id_eq {
            return Err(CErr::too_many_unknown());
        }
        let mut res: Equ = HashMap::new();

        for (pow, coef) in eq.into_iter() {
            res.insert(pow + 1, coef);
        }
        return Ok(Comp::Equ(id_var, res));
    }

    fn val_eq(&self, val: Im, id: String, mut eq: Equ, _: bool) -> TreeResult {
        for (_, coef) in eq.iter_mut() {
            *coef = coef.mul(&val)?;
        }
        return Ok(Comp::Equ(id, eq));
    }

    fn new_eq(&self, var: String, val: Im, _: bool) -> TreeResult {
        let mut eq: Equ = HashMap::new();
        eq.insert(1, val);
        return Ok(Comp::Equ(var, eq));
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

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> TreeResult {
        Err(CErr::unparsed_token(self))
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

    fn op(&self, val_a: Im, val_b: Im) -> TreeResult {
        match val_a.add(&val_b) {
            Ok(res) => Ok(Comp::Val(res)),
            Err(err) => Err(err),
        }
    }

    fn dual_var(&self, var_a: String, var_b: String) -> TreeResult {
        if var_a != var_b {
            return Err(CErr::too_many_unknown());
        }
        let mut eq: Equ = HashMap::new();
        eq.insert(1, Im::new(2.0, 0.0));
        return Ok(Comp::Equ(var_a, eq));
    }

    fn dual_mat(&self, mat_a: Matrix, mat_b: Matrix) -> TreeResult {
        Err(CErr::op_matrix('+'))
    }

    fn op_mat(&self, _: Matrix, _: Im) -> TreeResult {
        Err(CErr::op_matrix('+'))
    }

    fn fus_eq(
        &self,
        id_a: String,
        id_b: String,
        mut eq_a: Equ,
        eq_b: Equ,
    ) -> TreeResult {
        if id_a != id_b {
            return Err(CErr::too_many_unknown());
        }

        for (pow_b, coef_b) in eq_b.into_iter() {
            match eq_a.get_mut(&pow_b) {
                None => {
                    eq_a.insert(pow_b, coef_b);
                }
                Some(coef_a) => *coef_a = coef_a.add(&coef_b)?,
            }
        }
        return Ok(Comp::Equ(id_a, eq_a));
    }

    fn var_eq(
        &self,
        id_var: String,
        id_eq: String,
        mut eq: Equ,
        _: bool,
    ) -> TreeResult {
        if id_var != id_eq {
            return Err(CErr::too_many_unknown());
        }
        let one: i32 = 1;
        let one_im = Im::new(1.0, 0.0);

        match eq.get_mut(&one) {
            None => {
                eq.insert(one, one_im);
            }
            Some(coef) => *coef = coef.add(&one_im)?,
        };
        return Ok(Comp::Equ(id_var, eq));
    }

    fn val_eq(&self, val: Im, id: String, mut eq: Equ, _: bool) -> TreeResult {
        let zero: i32 = 0;
        match eq.get_mut(&zero) {
            None => {
                eq.insert(zero, val);
            }
            Some(coef) => *coef = coef.add(&val)?,
        }
        return Ok(Comp::Equ(id, eq));
    }

    fn new_eq(&self, var: String, val: Im, _: bool) -> TreeResult {
        let mut eq: Equ = HashMap::new();
        eq.insert(1, Im::new(1.0, 0.0));
        eq.insert(0, val);
        return Ok(Comp::Equ(var, eq));
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

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> TreeResult {
        Err(CErr::unparsed_token(self))
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

    fn op(&self, val_a: Im, val_b: Im) -> TreeResult {
        match val_a.sub(&val_b) {
            Ok(res) => Ok(Comp::Val(res)),
            Err(err) => Err(err),
        }
    }

    fn dual_mat(&self, mat_a: Matrix, mat_b: Matrix) -> TreeResult {
        Err(CErr::op_matrix('-'))
    }

    fn op_mat(&self, _: Matrix, _: Im) -> TreeResult {
        Err(CErr::op_matrix('-'))
    }

    fn dual_var(&self, var_a: String, var_b: String) -> TreeResult {
        if var_a != var_b {
            return Err(CErr::too_many_unknown());
        }
        return Ok(Comp::Val(Im::new(0.0, 0.0)));
    }

    fn fus_eq(
        &self,
        id_a: String,
        id_b: String,
        mut eq_a: Equ,
        eq_b: Equ,
    ) -> TreeResult {
        if id_a != id_b {
            return Err(CErr::too_many_unknown());
        }

        for (pow_b, coef_b) in eq_b.into_iter() {
            match eq_a.get_mut(&pow_b) {
                None => {
                    eq_a.insert(pow_b, coef_b);
                }
                Some(coef_a) => *coef_a = coef_a.sub(&coef_b)?,
            }
        }
        return Ok(Comp::Equ(id_a, eq_a));
    }

    fn var_eq(
        &self,
        id_var: String,
        id_eq: String,
        mut eq: Equ,
        eq_left: bool,
    ) -> TreeResult {
        if id_var != id_eq {
            return Err(CErr::too_many_unknown());
        }
        let one: i32 = 1;
        let one_im = Im::new(1.0, 0.0);

        match eq.get_mut(&one) {
            None => {
                eq.insert(one, one_im);
            }
            Some(coef) => {
                *coef = if eq_left {
                    coef.sub(&one_im)?
                } else {
                    one_im.sub(coef)?
                }
            }
        };
        return Ok(Comp::Equ(id_var, eq));
    }

    fn val_eq(
        &self,
        val: Im,
        id: String,
        mut eq: Equ,
        eq_left: bool,
    ) -> TreeResult {
        let zero: i32 = 0;
        match eq.get_mut(&zero) {
            None => {
                eq.insert(zero, -val);
            }
            Some(coef) => {
                *coef = if eq_left {
                    coef.sub(&val)?
                } else {
                    val.sub(coef)?
                }
            }
        }
        return Ok(Comp::Equ(id, eq));
    }

    fn new_eq(&self, var: String, val: Im, var_left: bool) -> TreeResult {
        let mut eq: Equ = HashMap::new();
        if var_left {
            eq.insert(1, Im::new(1.0, 0.0));
            eq.insert(0, -val);
        } else {
            eq.insert(1, Im::new(-1.0, 0.0));
            eq.insert(0, val);
        }
        return Ok(Comp::Equ(var, eq));
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

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> TreeResult {
        Err(CErr::unparsed_token(self))
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

    fn op(&self, val_a: Im, val_b: Im) -> TreeResult {
        if val_b == Im::new(0.0, 0.0) {
            return Err(CErr::div_by_zero());
        }
        match val_a.div(&val_b) {
            Ok(res) => Ok(Comp::Val(res)),
            Err(err) => Err(err),
        }
    }

    fn dual_var(&self, var_a: String, var_b: String) -> TreeResult {
        if var_a != var_b {
            return Err(CErr::too_many_unknown());
        }
        return Ok(Comp::Val(Im::new(1.0, 0.0)));
    }

    fn dual_mat(&self, _: Matrix, _: Matrix) -> TreeResult {
        Err(CErr::op_matrix('/'))
    }

    fn op_mat(&self, _: Matrix, _: Im) -> TreeResult {
        Err(CErr::op_matrix('/'))
    }

    fn fus_eq(
        &self,
        id_a: String,
        id_b: String,
        eq_a: Equ,
        mut eq_b: Equ,
    ) -> TreeResult {
        if id_a != id_b {
            return Err(CErr::too_many_unknown());
        }
        filter_eq(&mut eq_b);
        if eq_b.len() > 1 {
            return Err(CErr::div_by_eq());
        }
        let mut res: Equ = HashMap::new();
        let zero = Im::new(0.0, 0.0);

        for (pow_a, coef_a) in eq_a.iter() {
            for (pow_b, coef_b) in eq_b.iter() {
                let mut pow = *pow_a;
                let mut coef = *coef_a;
                if *coef_b != zero {
                    pow = pow - *pow_b;
                    coef = coef.div(coef_b)?;
                }
                match res.get_mut(&pow) {
                    None => {
                        res.insert(pow, coef);
                    }
                    Some(prev_coef) => *prev_coef = prev_coef.add(&coef)?,
                }
            }
        }
        return Ok(Comp::Equ(id_a, res));
    }

    fn var_eq(
        &self,
        id_var: String,
        id_eq: String,
        eq: Equ,
        eq_left: bool,
    ) -> TreeResult {
        if id_var != id_eq {
            return Err(CErr::too_many_unknown());
        }
        let mut res: Equ = HashMap::new();
        if eq_left {
            for (pow, coef) in eq.into_iter() {
                res.insert(pow - 1, coef);
            }
            return Ok(Comp::Equ(id_var, res));
        }
        let zero = Im::new(0.0, 0.0);
        for (pow, coef) in eq.into_iter() {
            if coef != zero {
                res.insert(1 - pow, -coef);
            }
        }
        if res.len() == 0 {
            return Err(CErr::div_by_zero());
        }
        return Ok(Comp::Equ(id_var, res));
    }

    fn val_eq(
        &self,
        val: Im,
        id: String,
        mut eq: Equ,
        eq_left: bool,
    ) -> TreeResult {
        if eq_left {
            if val == Im::new(0.0, 0.0) {
                return Err(CErr::div_by_zero());
            }
            for (_, coef) in eq.iter_mut() {
                *coef = coef.div(&val)?;
            }
            return Ok(Comp::Equ(id, eq));
        }
        let mut res: Equ = HashMap::new();
        let zero = Im::new(0.0, 0.0);
        for (pow, coef) in eq.into_iter() {
            if coef != zero {
                res.insert(0 - pow, val.div(&coef)?);
            }
        }
        if res.len() == 0 {
            return Err(CErr::div_by_zero());
        }
        return Ok(Comp::Equ(id, res));
    }

    fn new_eq(&self, var: String, val: Im, var_left: bool) -> TreeResult {
        if var_left && val == Im::new(0.0, 0.0) {
            return Err(CErr::div_by_zero());
        }
        let mut eq: Equ = HashMap::new();
        if var_left {
            eq.insert(1, Im::new(1.0, 0.0).div(&val)?);
        } else {
            eq.insert(-1, val);
        }
        return Ok(Comp::Equ(var, eq));
    }
}

struct OpMod {
    priority: i32,
}

impl OpMod {
    pub fn new() -> Self {
        OpMod { priority: 2 }
    }
}

impl fmt::Display for OpMod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%")
    }
}

impl fmt::Debug for OpMod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[op:%]")
    }
}

impl Token for OpMod {
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

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> TreeResult {
        Err(CErr::unparsed_token(self))
    }
}

impl Operator for OpMod {
    fn priority(&self) -> i32 {
        self.priority
    }

    fn set_prior_as_exp(&mut self) {
        self.priority = 4;
    }

    fn symbol(&self) -> char {
        '%'
    }

    fn op(&self, val_a: Im, val_b: Im) -> TreeResult {
        if val_b == Im::new(0.0, 0.0) {
            return Err(CErr::div_by_zero());
        }
        match val_a.rem(&val_b) {
            Ok(res) => Ok(Comp::Val(res)),
            Err(err) => Err(err),
        }
    }

    fn dual_var(&self, var_a: String, var_b: String) -> TreeResult {
        if var_a != var_b {
            return Err(CErr::too_many_unknown());
        }
        return Ok(Comp::Val(Im::new(0.0, 0.0)));
    }

    fn dual_mat(&self, _: Matrix, _: Matrix) -> TreeResult {
        Err(CErr::op_matrix('%'))
    }

    fn op_mat(&self, _: Matrix, _: Im) -> TreeResult {
        Err(CErr::op_matrix('%'))
    }

    fn fus_eq(&self, id_a: String, id_b: String, _: Equ, _: Equ) -> TreeResult {
        if id_a != id_b {
            return Err(CErr::too_many_unknown());
        }
        return Err(CErr::mod_with_unk());
    }

    fn var_eq(
        &self,
        id_var: String,
        id_eq: String,
        _: Equ,
        _: bool,
    ) -> TreeResult {
        if id_var != id_eq {
            return Err(CErr::too_many_unknown());
        }
        return Err(CErr::mod_with_unk());
    }

    fn val_eq(&self, val: Im, _: String, _: Equ, eq_left: bool) -> TreeResult {
        if eq_left && val == Im::new(0.0, 0.0) {
            return Err(CErr::div_by_zero());
        }
        return Err(CErr::mod_with_unk());
    }

    fn new_eq(&self, _: String, val: Im, var_left: bool) -> TreeResult {
        if var_left && val == Im::new(0.0, 0.0) {
            return Err(CErr::div_by_zero());
        }
        return Err(CErr::mod_with_unk());
    }
}

struct OpPow {
    priority: i32,
}

impl OpPow {
    pub fn new() -> Self {
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

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> TreeResult {
        Err(CErr::unparsed_token(self))
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

    fn op(&self, val_a: Im, val_b: Im) -> TreeResult {
        if !val_b.is_real() || !val_b.is_int() {
            return Err(CErr::bad_pow());
        }
        let power = val_b.get_real().get_val() as i32;
        match val_a.pow(power) {
            Ok(res) => Ok(Comp::Val(res)),
            Err(err) => Err(err),
        }
    }

    fn dual_var(&self, _: String, _: String) -> TreeResult {
        Err(CErr::bad_pow())
    }

    fn dual_mat(&self, _: Matrix, _: Matrix) -> TreeResult {
        Err(CErr::op_matrix('^'))
    }

    fn op_mat(&self, _: Matrix, _: Im) -> TreeResult {
        Err(CErr::op_matrix('^'))
    }

    fn fus_eq(&self, id_a: String, id_b: String, _: Equ, _: Equ) -> TreeResult {
        if id_a != id_b {
            return Err(CErr::too_many_unknown());
        }
        Err(CErr::bad_pow())
    }

    fn var_eq(
        &self,
        id_var: String,
        id_eq: String,
        _: Equ,
        _: bool,
    ) -> TreeResult {
        if id_var != id_eq {
            return Err(CErr::too_many_unknown());
        }
        Err(CErr::bad_pow())
    }

    fn val_eq(
        &self,
        val: Im,
        id: String,
        eq: Equ,
        eq_left: bool,
    ) -> TreeResult {
        if !eq_left || !val.is_real() || !val.is_int() {
            return Err(CErr::bad_pow());
        }
        let mut res: Equ = HashMap::new();
        let power = val.get_real().get_val() as i32;

        run_power(power, (0, Im::new(1.0, 0.0)), &mut res, &eq)?;
        return Ok(Comp::Equ(id, res));
    }

    fn new_eq(&self, var: String, val: Im, var_left: bool) -> TreeResult {
        if !var_left || !val.is_real() || !val.is_int() {
            return Err(CErr::bad_pow());
        }
        let mut eq: Equ = HashMap::new();
        eq.insert(val.get_real().get_val() as i32, Im::new(1.0, 0.0));
        return Ok(Comp::Equ(var, eq));
    }
}

fn run_power(
    i: i32,
    field: (i32, Im),
    res: &mut Equ,
    eq: &Equ,
) -> Result<(), CErr> {
    if i == 0 {
        match res.get_mut(&field.0) {
            None => {
                res.insert(field.0, field.1);
            }
            Some(prev_coef) => *prev_coef = prev_coef.add(&field.1)?,
        }
    }
    for (old_pow, old_coef) in eq.iter() {
        run_power(
            i - 1,
            (field.0 + *old_pow, field.1.mul(old_coef)?),
            res,
            eq,
        )?;
    }
    Ok(())
}

fn none_on_left(right: Comp, op: char) -> TreeResult {
    Ok(match op {
        '-' => match right {
            Comp::Val(val) => Comp::Val(-val),
            Comp::VarCall(_, val) => Comp::Val(-val),
            Comp::VarSet(id) => {
                let mut res: Equ = HashMap::new();
                res.insert(1, Im::new(-1.0, 0.0));
                Comp::Equ(id, res)
            }
            Comp::Equ(id, eq) => {
                let mut res = eq;
                for (_, coef) in res.iter_mut() {
                    *coef = -*coef;
                }
                Comp::Equ(id, res)
            }
            _ => return Err(CErr::bad_use_op(op)),
        },
        '+' => right,
        _ => return Err(CErr::bad_use_op(op)),
    })
}
