/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parser.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:16:31 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 18:52:13 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod token_tree;
mod tree_branch;
mod tree_leaf;

pub use token_tree::{token_to_node, TokenTree};
pub use tree_branch::TreeBranch;
pub use tree_leaf::TreeLeaf;

use crate::arg_parse::Param;
use crate::lexer::{token::Operator, Token};

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
    ) -> Option<Box<TokenTree>> {
        let mut tree: Box<TokenTree>;
        let mut tokens_iter = tokens.iter();

        match tokens_iter.next() {
            Some(token) => tree = token_to_node(token),
            None => return None,
        }
        for token in tokens_iter {
            let mut current_node = token_to_node(token);
            let curr = current_node.as_any().downcast_ref::<TreeBranch>();
            let prev = tree.as_any().downcast_ref::<TreeBranch>();
            tree = match (curr, prev) {
                (Some(op_left), Some(op_right)) => {
                    if op_right.operator().is_prior(op_left.operator()) {
                        current_node.set_branch_left(tree);
                        current_node
                    } else {
                        tree.set_branch_right(current_node);
                        tree
                    }
                }
                (Some(_), None) => {
                    current_node.set_branch_left(tree);
                    current_node
                }
                (None, Some(_)) => {
                    tree.set_branch_right(current_node);
                    tree
                }
                (None, None) => {
                    let tok: Rc<Token> = Rc::new(Operator::new('*').unwrap());
                    let mut new_node = TreeBranch::new(&tok);
                    new_node.set_branch_left(current_node);
                    new_node.set_branch_right(tree);
                    Box::new(new_node)
                }
            }
        }
        let foo = |tok: &Rc<Token>| println!("node:{:?}", tok);
        tree.iter(foo);
        return Some(tree);
    }
}
