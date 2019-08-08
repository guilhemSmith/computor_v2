/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/08 13:07:14 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod arg_parse;
mod error;
mod lexer;
mod types;

use crate::arg_parse::Param;
use crate::error::log_error;
use crate::lexer::Lexer;
use std::{env, process};

fn main() {
    let argv: Vec<String> = env::args().collect();
    match computor(argv.len(), argv) {
        err if err > 0 => process::exit(1),
        _ => {}
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
    let mut lex = Lexer::new(param.verbose());

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
                    _ => match expr.compute(param.verbose()) {
                        Ok(result) => println!(
                            "{}{}",
                            if param.verbose() { "[V:Result] - " } else { "" },
                            result
                        ),
                        Err(err) => log_error(&err, None),
                    },
                };
            }
            Err(err) => {
                log_error(&err, None);
                return 2;
            }
        }
    }
    return 0;
}
