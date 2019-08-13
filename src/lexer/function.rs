/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:22:09 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/13 17:42:22 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Variable;
use std::fmt;

#[derive(Clone)]
pub struct Function {
    id: String,
    param: Vec<Variable>,
}

impl Function {
    pub fn new(id: String, vars: String) -> Self {
        let mut new = Function {
            id: id,
            param: Vec::new(),
        };
        let mut var_name = vars.split_terminator(", ");

        loop {
            match var_name.next() {
                Some(name) => new.param.push(Variable::new(String::from(name))),
                None => break,
            };
        }
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
