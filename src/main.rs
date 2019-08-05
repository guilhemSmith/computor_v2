/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/05 18:14:05 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod error;
mod lexer;
mod types;

use crate::error::log_error;
use crate::lexer::{Expression, Lexer};

fn main() {
    let mut lex = Lexer::new(true);

    loop {
        match lex.read_input() {
            Ok(mut expr) => {
                if expr.is_empty() {
                    break;
                }
                expr.compute(true);
                println!("{}", expr);
            }
            Err(err) => log_error(err, 0),
        }
    }
}
