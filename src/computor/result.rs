/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   result.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/19 10:10:40 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/28 14:13:10 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::ComputorError;
use crate::memory::Value;
use crate::types::{Imaginary, Matrix};

use std::collections::HashMap;

pub enum Computed {
    None,
    Res,
    ValMat(Matrix),
    ValIm(Imaginary),
    VarCall(String, Value),
    VarSet(String),
    FunId(String, Vec<String>),
    Equ(String, HashMap<i32, Imaginary>),
}

impl Default for Computed {
    fn default() -> Self {
        Computed::None
    }
}

pub type TreeResult = Result<Computed, ComputorError>;
pub type ComputorResult = Result<(), ComputorError>;
