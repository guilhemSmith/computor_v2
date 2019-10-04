/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_branch.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:14:29 by gsmith            #+#    #+#             */
/*   Updated: 2019/10/04 18:30:15 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::computor::{Computed, ComputorResult, TreeResult};
use crate::lexer::{
    token::{new_operator, Operator},
    Token,
};
use crate::memory::{Extension, Memory};

use std::any::Any;
use std::fmt;

type TTree = Box<dyn TokenTree>;

pub struct TreeBranch {
    token: Box<dyn Token>,
    branch_left: Option<TTree>,
    branch_right: Option<TTree>,
    was_expr: bool,
}

impl TreeBranch {
    pub fn new(token: Box<dyn Token>) -> Self {
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

    pub fn op_mut(&mut self) -> &mut dyn Operator {
        let extractor = &mut self.token;
        return extractor.as_op_mut().unwrap();
    }

    pub fn op_ref(&self) -> &dyn Operator {
        let extractor = &self.token;
        return extractor.as_op_ref().unwrap();
    }

    pub fn extract(&mut self, side_l: bool) -> Option<TTree> {
        if side_l {
            self.branch_left.take()
        } else {
            self.branch_right.take()
        }
    }

    pub fn default_to_left(leaf: &mut TTree, next: TTree) {
        let op = new_operator('*').unwrap();
        let mut new_tree = TreeBranch::new(op);
        new_tree.rot_left(next);
        let mut box_tree: TTree = Box::new(new_tree);
        std::mem::swap(leaf, &mut box_tree);
        let any = leaf.as_any_mut();
        let nw_left = any.downcast_mut::<TreeBranch>().unwrap();
        nw_left.rot_right(box_tree);
    }

    fn insert_default_left(&mut self, new: TTree) {
        let op = new_operator('*').unwrap();
        let mut new_tree = TreeBranch::new(op);
        new_tree.branch_left = Some(new);
        if !self.op_ref().is_prior(new_tree.op_ref()) {
            new_tree.branch_right = self.branch_left.take();
            self.branch_left = Some(Box::new(new_tree));
        } else {
            std::mem::swap(self, &mut new_tree);
            self.branch_right = Some(Box::new(new_tree));
        }
    }

    pub fn rot_left(&mut self, mut new: TTree) {
        match new.as_any_mut().downcast_mut::<TreeBranch>() {
            None => self.insert_leaf_left(new),
            Some(branch) => {
                if branch.op_ref().is_prior(self.op_ref()) {
                    self.insert_branch_left(new);
                } else {
                    std::mem::swap(self, branch);
                    self.insert_branch_right(new);
                }
            }
        };
    }

    pub fn insert_leaf_left(&mut self, new: TTree) {
        if let Some(node) = &mut self.branch_left {
            match node.as_any_mut().downcast_mut::<TreeBranch>() {
                None => self.insert_default_left(new),
                Some(branch) => {
                    if !branch.was_expr {
                        branch.insert_leaf_left(new)
                    } else {
                        self.insert_default_left(new)
                    }
                }
            };
        } else {
            self.branch_left = Some(new);
        }
    }

    fn insert_branch_left(&mut self, mut new: TTree) {
        let n_branch = new.as_any_mut().downcast_mut::<TreeBranch>().unwrap();
        if let Some(node) = &mut self.branch_left {
            match node.as_any_mut().downcast_mut::<TreeBranch>() {
                None => {
                    if !n_branch.was_expr {
                        n_branch.branch_right = self.branch_left.take();
                        self.branch_left = Some(new);
                    } else {
                        self.insert_default_left(new);
                    }
                }
                Some(branch) => {
                    match (branch.was_expr, n_branch.was_expr) {
                        (false, false) => {
                            if n_branch.op_ref().is_prior(branch.op_ref()) {
                                branch.insert_branch_left(new);
                            } else {
                                n_branch.branch_right = self.branch_left.take();
                                self.branch_left = Some(new);
                            }
                        }
                        (true, false) => {
                            n_branch.branch_right = self.branch_left.take();
                            self.branch_left = Some(new);
                        }
                        (false, true) => branch.insert_branch_left(new),
                        (true, true) => self.insert_default_left(new),
                    };
                }
            };
        } else {
            if !self.was_expr || n_branch.was_expr {
                self.branch_left = Some(new);
            } else {
                std::mem::swap(self, n_branch);
                self.insert_branch_right(new);
            }
        }
    }

    fn insert_default_right(&mut self, new: TTree) {
        let op = new_operator('*').unwrap();
        let mut new_tree = TreeBranch::new(op);
        new_tree.branch_right = Some(new);
        if !self.op_ref().is_prior(new_tree.op_ref()) {
            new_tree.branch_left = self.branch_right.take();
            self.branch_right = Some(Box::new(new_tree));
        } else {
            std::mem::swap(self, &mut new_tree);
            self.branch_left = Some(Box::new(new_tree));
        }
    }

    pub fn rot_right(&mut self, mut new: TTree) {
        match new.as_any_mut().downcast_mut::<TreeBranch>() {
            None => self.insert_leaf_right(new),
            Some(branch) => {
                if branch.op_ref().is_prior(self.op_ref()) {
                    self.insert_branch_right(new);
                } else {
                    std::mem::swap(self, branch);
                    self.insert_branch_left(new);
                }
            }
        };
    }

    pub fn insert_leaf_right(&mut self, new: TTree) {
        if let Some(node) = &mut self.branch_right {
            match node.as_any_mut().downcast_mut::<TreeBranch>() {
                None => self.insert_default_right(new),
                Some(branch) => {
                    if !branch.was_expr {
                        branch.insert_leaf_right(new)
                    } else {
                        self.insert_default_right(new)
                    }
                }
            };
        } else {
            self.branch_right = Some(new);
        }
    }

    fn insert_branch_right(&mut self, mut new: TTree) {
        let n_branch = new.as_any_mut().downcast_mut::<TreeBranch>().unwrap();
        if let Some(node) = &mut self.branch_right {
            match node.as_any_mut().downcast_mut::<TreeBranch>() {
                None => {
                    if !n_branch.was_expr {
                        n_branch.branch_left = self.branch_right.take();
                        self.branch_right = Some(new);
                    } else {
                        self.insert_default_right(new);
                    }
                }
                Some(branch) => {
                    match (branch.was_expr, n_branch.was_expr) {
                        (false, false) => {
                            if n_branch.op_ref().is_prior(branch.op_ref()) {
                                branch.insert_branch_right(new);
                            } else {
                                n_branch.branch_left = self.branch_right.take();
                                self.branch_right = Some(new);
                            }
                        }
                        (true, false) => {
                            n_branch.branch_left = self.branch_right.take();
                            self.branch_right = Some(new);
                        }
                        (false, true) => branch.insert_branch_right(new),
                        (true, true) => self.insert_default_right(new),
                    };
                }
            };
        } else {
            if self.op_ref().symbol() == '='
                || !self.was_expr
                || n_branch.was_expr
            {
                self.branch_right = Some(new);
            } else {
                std::mem::swap(self, n_branch);
                self.insert_branch_left(new);
            }
        }
    }
}

impl TokenTree for TreeBranch {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn token(&self) -> &Box<dyn Token> {
        &self.token
    }

    fn iter(&self, foo: fn(&Box<dyn Token>)) {
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

    fn count(&self, foo: fn(&Box<dyn Token>) -> i32) -> i32 {
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

    fn fix_exp(&mut self, mem: &Memory, var: &Vec<String>) -> ComputorResult {
        if let Some(left) = &mut self.branch_left {
            left.fix_exp(mem, var)?;
        }
        if let Some(right) = &mut self.branch_right {
            right.fix_exp(mem, var)?;
        }
        Ok(())
    }

    fn compute(
        &self,
        mem: &Memory,
        mut ext: Option<&mut Extension>,
    ) -> TreeResult {
        let orand_left = match &self.branch_left {
            None => Computed::None,
            Some(tree) => match &mut ext {
                Some(extend) => {
                    let mut ext_clone = extend.clone();
                    tree.compute(mem, Some(&mut ext_clone))?
                }
                None => tree.compute(mem, None)?,
            },
        };
        let orand_right = match &self.branch_right {
            None => Computed::None,
            Some(tree) => match &mut ext {
                Some(extend) => {
                    let mut ext_clone = extend.clone();
                    tree.compute(mem, Some(&mut ext_clone))?
                }
                None => tree.compute(mem, None)?,
            },
        };
        self.op_ref().exec(mem, orand_left, orand_right)
    }
}

impl fmt::Display for TreeBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.was_expr {
            match (&self.branch_left, &self.branch_right) {
                (Some(l), Some(r)) => write!(f, "{}{}{}", l, self.token, r),
                (Some(left), None) => write!(f, "{}{}", left, self.token),
                (None, Some(right)) => write!(f, "{}{}", self.token, right),
                (None, None) => write!(f, "{}", self.token),
            }
        } else {
            match (&self.branch_left, &self.branch_right) {
                (Some(l), Some(r)) => write!(f, "({}{}{})", l, self.token, r),
                (Some(left), None) => write!(f, "({}{})", left, self.token),
                (None, Some(right)) => write!(f, "({}{})", self.token, right),
                (None, None) => write!(f, "({})", self.token),
            }
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
            (None, Some(right)) => {
                write!(f, "{{{}:_ {:?}}}", self.token, right)
            }
            (None, None) => write!(f, "{{{}: _ _}}", self.token),
        }
    }
}
