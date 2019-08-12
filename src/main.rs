/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/12 11:27:44 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod arg_parse;
mod error;
mod lexer;
mod memory;
mod timer;
mod types;

use crate::arg_parse::Param;
use crate::lexer::Lexer;
use crate::timer::Timer;
use std::{env, process};

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
    let lex = Lexer::new(param.verbose(), param.bench());

    loop {
        match lex.read_input() {
            Ok(expr) => {
                if expr.is_empty() {
                    if param.verbose() {
                        println!(
                            "[V:Computor] -  {}",
                            "empty input detected, stopping execution."
                        )
                    }
                    break;
                }
                match expr.check_errors() {
                    nb if nb > 0 => eprintln!(
                        "[err-Lexer:] - {} error(s) detected. {}.",
                        nb, "Expression computing aborted"
                    ),
                    _ => {
                        let result = if !param.bench() {
                            expr.compute(param.verbose())
                        } else {
                            let _timer = Timer::new("Computing");
                            expr.compute(param.verbose())
                        };
                        match result {
                            Ok(expr) => println!(
                                "{}{}",
                                if param.verbose() {
                                    "[V:result] - "
                                } else {
                                    ""
                                },
                                expr
                            ),
                            Err(err) => println!("{}", err),
                        };
                    }
                };
            }
            Err(err) => {
                println!("{}", err);
                return 2;
            }
        }
    }
    return 0;
}
