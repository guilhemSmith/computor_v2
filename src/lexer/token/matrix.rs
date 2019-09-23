/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   matrix.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/09/23 13:51:19 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/23 17:15:47 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{LexerError, Token};
use crate::computor::{Computed, TreeResult};
use crate::memory::{Extension, Memory};
use crate::types::Matrix as Matrix_type;

use std::any::Any;
use std::fmt;

pub struct Matrix {
    mat: Matrix_type,
}

impl Matrix {
    pub fn new(raw: String) -> Result<Matrix, LexerError> {
        match Matrix_type::parse(&raw) {
            Err(err) => Err(LexerError::InvalidMat(err)),
            Ok(mat) => Ok(Matrix { mat }),
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.mat.to_string().replace("\n", " ; "))
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[mat:{}]", self)
    }
}

impl Token for Matrix {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> TreeResult {
        Ok(Computed::Mat(self.mat.clone()))
    }
}
