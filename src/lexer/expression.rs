/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   expression.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:28:47 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/05 19:24:40 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Operand, Operator, Token};
use crate::error::{BadUseOperatorError, ComputorError, InvalidExprError};
use std::{collections::LinkedList, fmt};

pub struct Expression {
    tokens: LinkedList<Token>,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tokens_str = String::new();
        let mut iter_token = self.tokens.iter();

        loop {
            match iter_token.next() {
                Some(tok) => tokens_str = format!("{}{}", tokens_str, tok),
                None => break,
            };
        }
        write!(f, "[expr: {}]", tokens_str)
    }
}

impl Expression {
    pub fn new(input_trimed: String) -> Result<Self, ComputorError> {
        let mut expr = Expression {
            tokens: LinkedList::new(),
        };
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
        // let mut index = 1;
        // let mut iter = self.tokens.iter();
        // iter.next();
        // while index < self.tokens.len() - 1 {
        //     match iter.next() {
        //         Some(tok) => match tok {
        //             Token::Orator(op) => {
        //                 if op.prior() {
        //                     self.tokens = compute_op(self.tokens.split_off(index - 1))?;
        //                 } else {
        //                     index += 1;
        //                 }
        //             },
        //             _ => { index += 1 }
        //         },
        //         None => break,
        //     };
        // }
        while self.tokens.len() > 1 {
            self.tokens = compute_op(self.tokens.split_off(0))?;
        }
        Ok(())
    }

    pub fn push(&mut self, tok: Token) {
        self.tokens.push_back(tok);
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
    mut lst: LinkedList<Token>,
) -> Result<LinkedList<Token>, ComputorError> {
    let mut remain = lst.split_off(3);
    let lst_orand = (lst.pop_front(), lst.pop_back());
    let orator = match lst.pop_front() {
        Some(tok) => match tok {
            Token::Orator(op) => op,
            _ => return Err(InvalidExprError::new()),
        },
        None => return Err(InvalidExprError::new()),
    };
    let orands = (
        match lst_orand.0 {
            Some(tok) => match tok {
                Token::Orand(or) => or,
                _ => return Err(BadUseOperatorError::new(orator.symbol())),
            },
            None => return Err(InvalidExprError::new()),
        },
        match lst_orand.1 {
            Some(tok) => match tok {
                Token::Orand(or) => or,
                _ => return Err(BadUseOperatorError::new(orator.symbol())),
            },
            None => return Err(InvalidExprError::new()),
        },
    );
    let op_result = orator.exec(&orands.0, &orands.1)?;
    remain.push_front(Token::Orand(op_result));
    return Ok(remain);
}
