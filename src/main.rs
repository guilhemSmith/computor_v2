/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/25 12:46:01 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod types;

use crate::types::imaginary::Imaginary;
use crate::types::rational::Rational;
use crate::types::Raw;

fn main() {
    let weird = Rational::new(Raw::Float(1.003));
    let ra_values = vec![
        Rational::new(Raw::Couple(1, 3)),
        Rational::new(Raw::Couple(42, 12)),
        Rational::new(Raw::Couple(986, -3)),
        Rational::new(Raw::Float(42.5879631)),
        Rational::zero(),
    ];
    let im_values = vec![
        Imaginary::new(Raw::Float(-42.42), Raw::Couple(42, 12)),
        Imaginary::new(Raw::Couple(34, 3), Raw::Float(-13.1313)),
        Imaginary::new(Raw::Float(134.0), Raw::Zero),
        Imaginary::new(Raw::Zero, Raw::Couple(15, 89)),
        Imaginary::zero(),
    ];

    println!("'{}'", weird);
    for num in ra_values {
        println!("\n'{}'", num);
    }
    println!("\n");
    for num in im_values {
        println!("\n'{}'", num);
    }
}
