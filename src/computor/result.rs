/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   result.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/19 10:10:40 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/20 13:40:21 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::computor_error::ComputorError;
use crate::parser::TokenTree;
use crate::memory::Variable;
use crate::types::Imaginary;

pub enum ComputorResult {
    None,
    Res,
    Val(Imaginary),
    Var(String, Imaginary, Imaginary),
    Equ(String, Vec<Imaginary>),
    Fun(String, Vec<String>, Box<TokenTree>),
    Err(ComputorError),
}

impl Default for ComputorResult {
    fn default() -> Self {
        ComputorResult::None
    }
}
