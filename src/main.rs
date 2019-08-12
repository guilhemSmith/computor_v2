/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/12 17:35:20 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod arg_parse;
mod error;
mod lexer;
mod memory;
mod timer;
mod types;

use crate::arg_parse::Param;
use crate::lexer::{Expression, Lexer};
use crate::memory::Memory;
use crate::timer::Timer;
use std::{env, process};

use crate::types::{Imaginary, Raw};

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
    let mut mem = Memory::new();
    mem.set_var(String::from("Yo"), Imaginary::zero());
    mem.set_var(
        String::from("A"),
        Imaginary::new(Raw::Float(7.25), Raw::Zero),
    );
    mem.set_fun(
        String::from("foo"),
        vec![String::from("x"), String::from("y")],
        match Expression::new(String::from("120*9i+30"), 0) {
            Ok(exp) => exp,
            Err(err) => {
                println!("{}", err);
                return 3;
            }
        },
    );
    mem.set_var(String::from("B"), Imaginary::zero());
    mem.set_var(
        String::from("B"),
        Imaginary::new(Raw::Float(-0.25), Raw::Zero),
    );
    println!("{}", mem);

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
                match expr.check_errors(param.verbose()) {
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
