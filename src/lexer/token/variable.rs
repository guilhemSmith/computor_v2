/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   variable.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:16:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/14 15:16:13 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::error::ComputorError;
use std::fmt;

#[derive(Clone)]
pub struct Variable {
    id: String,
}

impl Variable {
    pub fn new(id: String) -> Result<Self, ComputorError> {
        let mut check = id.chars();
        if !check.next().unwrap().is_alphabetic() {
            return Err(ComputorError::invalid_token(id));
        }
        loop {
            match check.next() {
                Some(ch) if !ch.is_alphanumeric() => {
                    return Err(ComputorError::invalid_token(id))
                }
                Some(_) => {}
                None => break,
            }
        }
        Ok(Variable { id: id })
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
