/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   resolve.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/15 17:35:29 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/19 11:18:10 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Token;
use crate::computor::ComputorResult;
use crate::memory::Memory;
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn get_result(&self, _mem: &Memory) -> ComputorResult {
        ComputorResult::Resolve
    }
}
