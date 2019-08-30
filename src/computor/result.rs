/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   result.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/19 10:10:40 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/30 17:42:24 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::ComputorError;
use crate::types::Imaginary;

pub enum ComputorResult {
    None,
    Res,
    Err(ComputorError),
    Val(Imaginary),
    VarCall(String, Imaginary),
    VarSet(String),
    FunSet(String, Vec<String>),
    Equ(String, Vec<Imaginary>),
}

impl Default for ComputorResult {
    fn default() -> Self {
        ComputorResult::None
    }
}
