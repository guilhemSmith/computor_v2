/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   token_tree.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:13:01 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/12 16:42:49 by gsmith           ###   ########.fr       */
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
    fn token(&self) -> &Box<dyn Token>;
    fn iter(&self, foo: fn(&Box<dyn Token>));
    fn count(&self, foo: fn(&Box<dyn Token>) -> i32) -> i32;
    fn is_full(&self) -> bool;
    fn set_as_exp(&mut self);
    fn compute(
        &self,
        mem: &Memory,
        ext: Option<&mut Extension>,
    ) -> ComputorResult;
}

type TTree = Box<dyn TokenTree>;

pub fn insert_tree(b_tree: &mut TTree, mut b_new: TTree) {
    let mut tree = b_tree.as_any().downcast_mut::<TreeBranch>();
    let new = b_new.as_any().downcast_mut::<TreeBranch>();

    match (&mut tree, &new) {
        (None, None) => TreeBranch::default_to_left(b_tree, b_new),
        (Some(root), Some(branch)) => {
            if !root.was_expr() {
                root.rot_left(b_new);
            } else if !branch.was_expr() {
                swap_tree(b_tree, b_new);
            } else {
                TreeBranch::default_to_left(b_tree, b_new);
            }
        }
        (Some(root), None) => {
            if !root.was_expr() {
                root.rot_left(b_new);
            } else {
                TreeBranch::default_to_left(b_tree, b_new);
            }
        }
        (None, Some(branch)) => {
            if !branch.was_expr() {
                swap_tree(b_tree, b_new);
            } else {
                TreeBranch::default_to_left(b_tree, b_new);
            }
        }
    };
}

fn swap_tree(b_tree: &mut TTree, mut b_new: TTree) {
    std::mem::swap(b_tree, &mut b_new);
    let any = b_tree.as_any();
    let root = any.downcast_mut::<TreeBranch>().unwrap();
    root.rot_right(b_new)
}
