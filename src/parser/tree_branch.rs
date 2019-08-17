/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_branch.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:14:29 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 11:55:39 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::lexer::Token;

use std::rc::Rc;

pub struct TreeBranch {
    token: Rc<Token>,
    branch_left: Option<Rc<TokenTree>>,
    branch_right: Option<Rc<TokenTree>>,
}

impl TreeBranch {
    pub fn new(token: Rc<Token>) -> Self {
        TreeBranch {
            token: token,
            branch_left: None,
            branch_right: None,
        }
    }
}

impl TokenTree for TreeBranch {
    fn token(&self) -> &Rc<Token> {
        &self.token
    }
}
