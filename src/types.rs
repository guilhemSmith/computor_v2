/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   types.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:52:05 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/25 17:36:19 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod imaginary;
pub mod rational;

pub use imaginary::Imaginary;
pub use rational::Rational;

pub enum Raw {
    Float(f64),
    Couple(i64, i64),
    Zero,
}
