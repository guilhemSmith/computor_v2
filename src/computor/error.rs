/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/10 15:37:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/09 12:21:06 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::lexer::Token;
use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    BadResolve,
    BadUseOperator,
    FunUndefinded,
    FunArgInv,
    DivByZero,
    InvalidInput,
    IO,
    IOStop,
    TooManyEqual,
    UnparsedToken,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::BadResolve => write!(f, "bad use"),
            ErrorKind::BadUseOperator => write!(f, "bad use"),
            ErrorKind::FunUndefinded => write!(f, "function"),
            ErrorKind::FunArgInv => write!(f, "function"),
            ErrorKind::DivByZero => write!(f, "math"),
            ErrorKind::InvalidInput => write!(f, "syntax"),
            ErrorKind::IO => write!(f, "input"),
            ErrorKind::IOStop => write!(f, "input"),
            ErrorKind::TooManyEqual => write!(f, "instruction"),
            ErrorKind::UnparsedToken => write!(f, "parser"),
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

    pub fn bad_resolve() -> Self {
        ComputorError {
            kind: ErrorKind::BadResolve,
            info: String::from(
                "Resolve symbol '?' must be at the end of the instruction.",
            ),
        }
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

    pub fn fun_undef(name: &String) -> Self {
        ComputorError {
            kind: ErrorKind::FunUndefinded,
            info: format!("'{}' is not defined as a function.", name),
        }
    }

    pub fn fun_arg_inv(name: &String) -> Self {
        ComputorError {
            kind: ErrorKind::FunArgInv,
            info: format!("'{}' did not received valid argument(s).", name),
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

    pub fn too_many_equal() -> Self {
        ComputorError {
            kind: ErrorKind::TooManyEqual,
            info: String::from("Too many equal sign given."),
        }
    }

    pub fn unparsed_token(token: &Token) -> Self {
        ComputorError {
            kind: ErrorKind::UnparsedToken,
            info: format!("Token left behind: {:?}.", token),
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
