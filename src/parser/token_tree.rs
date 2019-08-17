/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   token_tree.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:13:01 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 16:02:38 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TreeBranch;
use super::TreeLeaf;
use crate::lexer::{token::Operator, Token};

use std::rc::Rc;

pub trait TokenTree {
    fn token(&self) -> &Rc<Token>;
}

pub fn token_to_node(token: &Rc<Token>) -> Box<TokenTree> {
    return match token.as_any().downcast_ref::<Operator>() {
        None => Box::new(TreeLeaf::new(token)),
        Some(op) => Box::new(TreeBranch::new(op.clone(), token)),
    };
}
