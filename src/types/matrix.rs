/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   matrix.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/09/23 10:31:02 by gsmith            #+#    #+#             */
/*   Updated: 2019/10/05 10:12:09 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Imaginary, OpResult};
use crate::computor::ComputorError;

use std::fmt;

#[derive(Debug)]
pub enum MatrixError {
    IncoherentSizes,
    InvalidVal(String),
    InvalidFormat,
}

impl std::error::Error for MatrixError {}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatrixError::IncoherentSizes => {
                write!(f, "Matrix row sizes are not equal.")
            }
            MatrixError::InvalidVal(s) => {
                write!(f, "Could not read the value '{}'.", s)
            }
            MatrixError::InvalidFormat => {
                write!(f, "Matrix is not correctly formatted.")
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix {
    width: u32,
    height: u32,
    data: Vec<Imaginary>,
}

impl Matrix {
    pub fn new(width: u32, height: u32) -> Self {
        Matrix {
            width,
            height,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, val: Imaginary) {
        self.data.push(val);
    }

    pub fn apply_mul(&self, coef: Imaginary) -> OpResult<Self> {
        let mut data: Vec<Imaginary> = Vec::new();
        for val in self.data.iter() {
            data.push(coef.mul(val)?);
        }
        Ok(Matrix {
            width: self.width,
            height: self.height,
            data,
        })
    }

    pub fn add(&self, other: &Matrix) -> OpResult<Self> {
        let mut data: Vec<Imaginary> = Vec::new();
        if self.width != other.width || self.height != other.height {
            return Err(ComputorError::matrix_dim(false));
        }
        for i in 0..self.data.len() {
            data.push(self.data[i].add(&other.data[i])?)
        }
        Ok(Matrix {
            width: self.width,
            height: self.height,
            data,
        })
    }

    pub fn sub(&self, other: &Matrix) -> OpResult<Self> {
        let mut data: Vec<Imaginary> = Vec::new();
        if self.width != other.width || self.height != other.height {
            return Err(ComputorError::matrix_dim(false));
        }
        for i in 0..self.data.len() {
            data.push(self.data[i].sub(&other.data[i])?)
        }
        Ok(Matrix {
            width: self.width,
            height: self.height,
            data,
        })
    }

    pub fn mul(&self, other: &Matrix) -> OpResult<Self> {
        if self.width != other.height {
            return Err(ComputorError::matrix_dim(true));
        }
        let mut data: Vec<Imaginary> = Vec::new();
        let width = other.width;
        let height = self.height;
        for i in 0..width * height {
            let mut val = Imaginary::new(0.0, 0.0);
            for j in 0..self.width {
                let l = (i / width * self.width + j) as usize;
                let r = (j * width + i % width) as usize;
                let new = self.data[l].mul(&other.data[r])?;
                val = val.add(&new)?;
            }
            data.push(val);
        }
        Ok(Matrix {
            width,
            height,
            data,
        })
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut print = String::new();
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        for cell in self.data.iter() {
            if i == 0 {
                print.push_str("[ ");
            }
            i += 1;
            print += &cell.to_string();
            if i == self.width {
                print.push_str(" ]");
                i = 0;
                j += 1;
                if j < self.height {
                    print.push_str(" ; ");
                }
            } else {
                print.push_str(" , ");
            }
        }
        write!(f, "{}", print)
    }
}
