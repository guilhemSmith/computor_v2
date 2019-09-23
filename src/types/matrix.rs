/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   matrix.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/09/23 10:31:02 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/23 14:16:48 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Imaginary;

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
                write!(f, "Matrix row sizes are not the equal.")
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

#[derive(Clone, Debug)]
pub struct Matrix {
    width: u32,
    height: u32,
    data: Vec<Vec<Imaginary>>,
}

impl Matrix {
    pub fn parse(raw_data: &str) -> Result<Self, MatrixError> {
        let width: u32;
        let mut height: u32 = 0;
        let mut data: Vec<Vec<Imaginary>> = Vec::new();

        if !raw_data.starts_with("[") || !raw_data.ends_with("]") {
            return Err(MatrixError::InvalidFormat);
        }
        let mut raw_rows = raw_data[1..raw_data.len() - 1].split(";");
        match raw_rows.next() {
            Some(first_row) => {
                let first = new_row(first_row)?;
                width = first.len() as u32;
                height += 1;
                data.push(first);
            }
            None => {
                return Ok(Matrix {
                    width: 0,
                    height: 0,
                    data: Vec::new(),
                })
            }
        }
        for raw_row in raw_rows {
            let row = new_row(raw_row)?;
            if row.len() as u32 != width {
                return Err(MatrixError::IncoherentSizes);
            }
            height += 1;
            data.push(row);
        }
        Ok(Matrix {
            width,
            height,
            data,
        })
    }
}

fn new_row(raw_row: &str) -> Result<Vec<Imaginary>, MatrixError> {
    if !raw_row.starts_with("[") || !raw_row.ends_with("]") {
        return Err(MatrixError::InvalidFormat);
    }
    let mut row: Vec<Imaginary> = Vec::new();
    let raw_cells = raw_row[1..raw_row.len() - 1].split(",");
    for raw_cell in raw_cells {
        match Imaginary::parse(raw_cell) {
            None => {
                return Err(MatrixError::InvalidVal(String::from(raw_cell)))
            }
            Some(val) => row.push(val),
        };
    }
    return Ok(row);
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut print = String::new();
        let mut count: u32 = 0;
        for row in self.data.iter() {
            add_row_to_string(&mut print, row, self.width);
            count += 1;
            if count != self.height {
                print.push('\n');
            }
        }
        write!(f, "{}", print)
    }
}

fn add_row_to_string(st: &mut String, row: &Vec<Imaginary>, width: u32) {
    st.push('[');
    let mut count: u32 = 0;
    for cell in row.iter() {
        *st = format!("{} {} ", st, cell);
        count += 1;
        if count != width {
            st.push(',');
        }
    }
    st.push(']');
}

#[cfg(test)]
mod constructor {
    use super::Matrix;

    #[test]
    fn parser_valid() {
        match Matrix::parse("[[1.1,1.2,1.3];[2.1,2.2,2.3];[3.1,3.2,3.3]]") {
            Ok(mat) => assert_eq!(
                "[ 1.1 , 1.2 , 1.3 ]\n[ 2.1 , 2.2 , 2.3 ]\n[ 3.1 , 3.2 , 3.3 ]",
                mat.to_string()
            ),
            Err(err) => panic!(err),
        }
    }

    #[test]
    fn parser_error_0() {
        Matrix::parse("[[1.1,1.2,1.3];[2.1,2.3];[3.1,3.2,3.3]]")
            .expect_err("Expect a size error");
    }

    #[test]
    fn parser_error_1() {
        Matrix::parse("[[1.1,1.2,1.3];[2.1,2.2,2.3];[3.1,3.2.4,3.3]]")
            .expect_err("Expect an invalid value");
    }

    #[test]
    fn parser_error_2() {
        Matrix::parse("[[1.1,1.2,1.3];[2.1,2.2,2.3];3.1,3.2,3.3]]")
            .expect_err("Expect a format error");
    }
}
