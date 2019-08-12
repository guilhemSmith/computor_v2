/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   error.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/10 15:37:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/12 13:23:48 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{error::Error, fmt, io::Error as IOErr};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    BadUseOperator,
    DivByZero,
    IncompleteExpr,
    InvalidExpr,
    InvalidOperand,
    InvalidOperator,
    IO,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::BadUseOperator => write!(f, "bad use"),
            ErrorKind::DivByZero => write!(f, "math"),
            ErrorKind::IncompleteExpr => write!(f, "incomplete"),
            ErrorKind::InvalidExpr => write!(f, "syntax"),
            ErrorKind::InvalidOperand => write!(f, "syntax"),
            ErrorKind::InvalidOperator => write!(f, "syntax"),
            ErrorKind::IO => write!(f, "input"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrorPosition {
    Char(usize),
    Global,
}

impl fmt::Display for ErrorPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorPosition::Char(pos) => write!(f, "-char:{}", pos),
            ErrorPosition::Global => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComputorError {
    kind: ErrorKind,
    position: ErrorPosition,
    info: String,
}

impl ComputorError {
    pub fn set_pos(&mut self, pos: ErrorPosition) -> &Self {
        self.position = pos;
        return self;
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn bad_use_op(op: char) -> Self {
        ComputorError {
            kind: ErrorKind::BadUseOperator,
            position: ErrorPosition::Global,
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
            position: ErrorPosition::Global,
            info: format!(
                "Division by zero is not allowed : {} {} {}",
                left_op, op, right_op
            ),
        }
    }

    pub fn incomplete_expr(expr: &str) -> Self {
        ComputorError {
            kind: ErrorKind::IncompleteExpr,
            position: ErrorPosition::Global,
            info: format!("Incomplete expression can't be parsed : '{}'", expr),
        }
    }

    pub fn invalid_expr() -> Self {
        ComputorError {
            kind: ErrorKind::InvalidExpr,
            position: ErrorPosition::Global,
            info: format!("Error while computing token from an expression."),
        }
    }

    pub fn invalid_operand(raw_str: &str, is_real: bool) -> Self {
        ComputorError {
            kind: ErrorKind::InvalidOperand,
            position: ErrorPosition::Global,
            info: format!(
                "Operand can't be interpreted as a numeric value : {}{}",
                raw_str,
                if is_real { "" } else { "i" }
            ),
        }
    }
    pub fn invalid_operator(symbol: char) -> Self {
        ComputorError {
            kind: ErrorKind::InvalidOperator,
            position: ErrorPosition::Global,
            info: format!("Invalid operator symbol caught : {}", symbol),
        }
    }

    pub fn io(err: IOErr) -> Self {
        ComputorError {
            kind: ErrorKind::IO,
            position: ErrorPosition::Global,
            info: format!("{}", err),
        }
    }
}

impl Error for ComputorError {}

impl fmt::Display for ComputorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[err:{}{}] -> {}", self.kind, self.position, self.info)
    }
}
