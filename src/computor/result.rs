/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   result.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/19 10:10:40 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/20 10:04:51 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::computor_error::ComputorError;
use crate::parser::TokenTree;
use crate::types::Imaginary;

pub enum ComputorResult {
    None,
    Resolve,
    Value(Imaginary),
    Unknown(String, Imaginary, Imaginary),
    SolveVar(String, Vec<Imaginary>),
    AssignFun(String, Vec<String>, Box<TokenTree>),
    Err(ComputorError),
}

impl Default for ComputorResult {
    fn default() -> Self {
        ComputorResult::None
    }
}
