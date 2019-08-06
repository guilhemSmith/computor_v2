/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   invalid_operand.rs                                 :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/26 11:00:34 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/06 12:51:03 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::ComputorError::{self, InvalidOperand};
use std::{error::Error, fmt};

#[derive(Debug,Clone)]
pub struct InvalidOperandError {
    raw_value: String,
}

impl Error for InvalidOperandError {}

impl fmt::Display for InvalidOperandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Operand can't be interpreted as a numeric value : {}",
            self.raw_value
        )
    }
}

impl InvalidOperandError {
    pub fn new(raw_str: &str, is_real: bool) -> ComputorError {
        InvalidOperand(InvalidOperandError {
            raw_value: format!("{}{}", raw_str, if is_real { "" } else { "i" }),
        })
    }
}
