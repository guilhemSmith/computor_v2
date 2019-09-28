/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   variable.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/08 18:28:27 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/28 13:01:44 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::types::{Imaginary, Matrix};
use std::fmt;

#[derive(Clone)]
pub struct Variable {
    name: String,
    val: Value,
}

impl Variable {
    pub fn new(name: String, val: Value) -> Self {
        Variable { name, val }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn val(&self) -> Value {
        self.val.clone()
    }
}

#[derive(Clone)]
pub enum Value {
    Im(Imaginary),
    Mat(Matrix),
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.val {
            Value::Im(val) => write!(f, "{}: {}", self.name, val),
            Value::Mat(val) => write!(
                f,
                "{}:\n{}",
                self.name,
                val.to_string().replace(" ; ", "\n")
            ),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Im(val) => write!(f, "{}", val),
            Value::Mat(val) => {
                write!(f, "{}", val.to_string().replace(" ; ", "\n"))
            }
        }
    }
}
