/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/10 15:37:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/24 16:23:25 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::lexer::Token;
use std::{error::Error, fmt};

extern crate colored;
use colored::Colorize;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    BadPow,
    BadResolve,
    BadUseOperator,
    DivByEq,
    DivByZero,
    EmptyInstr,
    FunUndefinded,
    FunArgInv,
    MatrixInEq,
    MatrixVal,
    ModWithIm,
    ModWithUnk,
    InvalidInput,
    InvalidTokens,
    IO,
    IOStop,
    OpMatrix,
    OverflowAbort,
    TooManyEqual,
    TooManyUnknown,
    UnparsedToken,
    UncompleteEq,
    UnknownId,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::BadPow => write!(f, "bad use"),
            ErrorKind::BadResolve => write!(f, "bad use"),
            ErrorKind::BadUseOperator => write!(f, "bad use"),
            ErrorKind::DivByEq => write!(f, "math"),
            ErrorKind::DivByZero => write!(f, "math"),
            ErrorKind::EmptyInstr => write!(f, "parser"),
            ErrorKind::FunUndefinded => write!(f, "function"),
            ErrorKind::FunArgInv => write!(f, "function"),
            ErrorKind::MatrixInEq => write!(f, "parser"),
            ErrorKind::MatrixVal => write!(f, "parser"),
            ErrorKind::ModWithIm => write!(f, "math"),
            ErrorKind::ModWithUnk => write!(f, "parser"),
            ErrorKind::InvalidInput => write!(f, "syntax"),
            ErrorKind::InvalidTokens => write!(f, "lexer"),
            ErrorKind::IO => write!(f, "input"),
            ErrorKind::IOStop => write!(f, "input"),
            ErrorKind::OpMatrix => write!(f, "matrix"),
            ErrorKind::OverflowAbort => write!(f, "limit"),
            ErrorKind::TooManyEqual => write!(f, "parser"),
            ErrorKind::TooManyUnknown => write!(f, "instruction"),
            ErrorKind::UnparsedToken => write!(f, "parser"),
            ErrorKind::UncompleteEq => write!(f, "parser"),
            ErrorKind::UnknownId => write!(f, "parser"),
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

    pub fn empty_instr() -> Self {
        ComputorError {
            kind: ErrorKind::EmptyInstr,
            info: String::from("Empty instruction given."),
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

    pub fn matrix_in_eq() -> Self {
        ComputorError {
            kind: ErrorKind::MatrixInEq,
            info: String::from("Can't solve equation with matrix."),
        }
    }

    pub fn matrix_val() -> Self {
        ComputorError {
            kind: ErrorKind::MatrixVal,
            info: String::from("Matrix can only contain imaginary value."),
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

    pub fn invalid_tokens(nb: i32) -> Self {
        ComputorError {
            kind: ErrorKind::InvalidTokens,
            info: format!("{} invalid tokens. Abort.", nb),
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

    pub fn op_matrix(op: char) -> Self {
        ComputorError {
            kind: ErrorKind::OpMatrix,
            info: format!("'{}' can't be used with matrix.", op),
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
            info: String::from("Too many unknown variables given."),
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

    pub fn unknown_id(id: String, is_var: bool) -> Self {
        ComputorError {
            kind: ErrorKind::UnknownId,
            info: format!(
                "Unknown {} '{}'",
                if is_var { "variable" } else { "function" },
                id
            ),
        }
    }
}

impl Error for ComputorError {}

impl fmt::Display for ComputorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::IOStop => write!(f, "{}", self.info),
            _ => write!(
                f,
                "{} - {}",
                format!("[err:{}]", self.kind).red(),
                self.info
            ),
        }
    }
}
