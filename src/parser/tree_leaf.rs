/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_leaf.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:15:13 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/19 09:33:47 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::lexer::Token;

use std::any::Any;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct TreeLeaf {
    token: Rc<Token>,
}

impl TreeLeaf {
    pub fn new(token: Rc<Token>) -> Self {
        TreeLeaf {
            token: token.clone(),
        }
    }
}

impl TokenTree for TreeLeaf {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn token(&self) -> &Rc<Token> {
        &self.token
    }

    fn iter(&self, foo: fn(&Rc<Token>)) {
        foo(self.token());
    }

    fn count(&self, foo: fn(&Rc<Token>) -> i32) -> i32 {
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
        write!(f, "{{L:{:?}}}", self.token)
    }
}
