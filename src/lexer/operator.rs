/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operator.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:24 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/13 14:09:41 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Expression, Operand, Token};
use crate::error::ComputorError;
use std::{collections::LinkedList as LList, fmt};

#[derive(Clone)]
pub struct Operator {
    symbol: char,
    op: fn(
        &Self,
        &Token,
        &Token,
        bool,
    ) -> Result<(LList<Token>), ComputorError>,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

impl Operator {
    pub fn new(symbol: char) -> Result<Operator, ComputorError> {
        let op = match symbol {
            '+' => Operator::add,
            '-' => Operator::sub,
            '*' => Operator::mul,
            '/' => Operator::div,
            _ => return Err(ComputorError::invalid_operator(symbol)),
        };
        Ok(Operator {
            symbol: symbol,
            op: op,
        })
    }

    pub fn exec(
        &self,
        val_a: &Token,
        val_b: &Token,
        verbose: bool,
    ) -> Result<(LList<Token>), ComputorError> {
        (self.op)(self, val_a, val_b, verbose)
    }

    pub fn prior(&self) -> bool {
        self.symbol == '*' || self.symbol == '/' || self.symbol == '%'
    }

    fn add(
        &self,
        val_a: &Token,
        val_b: &Token,
        verbose: bool,
    ) -> Result<(LList<Token>), ComputorError> {
        let mut result: LList<Token> = LList::new();
        match (val_a, val_b) {
            (Token::Orand(op_a), Token::Orand(op_b)) => {
                result.push_back(Operand::add_orand(op_a, op_b))
            }
            (Token::Orand(op_a), Token::Expr(ep_b)) => {
                result.push_back(self.with_expr(op_a, ep_b, true, verbose)?)
            }
            (Token::Expr(ep_a), Token::Orand(op_b)) => {
                result.push_back(self.with_expr(op_b, ep_a, false, verbose)?)
            }
            _ => return Err(ComputorError::bad_use_op('+')),
        };
        return Ok(result);
    }

    fn sub(
        &self,
        val_a: &Token,
        val_b: &Token,
        verbose: bool,
    ) -> Result<(LList<Token>), ComputorError> {
        let mut result: LList<Token> = LList::new();
        match (val_a, val_b) {
            (Token::Orand(op_a), Token::Orand(op_b)) => {
                result.push_back(Operand::sub_orand(op_a, op_b))
            }
            (Token::Orand(op_a), Token::Expr(ep_b)) => {
                result.push_back(self.with_expr(op_a, ep_b, true, verbose)?)
            }
            (Token::Expr(ep_a), Token::Orand(op_b)) => {
                result.push_back(self.with_expr(op_b, ep_a, false, verbose)?)
            }
            _ => return Err(ComputorError::bad_use_op('-')),
        };
        return Ok(result);
    }

    fn mul(
        &self,
        val_a: &Token,
        val_b: &Token,
        verbose: bool,
    ) -> Result<(LList<Token>), ComputorError> {
        let mut result: LList<Token> = LList::new();
        match (val_a, val_b) {
            (Token::Orand(op_a), Token::Orand(op_b)) => {
                result.push_back(Operand::mul_orand(op_a, op_b))
            }
            (Token::Orand(op_a), Token::Expr(ep_b)) => {
                result.push_back(self.with_expr(op_a, ep_b, true, verbose)?)
            }
            (Token::Expr(ep_a), Token::Orand(op_b)) => {
                result.push_back(self.with_expr(op_b, ep_a, false, verbose)?)
            }
            _ => return Err(ComputorError::bad_use_op('*')),
        };
        return Ok(result);
    }

    fn div(
        &self,
        val_a: &Token,
        val_b: &Token,
        verbose: bool,
    ) -> Result<(LList<Token>), ComputorError> {
        let mut result: LList<Token> = LList::new();
        match (val_a, val_b) {
            (Token::Orand(op_a), Token::Orand(op_b)) => {
                result.push_back(Operand::div_orand(op_a, op_b)?)
            }
            // (Token::Orand(op_a), Token::Expr(ep_b)) => {
            //     result.push_back(self.with_expr(op_a, ep_b, true, verbose)?)
            // }
            // (Token::Expr(ep_a), Token::Orand(op_b)) => {
            //     result.push_back(self.with_expr(op_b, ep_a, false, verbose)?)
            // }
            _ => return Err(ComputorError::bad_use_op('/')),
        };
        return Ok(result);
    }

    fn with_expr(
        &self,
        op_tok: &Operand,
        exp_tok: &Expression,
        exp_right: bool,
        verbose: bool,
    ) -> Result<Token, ComputorError> {
        let exp = exp_tok.compute(verbose)?;
        let orand = Token::Orand(op_tok.clone());
        if exp.len() == 1 {
            match exp.front() {
                Some(exp_tok) => {
                    let exp = if exp_right {
                        self.exec(&orand, exp_tok, verbose)?
                    } else {
                        self.exec(exp_tok, &orand, verbose)?
                    };
                    if exp.len() == 1 {
                        match exp.front() {
                            Some(tok) => Ok(tok.clone()),
                            None => Ok(orand),
                        }
                    } else {
                        Ok(Token::Expr(Expression::new(exp)))
                    }
                }
                None => Ok(orand),
            }
        } else {
            let mut new_list: LList<Token> = LList::new();
            new_list.push_back(orand);
            new_list.push_back(Token::Orator(self.clone()));
            new_list.push_back(Token::Expr(exp));
            Ok(Token::Expr(Expression::new(new_list)))
        }
    }
}
