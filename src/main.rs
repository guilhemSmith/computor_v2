/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/22 12:35:05 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod types;

// use crate::types::imaginary::Imaginary;
use crate::types::rational::{Rational, RationalParam};

fn main() {
    let values = vec![
        Rational::new(RationalParam::Float(-42.42)),
        Rational::new(RationalParam::Float(134.0)),
        Rational::new(RationalParam::Couple(-986, -3)),
        Rational::new(RationalParam::Couple(-587, 2)),
        Rational::new(RationalParam::Zero),
    ];

    for num in values {
        println!("'{}' also known as {:?}", num, num);
    }
}
