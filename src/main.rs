/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/15 12:10:44 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod arg_parse;
mod computor;
mod error;
mod lexer;
mod memory;
mod timer;
mod types;

use crate::arg_parse::Param;
use crate::computor::Computor;
use crate::error::ErrorKind;
use crate::lexer::Lexer;
use crate::memory::Memory;
use crate::timer::Timer;
use std::{env, process};

fn main() {
    println!("Computor start.");
    let exit_code = main_wrapped();
    println!("Computor stop.");
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
    let mut lex = Lexer::new(param.verbose(), param.bench());
    let mut computor = Computor::new(param.verbose(), param.bench());

    loop {
        match lex.read_input() {
            Ok(tokens) => {
                if !param.bench() {
                    computor.read_tokens(tokens);
                } else {
                    let _timer = Timer::new("Computor");
                    computor.read_tokens(tokens);
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
