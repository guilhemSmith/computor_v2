/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_leaf.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:15:13 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/20 09:37:53 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::lexer::Token;

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

    fn set_prior_as_exp(&mut self) {}
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
