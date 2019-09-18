/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   result.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/19 10:10:40 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/18 15:27:18 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::ComputorError;
use crate::types::Imaginary;

use std::collections::HashMap;

pub enum Computed {
    None,
    Res,
    Val(Imaginary),
    VarCall(String, Imaginary),
    VarSet(String),
    FunSet(String, Vec<String>),
    Equ(String, HashMap<i32, Imaginary>),
}

impl Default for Computed {
    fn default() -> Self {
        Computed::None
    }
}

pub type TreeResult = Result<Computed, ComputorError>;
pub type ComputorResult = Result<(), ComputorError>;
