/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:22:09 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/15 09:40:13 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Token;
use std::fmt;

#[derive(Clone)]
pub struct Function {
    id: String,
    param: Vec<Token>,
}

impl Function {
    pub fn new(id: String, vars: Vec<Token>) -> Self {
        let mut new = Function {
            id: id,
            param: vars,
        };
        return new;
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut param = String::new();
        let mut iter = self.param.iter();

        loop {
            match iter.next() {
                Some(var) => param = format!("{}, {}", param, var),
                None => break,
            }
        }
        write!(f, "{}({})", self.id, param.trim_start_matches(", "))
    }
}
