/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   expression.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:28:47 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/18 16:54:39 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Token;
use crate::computor::{ComputorError, TreeResult};
use crate::memory::{Extension, Memory};
use std::any::Any;
use std::fmt;

pub struct Expression {
    tokens: Vec<Box<dyn Token>>,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", super::display_token(&self.tokens))
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[exp:({})]", super::debug_token(&self.tokens))
    }
}

impl Token for Expression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> TreeResult {
        TreeResult::Err(ComputorError::unparsed_token(self))
    }
}

impl Expression {
    pub fn new(tokens: Vec<Box<dyn Token>>) -> Self {
        Expression { tokens: tokens }
    }

    pub fn count(&self) -> usize {
        self.tokens.len()
    }

    pub fn consume_tokens(&mut self) -> Vec<Box<dyn Token>> {
        let mut extractor: Vec<Box<dyn Token>> = Vec::new();
        std::mem::swap(&mut self.tokens, &mut extractor);
        return extractor;
    }
}
