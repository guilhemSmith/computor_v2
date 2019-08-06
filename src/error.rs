/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/26 09:36:25 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/06 14:36:11 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod bad_use_operator;
mod div_by_zero;
mod invalid_expr;
mod invalid_operand;
mod invalid_operator;
mod io_error;

pub use bad_use_operator::BadUseOperatorError;
pub use div_by_zero::DivByZeroError;
pub use invalid_expr::InvalidExprError;
pub use invalid_operand::InvalidOperandError;
pub use invalid_operator::InvalidOperatorError;
pub use io_error::IOError;

#[derive(Clone)]
pub enum ComputorError {
    DivByZero(DivByZeroError),
    InvalidOperand(InvalidOperandError),
    InvalidOperator(InvalidOperatorError),
    IO(IOError),
    BadUseOperator(BadUseOperatorError),
    InvalidExpr(InvalidExprError),
}

pub fn log_error(error: &ComputorError, position: Option<&usize>) {
    match (error, position) {
        (ComputorError::BadUseOperator(err), None) => {
            eprintln!("[err-syntax] - {}", err)
        }
        (ComputorError::DivByZero(err), Some(pos)) => {
            eprintln!("[err-math:c{}] - {}", pos, err)
        }
        (ComputorError::InvalidOperand(err), Some(pos)) => {
            eprintln!("[err-input:c{}] - {}", pos, err)
        }
        (ComputorError::InvalidOperator(err), Some(pos)) => {
            eprintln!("[err-input:c{}] - {}", pos, err)
        }
        (ComputorError::InvalidExpr(err), None) => {
            eprintln!("[err-input:] - {}", err)
        }
        (ComputorError::IO(err), None) => eprintln!("[err-io] - {}", err),
        _ => eprintln!("[err-error:] - Invalid error format."),
    }
}
