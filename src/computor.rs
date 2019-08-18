/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 11:31:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/18 19:28:25 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::arg_parse::Param;
use crate::parser::TokenTree;
use crate::timer::Timer;
use crate::Memory;

pub struct Computor {
    verbose: bool,
    bench: bool,
    _memory: Memory,
}

impl Computor {
    pub fn new(param: &Param) -> Self {
        Computor {
            verbose: param.verbose(),
            bench: param.bench(),
            _memory: Memory::new(),
        }
    }

    pub fn read_tokens(&mut self, tree: Box<TokenTree>) {
        if self.verbose {
            println!("[v:Computor] - tree received: {}", tree)
        }
        if !self.bench {
            self.compute(tree);
        } else {
            let _timer = Timer::new("Computor");
            self.compute(tree);
        }
    }

    fn compute(&self, _tree: Box<TokenTree>) {}
}
