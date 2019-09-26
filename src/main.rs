/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/26 16:05:31 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod arg_parse;
mod computor;
mod lexer;
mod memory;
mod parser;
mod timer;
mod types;

use crate::arg_parse::Param;
use crate::computor::Computor;
use crate::computor::ErrorKind;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::timer::Timer;
use std::{env, process};

fn main() {
    let exit_code = main_wrapped();
    process::exit(exit_code);
}

fn main_wrapped() -> i32 {
    let argv: Vec<String> = env::args().collect();
    let param = match Param::new(argv.len(), argv) {
        Some(p) => p,
        None => return 1,
    };
    if !param.run() {
        return 0;
    }
    let mut lex = Lexer::new(&param);
    let mut pars = Parser::new(&param);
    let mut computor = Computor::new(&param);

    loop {
        match lex.read_input() {
            Ok(tokens) => {
                if tokens.len() != 0 {
                    let tree = pars.parse_tokens(tokens);
                    if let Some(root) = tree {
                        if let Err(err) = computor.read_tokens(root) {
                            eprintln!("{}", err);
                        }
                    }
                } else {
                    pars.update_param(lex.verbose(), lex.benchmark());
                    computor.update_param(lex.verbose(), lex.benchmark());
                }
            }
            Err(err) => {
                if *err.kind() == ErrorKind::IOStop {
                    println!("{}", err);
                    break;
                } else {
                    eprintln!("{}", err);
                }
            }
        }
    }
    return 0;
}
