/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/06 14:36:37 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod error;
mod lexer;
mod types;

use crate::error::log_error;
use crate::lexer::Lexer;

fn main() {
    let mut lex = Lexer::new(true);

    loop {
        match lex.read_input() {
            Ok(expr) => {
                if expr.is_empty() {
                    break;
                }
                match expr.check_errors() {
                    nb if nb > 0 => eprintln!(
                        "[err-Lexer:] - {} error(s) detected. {}.",
                        nb, "Expression computing aborted"
                    ),
                    _ => match expr.compute(true) {
                        Ok(result) => {
                            println!("orignal: {}\nresult: {}", expr, result)
                        }
                        Err(err) => log_error(&err, None),
                    },
                };
            }
            Err(err) => log_error(&err, None),
        }
    }
}
