/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parser.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:16:31 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/18 17:06:33 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod token_tree;
mod tree_branch;
mod tree_leaf;

pub use token_tree::TokenTree;
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
        mut tokens: Vec<Rc<Token>>,
    ) -> Option<Box<TokenTree>> {
        let mut tree: Box<TokenTree>;

        match tokens.pop() {
            Some(token) => tree = token_tree::token_to_node(token),
            None => return None,
        }
        loop {
            match tokens.pop() {
                None => return Some(tree),
                Some(token) => {
                    let new_node = token_tree::token_to_node(token);
                    token_tree::insert_in_tree(&mut tree, new_node);
                }
            };
        }
    }
}
