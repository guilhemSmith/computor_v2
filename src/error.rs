/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/26 09:36:25 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/05 19:23:27 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod bad_use_operator;
mod div_by_zero;
mod invalid_expr;
mod invalid_operand;
mod invalid_operator;

pub use bad_use_operator::BadUseOperatorError;
pub use div_by_zero::DivByZeroError;
pub use invalid_expr::InvalidExprError;
pub use invalid_operand::InvalidOperandError;
pub use invalid_operator::InvalidOperatorError;

use std::io::Error as IOErr;

pub enum ComputorError {
    DivByZero(DivByZeroError),
    InvalidOperand(InvalidOperandError),
    InvalidOperator(InvalidOperatorError),
    IO(IOErr),
    BadUseOperator(BadUseOperatorError),
    InvalidExpr(InvalidExprError),
}

pub fn log_error(error: ComputorError, position: u32) {
    match error {
        ComputorError::BadUseOperator(err) => {
            eprintln!("[err-syntax] - {}", err)
        }
        ComputorError::DivByZero(err) => {
            eprintln!("[err-math:c{}] - {}", position, err)
        }
        ComputorError::InvalidOperand(err) => {
            eprintln!("[err-input:c{}] - {}", position, err)
        }
        ComputorError::InvalidOperator(err) => {
            eprintln!("[err-input:c{}] - {}", position, err)
        }
        ComputorError::InvalidExpr(err) => eprintln!("[err-input:] - {}", err),
        ComputorError::IO(err) => eprintln!("[err-io] - {}", err),
    }
}
