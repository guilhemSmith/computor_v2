/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   resolve.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 17:35:29 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/18 16:55:51 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Token;
use crate::computor::{Computed, TreeResult};
use crate::memory::{Extension, Memory};

use std::any::Any;
use std::fmt;

pub struct Resolve;

impl fmt::Display for Resolve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "?")
    }
}

impl fmt::Debug for Resolve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[?]")
    }
}

impl Token for Resolve {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(
        &self,
        _mem: &Memory,
        _ext: Option<&mut Extension>,
    ) -> TreeResult {
        Ok(Computed::Res)
    }
}
