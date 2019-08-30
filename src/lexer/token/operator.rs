/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/30 12:24:25 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{ComputorError as CError, ComputorResult as CResult};
use crate::memory::{Extension, Memory};
use crate::types::Imaginary;

use std::any::Any;
use std::fmt;

#[derive(Clone)]
pub struct Operator {
    symbol: char,
    priority: i32,
    op: fn(&Self, CResult, CResult) -> CResult,
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

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> CResult {
        CResult::Err(CError::unparsed_token(self))
    }
}

impl Operator {
    pub fn new(symbol: char) -> Result<Self, LexerError> {
        let priority: i32;
        let op: fn(&Self, CResult, CResult) -> CResult;
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
        self.priority = 3;
    }

    pub fn exec(
        &self,
        _mem: &Memory,
        orand_l: CResult,
        orand_r: CResult,
    ) -> CResult {
        (self.op)(self, orand_l, orand_r)
    }

    fn equal(&self, orand_l: CResult, orand_r: CResult) -> CResult {
        match (orand_l, orand_r) {
            (CResult::Err(err), _) => CResult::Err(err),
            (_, CResult::Err(err)) => CResult::Err(err),
            (CResult::None, _) => CResult::Err(CError::bad_use_op(self.symbol)),
            (_, CResult::None) => CResult::Err(CError::bad_use_op(self.symbol)),
            (CResult::Val(val), CResult::Res) => CResult::Val(val),
            (CResult::Var(var, coef, pow), CResult::Res) => {
                CResult::Var(var, coef, pow)
            }
            (CResult::Var(var, coef, pow), CResult::Val(val)) => {
                let one = Imaginary::new(1.0, 0.0);
                if pow == one {
                    if coef == one {
                        CResult::Set(var, val)
                    } else {
                        CResult::default()
                    }
                } else {
                    CResult::default()
                }
            }
            (_, _) => CResult::default(),
        }
    }

    fn mul(&self, orand_l: CResult, orand_r: CResult) -> CResult {
        CResult::default()
    }

    fn div(&self, orand_l: CResult, orand_r: CResult) -> CResult {
        CResult::default()
    }

    fn add(&self, orand_l: CResult, orand_r: CResult) -> CResult {
        CResult::default()
    }

    fn sub(&self, orand_l: CResult, orand_r: CResult) -> CResult {
        CResult::default()
    }
}
