/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   rational.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:47:05 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/09 12:18:05 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Number;
use std::fmt;

pub struct Rational {
    pub below_zero: bool,
    pub numerator: u32,
    pub denominator: u32,
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if self.below_zero { "- " } else { "" },
            self.numerator as f64 / self.denominator as f64
        )
    }
}

impl Number for Rational {
    fn is_null(&self) -> bool {
        self.numerator == 0
    }
}
