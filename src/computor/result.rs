/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   result.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/19 10:10:40 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/11 11:54:20 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::ComputorError;
use crate::types::Imaginary;

use std::collections::HashMap;

pub enum ComputorResult {
    None,
    Res,
    Err(ComputorError),
    Val(Imaginary),
    VarCall(String, Imaginary),
    VarSet(String),
    FunSet(String, Vec<String>),
    Equ(String, HashMap<i32, Imaginary>),
}

impl Default for ComputorResult {
    fn default() -> Self {
        ComputorResult::None
    }
}
