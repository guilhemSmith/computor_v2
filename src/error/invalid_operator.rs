/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   invalid_operator.rs                                :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/26 10:56:21 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/26 10:59:45 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{error::Error, fmt};

#[derive(Debug)]
pub struct InvalidOperatorError {
    symbol: char,
}

impl Error for InvalidOperatorError {}

impl fmt::Display for InvalidOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid operator symbol caught : {}", self.symbol)
    }
}

impl InvalidOperatorError {
    pub fn new(symbol: char) -> Self {
        InvalidOperatorError { symbol: symbol }
    }
}
