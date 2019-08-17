/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parser.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/17 11:16:31 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 11:53:29 by gsmith           ###   ########.fr       */
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
    ) -> Option<Rc<TokenTree>> {
        match tokens.pop() {
            Some(token) => Some(Rc::new(TreeLeaf::new(token))),
            None => None,
        }
    }
}
