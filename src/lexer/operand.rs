/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   operand.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/25 17:20:49 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/25 18:39:57 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::types::{Imaginary, Raw};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct InvalidOperandError {
    raw_value: String,
}

impl fmt::Display for InvalidOperandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Operand can't be interpreted as a numeric value : {}",
            self.raw_value
        )
    }
}

impl Error for InvalidOperandError {}

pub struct Operand {
    value: Imaginary,
}

impl Operand {
    pub fn new(
        raw_str: &str,
        is_real: bool,
    ) -> Result<Operand, InvalidOperandError> {
        let fl_value = match raw_str.parse::<f64>() {
            Ok(val) => val,
            Err(_err) => {
                return Err(InvalidOperandError {
                    raw_value: format!(
                        "{}{}",
                        raw_str,
                        if is_real { "" } else { "i" }
                    ),
                })
            }
        };

        if is_real {
            Ok(Operand {
                value: Imaginary::new(Raw::Float(fl_value), Raw::Zero),
            })
        } else {
            Ok(Operand {
                value: Imaginary::new(Raw::Zero, Raw::Float(fl_value)),
            })
        }
    }
}
