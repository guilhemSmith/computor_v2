/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   expression.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:28:47 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/30 11:37:03 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Operator, Token};
use crate::error::ComputorError;
use std::vec::Vec;

pub struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    pub fn new(input_trimed: String) -> Result<Self, ComputorError> {
        let mut expr = Expression { tokens: Vec::new() };
        let mut iter_char = input_trimed.char_indices();
        loop {
            match iter_char.next() {
                Some((i, op))
                    if op == '+' || op == '-' || op == '*' || op == '/' =>
                {
                    let orator = Operator::new(op)?;
                    expr.push(Token::Orator(orator));
                }
                Some((i, d)) => println!("'{}:{}'", i, d),
                None => break,
            }
        }
        return Ok(expr);
    }

    pub fn push(&mut self, tok: Token) {
        self.tokens.push(tok);
    }
}
