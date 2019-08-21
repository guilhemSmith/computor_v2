/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   token_tree.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:13:01 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/21 10:38:54 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TreeBranch;
use crate::computor::ComputorResult;
use crate::lexer::Token;
use crate::memory::{Extension, Memory};

use std::any::Any;
use std::fmt;

pub trait TokenTree: fmt::Display + fmt::Debug {
    fn as_any(&mut self) -> &mut dyn Any;
    fn token(&self) -> &Box<Token>;
    fn iter(&self, foo: fn(&Box<Token>));
    fn count(&self, foo: fn(&Box<Token>) -> i32) -> i32;
    fn is_full(&self) -> bool;
    fn set_as_exp(&mut self);
    fn compute(
        &self,
        mem: &Memory,
        ext: Option<&mut Extension>,
    ) -> ComputorResult;
}

pub fn insert_in_tree(b_tree: &mut Box<TokenTree>, mut b_new: Box<TokenTree>) {
    let tree = b_tree.as_any().downcast_mut::<TreeBranch>();
    let new = b_new.as_any().downcast_mut::<TreeBranch>();

    match (tree, new) {
        (Some(ref mut root), _) if !root.was_expr() => root.insert_left(b_new),
        (Some(_), _) => TreeBranch::default_to_left(b_tree, b_new),
        (None, Some(ref branch)) if !branch.was_expr() => {
            std::mem::swap(b_tree, &mut b_new);
            let any = b_tree.as_any();
            let nw_root = any.downcast_mut::<TreeBranch>().unwrap();
            nw_root.insert_right(b_new);
        }
        _ => TreeBranch::default_to_left(b_tree, b_new),
    }
}
