/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   invalid_expr.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/05 17:46:36 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/10 15:15:37 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::ComputorError;
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct InvalidExprError {}

impl Error for InvalidExprError {}

impl fmt::Display for InvalidExprError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while computing token from an expression.")
    }
}

impl InvalidExprError {
    pub fn new() -> Self {
        InvalidExprError {}
    }
}
 impl ComputorError for InvalidExprError {
     
 }