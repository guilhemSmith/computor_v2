/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   timer.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/08 16:05:42 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/19 18:20:14 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::time::Instant;

extern crate colored;
use colored::{ColoredString, Colorize};

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
        let micro = duration.as_micros();
        let milli = duration.as_millis();
        println!(
            "{}",
            format!(
                "{} - time elapsed: {}us ({}ms).",
                to_color(format!("[b:{}]", self.title), milli).bold(),
                micro,
                milli
            )
            .dimmed()
        );
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        self.top()
    }
}

fn to_color(msg: String, millis: u128) -> ColoredString {
    match millis {
        0..=99 => msg.green(),
        100..=999 => msg.yellow(),
        _ => msg.red(),
    }
}