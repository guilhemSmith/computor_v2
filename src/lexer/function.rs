/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:22:09 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/13 17:32:44 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Variable;
use std::fmt;

#[derive(Clone)]
pub struct Function {
    id: String,
    param: Vec<Variable>,
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
