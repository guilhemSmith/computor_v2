/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   token.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/30 14:43:15 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/30 14:44:59 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::{Expression, Operand, Operator};
use crate::error::ComputorError;

pub enum Token {
    Expr(Expression),
    Orand(Operand),
    Orator(Operator),
    Invalid(ComputorError, usize),
}

impl Token {
    pub fn is_orand(&self) -> bool {
        match self {
            Token::Orand(_) => true,
            _ => false,
        }
    }

    pub fn is_orator(&self) -> bool {
        match self {
            Token::Orator(_) => true,
            _ => false,
        }
    }
}
