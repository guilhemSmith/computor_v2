/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 11:31:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/17 16:42:47 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::arg_parse::Param;
use crate::parser::TokenTree;
use crate::Memory;

pub struct Computor {
    verbose: bool,
    bench: bool,
    memory: Memory,
}

impl Computor {
    pub fn new(param: &Param) -> Self {
        Computor {
            verbose: param.verbose(),
            bench: param.bench(),
            memory: Memory::new(),
        }
    }

    pub fn read_tokens(&mut self, tree: Box<TokenTree>) {
        if self.verbose {
            println!("[v:Computor] - Token received: {}", tree.token())
        }
    }
}
