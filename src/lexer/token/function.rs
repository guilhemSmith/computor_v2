/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   function.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:22:09 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/18 17:27:37 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use std::any::Any;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Function {
    id: String,
    param: Vec<Rc<Token>>,
}

impl Function {
    pub fn new(id: String, vars: Vec<Rc<Token>>) -> Result<Self, LexerError> {
        let mut chars = id.chars();

        if !chars.next().unwrap().is_alphabetic() {
            return Err(LexerError::InvalidFun(id, vars));
        }
        for ch in chars {
            if !ch.is_alphanumeric() {
                return Err(LexerError::InvalidFun(id, vars));
            }
        }
        Ok(Function {
            id: id,
            param: vars,
        })
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let param = super::display_token(&self.param, ", ");
        write!(f, "{}({})", self.id, param)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let param = super::debug_token(&self.param, ", ");
        write!(f, "[fun:{}({})]", self.id, param)
    }
}

impl Token for Function {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
