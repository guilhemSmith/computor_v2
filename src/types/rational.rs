/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   rational.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:47:05 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/09 17:07:01 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{cmp, fmt, ops};

const PRECISION: usize = 10;
const EPSILON: f64 = 0.0000001;

#[derive(Eq, Ord, Copy, Clone, Debug)]
pub struct Rational {
    positiv: bool,
    numerator: u64,
    denominator: u64,
}

impl Rational {
    pub fn new(param: f64) -> Self {
        if param != 0.0 {
            let mut den = dec_div(param.abs());
            let mut num = (param.abs() * den as f64).round() as u64;

            simplify_gcd(&mut num, &mut den);
            Rational {
                positiv: param >= 0.0,
                numerator: num,
                denominator: den,
            }
        } else {
            Rational {
                positiv: true,
                numerator: 0,
                denominator: 1,
            }
        }
    }

    pub fn zero() -> Self {
        Rational {
            positiv: true,
            numerator: 0,
            denominator: 1,
        }
    }

    pub fn pow(&self, power: u32) -> Rational {
        match power {
            0 => Rational {
                positiv: true,
                numerator: 1,
                denominator: 1,
            },
            _ => {
                let mut num = self.numerator.pow(power);
                let mut den = self.denominator.pow(power);
                simplify_gcd(&mut num, &mut den);
                Rational {
                    positiv: self.positiv || power % 2 == 0,
                    numerator: self.numerator.pow(power),
                    denominator: self.denominator.pow(power),
                }
            }
        }
    }

    pub fn is_int(&self) -> bool {
        (self.numerator as f64 / self.denominator as f64).fract() == 0.0
    }

    pub fn get_val(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
            * if self.positiv { 1.0 } else { -1.0 }
    }

    pub fn simplify(&mut self) {
        simplify_gcd(&mut self.numerator, &mut self.denominator)
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.positiv { "" } else { "- " };
        if self.denominator != 0 {
            let float_value: f64 =
                self.numerator as f64 / self.denominator as f64;
            let fract_len = dec_len(float_value);
            if fract_len <= PRECISION {
                write!(f, "{}{:.*}", sign, fract_len, float_value)
            } else {
                write!(f, "{}{:.*}..", sign, PRECISION, float_value)
            }
        } else {
            write!(f, "{}inf", sign)
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

impl cmp::PartialOrd for Rational {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        match (self.positiv, rhs.positiv) {
            (left_sign, right_sign) if left_sign && !right_sign => {
                Some(cmp::Ordering::Greater)
            }
            (left_sign, right_sign) if !left_sign && right_sign => {
                Some(cmp::Ordering::Less)
            }
            (left_sign, right_sign) if left_sign && right_sign => Some(
                (self.numerator * rhs.denominator)
                    .cmp(&(self.denominator * rhs.numerator)),
            ),
            (left_sign, right_sign) if !left_sign && !right_sign => Some(
                (rhs.numerator * self.denominator)
                    .cmp(&(rhs.denominator * self.numerator)),
            ),
            _ => None,
        }
    }
}

impl ops::Add<Rational> for Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Rational {
        let sign;
        let mut num = if (self.positiv && rhs.positiv)
            || !(self.positiv || rhs.positiv)
        {
            sign = self.positiv && rhs.positiv;
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
            if val_a == self.numerator * rhs.denominator {
                sign = self.positiv
            } else {
                sign = rhs.positiv
            };
            val_a - val_b
        };
        let mut den = self.denominator * rhs.denominator;

        simplify_gcd(&mut num, &mut den);
        Rational {
            positiv: sign || num == 0,
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
        let sign;
        let mut num = if (self.positiv && rhs_sig) || !(self.positiv || rhs_sig)
        {
            sign = self.positiv && rhs_sig;
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
            if val_a == self.numerator * rhs.denominator {
                sign = self.positiv
            } else {
                sign = rhs_sig
            };
            val_a - val_b
        };
        let mut den = self.denominator * rhs.denominator;

        simplify_gcd(&mut num, &mut den);
        Rational {
            positiv: sign || num == 0,
            numerator: num,
            denominator: den,
        }
    }
}

impl ops::Mul<Rational> for Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Rational {
        let mut num = self.numerator * rhs.numerator;
        let mut den = self.denominator * rhs.denominator;

        simplify_gcd(&mut num, &mut den);
        Rational {
            positiv: num == 0
                || (self.positiv && rhs.positiv)
                || !(self.positiv || rhs.positiv),
            numerator: num,
            denominator: den,
        }
    }
}

impl ops::Div<Rational> for Rational {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Rational {
        let mut num = self.numerator * rhs.denominator;
        let mut den = self.denominator * rhs.numerator;

        simplify_gcd(&mut num, &mut den);
        Rational {
            positiv: num == 0
                || (self.positiv && rhs.positiv)
                || !(self.positiv || rhs.positiv),
            numerator: num,
            denominator: den,
        }
    }
}

impl ops::Rem<Rational> for Rational {
    type Output = Rational;

