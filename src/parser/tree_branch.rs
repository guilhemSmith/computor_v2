/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_branch.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:14:29 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 13:13:07 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::lexer::{token::Operator, Token};

use std::rc::Rc;

pub struct TreeBranch<'op, 'bl, 'br> {
    operator: &'op Operator,
    as_token: Rc<Token>,
    branch_left: Option<&'bl TokenTree>,
    branch_right: Option<&'br TokenTree>,
}

impl<'op, 'bl, 'br> TreeBranch<'op, 'bl, 'br> {
    pub fn new(operator: &'op Operator, token: Rc<Token>) -> Self {
        TreeBranch {
            operator: operator,
            as_token: token,
            branch_left: None,
            branch_right: None,
        }
    }

    pub fn set_branch_left(&mut self, child: &'bl TokenTree) {
        self.branch_left = Some(child);
    }

    pub fn set_branch_right(&mut self, child: &'br TokenTree) {
        self.branch_right = Some(child);
    }
}

impl<'op, 'bl, 'br> TokenTree for TreeBranch<'op, 'bl, 'br> {
    fn token(&self) -> &Rc<Token> {
        &self.as_token
    }
}
