/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   imaginary.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:46:59 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/23 14:34:56 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::rational::Rational;
use super::Raw;
use std::fmt;

pub struct Imaginary {
    pub real: Rational,
    pub irreal: Rational,
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
