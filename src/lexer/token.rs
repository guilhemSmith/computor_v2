/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   token.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/30 14:43:15 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/12 11:54:40 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Expression, Operand, Operator};
use crate::error::ComputorError;
use std::fmt;

#[derive(Clone)]
pub enum Token {
    Expr(Expression),
    Orand(Operand),
    Orator(Operator),
    Invalid(ComputorError),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Expr(exp) => write!(f, "({})", exp),
            Token::Orand(orand) => write!(f, "{}", orand),
            Token::Orator(orator) => write!(f, "{}", orator),
            Token::Invalid(err) => write!(f, "{{{:.10}}}", err),
        }
    }
}
