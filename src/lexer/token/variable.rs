/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   variable.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:16:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/15 16:52:48 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use std::fmt;

#[derive(Clone)]
pub struct Variable {
    id: String,
}

impl Variable {
    pub fn new(id: String) -> Result<Self, LexerError> {
        let mut chars = id.chars();

        if !chars.next().unwrap().is_alphabetic() {
            return Err(LexerError::InvalidVar(id));
        }
        for ch in chars {
            if !ch.is_alphanumeric() {
                return Err(LexerError::InvalidVar(id));
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

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[var:{}]", self)
    }
}

impl Token for Variable {}
