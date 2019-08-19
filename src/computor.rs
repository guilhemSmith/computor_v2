/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 11:31:54 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/19 10:34:22 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod result;
pub use result::ComputorResult;

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
            println!("[v:Computor] - tree received: {:?}", tree)
        }
        if !self.bench {
            self.compute(tree);
        } else {
            let display = format!("Computor({})", tree);
            let _timer = Timer::new(&display[..]);
            self.compute(tree);
        }
    }

    fn compute(&self, _tree: Box<TokenTree>) {}
}
