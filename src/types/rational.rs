/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   rational.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:47:05 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/22 17:32:26 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{cmp, fmt, ops};

pub enum RationalParam {
    Float(f64),
    Couple(i64, i64),
    Zero,
}

#[derive(Eq, Copy, Clone, Debug)]
pub struct Rational {
    positiv: bool,
    numerator: u64,
    denominator: u64,
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            if self.positiv { "" } else { "- " },
            self.numerator,
            if self.denominator > 1 {
                format!(" / {}", self.denominator)
            } else {
                String::from("")
            }
        )
    }
}

impl Rational {
    pub fn new(param: RationalParam) -> Self {
        match param {
            RationalParam::Float(f_value) => {
                let mut den = dec_div(f_value);
                let mut num = (f_value.abs() * den as f64) as u64;

                simplify_gcd(&mut num, &mut den);
                Rational {
                    positiv: f_value >= 0.0,
                    numerator: num,
                    denominator: den,
                }
            }
            RationalParam::Couple(n_value, d_value) => {
                let mut num = n_value.abs() as u64;
                let mut den = d_value.abs() as u64;

                simplify_gcd(&mut num, &mut den);
                Rational {
                    positiv: n_value * d_value >= 0,
                    numerator: num,
                    denominator: den,
                }
            }
            RationalParam::Zero => Rational {
                positiv: true,
                numerator: 0,
                denominator: 1,
            },
        }
    }
}

impl cmp::PartialEq for Rational {
    fn eq(&self, rhs: &Self) -> bool {
        self.positiv == rhs.positiv
            && self.numerator == rhs.numerator
            && self.denominator == rhs.denominator
    }
}

impl ops::Add<Rational> for Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Rational {
        let mut sign = self.positiv && rhs.positiv;
        let mut num = if (self.positiv && rhs.positiv)
            || !(self.positiv || rhs.positiv)
        {
            self.numerator * rhs.denominator + rhs.numerator * self.denominator
        } else {
            let val_a = cmp::max(
                self.numerator * rhs.denominator,
                rhs.numerator * self.denominator,
            );
            let val_b = cmp::min(
                self.numerator * rhs.denominator,
                rhs.numerator * self.denominator,
            );
            sign = if val_a == self.numerator * rhs.denominator {
                self.positiv
            } else {
                rhs.positiv
            };
            val_a - val_b
        };
        let mut den = self.denominator * rhs.denominator;

        simplify_gcd(&mut num, &mut den);
        Rational {
            positiv: sign,
            numerator: num,
            denominator: den,
        }
    }
}

impl ops::Sub<Rational> for Rational {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Rational {
        let rhs_sig = if rhs.numerator != 0 {
            !rhs.positiv
        } else {
            true
        };
        let mut sign = self.positiv && rhs_sig;
        let mut num = if (self.positiv && rhs_sig) || !(self.positiv || rhs_sig)
        {
            self.numerator * rhs.denominator + rhs.numerator * self.denominator
        } else {
            let val_a = cmp::max(
                self.numerator * rhs.denominator,
                rhs.numerator * self.denominator,
            );
            let val_b = cmp::min(
                self.numerator * rhs.denominator,
                rhs.numerator * self.denominator,
            );
            sign = if val_a == self.numerator * rhs.denominator {
                self.positiv
            } else {
                rhs_sig
            };
            val_a - val_b
        };
        let mut den = self.denominator * rhs.denominator;

        simplify_gcd(&mut num, &mut den);
        Rational {
            positiv: sign,
            numerator: num,
            denominator: den,
        }
    }
}

fn dec_div(nb: f64) -> u64 {
    let mut dec: f64 = 1.0;
    while (nb * dec).fract() != 0.0 {
        dec = dec * 10.0;
    }
    return dec as u64;
}

fn simplify_gcd(num: &mut u64, den: &mut u64) {
    let div = gcd(*num, *den);
    *num = *num / div;
    *den = *den / div;
}

fn gcd(val_a: u64, val_b: u64) -> u64 {
    match val_b {
        0 => val_a,
        _ => gcd(val_b, val_a % val_b),
    }
}

#[cfg(test)]
mod tests {
    use super::{gcd, Rational, RationalParam};

    #[test]
    fn new_zero() {
        let zero = Rational::new(RationalParam::Zero);
        assert!(zero.positiv, "Zero is positive.");
        assert_eq!(zero.numerator, 0, "Zero numerator is not null");
        assert_eq!(
            zero.denominator, 1,
            "Zero denominator should never be null"
        );
    }

