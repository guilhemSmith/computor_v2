/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   extension.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/21 10:31:53 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/21 10:37:18 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::Variable;

use std::collections::HashMap;

#[derive(Clone)]
pub struct Extension {
	tmp_var: HashMap<String, Variable>,
}

impl Extension {
	pub fn new() -> Self {
		Extension {
			tmp_var: HashMap::new(),
		}
	}
}