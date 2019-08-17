/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   token_tree.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:13:01 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 18:45:40 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TreeBranch;
use super::TreeLeaf;
use crate::lexer::{token::Operator, Token};

use std::any::Any;
use std::rc::Rc;

pub trait TokenTree {
    fn as_any(&self) -> &dyn Any;
    fn token(&self) -> &Rc<Token>;
    fn set_branch_left(&mut self, leaf: Box<TokenTree>);
    fn set_branch_right(&mut self, leaf: Box<TokenTree>);
    fn iter(&self, foo: fn(&Rc<Token>));
}

pub fn token_to_node(token: &Rc<Token>) -> Box<TokenTree> {
    return match token.as_any().downcast_ref::<Operator>() {
        None => Box::new(TreeLeaf::new(token)),
        Some(op) => Box::new(TreeBranch::new(token)),
    };
}
