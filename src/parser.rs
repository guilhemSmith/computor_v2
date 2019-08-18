/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parser.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:16:31 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/18 17:51:21 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod token_tree;
mod tree_branch;
mod tree_leaf;

pub use token_tree::TokenTree;
pub use tree_branch::TreeBranch;
pub use tree_leaf::TreeLeaf;

use crate::lexer::token::{Expression, Operator};

use crate::arg_parse::Param;
use crate::lexer::Token;

use std::rc::Rc;

pub struct Parser {
    verbose: bool,
    bench: bool,
}

impl Parser {
    pub fn new(param: &Param) -> Self {
        Parser {
            verbose: param.verbose(),
            bench: param.bench(),
        }
    }

    pub fn parse_tokens(
        &self,
        mut tokens: Vec<Rc<Token>>,
    ) -> Option<Box<TokenTree>> {
        let mut tree: Box<TokenTree>;

        match tokens.pop() {
            Some(token) => {
                tree = match self.token_to_node(token) {
                    None => return None,
                    Some(token) => token,
                }
            }
            None => return None,
        }
        loop {
            match tokens.pop() {
                None => return Some(tree),
                Some(token) => {
                    let new_node = match self.token_to_node(token) {
                        None => continue,
                        Some(token) => token,
                    };
                    token_tree::insert_in_tree(&mut tree, new_node);
                }
            };
        }
    }

    fn token_to_node(&self, mut token: Rc<Token>) -> Option<Box<TokenTree>> {
        let op = Rc::get_mut(&mut token).unwrap();
        match op.as_any().downcast_mut::<Operator>() {
            None => match op.as_any().downcast_mut::<Expression>() {
                None => Some(Box::new(TreeLeaf::new(token))),
                Some(exp) => {
                    let mut exp_token = self.parse_tokens(exp.consume_tokens());
                    match &mut exp_token {
                        None => {}
                        Some(tokens) => tokens.set_prior_as_exp(),
                    };
                    exp_token
                }
            },
            Some(_) => Some(Box::new(TreeBranch::new(token))),
        }
    }
}
