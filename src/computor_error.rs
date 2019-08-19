/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor_error.rs                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/10 15:37:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/19 12:01:08 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    BadUseOperator,
    DivByZero,
    InvalidFun,
    InvalidInput,
    IO,
    IOStop,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::BadUseOperator => write!(f, "bad use"),
            ErrorKind::DivByZero => write!(f, "math"),
            ErrorKind::InvalidFun => write!(f, "syntax"),
            ErrorKind::InvalidInput => write!(f, "syntax"),
            ErrorKind::IO => write!(f, "input"),
            ErrorKind::IOStop => write!(f, "input"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComputorError {
    kind: ErrorKind,
    info: String,
}

impl ComputorError {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn bad_use_op(op: char) -> Self {
        ComputorError {
            kind: ErrorKind::BadUseOperator,
            info: format!(
                "Operator: '{}', {}, and {}",
                op,
                "must be preceded by a value",
                "followed by a another value."
            ),
        }
    }

    pub fn div_by_zero(left_op: String, right_op: String, op: char) -> Self {
        ComputorError {
            kind: ErrorKind::DivByZero,
            info: format!(
                "Division by zero is not allowed : {} {} {}",
                left_op, op, right_op
            ),
        }
    }

    pub fn invalid_fun() -> Self {
        ComputorError {
            kind: ErrorKind::InvalidFun,
            info: String::from("bad function."),
        }
    }

    pub fn invalid_input() -> Self {
        ComputorError {
            kind: ErrorKind::InvalidInput,
            info: format!(
                "{} {}",
                "This input can't be read correctly.",
                "Might be du to an orphan parenthesis."
            ),
        }
    }

    pub fn io(cut: &str) -> Self {
        ComputorError {
            kind: ErrorKind::IO,
            info: format!("{}", cut),
        }
    }

    pub fn io_stop() -> Self {
        ComputorError {
            kind: ErrorKind::IOStop,
            info: String::from("Input interrupted."),
        }
    }
}

impl Error for ComputorError {}

impl fmt::Display for ComputorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::IOStop => write!(f, "{}", self.info),
            _ => write!(f, "[err:{}] -> {}", self.kind, self.info),
        }
    }
}
