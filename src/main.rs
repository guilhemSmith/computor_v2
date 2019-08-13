/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/13 17:08:35 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod arg_parse;
mod error;
mod lexer;
mod memory;
mod timer;
mod types;

use crate::arg_parse::Param;
use crate::error::ErrorKind;
use crate::lexer::Lexer;
use crate::memory::Memory;
use crate::timer::Timer;
use std::{env, process};

use crate::lexer::Expression;

fn main() {
    let argv: Vec<String> = env::args().collect();

    println!("Computor start.");
    match computor(argv.len(), argv) {
        err if err > 0 => {
            println!("Computor stop.");
            process::exit(1)
        }
        _ => println!("Computor stop."),
    }
}

fn computor(argc: usize, argv: Vec<String>) -> u32 {
    let param = match Param::new(argc, argv) {
        Some(p) => p,
        None => return 1,
    };
    if !param.run() {
        return 0;
    }
    let mut lex = Lexer::new(param.verbose(), param.bench());
    let mut mem = Memory::new();

    loop {
        match lex.read_input() {
            Ok(tokens) => {
                println!("{}", lexer::token::tokens_to_string(&tokens));
                // let expr = Expression::new(tokens);
                // match expr.check_errors(param.verbose()) {
                //     nb if nb > 0 => eprintln!(
                //         "[err-Lexer:] - {} error(s) detected. {}.",
                //         nb, "Expression computing aborted"
                //     ),
                //     _ => {
                //         let result = if !param.bench() {
                //             expr.compute(param.verbose())
                //         } else {
                //             let _timer = Timer::new("Computing");
                //             expr.compute(param.verbose())
                //         };
                //         match result {
                //             Ok(expr) => println!(
                //                 "{}{}",
                //                 if param.verbose() {
                //                     "[V:result] - "
                //                 } else {
                //                     ""
                //                 },
                //                 expr
                //             ),
                //             Err(err) => eprintln!("{}", err),
                //         };
                //     }
                // };
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
