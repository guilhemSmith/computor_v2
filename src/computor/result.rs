/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   result.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/19 10:10:40 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/28 16:36:27 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::ComputorError;
use crate::memory::Value;
use crate::types::{Imaginary, Matrix};

use std::{collections::HashMap, fmt};

pub enum Computed {
    None,
    Res,
    ValMat(Matrix),
    ValIm(Imaginary),
    VarCall(String, Value),
    VarSet(String),
    FunId(String, Vec<Computed>),
    Equ(String, HashMap<i32, Imaginary>),
}

impl Default for Computed {
    fn default() -> Self {
        Computed::None
    }
}

impl fmt::Display for Computed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Computed::None => write!(f, ""),
            Computed::Res => write!(f, "?"),
            Computed::ValMat(mat) => write!(f, "{}", mat),
            Computed::ValIm(im) => write!(f, "{}", im),
            Computed::VarCall(id, _) => write!(f, "{}", id),
            Computed::VarSet(id) => write!(f, "{}", id),
            Computed::FunId(id, _) => write!(f, "{}(...)", id),
            Computed::Equ(_, _) => write!(f, "..."),
        }
    }
}

pub type TreeResult = Result<Computed, ComputorError>;
pub type ComputorResult = Result<(), ComputorError>;
