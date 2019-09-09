/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parser.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:16:31 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/09 11:56:41 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod token_tree;
mod tree_branch;
mod tree_leaf;

pub use token_tree::TokenTree;
pub use tree_branch::TreeBranch;
pub use tree_leaf::TreeLeaf;

use crate::lexer::token::{
    self, Expression, FunctionToken, FunctionTree, LexerError, Operator,
};

use crate::arg_parse::Param;
use crate::lexer::Token;
use crate::timer::Timer;

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
        tokens: Vec<Box<dyn Token>>,
    ) -> Option<Box<dyn TokenTree>> {
        if self.verbose {
            println!(
                "[v:Parser] - token stack received : {}",
                token::debug_token(&tokens)
            );
        }
        if !self.bench {
            self.parse(tokens)
        } else {
            let display = format!("Parser({})", token::display_token(&tokens));
            let _timer = Timer::new(&display[..]);
            self.parse(tokens)
        }
    }

    fn parse(
        &self,
        mut tokens: Vec<Box<dyn Token>>,
    ) -> Option<Box<dyn TokenTree>> {
        let mut tree: Box<dyn TokenTree>;

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

    fn token_to_node(
        &self,
        mut token: Box<dyn Token>,
    ) -> Option<Box<dyn TokenTree>> {
        let op = &mut token;
        match op.as_any_mut().downcast_mut::<Operator>() {
            None => match op.as_any_mut().downcast_mut::<Expression>() {
                None => match op.as_any_mut().downcast_mut::<FunctionToken>() {
                    None => Some(Box::new(TreeLeaf::new(token))),
                    Some(fun) => self.fun_to_node(fun),
                },
                Some(exp) => self.expr_to_node(exp),
            },
            Some(_) => Some(Box::new(TreeBranch::new(token))),
        }
    }

    fn expr_to_node(&self, exp: &mut Expression) -> Option<Box<dyn TokenTree>> {
        let mut exp_token = self.parse_tokens(exp.consume_tokens());
        match &mut exp_token {
            None => {}
            Some(tokens) => tokens.set_as_exp(),
        };
        exp_token
    }

    fn fun_to_node(
        &self,
        fun: &mut FunctionToken,
    ) -> Option<Box<dyn TokenTree>> {
        let id = fun.id().clone();
        let mut param_tree: Vec<Box<dyn TokenTree>> = Vec::new();
        for param in fun.consume_param() {
            let arg = self.parse_tokens(param);
            match arg {
                None => {
                    let error = LexerError::InvalidPar(String::from(""));
                    param_tree.push(Box::new(TreeLeaf::new(Box::new(error))));
                }
                Some(boxed_tree) => param_tree.push(boxed_tree),
            }
        }
        let token: Box<dyn Token> = Box::new(FunctionTree::new(id, param_tree));
        return Some(Box::new(TreeLeaf::new(token)));
    }
}
