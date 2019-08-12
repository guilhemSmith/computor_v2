/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   expression.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:28:47 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/12 12:04:26 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Operand, Operator, Token};
use crate::error::{ComputorError, ErrorPosition};
use std::{collections::LinkedList, fmt};

#[derive(Clone)]
pub struct Expression {
    tokens: LinkedList<Token>,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", tokens_to_string(&self.tokens))
    }
}

impl Expression {
    pub fn new(input: String, start: usize) -> Result<Self, ComputorError> {
        let mut expr = Expression {
            tokens: LinkedList::new(),
        };
        let mut op_index: i64 = -1;
        let last = input.len();
        let mut iter_char = input.char_indices();

        loop {
            match iter_char.next() {
                Some((i, ch))
                    if ch == '+' || ch == '-' || ch == '*' || ch == '/' =>
                {
                    expr.push_orand(&mut op_index, &input, i, start);
                    let orator = Operator::new(ch)?;
                    expr.push_back(Token::Orator(orator));
                }
                Some((i, ch)) if ch == '(' => {
                    expr.push_orand(&mut op_index, &input, i, start);
                    expr.push_expr(i + 1, &mut iter_char, &input, start)?;
                }
                Some((i, _)) => {
                    if op_index < 0 {
                        op_index = i as i64;
                    }
                }
                None => {
                    expr.push_orand(&mut op_index, &input, last, start);
                    break;
                }
            }
        }
        return Ok(expr);
    }

    pub fn from_tokens(tokens: LinkedList<Token>) -> Self {
        Expression { tokens: tokens }
    }

    fn push_orand(
        &mut self,
        start_index: &mut i64,
        input: &String,
        current: usize,
        start_expr: usize,
    ) {
        let i = *start_index;
        if i >= 0 {
            let u = i as usize;
            self.push_back(read_operand(&input[u..current], u + start_expr));
            *start_index = -1;
        }
    }

    fn push_expr(
        &mut self,
        index: usize,
        iter: &mut std::str::CharIndices<'_>,
        input: &String,
        start: usize,
    ) -> Result<(), ComputorError> {
        let mut end_exp = index;
        let mut opening = 1;
        while opening > 0 {
            match iter.next() {
                Some((_, ch)) => {
                    end_exp += 1;
                    if ch == ')' {
                        opening -= 1;
                        continue;
                    }
                    if ch == '(' {
                        opening += 1;
                        continue;
                    }
                }
                None => {
                    return Err(ComputorError::incomplete_expr(
                        &input[index - 1..],
                    ))
                }
            }
        }
        let tok = Token::Expr(Expression::new(
            String::from(&input[index..end_exp - 1]),
            start + index + 1,
        )?);
        self.push_back(tok);
        Ok(())
    }

    fn push_back(&mut self, tok: Token) {
        self.tokens.push_back(tok);
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.len() == 0
    }

    pub fn front(&self) -> Option<&Token> {
        self.tokens.front()
    }

    pub fn check_errors(&self) -> u32 {
        let mut count = 0;
        let mut iter = self.tokens.iter();
        loop {
            match iter.next() {
                Some(tok) => match tok {
                    Token::Invalid(err) => {
                        count += 1;
                        println!("{}", err);
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
        result = compute_all(result, true, verbose)?;
        if verbose {
            println!(
                "[V:computor] - prior operations computed: {}",
                tokens_to_string(&result)
            );
        }
        if result.len() > 1 {
            result = compute_all(result, false, verbose)?;
            if verbose {
                println!(
                    "[V:computor] - remaining operations computed: {}",
                    tokens_to_string(&result)
                );
            }
        }
        Ok(Expression { tokens: result })
    }
}

fn compute_all(
    mut lst: LinkedList<Token>,
    prior: bool,
    verbose: bool,
) -> Result<LinkedList<Token>, ComputorError> {
    let mut result: LinkedList<Token> = LinkedList::new();
    while lst.len() > 2 {
        let mut reduced = compute_op(&mut lst, prior, verbose)?;
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
    verbose: bool,
) -> Result<LinkedList<Token>, ComputorError> {
    let remain = lst.split_off(3);
    let lst_orand = (lst.pop_front(), lst.pop_back());
    let orator = match lst.pop_front() {
        Some(tok) => match tok {
            Token::Orator(op) => op,
            _ => return Err(ComputorError::invalid_expr()),
        },
        None => return Err(ComputorError::invalid_expr()),
    };
    *lst = remain;
    let orands = (
        match lst_orand.0 {
            Some(tok) => tok,
            None => return Err(ComputorError::invalid_expr()),
        },
        match lst_orand.1 {
            Some(tok) => tok,
            None => return Err(ComputorError::invalid_expr()),
        },
    );

    let mut result: LinkedList<Token> = LinkedList::new();
    if orator.prior() == prior {
        let mut op_result = orator.exec(&orands.0, &orands.1, verbose)?;
        match op_result.pop_back() {
            Some(tok) => lst.push_front(tok),
            None => lst.push_front(orands.1),
        };
    } else {
        lst.push_front(orands.1);
        result.push_front(Token::Orator(orator));
        result.push_front(orands.0);
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
        Err(mut err) => {
            err.set_pos(ErrorPosition::Char(pos));
            Token::Invalid(err)
        }
    }
}

fn tokens_to_string(lst: &LinkedList<Token>) -> String {
    let mut tokens_str = String::new();
    let mut iter_token = lst.iter();

    loop {
        match iter_token.next() {
            Some(tok) => tokens_str = format!("{} {}", tokens_str, tok),
            None => break,
        };
    }
    return tokens_str.replace("- -", "+");
}
