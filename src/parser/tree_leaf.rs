/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_leaf.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:15:13 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/20 11:13:02 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::computor::ComputorResult;
use crate::lexer::Token;
use crate::memory::Memory;

use std::any::Any;
use std::fmt;

pub struct TreeLeaf {
    token: Box<Token>,
}

impl TreeLeaf {
    pub fn new(token: Box<Token>) -> Self {
        TreeLeaf { token: token }
    }
}

impl TokenTree for TreeLeaf {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn token(&self) -> &Box<Token> {
        &self.token
    }

    fn iter(&self, foo: fn(&Box<Token>)) {
        foo(self.token());
    }

    fn count(&self, foo: fn(&Box<Token>) -> i32) -> i32 {
        foo(&self.token)
    }

    fn is_full(&self) -> bool {
        true
    }

    fn set_as_exp(&mut self) {}

    fn compute(&self, mem: &Memory) -> ComputorResult {
        self.token.get_result(mem)
    }
}

impl fmt::Display for TreeLeaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl fmt::Debug for TreeLeaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{:?}}}", self.token)
    }
}
