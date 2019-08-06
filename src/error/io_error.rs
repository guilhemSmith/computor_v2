/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   io_error.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/06 12:52:13 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/06 12:54:54 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::ComputorError::{self, IO};
use std::{error::Error, fmt, io::Error as IOErr};

#[derive(Debug,Clone)]
pub struct IOError {
	description: String,
}

impl Error for IOError {}

impl fmt::Display for IOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl IOError {
    pub fn new(err: IOErr) -> ComputorError {
        IO(IOError {
			description: format!("{}", err)
		})
    }
}
