/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   memory.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/08 18:14:00 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/08 19:07:13 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod function;
mod variable;

pub use function::Function;
pub use variable::Variable;

use crate::types::Imaginary;
use std::collections::HashMap;

pub struct Memory<'v, 'f> {
    var: HashMap<&'v String, Variable>,
    fun: HashMap<&'f String, Function<'v>>,
}

impl<'v, 'f> Memory<'v, 'f> {
    pub fn new() -> Self {
        Memory {
            var: HashMap::new(),
            fun: HashMap::new(),
        }
    }
}
