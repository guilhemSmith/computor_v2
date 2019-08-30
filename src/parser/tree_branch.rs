/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_branch.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:14:29 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/30 18:19:15 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::computor::ComputorResult;
use crate::lexer::{token::Operator, Token};
use crate::memory::{Extension, Memory};

use std::any::Any;
use std::fmt;

pub struct TreeBranch {
    token: Box<Token>,
    branch_left: Option<Box<TokenTree>>,
    branch_right: Option<Box<TokenTree>>,
    was_expr: bool,
}

impl TreeBranch {
    pub fn new(token: Box<Token>) -> Self {
        TreeBranch {
            token: token,
            branch_left: None,
            branch_right: None,
            was_expr: false,
        }
    }

    pub fn was_expr(&self) -> bool {
        self.was_expr
    }

    pub fn op_mut(&mut self) -> &mut Operator {
        let extractor = &mut self.token;
        return extractor.as_any_mut().downcast_mut::<Operator>().unwrap();
    }

    pub fn op_ref(&self) -> &Operator {
        let extractor = &self.token;
        return extractor.as_any().downcast_ref::<Operator>().unwrap();
    }

    pub fn extract(&mut self, extr: &mut Option<Box<TokenTree>>, side_l: bool) {
        if side_l {
            std::mem::swap(extr, &mut self.branch_left);
        } else {
            std::mem::swap(extr, &mut self.branch_right);
        }
    }

    pub fn default_to_left(leaf: &mut Box<TokenTree>, next: Box<TokenTree>) {
        let op = Operator::new('*').unwrap();
        let mut new_tree = TreeBranch::new(Box::new(op));
        new_tree.insert_left(next);
        let mut box_tree: Box<TokenTree> = Box::new(new_tree);
        std::mem::swap(leaf, &mut box_tree);
        let any = leaf.as_any();
        let nw_left = any.downcast_mut::<TreeBranch>().unwrap();
        nw_left.insert_right(box_tree);
    }

    fn default_to_right(leaf: &mut Box<TokenTree>, next: Box<TokenTree>) {
        let op = Operator::new('*').unwrap();
        let mut new_tree = TreeBranch::new(Box::new(op));
        new_tree.insert_right(next);
        let mut box_tree: Box<TokenTree> = Box::new(new_tree);
        std::mem::swap(leaf, &mut box_tree);
        let any = leaf.as_any();
        let nw_left = any.downcast_mut::<TreeBranch>().unwrap();
        nw_left.insert_left(box_tree);
    }

    pub fn insert_left(&mut self, mut new: Box<TokenTree>) {
        match new.as_any().downcast_mut::<TreeBranch>() {
            None => self.rotate_left(new),
            Some(branch) => {
                if self.op_mut().is_prior(branch.op_mut()) {
                    std::mem::swap(self, branch);
                    self.insert_right(new);
                } else {
                    self.rotate_left(new);
                }
            }
        };
    }

    pub fn insert_right(&mut self, mut new: Box<TokenTree>) {
        match new.as_any().downcast_mut::<TreeBranch>() {
            None => self.rotate_right(new),
            Some(branch) => {
                if self.op_mut().is_prior(branch.op_mut()) {
                    std::mem::swap(self, branch);
                    self.insert_left(new);
                } else {
                    self.rotate_right(new);
                }
            }
        };
    }

    fn rotate_right(&mut self, mut new: Box<TokenTree>) {
        match &mut self.branch_right {
            None => self.branch_right = Some(new),
            Some(child) => match child.as_any().downcast_mut::<TreeBranch>() {
                Some(branch) => branch.insert_right(new),
                None => match new.as_any().downcast_mut::<TreeBranch>() {
                    None => TreeBranch::default_to_right(child, new),
                    Some(_) => {
                        std::mem::swap(child, &mut new);
                        let any = child.as_any();
                        let nw_child = any.downcast_mut::<TreeBranch>();
                        nw_child.unwrap().insert_left(new);
                    }
                },
            },
        }
    }

    fn rotate_left(&mut self, mut new: Box<TokenTree>) {
        match &mut self.branch_left {
            None => self.branch_left = Some(new),
            Some(child) => match child.as_any().downcast_mut::<TreeBranch>() {
                Some(branch) => branch.insert_left(new),
                None => match new.as_any().downcast_mut::<TreeBranch>() {
                    None => TreeBranch::default_to_left(child, new),
                    Some(_) => {
                        std::mem::swap(child, &mut new);
                        let any = child.as_any();
                        let nw_child = any.downcast_mut::<TreeBranch>();
                        nw_child.unwrap().insert_right(new);
                    }
                },
            },
        }
    }
}

impl TokenTree for TreeBranch {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn token(&self) -> &Box<Token> {
        &self.token
    }

    fn iter(&self, foo: fn(&Box<Token>)) {
        match &self.branch_left {
            Some(tree) => tree.iter(foo),
            None => {}
        }
        foo(self.token());
        match &self.branch_right {
            Some(tree) => tree.iter(foo),
            None => {}
        }
    }

    fn count(&self, foo: fn(&Box<Token>) -> i32) -> i32 {
        let mut sum = match &self.branch_left {
            Some(tree) => tree.count(foo),
            None => 0,
        };
        sum += foo(self.token());
        sum += match &self.branch_right {
            Some(tree) => tree.count(foo),
            None => 0,
        };
        return sum;
    }

    fn is_full(&self) -> bool {
        match (&self.branch_left, &self.branch_right) {
            (Some(left), Some(right)) => left.is_full() && right.is_full(),
            _ => false,
        }
    }

    fn set_as_exp(&mut self) {
        self.op_mut().set_prior_as_exp();
        self.was_expr = true;
    }

    fn compute(
        &self,
        mem: &Memory,
        mut ext: Option<&mut Extension>,
    ) -> ComputorResult {
        let orand_left = match &self.branch_left {
            None => ComputorResult::None,
            Some(tree) => match &mut ext {
                Some(extend) => {
                    let mut ext_clone = extend.clone();
                    tree.compute(mem, Some(&mut ext_clone))
                }
                None => tree.compute(mem, None),
            },
        };
        let orand_right = match &self.branch_right {
            None => ComputorResult::None,
            Some(tree) => match &mut ext {
                Some(extend) => {
                    let mut ext_clone = extend.clone();
                    tree.compute(mem, Some(&mut ext_clone))
                }
                None => tree.compute(mem, None),
            },
        };
        self.op_ref().exec(mem, orand_left, orand_right)
    }
}

impl fmt::Display for TreeBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.branch_left, &self.branch_right) {
            (Some(l), Some(r)) => write!(f, "{}{}{}", l, self.token, r),
            (Some(left), None) => write!(f, "{}{}", left, self.token),
            (None, Some(right)) => write!(f, "{}{}", self.token, right),
            (None, None) => write!(f, "{}", self.token),
        }
    }
}

impl fmt::Debug for TreeBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.branch_left, &self.branch_right) {
            (Some(l), Some(r)) => {
                write!(f, "{{{}:{:?} {:?}}}", self.token, l, r)
            }
            (Some(left), None) => write!(f, "{{{}:{:?} _}}", self.token, left),
            (None, Some(right)) => write!(f, "{{{}:_ {:?}]", self.token, right),
            (None, None) => write!(f, "{{{}: _ _}}", self.token),
        }
    }
}
