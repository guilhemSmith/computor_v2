/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   types.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:52:05 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/23 17:14:13 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod imaginary;
mod matrix;
mod rational;

pub use imaginary::Imaginary;
pub use matrix::{Matrix, MatrixError};
pub use rational::Rational;

use crate::computor::ComputorError;
type OpResult<T> = Result<T, ComputorError>;

fn read_overflow(res: (u64, bool)) -> OpResult<u64> {
    if !res.1 {
        Ok(res.0)
    } else {
        Err(ComputorError::overflow_abort())
    }
}
