/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/26 09:36:25 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/26 11:02:41 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod invalid_operand;
mod invalid_operator;

pub use invalid_operand::InvalidOperandError;
pub use invalid_operator::InvalidOperatorError;
use std::{error::Error, fmt};

pub fn log_error<T>(error: T, position: u32)
where
    T: Error + fmt::Display,
{
    eprintln!("[c:{}] - {}", position, error);
}
