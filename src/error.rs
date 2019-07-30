/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/26 09:36:25 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/30 11:48:33 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod div_by_zero;
mod invalid_operand;
mod invalid_operator;

pub use div_by_zero::DivByZeroError;
pub use invalid_operand::InvalidOperandError;
pub use invalid_operator::InvalidOperatorError;

pub enum ComputorError {
    DivByZero(DivByZeroError),
    InvalidOperand(InvalidOperandError),
    InvalidOperator(InvalidOperatorError),
    IO(std::io::Error),
}

pub fn log_error(error: ComputorError, position: u32) {
    match error {
        ComputorError::DivByZero(err) => {
            eprintln!("[err:c{}] - {}", position, err)
        }
        ComputorError::InvalidOperand(err) => {
            eprintln!("[err:c{}] - {}", position, err)
        }
        ComputorError::InvalidOperator(err) => {
            eprintln!("[err:c{}] - {}", position, err)
        }
        ComputorError::IO(err) => eprintln!("[err:io] - {}", err),
    }
}
