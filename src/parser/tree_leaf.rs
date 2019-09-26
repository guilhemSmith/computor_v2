/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tree_leaf.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:15:13 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/26 17:13:12 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::TokenTree;
use crate::computor::{ComputorError, ComputorResult, TreeResult};
use crate::lexer::{
    token::{FunctionTree, MatrixComp, MatrixTree, Value, Variable},
    Token,
};
use crate::memory::{Extension, Memory, Value as Val};

use std::any::Any;
use std::fmt;

pub struct TreeLeaf {
    token: Box<dyn Token>,
}

impl TreeLeaf {
    pub fn new(token: Box<dyn Token>) -> Self {
        TreeLeaf { token: token }
    }
}

impl TokenTree for TreeLeaf {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn token(&self) -> &Box<dyn Token> {
        &self.token
    }

    fn iter(&self, foo: fn(&Box<dyn Token>)) {
        foo(self.token());
    }

    fn count(&self, foo: fn(&Box<dyn Token>) -> i32) -> i32 {
        foo(&self.token)
    }

    fn is_full(&self) -> bool {
        true
    }

    fn set_as_exp(&mut self) {}

    fn fix_exp(&mut self, mem: &Memory, vars: &Vec<String>) -> ComputorResult {
        let any = self.token.as_any_mut();
        let var = any.downcast_mut::<Variable>();
        if let Some(var) = var {
            let id = var.id();
            for arg in vars.iter() {
                if *arg == *id {
                    return Ok(());
                }
            }
            let mut new: Box<dyn Token> = match mem.get_var(id) {
                None => {
                    return Err(ComputorError::unknown_id(id.clone(), true))
                }
                Some(var) => match var.val() {
                    Val::Im(val) => Box::new(Value::from(val)),
                    Val::Mat(mat) => Box::new(MatrixComp::new(mat)),
                },
            };
            std::mem::swap(&mut new, &mut self.token);
        } else {
            let fun = any.downcast_mut::<FunctionTree>();
            if let Some(fun) = fun {
                let args = fun.param_mut();
                for arg in args.iter_mut() {
                    arg.fix_exp(mem, vars)?;
                }
            } else {
                let mat = any.downcast_mut::<MatrixTree>();
                if let Some(mat) = mat {
                    let trees = mat.trees_mut();
                    for tree in trees.iter_mut() {
                        tree.fix_exp(mem, vars)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn compute(&self, mem: &Memory, ext: Option<&mut Extension>) -> TreeResult {
        self.token.get_result(mem, ext)
    }
}

impl fmt::Display for TreeLeaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl fmt::Debug for TreeLeaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{:?}}}", self.token)
    }
}
