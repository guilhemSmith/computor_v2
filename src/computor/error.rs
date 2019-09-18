/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/10 15:37:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/18 16:04:28 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::lexer::Token;
use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    BadPow,
    BadResolve,
    BadUseOperator,
    DivByEq,
    DivByZero,
    FunUndefinded,
    FunArgInv,
    ModWithIm,
    ModWithUnk,
    InvalidInput,
    IO,
    IOStop,
    OverflowAbort,
    TooManyEqual,
    TooManyUnknown,
    UnparsedToken,
    UncompleteEq,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::BadPow => write!(f, "bad use"),
            ErrorKind::BadResolve => write!(f, "bad use"),
            ErrorKind::BadUseOperator => write!(f, "bad use"),
            ErrorKind::DivByEq => write!(f, "math"),
            ErrorKind::DivByZero => write!(f, "math"),
            ErrorKind::FunUndefinded => write!(f, "function"),
            ErrorKind::FunArgInv => write!(f, "function"),
            ErrorKind::ModWithIm => write!(f, "math"),
            ErrorKind::ModWithUnk => write!(f, "instruction"),
            ErrorKind::InvalidInput => write!(f, "syntax"),
            ErrorKind::IO => write!(f, "input"),
            ErrorKind::IOStop => write!(f, "input"),
            ErrorKind::OverflowAbort => write!(f, "limit"),
            ErrorKind::TooManyEqual => write!(f, "instruction"),
            ErrorKind::TooManyUnknown => write!(f, "instruction"),
            ErrorKind::UnparsedToken => write!(f, "parser"),
            ErrorKind::UncompleteEq => write!(f, "instruction"),
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

    pub fn bad_pow() -> Self {
        ComputorError {
            kind: ErrorKind::BadPow,
            info: format!(
                "Pow operator '^' only accept {} for the second argument.",
                "integer value"
            ),
        }
    }

    pub fn bad_resolve() -> Self {
        ComputorError {
            kind: ErrorKind::BadResolve,
            info: String::from(
                "Resolve symbol must be alone at the end of the instruction.",
            ),
        }
    }

    pub fn bad_use_op(op: char) -> Self {
        ComputorError {
            kind: ErrorKind::BadUseOperator,
            info: format!(
                "Operator: '{}' {}, and {}",
                op,
                "must be preceded by a value",
                "followed by a another value."
            ),
        }
    }

    pub fn div_by_eq() -> Self {
        ComputorError {
            kind: ErrorKind::DivByEq,
            info: String::from("Can't divide an equation by another equation."),
        }
    }

    pub fn div_by_zero() -> Self {
        ComputorError {
            kind: ErrorKind::DivByZero,
            info: String::from("Trying to div by zero, abort."),
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

    pub fn mod_with_im() -> Self {
        ComputorError {
            kind: ErrorKind::ModWithIm,
            info: String::from("Mod with Imaginary numbers, abort."),
        }
    }

    pub fn mod_with_unk() -> Self {
        ComputorError {
            kind: ErrorKind::ModWithUnk,
            info: String::from("Mod with unknown values, abort."),
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

    pub fn overflow_abort() -> Self {
        ComputorError {
            kind: ErrorKind::OverflowAbort,
            info: String::from("Operation will overflow, aborting."),
        }
    }

    pub fn too_many_equal() -> Self {
        ComputorError {
            kind: ErrorKind::TooManyEqual,
            info: String::from("Too many equal sign given."),
        }
    }

    pub fn too_many_unknown() -> Self {
        ComputorError {
            kind: ErrorKind::TooManyUnknown,
            info: String::from("Too many unknown variable given."),
        }
    }

    pub fn unparsed_token(token: &dyn Token) -> Self {
        ComputorError {
            kind: ErrorKind::UnparsedToken,
            info: format!("Token left behind: {:?}.", token),
        }
    }

    pub fn uncomplete_eq() -> Self {
        ComputorError {
            kind: ErrorKind::UncompleteEq,
            info: format!("Equation not complete."),
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
