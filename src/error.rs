/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/10 15:37:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/15 11:06:01 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    BadUseOperator,
    DivByZero,
    InvalidExpr,
    InvalidValue,
    InvalidOperator,
    InvalidToken,
    IO,
    IOStop,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::BadUseOperator => write!(f, "bad use"),
            ErrorKind::DivByZero => write!(f, "math"),
            ErrorKind::InvalidExpr => write!(f, "syntax"),
            ErrorKind::InvalidValue => write!(f, "syntax"),
            ErrorKind::InvalidOperator => write!(f, "syntax"),
            ErrorKind::IO => write!(f, "input"),
            ErrorKind::IOStop => write!(f, "input"),
            ErrorKind::InvalidToken => write!(f, "parsing"),
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

    pub fn invalid_expr() -> Self {
        ComputorError {
            kind: ErrorKind::InvalidExpr,
            info: format!("This expression can't be read correctly."),
        }
    }

    pub fn invalid_value(raw_str: String) -> Self {
        ComputorError {
            kind: ErrorKind::InvalidValue,
            info: format!(
                "Value can't be interpreted as a numeric value : {}",
                raw_str
            ),
        }
    }
    pub fn invalid_operator(symbol: char) -> Self {
        ComputorError {
            kind: ErrorKind::InvalidOperator,
            info: format!("Invalid operator symbol caught : {}", symbol),
        }
    }

    pub fn invalid_token(token: String) -> Self {
        ComputorError {
            kind: ErrorKind::InvalidToken,
            info: format!("Invalid token parsed : {}", token),
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
