/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/20 10:29:21 by gsmith           ###   ########.fr       */
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
        _orand_l: ComputorResult,
        _orand_r: ComputorResult,
    ) -> ComputorResult {
        ComputorResult::default()
    }
}

// impl Operator {
//     pub fn new(symbol: char) -> Result<Operator, ComputorError> {
//         let op = match symbol {
//             '+' => Operator::add,
//             '-' => Operator::sub,
//             '*' => Operator::mul,
//             '/' => Operator::div,
//             _ => return Err(ComputorError::invalid_operator(symbol)),
//         };
//         Ok(Operator {
//             symbol: symbol,
//             op: op,
//         })
//     }

//     pub fn exec(
//         &self,
//         val_a: &Token,
//         val_b: &Token,
//         verbose: bool,
//     ) -> Result<(LList<Token>), ComputorError> {
//         (self.op)(self, val_a, val_b, verbose)
//     }

//     pub fn prior(&self) -> bool {
//         self.symbol == '*' || self.symbol == '/' || self.symbol == '%'
//     }

//     fn add(
//         &self,
//         val_a: &Token,
//         val_b: &Token,
//         verbose: bool,
//     ) -> Result<(LList<Token>), ComputorError> {
//         let mut result: LList<Token> = LList::new();
//         match (val_a, val_b) {
//             (Token::Val(op_a), Token::Val(op_b)) => {
//                 result.push_back(Value::add_val(op_a, op_b))
//             }
//             (Token::Val(op_a), Token::Expr(ep_b)) => {
//                 result.push_back(self.with_expr(op_a, ep_b, true, verbose)?)
//             }
//             (Token::Expr(ep_a), Token::Val(op_b)) => {
//                 result.push_back(self.with_expr(op_b, ep_a, false, verbose)?)
//             }
//             _ => return Err(ComputorError::bad_use_op('+')),
//         };
//         return Ok(result);
//     }

//     fn sub(
//         &self,
//         val_a: &Token,
//         val_b: &Token,
//         verbose: bool,
//     ) -> Result<(LList<Token>), ComputorError> {
//         let mut result: LList<Token> = LList::new();
//         match (val_a, val_b) {
//             (Token::Val(op_a), Token::Val(op_b)) => {
//                 result.push_back(Value::sub_val(op_a, op_b))
//             }
//             (Token::Val(op_a), Token::Expr(ep_b)) => {
//                 result.push_back(self.with_expr(op_a, ep_b, true, verbose)?)
//             }
//             (Token::Expr(ep_a), Token::Val(op_b)) => {
//                 result.push_back(self.with_expr(op_b, ep_a, false, verbose)?)
//             }
//             _ => return Err(ComputorError::bad_use_op('-')),
//         };
//         return Ok(result);
//     }

//     fn mul(
//         &self,
//         val_a: &Token,
//         val_b: &Token,
//         verbose: bool,
//     ) -> Result<(LList<Token>), ComputorError> {
//         let mut result: LList<Token> = LList::new();
//         match (val_a, val_b) {
//             (Token::Val(op_a), Token::Val(op_b)) => {
//                 result.push_back(Value::mul_val(op_a, op_b))
//             }
//             (Token::Val(op_a), Token::Expr(ep_b)) => {
//                 result.push_back(self.with_expr(op_a, ep_b, true, verbose)?)
//             }
//             (Token::Expr(ep_a), Token::Val(op_b)) => {
//                 result.push_back(self.with_expr(op_b, ep_a, false, verbose)?)
//             }
//             _ => return Err(ComputorError::bad_use_op('*')),
//         };
//         return Ok(result);
//     }

//     fn div(
//         &self,
//         val_a: &Token,
//         val_b: &Token,
//         verbose: bool,
//     ) -> Result<(LList<Token>), ComputorError> {
//         let mut result: LList<Token> = LList::new();
//         match (val_a, val_b) {
//             (Token::Val(op_a), Token::Val(op_b)) => {
//                 result.push_back(Value::div_val(op_a, op_b)?)
//             }
//             // (Token::Val(op_a), Token::Expr(ep_b)) => {
//             //     result.push_back(self.with_expr(op_a, ep_b, true, verbose)?)
//             // }
//             // (Token::Expr(ep_a), Token::Val(op_b)) => {
//             //     result.push_back(self.with_expr(op_b, ep_a, false, verbose)?)
//             // }
//             _ => return Err(ComputorError::bad_use_op('/')),
//         };
//         return Ok(result);
//     }

//     fn with_expr(
//         &self,
//         op_tok: &Value,
//         exp_tok: &Expression,
//         exp_right: bool,
//         verbose: bool,
//     ) -> Result<Token, ComputorError> {
//         let exp = exp_tok.compute(verbose)?;
//         let val = Token::Val(op_tok.clone());
//         if exp.len() == 1 {
//             match exp.front() {
//                 Some(exp_tok) => {
//                     let exp = if exp_right {
//                         self.exec(&val, exp_tok, verbose)?
//                     } else {
//                         self.exec(exp_tok, &val, verbose)?
//                     };
//                     if exp.len() == 1 {
//                         match exp.front() {
//                             Some(tok) => Ok(tok.clone()),
//                             None => Ok(val),
//                         }
//                     } else {
//                         Ok(Token::Expr(Expression::new(exp)))
//                     }
//                 }
//                 None => Ok(val),
//             }
//         } else {
//             let mut new_list: LList<Token> = LList::new();
//             new_list.push_back(val);
//             new_list.push_back(Token::Orator(self.clone()));
//             new_list.push_back(Token::Expr(exp));
//             Ok(Token::Expr(Expression::new(new_list)))
//         }
//     }
// }
