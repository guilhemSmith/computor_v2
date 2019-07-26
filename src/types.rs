/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   types.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:52:05 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/26 11:50:43 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod imaginary;
mod rational;

pub use imaginary::Imaginary;
pub use rational::Rational;

pub enum Raw {
    Float(f64),
    Couple(i64, i64),
    Zero,
}
