/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_leaf.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:15:13 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 18:46:06 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{TokenTree, TreeBranch};
use crate::lexer::Token;

use std::any::Any;
use std::rc::Rc;

#[derive(Clone)]
pub struct TreeLeaf {
    token: Rc<Token>,
}

impl TreeLeaf {
    pub fn new(token: &Rc<Token>) -> Self {
        TreeLeaf {
            token: token.clone(),
        }
    }
}

impl TokenTree for TreeLeaf {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn token(&self) -> &Rc<Token> {
        &self.token
    }

    fn set_branch_left(&mut self, _leaf: Box<TokenTree>) {
        panic!("Can't set a branch to a leaf.");
    }

    fn set_branch_right(&mut self, _leaf: Box<TokenTree>) {
        panic!("Can't set a branch to a leaf.");
    }

    fn iter(&self, foo: fn(&Rc<Token>)) {
        foo(self.token());
    }
}
