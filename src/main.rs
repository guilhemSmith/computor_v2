/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/26 17:18:50 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod error;
mod lexer;
mod types;

use crate::lexer::Lexer;

fn main() {
    let mut lex = Lexer::new(true);
    loop {
        match lex.read_input() {
            Some(_) => {},
            None => break,
        }
    }
}
