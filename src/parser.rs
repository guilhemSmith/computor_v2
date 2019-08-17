/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parser.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:16:31 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 16:27:03 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod token_tree;
mod tree_branch;
mod tree_leaf;

pub use token_tree::{token_to_node, TokenTree};
pub use tree_branch::TreeBranch;
pub use tree_leaf::TreeLeaf;

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
        tokens: Vec<Rc<Token>>,
    ) -> Option<Rc<TokenTree>> {
        let mut tree: Rc<TokenTree>;
        let prev_node: Rc<TokenTree>;

        // match tokens.len() {
        // 	0 => None,
        // 	1 => tokens.pop(),
        // 	_ => {None}
        // }
        for token in tokens.iter() {
            let current_node = token_to_node(token);
        }
        None
        // match tokens.pop() {
        //     Some(token) => match token.as_operator() {
        //         Some(op) => Some(Rc::new(TreeBranch::new(op, token))),
        //         None => Some(Rc::new(TreeLeaf::new(token))),
        //     },
        //     None => None,
        // }
    }
}
