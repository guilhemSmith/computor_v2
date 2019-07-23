/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:56:56 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/23 14:30:31 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod types;

use crate::types::imaginary::Imaginary;
use crate::types::rational::Rational;
use crate::types::Raw;

fn main() {
    let values = vec![
        Rational::new(Raw::Float(-42.42)),
        Rational::new(Raw::Float(134.0)),
        Rational::new(Raw::Couple(-986, -3)),
        Rational::new(Raw::Couple(-587, 2)),
        Rational::new(Raw::Zero),
    ];

    for num in values {
        println!("'{}' also known as {:?}", num, num);
    }
}
