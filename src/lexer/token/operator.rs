/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/20 13:09:24 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::ComputorResult;
use crate::memory::Memory;
use std::any::Any;
use std::fmt;

#[derive(Clone)]
pub struct Operator {
    symbol: char,
    priority: i32,
    // op: fn(
    //     &Self,
    //     &Token,
    //     &Token,
    //     bool,
    // ) -> Result<(Vec<Rc<Token>>), ComputorError>,
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

    fn get_result(&self, _mem: &Memory) -> ComputorResult {
        panic!("Operator left behind by Parser: {:?}", self)
    }
}

impl Operator {
    pub fn new(symbol: char) -> Result<Self, LexerError> {
        let priority: i32;
        match symbol {
            '=' => priority = 0,
            '+' | '-' => priority = 1,
            '*' | '/' => priority = 2,
            _ => return Err(LexerError::InvalidOp(symbol)),
        };
        Ok(Operator {
            symbol: symbol,
            priority: priority,
            // op: op,
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
        orand_l: ComputorResult,
        orand_r: ComputorResult,
    ) -> ComputorResult {
        match (orand_l, orand_r) {
            _ => ComputorResult::default(),
        }
    }
}
