/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   incomplete_expr.rs                                 :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/06 15:25:19 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/06 15:31:45 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::ComputorError::{self, IncompleteExpr};
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct IncompleteExprError {
    expr: String,
}

impl Error for IncompleteExprError {}

impl fmt::Display for IncompleteExprError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incomplete expression can't be parsed : '{}'", self.expr)
    }
}

impl IncompleteExprError {
    pub fn new(expr: &str) -> ComputorError {
        IncompleteExpr(IncompleteExprError {
            expr: String::from(expr),
        })
    }
}
