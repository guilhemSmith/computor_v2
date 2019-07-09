/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/09 12:18:40 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod numbers;

use crate::numbers::imaginary::Imaginary;
use crate::numbers::rational::Rational;

fn main() {
    let num = Imaginary {
        real: Rational {
            below_zero: true,
            numerator: 100,
            denominator: 42,
        },
        irreal: Rational {
            below_zero: false,
            numerator: 100,
            denominator: 10,
        },
    };

    println!("Hello, world! '{}'", num);
}
