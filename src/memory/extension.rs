/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   extension.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/21 10:31:53 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/30 17:14:06 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Variable;

use crate::types::Imaginary;

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

    pub fn add(&mut self, name: &String, val: Imaginary) {
        let lowercase = name.to_lowercase();
        let mut new_tmp = Variable::new(name.clone());
        new_tmp.set(Some(val));
        self.tmp_var.insert(lowercase, new_tmp);
    }

    pub fn get(&self, name: &String) -> Option<Imaginary> {
        let lowercase = name.to_lowercase();
        match self.tmp_var.get(&lowercase) {
            None => None,
            Some(var) => var.get(),
        }
    }
}
