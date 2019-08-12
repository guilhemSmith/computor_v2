/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   variable.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/08 18:28:27 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/12 15:33:33 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::types::Imaginary;
use std::fmt;

pub struct Variable {
    name: String,
    value: Option<Imaginary>,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Variable {
            name: name,
            value: None,
        }
    }

    pub fn set(&mut self, im: Option<Imaginary>) {
        self.value = im;
    }

    pub fn get(&self) -> Option<Imaginary> {
        self.value
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(im) => write!(f, "{}: {}", self.name, im),
            None => write!(f, "{}: unknown", self.name),
        }
    }
}
