/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_branch.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:14:29 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 16:02:03 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::lexer::{token::Operator, Token};

use std::rc::Rc;

pub struct TreeBranch {
    operator: Operator,
    as_token: Rc<Token>,
    branch_left: Option<Box<TokenTree>>,
    branch_right: Option<Box<TokenTree>>,
}

impl TreeBranch {
    pub fn new(operator: Operator, token: &Rc<Token>) -> Self {
        TreeBranch {
            operator: operator,
            as_token: token.clone(),
            branch_left: None,
            branch_right: None,
        }
    }

    pub fn set_branch_left(&mut self, child: Box<TokenTree>) {
        self.branch_left = Some(child);
    }

    pub fn set_branch_right(&mut self, child: Box<TokenTree>) {
        self.branch_right = Some(child);
    }
}

impl TokenTree for TreeBranch {
    fn token(&self) -> &Rc<Token> {
        &self.as_token
    }
}
