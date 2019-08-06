/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   expression.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:28:47 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/06 15:45:06 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Operand, Operator, Token};
use crate::error::{
    log_error, BadUseOperatorError, ComputorError, IncompleteExprError,
    InvalidExprError,
};
use std::{collections::LinkedList, fmt};

#[derive(Clone)]
pub struct Expression {
    tokens: LinkedList<Token>,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[expr: {}]", tokens_to_string(&self.tokens))
    }
}

impl Expression {
    pub fn new(
        input_trimed: String,
        start: usize,
    ) -> Result<Self, ComputorError> {
        let mut expr = Expression {
            tokens: LinkedList::new(),
        };
        let mut operand_index: i32 = -1;
        let mut iter_char = input_trimed.char_indices();

        loop {
            match iter_char.next() {
                Some((i, ch))
                    if ch == '+' || ch == '-' || ch == '*' || ch == '/' =>
                {
                    if operand_index >= 0 {
                        expr.push(read_operand(
                            &input_trimed[operand_index as usize..i],
                            operand_index as usize + start,
                        ));
                        operand_index = -1;
                    }
                    let orator = Operator::new(ch)?;
                    expr.push(Token::Orator(orator));
                }
                Some((i, ch)) if ch == '(' => {
                    if operand_index >= 0 {
                        expr.push(read_operand(
                            &input_trimed[operand_index as usize..i],
                            operand_index as usize + start,
                        ));
                        operand_index = -1;
                    }
                    let start_exp = i + 1;
                    let mut end_exp = i + 1;
                    let mut opening = 1;
                    while opening > 0 {
                        match iter_char.next() {
                            Some((_, ch)) => {
                                end_exp += 1;
                                if ch == ')' {
                                    opening -= 1;
                                } else if ch == '(' {
                                    opening += 1;
                                }
                            }
                            None => {
                                return Err(IncompleteExprError::new(
                                    &input_trimed[start_exp - 1..],
                                ))
                            }
                        }
                    }
                    expr.push(Token::Expr(Expression::new(
                        String::from(&input_trimed[start_exp..end_exp - 1]),
                        start + start_exp + 1,
                    )?))
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
                            operand_index as usize + start,
                        ));
                    }
                    break;
                }
            }
        }
        return Ok(expr);
    }

    fn push(&mut self, tok: Token) {
        self.tokens.push_back(tok);
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.len() == 0
    }

    pub fn check_errors(&self) -> u32 {
        let mut count = 0;
        let mut iter = self.tokens.iter();
        loop {
            match iter.next() {
                Some(tok) => match tok {
                    Token::Invalid(err, pos) => {
                        count += 1;
                        log_error(err, Some(pos));
                    }
                    Token::Expr(expr) => count += expr.check_errors(),
                    _ => {}
                },
                None => break,
            }
        }
        return count;
    }

    pub fn compute(&self, verbose: bool) -> Result<Self, ComputorError> {
        if verbose {
            println!("[V:computor] - computing expression: {}", self);
        }
        let mut result = self.tokens.clone();
        result = compute_all(result, true)?;
        if verbose {
            println!(
                "[V:computor] - prior operations computed: {}",
                tokens_to_string(&result)
            );
        }
        result = compute_all(result, false)?;
        if verbose {
            println!(
                "[V:computor] - remaining operations computed: {}",
                tokens_to_string(&result)
            );
        }
        Ok(Expression { tokens: result })
    }
}

fn compute_all(
    mut lst: LinkedList<Token>,
    prior: bool,
) -> Result<LinkedList<Token>, ComputorError> {
    let mut result: LinkedList<Token> = LinkedList::new();
    while lst.len() > 2 {
        let mut reduced = compute_op(&mut lst, prior)?;
        result.append(&mut reduced);
    }
    loop {
        match lst.pop_front() {
            Some(tok) => result.push_back(tok),
            None => break,
        };
    }
    Ok(result)
}

fn compute_op(
    lst: &mut LinkedList<Token>,
    prior: bool,
) -> Result<LinkedList<Token>, ComputorError> {
    let remain = lst.split_off(3);
    let lst_orand = (lst.pop_front(), lst.pop_back());
    let orator = match lst.pop_front() {
        Some(tok) => match tok {
            Token::Orator(op) => op,
            _ => return Err(InvalidExprError::new()),
        },
        None => return Err(InvalidExprError::new()),
    };
    *lst = remain;
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

    let mut result: LinkedList<Token> = LinkedList::new();
    if orator.prior() == prior {
        let op_result = orator.exec(&orands.0, &orands.1)?;
        lst.push_front(Token::Orand(op_result));
    } else {
        lst.push_front(Token::Orand(orands.1));
        result.push_front(Token::Orator(orator));
        result.push_front(Token::Orand(orands.0));
    }
    return Ok(result);
}

fn read_operand(raw_operand: &str, pos: usize) -> Token {
    let is_real = match raw_operand.chars().rev().next() {
        Some(c) => c != 'i',
        None => true,
    };

    match Operand::new(
        if is_real {
            raw_operand
        } else {
            let len = raw_operand.len();
            if len > 1 {
                &raw_operand[..raw_operand.len() - 1]
            } else {
                "1"
            }
        },
        is_real,
    ) {
        Ok(orand) => Token::Orand(orand),
        Err(err) => Token::Invalid(err, pos),
    }
}

fn tokens_to_string(lst: &LinkedList<Token>) -> String {
    let mut tokens_str = String::new();
    let mut iter_token = lst.iter();

    loop {
        match iter_token.next() {
            Some(tok) => tokens_str = format!("{}{}", tokens_str, tok),
            None => break,
        };
    }
    return tokens_str;
}
