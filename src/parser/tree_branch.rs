/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_branch.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:14:29 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/18 17:03:46 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::lexer::{token::Operator, Token};

use std::any::Any;
use std::fmt;
use std::rc::Rc;

pub struct TreeBranch {
    token: Rc<Token>,
    branch_left: Option<Box<TokenTree>>,
    branch_right: Option<Box<TokenTree>>,
}

impl TreeBranch {
    pub fn new(token: Rc<Token>) -> Self {
        TreeBranch {
            token: token,
            branch_left: None,
            branch_right: None,
        }
    }

    pub fn operator(&self) -> &Operator {
        &self.token.as_any().downcast_ref::<Operator>().unwrap()
    }

    pub fn default_to_left(leaf: &mut Box<TokenTree>, next: Box<TokenTree>) {
        let op = Operator::new('*').unwrap();
        let mut new_tree = TreeBranch::new(Rc::new(op));
        new_tree.insert_left(next);
        let mut box_tree: Box<TokenTree> = Box::new(new_tree);
        std::mem::swap(leaf, &mut box_tree);
        let any = leaf.as_any();
        let nw_left = any.downcast_mut::<TreeBranch>().unwrap();
        nw_left.insert_right(box_tree);
    }

    fn default_to_right(leaf: &mut Box<TokenTree>, next: Box<TokenTree>) {
        let op = Operator::new('*').unwrap();
        let mut new_tree = TreeBranch::new(Rc::new(op));
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
                if self.operator().is_prior(branch.operator()) {
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
                if self.operator().is_prior(branch.operator()) {
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

    fn token(&self) -> &Rc<Token> {
        &self.token
    }

    fn iter(&self, foo: fn(&Rc<Token>)) {
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
}

impl fmt::Display for TreeBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.branch_left, &self.branch_right) {
            (Some(l), Some(r)) => write!(f, "[B:{}{}{}]", l, self.token, r),
            (Some(left), None) => write!(f, "[B:{}{}]", left, self.token),
            (None, Some(right)) => write!(f, "[B:{}{}]", self.token, right),
            (None, None) => write!(f, "[B:{}]", self.token),
        }
    }
}
