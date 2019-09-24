/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   matrix.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/09/23 10:31:02 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/24 16:12:11 by gsmith           ###   ########.fr       */
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

// #[cfg(test)]
// mod constructor {
//     use super::Matrix;

//     #[test]
//     fn parser_valid() {
//         match Matrix::parse("[[1.1,1.2,1.3];[2.1,2.2,2.3];[3.1,3.2,3.3]]") {
//             Ok(mat) => assert_eq!(
//                 "[ 1.1 , 1.2 , 1.3 ]\n[ 2.1 , 2.2 , 2.3 ]\n[ 3.1 , 3.2 , 3.3 ]",
//                 mat.to_string()
//             ),
//             Err(err) => panic!(err),
//         }
//     }

//     #[test]
//     fn parser_error_0() {
//         Matrix::parse("[[1.1,1.2,1.3];[2.1,2.3];[3.1,3.2,3.3]]")
//             .expect_err("Expect a size error");
//     }

//     #[test]
//     fn parser_error_1() {
//         Matrix::parse("[[1.1,1.2,1.3];[2.1,2.2,2.3];[3.1,3.2.4,3.3]]")
//             .expect_err("Expect an invalid value");
//     }

//     #[test]
//     fn parser_error_2() {
//         Matrix::parse("[[1.1,1.2,1.3];[2.1,2.2,2.3];3.1,3.2,3.3]]")
//             .expect_err("Expect a format error");
//     }
// }