    #[test]
    fn new_float() {
        let values = (
            Rational::new(RationalParam::Float(0.0)),
            Rational::new(RationalParam::Float(-42.42)),
            Rational::new(RationalParam::Float(123.0)),
            Rational::new(RationalParam::Float(-99999999.9)),
            Rational::new(RationalParam::Float(111111111.1)),
            Rational::new(RationalParam::Float(-7.77777777)),
            Rational::new(RationalParam::Float(3.333333333)),
        );

        assert!(values.0.positiv, "Float invalid sign");
        assert_eq!(values.0.numerator, 0, "Float invalid numerator");
        assert_eq!(values.0.denominator, 1, "Float invalid denominator");

        assert!(!values.1.positiv, "Float invalid sign");
        assert_eq!(values.1.numerator, 2121, "Float invalid numerator");
        assert_eq!(values.1.denominator, 50, "Float invalid denominator");

        assert!(values.2.positiv, "Float invalid sign");
        assert_eq!(values.2.numerator, 123, "Float invalid numerator");
        assert_eq!(values.2.denominator, 1, "Float invalid denominator");

        assert!(!values.3.positiv, "Float invalid sign");
        assert_eq!(values.3.numerator, 999999999, "Float invalid numerator");
        assert_eq!(values.3.denominator, 10, "Float invalid denominator");

        assert!(values.4.positiv, "Float invalid sign");
        assert_eq!(values.4.numerator, 1111111111, "Float invalid numerator");
        assert_eq!(values.4.denominator, 10, "Float invalid denominator");

        assert!(!values.5.positiv, "Float invalid sign");
        assert_eq!(values.5.numerator, 777777777, "Float invalid numerator");
        assert_eq!(
            values.5.denominator, 100000000,
            "Float invalid denominator"
        );

        assert!(values.6.positiv, "Float invalid sign");
        assert_eq!(values.6.numerator, 3333333333, "Float invalid numerator");
        assert_eq!(
            values.6.denominator, 1000000000,
            "Float invalid denominator"
        );
    }

    #[test]
    fn new_couple() {
        let values = (
            Rational::new(RationalParam::Couple(0, 1)),
            Rational::new(RationalParam::Couple(-986, -3)),
            Rational::new(RationalParam::Couple(-1, 3)),
        );

        assert!(values.0.positiv, "Couple invalid sign");
        assert_eq!(values.0.numerator, 0, "Couple invalid numerator");
        assert_eq!(values.0.denominator, 1, "Couple invalid denominator");

        assert!(values.1.positiv, "Couple invalid sign");
        assert_eq!(values.1.numerator, 986, "Couple invalid numerator");
        assert_eq!(values.1.denominator, 3, "Couple invalid denominator");

        assert!(!values.2.positiv, "Couple invalid sign");
        assert_eq!(values.2.numerator, 1, "Couple invalid numerator");
        assert_eq!(values.2.denominator, 3, "Couple invalid denominator");
    }

    #[test]
    fn add_rational() {
        let zero = Rational::new(RationalParam::Zero);
        let neg_big = Rational::new(RationalParam::Couple(-133, 4));
        let neg_small = Rational::new(RationalParam::Couple(-1, 2));
        let pos_big = Rational::new(RationalParam::Couple(123, 4));
        let pos_small = Rational::new(RationalParam::Couple(2, 3));

        assert_eq!(zero + pos_big, pos_big);
        assert_eq!(pos_small + zero, pos_small);
        assert_eq!(zero + neg_big, neg_big);
        assert_eq!(neg_small + zero, neg_small);

        assert_eq!(
            pos_small + pos_big,
            Rational::new(RationalParam::Couple(377, 12))
        );
        assert_eq!(
            neg_small + neg_big,
            Rational::new(RationalParam::Couple(-270, 8))
        );
        assert_eq!(
            pos_small + neg_big,
            Rational::new(RationalParam::Couple(-391, 12))
        );
        assert_eq!(
            neg_small + pos_big,
            Rational::new(RationalParam::Couple(242, 8))
        );
    }

    #[test]
    fn sub_rational() {
        let zero = Rational::new(RationalParam::Zero);
        let neg_big = Rational::new(RationalParam::Couple(-133, 4));
        let neg_small = Rational::new(RationalParam::Couple(-1, 2));
        let pos_big = Rational::new(RationalParam::Couple(123, 4));
        let pos_small = Rational::new(RationalParam::Couple(2, 3));

        assert_eq!(
            zero - pos_big,
            Rational::new(RationalParam::Couple(-123, 4))
        );
        assert_eq!(pos_small - zero, pos_small);
        assert_eq!(
            zero - neg_big,
            Rational::new(RationalParam::Couple(133, 4))
        );
        assert_eq!(neg_small - zero, neg_small);

        assert_eq!(
            pos_small - pos_big,
            Rational::new(RationalParam::Couple(-361, 12))
        );
        assert_eq!(
            neg_small - neg_big,
            Rational::new(RationalParam::Couple(262, 8))
        );
        assert_eq!(
            pos_small - neg_big,
            Rational::new(RationalParam::Couple(407, 12))
        );
        assert_eq!(
            neg_small - pos_big,
            Rational::new(RationalParam::Couple(-250, 8))
        );
    }

    #[test]
    fn gcd_result() {
        assert_eq!(gcd(1029, 1071), 21);
        assert_eq!(gcd(221, 782), 17);
        assert_eq!(gcd(782, 32), 2);
        assert_eq!(gcd(78752, 3), 1);
    }
}
