/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_leaf.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:15:13 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 15:50:48 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::lexer::Token;

use std::rc::Rc;

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
    fn token(&self) -> &Rc<Token> {
        &self.token
    }
}
