/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 11:31:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/15 18:02:20 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::lexer::{
    token::{debug_token, display_token},
    Token,
};
use crate::Memory;
use std::rc::Rc;

pub struct Computor {
    verbose: bool,
    bench: bool,
    memory: Memory,
}

impl Computor {
    pub fn new(verbose: bool, bench: bool) -> Self {
        Computor {
            verbose: verbose,
            bench: bench,
            memory: Memory::new(),
        }
    }

    pub fn read_tokens(&mut self, tokens: Vec<Rc<Token>>) {
        if self.verbose {
            println!(
                "[v:Computor] - Token received: {}",
                debug_token(&tokens, "")
            )
        }
        let iter = tokens.split(|token| token.is_equal());
        for vec in iter {
            for tok in vec {
                println!("tok: {}", tok);
            }
            println!("new side");
        }
    }
}
