/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/08 18:14:20 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/08 18:57:31 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Variable;
use crate::lexer::Expression;
use std::collections::HashMap;

pub struct Function<'v> {
    name: String,
    var: HashMap<&'v String, &'v Variable>,
    expr: Option<Expression>,
}

impl<'v> Function<'v> {
    pub fn new(name: String) -> Self {
        Function {
            name: name,
            var: HashMap::new(),
            expr: None,
        }
    }

    pub fn set(&mut self, mut vars: Vec<&'v Variable>, expr: Expression) {
        loop {
            match vars.pop() {
                Some(var) => self.var.insert(var.name(), var),
                None => break,
            };
        }
        self.expr = Some(expr);
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
