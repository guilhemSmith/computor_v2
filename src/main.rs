/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/06 13:10:42 by gsmith           ###   ########.fr       */
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
            Ok(mut expr) => {
                if expr.is_empty() {
                    break;
                }
                match expr.compute(true) {
                    Ok(result) => println!("orignal: {}\nresult: {}", expr, result),
                    Err(err) => log_error(err, 0),
                };
            }
            Err(err) => log_error(err, 0),
        }
    }
}
