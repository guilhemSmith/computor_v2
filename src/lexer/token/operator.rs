/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/09 11:58:57 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{ComputorError as CErr, ComputorResult as CRes};
use crate::memory::{Extension, Memory};

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

    fn mul(&self, orand_l: CRes, orand_r: CRes) -> CRes {
        CRes::default()
    }

    fn div(&self, orand_l: CRes, orand_r: CRes) -> CRes {
        CRes::default()
    }

    fn add(&self, orand_l: CRes, orand_r: CRes) -> CRes {
        CRes::default()
    }

    fn sub(&self, orand_l: CRes, orand_r: CRes) -> CRes {
        CRes::default()
    }

    fn pow(&self, orand_l: CRes, orand_r: CRes) -> CRes {
        CRes::default()
    }
}
