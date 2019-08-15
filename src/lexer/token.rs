/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   token.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/30 14:43:15 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/15 17:37:05 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod equal;
mod expression;
mod function;
mod lexer_error;
mod operator;
mod resolve;
mod value;
mod variable;

pub use equal::Equal;
pub use expression::Expression;
pub use function::Function;
pub use lexer_error::LexerError;
pub use operator::Operator;
pub use resolve::Resolve;
pub use value::Value;
pub use variable::Variable;

use std::fmt;
use std::rc::Rc;

pub trait Token: fmt::Display + fmt::Debug {}

pub fn debug_token(tokens: &Vec<Rc<Token>>, sep: &str) -> String {
    let mut debug = String::new();

    for token in tokens {
        debug.push_str(&format!("{}{:?}", sep, token)[..]);
    }
    return String::from(debug.trim_start_matches(sep));
}

pub fn display_token(tokens: &Vec<Rc<Token>>, sep: &str) -> String {
    let mut display = String::new();

    for token in tokens {
        display.push_str(&format!("{}{}", sep, token)[..]);
    }
    return String::from(display.trim_start_matches(sep));
}
