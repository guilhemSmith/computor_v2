/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   extension.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/21 10:31:53 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/25 17:37:46 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Value, Variable};

use std::collections::HashMap;

#[derive(Clone)]
pub struct Extension {
    tmp_var: HashMap<String, Variable>,
}

impl Extension {
    pub fn new() -> Self {
        Extension {
            tmp_var: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &String, val: Value) {
        self.tmp_var
            .insert(name.clone(), Variable::new(name.clone(), val));
    }

    pub fn get(&self, name: &String) -> Option<&Variable> {
        match self.tmp_var.get(name) {
            None => None,
            Some(var) => Some(var),
        }
    }
}
