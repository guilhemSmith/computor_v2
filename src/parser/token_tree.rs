/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   token_tree.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:13:01 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/19 17:12:21 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TreeBranch;
use crate::lexer::Token;

use std::any::Any;
use std::fmt;

pub trait TokenTree: fmt::Display + fmt::Debug {
    fn as_any(&mut self) -> &mut dyn Any;
    fn token(&self) -> &Box<Token>;
    fn iter(&self, foo: fn(&Box<Token>));
    fn count(&self, foo: fn(&Box<Token>) -> i32) -> i32;
    fn set_prior_as_exp(&mut self);
}

pub fn insert_in_tree(b_tree: &mut Box<TokenTree>, mut b_new: Box<TokenTree>) {
    let tree = b_tree.as_any().downcast_mut::<TreeBranch>();
    let new = b_new.as_any().downcast_mut::<TreeBranch>();

    match (tree, new) {
        (Some(root), _) => root.insert_left(b_new),
        (None, Some(_)) => {
            std::mem::swap(b_tree, &mut b_new);
            let any = b_tree.as_any();
            let nw_root = any.downcast_mut::<TreeBranch>().unwrap();
            nw_root.insert_right(b_new);
        }
        (None, None) => TreeBranch::default_to_left(b_tree, b_new),
    }
}
