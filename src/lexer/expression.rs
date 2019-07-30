/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   expression.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:28:47 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/30 17:46:28 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Operand, Operator, Token};
use crate::error::{BadUseOperatorError, ComputorError};
use std::{fmt, vec::Vec};

pub struct Expression {
    tokens: Vec<Token>,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tokens_str = String::new();
        let mut iter_token = self.tokens.iter();

        loop {
            match iter_token.next() {
                Some(tok) => match tok {
                    Token::Expr(exp) => {
                        tokens_str = format!("{}{}", tokens_str, exp)
                    }
                    Token::Orand(op) => {
                        tokens_str = format!("{}{}", tokens_str, op)
                    }
                    Token::Orator(op) => {
                        tokens_str = format!("{}{}", tokens_str, op)
                    }
                    Token::Invalid(_, _) => {
                        tokens_str = format!("{}{}", tokens_str, "[error]")
                    }
                },
                None => break,
            };
        }
        write!(f, "[expr: {}]", tokens_str)
    }
}

impl Expression {
    pub fn new(input_trimed: String) -> Result<Self, ComputorError> {
        let mut expr = Expression { tokens: Vec::new() };
        let mut operand_index: i32 = -1;
        let mut iter_char = input_trimed.char_indices();

        loop {
            match iter_char.next() {
                Some((i, op))
                    if op == '+' || op == '-' || op == '*' || op == '/' =>
                {
                    if operand_index >= 0 {
                        expr.push(read_operand(
                            &input_trimed[operand_index as usize..i],
                            operand_index as usize,
                        ));
                        operand_index = -1;
                    }
                    let orator = Operator::new(op)?;
                    expr.push(Token::Orator(orator));
                }
                Some((i, _)) => {
                    if operand_index < 0 {
                        operand_index = i as i32;
                    }
                }
                None => {
                    if operand_index >= 0 {
                        expr.push(read_operand(
                            &input_trimed[operand_index as usize..],
                            operand_index as usize,
                        ));
                    }
                    break;
                }
            }
        }
        return Ok(expr);
    }

    pub fn compute(&mut self, verbose: bool) -> Result<(), ComputorError> {
        if verbose {
            println!("[V:computor] - {}", self);
        }

        let mut tokens = &mut self.tokens;
        let mut i: usize = 1;

        while i < tokens.len() - 2 {
            compute_op(&mut tokens, &mut i)?;
        }
        Ok(())
    }

    pub fn push(&mut self, tok: Token) {
        self.tokens.push(tok);
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.len() == 0
    }
}

fn read_operand(raw_operand: &str, pos: usize) -> Token {
    let is_real = if raw_operand.len() > 1 {
        match raw_operand.chars().rev().next() {
            Some(c) => c != 'i',
            None => true,
        }
    } else {
        true
    };

    match Operand::new(
        if is_real {
            raw_operand
        } else {
            &raw_operand[..raw_operand.len() - 1]
        },
        is_real,
    ) {
        Ok(orand) => Token::Orand(orand),
        Err(err) => Token::Invalid(err, pos),
    }
}

fn compute_op(
    tokens: &mut Vec<Token>,
    i: &mut usize,
) -> Result<Option<Operand>, ComputorError> {
    match (tokens[*i - 1], tokens[*i], tokens[*i + 1]) {
        (Token::Orator(ref op), _, _) if op.symbol() == '*' => {
            Err(BadUseOperatorError::new(op.symbol()))
        }
        (_, _, Token::Orator(ref op)) if op.symbol() == '*' => {
            Err(BadUseOperatorError::new(op.symbol()))
        }
        (Token::Orand(ref lhs), Token::Orator(ref op), Token::Orand(ref rhs)) => {
            Ok(Some(op.exec(&lhs, &rhs)?))
        }
        _ => Ok(None),
    }
}