    fn rem(self, rhs: Rational) -> Rational {
        if self < rhs {
            self
        } else {
            let mut num = (self.numerator * rhs.denominator)
                % (rhs.numerator * self.denominator);
            let mut den = self.denominator * rhs.denominator;

            simplify_gcd(&mut num, &mut den);
            Rational {
                positiv: num == 0 || self.positiv,
                numerator: num,
                denominator: den,
            }
        }
    }
}

fn dec_len(nb: f64) -> usize {
    let mut len = PRECISION + 1;
    let mut ten_power: f64 = 10.0_f64.powi(PRECISION as i32 + 1);
    let mut limited = (nb * ten_power).round();
    let mut fract = limited.fract();
    while len > 0 && fract < EPSILON {
        len -= 1;
        limited = limited / 10.0;
        fract = limited.fract();
        ten_power = ten_power / 10.0;
    }
    if fract < EPSILON {
        len
    } else {
        len + 1
    }
}

fn dec_div(nb: f64) -> u64 {
    let mut ten_power: f64 = 10.0_f64.powi(PRECISION as i32 + 1);
    let mut limited = (nb * ten_power).round();
    let mut fract = limited.fract();
    while ten_power >= 1.0 && fract < EPSILON {
        limited = limited / 10.0;
        fract = limited.fract();
        ten_power = ten_power / 10.0;
    }
    return (ten_power * 10.0) as u64;
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
mod constructor {
    use super::Rational;
    #[test]
    fn new_zero() {
        let zero = Rational::zero();
        assert!(zero.positiv, "Zero is positive.");
        assert_eq!(zero.numerator, 0, "Zero numerator is not null");
        assert_eq!(
            zero.denominator, 1,
            "Zero denominator should never be null"
        );
    }

    #[test]
    fn new_float_0() {
        let value = Rational::new(0.0);

        assert!(value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 0, "Float invalid numerator");
        assert_eq!(value.denominator, 1, "Float invalid denominator");
    }

    #[test]
    fn new_float_1() {
        let value = Rational::new(-42.42);

        assert!(!value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 2121, "Float invalid numerator");
        assert_eq!(value.denominator, 50, "Float invalid denominator");
    }

    #[test]
    fn new_float_2() {
        let value = Rational::new(123.0);

        assert!(value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 123, "Float invalid numerator");
        assert_eq!(value.denominator, 1, "Float invalid denominator");
    }

    #[test]
    fn new_float_3() {
        let value = Rational::new(-99999999.9);

        assert!(!value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 999999999, "Float invalid numerator");
        assert_eq!(value.denominator, 10, "Float invalid denominator");
    }

    #[test]
    fn new_float_4() {
        let value = Rational::new(111111111.1);

        assert!(value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 1111111111, "Float invalid numerator");
        assert_eq!(value.denominator, 10, "Float invalid denominator");
    }

    #[test]
    fn new_float_5() {
        let value = Rational::new(-7.77777777);

        assert!(!value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 777777777, "Float invalid numerator");
        assert_eq!(value.denominator, 100000000, "Float invalid denominator");
    }

    #[test]
    fn new_float_6() {
        let value = Rational::new(3.333333333);

        assert!(value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 3333333333, "Float invalid numerator");
        assert_eq!(value.denominator, 1000000000, "Float invalid denominator");
    }
}

#[cfg(test)]
mod operator {
    use super::Rational;

    #[test]
    fn add_rational() {
        let zero = Rational::zero();
        let neg_big = Rational::new(-100042.4242);
        let neg_small = Rational::new(-0.4256);
        let pos_big = Rational::new(12345678.254);
        let pos_small = Rational::new(0.85642);

        assert_eq!(zero + pos_big, pos_big);
        assert_eq!(pos_small + zero, pos_small);
        assert_eq!(zero + neg_big, neg_big);
        assert_eq!(neg_small + zero, neg_small);

        assert_eq!(pos_small + pos_big, Rational::new(12345679.11042));
        assert_eq!(neg_small + neg_big, Rational::new(-100042.8498));
        assert_eq!(pos_small + neg_big, Rational::new(-100041.56778));
        assert_eq!(neg_small + pos_big, Rational::new(12345677.8284));
    }

    #[test]
    fn sub_rational() {
        let zero = Rational::zero();
        let neg_big = Rational::new(-100042.4242);
        let neg_small = Rational::new(-0.4256);
        let pos_big = Rational::new(12345678.254);
        let pos_small = Rational::new(0.85642);

        assert_eq!(zero - pos_big, Rational::new(-12345678.254));
        assert_eq!(pos_small - zero, pos_small);
        assert_eq!(zero - neg_big, Rational::new(100042.4242));
        assert_eq!(neg_small - zero, neg_small);

        assert_eq!(pos_small - pos_big, Rational::new(-12345677.39758));
        assert_eq!(neg_small - neg_big, Rational::new(100041.9986));
        assert_eq!(pos_small - neg_big, Rational::new(100043.28062));
        assert_eq!(neg_small - pos_big, Rational::new(-12345678.6796));
    }

    #[test]
    fn mul_rational() {
        let zero = Rational::zero();
        let neg_big = Rational::new(-100042.4242);
        let neg_small = Rational::new(-0.4256);
        let pos_big = Rational::new(12345678.254);
        let pos_small = Rational::new(0.85642);

        assert_eq!(zero * pos_big, zero);
        assert_eq!(pos_small * zero, zero);
        assert_eq!(zero * neg_big, zero);
        assert_eq!(neg_small * zero, zero);

        assert_eq!(pos_small * pos_big, Rational::new(10573085.77029068));
        assert_eq!(neg_small * neg_big, Rational::new(42578.05573952));
        assert_eq!(pos_small * neg_big, Rational::new(-85678.332933364));
        assert_eq!(neg_small * pos_big, Rational::new(-5254320.6649024));
    }

    #[test]
    fn div_rational() {
        let zero = Rational::zero();
        let one = Rational::new(1.0);
        let three = Rational::new(3.0);

        assert_eq!(zero / one, zero);
        assert_eq!(zero / three, zero);

        let third = one / three;
        assert_ne!(third, Rational::new(0.3333333333333333333));
        assert_eq!(third * three, one);
    }

    #[test]
    fn mod_rational() {
        let zero = Rational::zero();
        let neg_big = Rational::new(-133.33);
        let neg_small = Rational::new(-1.42);
        let pos_big = Rational::new(123.123);
        let pos_small = Rational::new(2.222);

        assert_eq!(pos_big % pos_big, zero);
        assert_eq!(zero % pos_big, zero);
        assert_eq!(zero % neg_big, zero);

        assert_eq!(pos_small % pos_big, Rational::new(2.222));
        assert_eq!(neg_small % neg_big, Rational::new(-1.42));
    }

    #[test]
    fn cmp_0() {
        let val_a = Rational::zero();
        let val_b = Rational::new(42.0 / 10.0);

        assert!(val_a < val_b);
        assert!(val_a <= val_b);
        assert!(!(val_a > val_b));
        assert!(!(val_a >= val_b));
    }

    #[test]
    fn cmp_1() {
        let val_a = Rational::new(8.0 / 10.0);
        let val_b = Rational::new(42.0 / 10.0);

        assert!(val_a < val_b);
        assert!(val_a <= val_b);
        assert!(!(val_a > val_b));
        assert!(!(val_a >= val_b));
    }

    #[test]
    fn cmp_2() {
        let val_a = Rational::zero();
        let val_b = Rational::new(-42.0 / 10.0);

        assert!(val_a > val_b);
        assert!(val_a >= val_b);
        assert!(!(val_a < val_b));
        assert!(!(val_a <= val_b));
    }

    #[test]
    fn cmp_3() {
        let val_a = Rational::new(8.0 / 10.0);
        let val_b = Rational::new(-42.0 / 10.0);

        assert!(val_a > val_b);
        assert!(val_a >= val_b);
        assert!(!(val_a < val_b));
        assert!(!(val_a <= val_b));
    }

    #[test]
    fn cmp_4() {
        let val_a = Rational::new(-8.0 / 10.0);
        let val_b = Rational::new(-42.0 / 10.0);

        assert!(val_a > val_b);
        assert!(val_a >= val_b);
        assert!(!(val_a < val_b));
        assert!(!(val_a <= val_b));
    }
}

#[cfg(test)]
mod pow {
    use super::Rational;

    #[test]
    fn square() {
        let zero = Rational::zero();
        let neg_big = Rational::new(-100042.4242);
        let neg_small = Rational::new(-0.4256);
        let pos_big = Rational::new(2345678.254);
        let pos_small = Rational::new(0.85642);

        assert_eq!(zero.pow(2).get_val(), 0.0);
        assert_eq!(neg_big.pow(2).get_val(), 10008486639.81274564);
        assert_eq!(neg_small.pow(2).get_val(), 0.18113536);
        assert_eq!(pos_big.pow(2).get_val(), 5502206471288.489);
        assert_eq!(pos_small.pow(2).get_val(), 0.7334552164);
    }

    #[test]
    fn fith() {
        let zero = Rational::zero();
        let neg = Rational::new(-12.42);
        let pos = Rational::new(53.89);

        assert_eq!(zero.pow(5).get_val(), 0.0);
        assert_eq!(neg.pow(5).get_val(), -295534.3588067232);
        assert_eq!(pos.pow(5).get_val(), 454507357.5715545949);
    }
}

#[cfg(test)]
mod other {
    use super::gcd;

    #[test]
    fn gcd_result() {
        assert_eq!(gcd(1029, 1071), 21);
        assert_eq!(gcd(221, 782), 17);
        assert_eq!(gcd(782, 32), 2);
        assert_eq!(gcd(78752, 3), 1);
    }
}
