/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   computor_error.rs                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/10 15:37:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/10 15:59:22 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{error::Error, fmt};

#[derive(Debug)]
enum ErrorKind {
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
			ErrorKind::BadUseOperator => write!(f, "syntax"),
			ErrorKind::DivByZero => write!(f, "math"),
			ErrorKind::IncompleteExpr => write!(f, "syntax"),
			ErrorKind::InvalidExpr => write!(f, "syntax"),
			ErrorKind::InvalidOperand => write!(f, "syntax"),
			ErrorKind::InvalidOperator => write!(f, "syntax"),
			ErrorKind::IO => write!(f, "input"),
		}
	}
}

#[derive(Debug)]
enum ErrorPosition {
	Char(u32),
	Token(u32),
	Global,
}

impl fmt::Display for ErrorPosition {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ErrorPosition::Char(pos) => write!(f, "char:{}", pos),
			ErrorPosition::Token(pos) => write!(f, "token:{}", pos),
			ErrorPosition::Global => write!(f, "all"),
		}
	}
}

#[derive(Debug)]
pub struct ComputorError {
	kind: ErrorKind,
	position: ErrorPosition,
	info: String,
}

impl ComputorError {
}

impl Error for ComputorError {}

impl fmt::Display for ComputorError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[err:{}-{}] -> {}", self.kind, self.position, self.info)
	}
}