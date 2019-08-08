/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   timer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/08 16:05:42 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/08 16:52:19 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::time::Instant;

pub struct Timer<'a> {
    start: Instant,
    title: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(title: &'a str) -> Self {
        Timer {
            start: Instant::now(),
            title: title,
        }
    }

    pub fn top(&self) {
        let duration = self.start.elapsed();
        println!(
            "[T:{}] - time elapsed: {}us ({}ms).",
            self.title,
            duration.as_micros(),
            duration.as_millis()
        );
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        self.top()
    }
}
