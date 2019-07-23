/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   imaginary.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:46:59 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/23 14:44:07 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::rational::Rational;
use super::Raw;
use std::fmt;

pub struct Imaginary {
    real: Rational,
    irreal: Rational,
}

impl Imaginary {
    pub fn new(real: Raw, irreal: Raw) -> Self {
        Imaginary {
            real: Rational::new(real),
            irreal: Rational::new(irreal),
        }
    }

    pub fn zero() -> Self {
        Imaginary {
            real: Rational::zero(),
            irreal: Rational::zero(),
        }
    }
}

impl fmt::Display for Imaginary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.real,
            if self.irreal == Rational::zero() {
                format!(
                    " {}{}i",
                    if self.irreal > Rational::zero() {
                        "+ "
                    } else {
                        ""
                    },
                    self.irreal
                )
            } else {
                String::from("")
            }
        )
    }
}
