/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   expression.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:28:47 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/26 11:56:47 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Token;
use std::vec::Vec;

pub struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    pub fn new() -> Self {
        Expression { tokens: Vec::new() }
    }

    pub fn push(&mut self, tok: Token) {
        self.tokens.push(tok);
    }
}
