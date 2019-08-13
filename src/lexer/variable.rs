/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   variable.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/13 17:16:26 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/13 17:27:15 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::fmt;

#[derive(Clone)]
pub struct Variable {
    id: String,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
