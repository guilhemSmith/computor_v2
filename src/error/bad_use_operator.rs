/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   bad_use_operator.rs                                :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/30 14:48:53 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/06 12:50:42 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::ComputorError::{self, BadUseOperator};
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct BadUseOperatorError {
    symbol: char,
}

impl Error for BadUseOperatorError {}

impl fmt::Display for BadUseOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Operator: '{}', {}, and {}",
            self.symbol,
            "must be preceded by a value",
            "followed by a another value."
        )
    }
}

impl BadUseOperatorError {
    pub fn new(op: char) -> ComputorError {
        BadUseOperator(BadUseOperatorError { symbol: op })
    }
}