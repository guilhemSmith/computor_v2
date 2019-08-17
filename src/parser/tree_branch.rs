/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_branch.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:14:29 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 18:58:13 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::lexer::{token::Operator, Token};

use std::any::Any;
use std::rc::Rc;

pub struct TreeBranch {
    token: Rc<Token>,
    branch_left: Option<Box<TokenTree>>,
    branch_right: Option<Box<TokenTree>>,
}

impl TreeBranch {
    pub fn new(token: &Rc<Token>) -> Self {
        TreeBranch {
            token: token.clone(),
            branch_left: None,
            branch_right: None,
        }
    }

    pub fn operator(&self) -> &Operator {
        &self.token.as_any().downcast_ref::<Operator>().unwrap()
    }
}

impl TokenTree for TreeBranch {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn token(&self) -> &Rc<Token> {
        &self.token
    }

    fn set_branch_left(&mut self, leaf: Box<TokenTree>) {
        self.branch_left = Some(leaf);
    }

    fn set_branch_right(&mut self, leaf: Box<TokenTree>) {
        self.branch_right = Some(leaf);
    }

    fn iter(&self, foo: fn(&Rc<Token>)) {
        match &self.branch_left {
            Some(tree) => tree.iter(foo),
            None => {}
        }
        foo(self.token());
        match &self.branch_right {
            Some(tree) => tree.iter(foo),
            None => {}
        }
    }
}
