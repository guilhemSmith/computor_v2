/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   div_by_zero.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/26 11:17:33 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/30 11:28:35 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::ComputorError::{self, DivByZero};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct DivByZeroError {
    left_op: String,
    right_op: String,
    op: char,
}

impl Error for DivByZeroError {}

impl fmt::Display for DivByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Division by zero is not allowed : {} {} {}",
            self.left_op, self.op, self.right_op
        )
    }
}

impl DivByZeroError {
    pub fn new(left_op: String, right_op: String, op: char) -> ComputorError {
        DivByZero(DivByZeroError {
            left_op: left_op,
            right_op: right_op,
            op: op,
        })
    }
}
