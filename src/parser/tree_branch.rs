/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_branch.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:14:29 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 12:56:22 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::lexer::{token::Operator, Token};

use std::rc::Rc;

pub struct TreeBranch {
    operator: Rc<Operator>,
    as_token: Rc<Token>,
    branch_left: Option<Rc<TokenTree>>,
    branch_right: Option<Rc<TokenTree>>,
}

impl TreeBranch {
    pub fn new(operator: Rc<Operator>) -> Self {
        TreeBranch {
            operator: operator.clone(),
            as_token: operator,
            branch_left: None,
            branch_right: None,
        }
    }
}

impl TokenTree for TreeBranch {
    fn token(&self) -> &Rc<Token> {
        &self.as_token
    }
}
