/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   variable.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/08 18:28:27 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/08 18:50:40 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::types::Imaginary;

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

    pub fn set(&mut self, im: Imaginary) {
        self.value = Some(im);
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
