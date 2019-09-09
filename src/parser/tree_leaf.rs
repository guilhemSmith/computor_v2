/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_leaf.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:15:13 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/09 11:57:11 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::computor::ComputorResult;
use crate::lexer::Token;
use crate::memory::{Extension, Memory};

use std::any::Any;
use std::fmt;

pub struct TreeLeaf {
    token: Box<dyn Token>,
}

impl TreeLeaf {
    pub fn new(token: Box<dyn Token>) -> Self {
        TreeLeaf { token: token }
    }
}

impl TokenTree for TreeLeaf {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn token(&self) -> &Box<dyn Token> {
        &self.token
    }

    fn iter(&self, foo: fn(&Box<dyn Token>)) {
        foo(self.token());
    }

    fn count(&self, foo: fn(&Box<dyn Token>) -> i32) -> i32 {
        foo(&self.token)
    }

    fn is_full(&self) -> bool {
        true
    }

    fn set_as_exp(&mut self) {}

    fn compute(
        &self,
        mem: &Memory,
        ext: Option<&mut Extension>,
    ) -> ComputorResult {
        self.token.get_result(mem, ext)
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
