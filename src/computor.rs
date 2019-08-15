/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 11:31:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/15 12:56:27 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::lexer::{token, Token};
use crate::Memory;
use std::collections::LinkedList;

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

    pub fn read_tokens(&mut self, tokens: LinkedList<Token>) {
        if self.verbose {
            println!(
                "[v:Computor] - Token received: {}",
                token::tokens_to_debug(&tokens)
            )
        }
    }
}
