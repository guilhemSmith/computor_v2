/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   rational.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:47:05 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/19 16:56:12 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{cmp, fmt, ops};

use std::i32::{MAX as I32_MAX, MIN as I32_MIN};

use super::{read_overflow, OpResult};
use crate::computor::ComputorError;

const PRECISION: usize = 10;
const EPSILON: f64 = 0.0000001;

#[derive(Eq, Ord, Copy, Clone, Debug)]
pub struct Rational {
    posit: bool,
    num: u64,
    den: u64,
}

impl Rational {
    pub fn new(param: f64) -> Self {
        if param != 0.0 {
            let mut den = dec_div(param.abs());
            let mut num = (param.abs() * den as f64).round() as u64;

            simplify_gcd(&mut num, &mut den);
            Rational {
                posit: param >= 0.0,
                num,
                den: den,
            }
        } else {
            Rational {
                posit: true,
                num: 0,
                den: 1,
            }
        }
    }

    pub fn zero() -> Self {
        Rational {
            posit: true,
            num: 0,
            den: 1,
        }
    }

    pub fn is_int(&self) -> bool {
        self.den == 1
    }

    pub fn get_val(&self) -> f64 {
        self.num as f64 / self.den as f64 * if self.posit { 1.0 } else { -1.0 }
    }

    pub fn simplify(&mut self) {
        simplify_gcd(&mut self.num, &mut self.den);
        if self.num == 0 {
            self.posit = true
        }
    }

    pub fn add(&self, other: &Rational) -> OpResult<Self> {
        let sig;
        let left = read_overflow(self.num.overflowing_mul(other.den))?;
        let right = read_overflow(other.num.overflowing_mul(self.den))?;
        let mut num =
            if (self.posit && other.posit) || !(self.posit || other.posit) {
                sig = self.posit && other.posit;
                read_overflow(left.overflowing_add(right))?
            } else {
                let val_a = cmp::max(left, right);
                let val_b = cmp::min(left, right);
                sig = if val_a == left {
                    self.posit
                } else {
                    other.posit
                };
                read_overflow(val_a.overflowing_sub(val_b))?
            };
        let mut den = read_overflow(self.den.overflowing_mul(other.den))?;

        simplify_gcd(&mut num, &mut den);
        Ok(Rational {
            posit: sig || num == 0,
            num,
            den,
        })
    }

    pub fn sub(&self, other: &Rational) -> OpResult<Self> {
        let negated = -*other;
        self.add(&negated)
    }

    pub fn mul(&self, other: &Rational) -> OpResult<Self> {
        let mut num = read_overflow(self.num.overflowing_mul(other.num))?;
        let mut den = read_overflow(self.den.overflowing_mul(other.den))?;

        simplify_gcd(&mut num, &mut den);
        Ok(Rational {
            posit: num == 0
                || (self.posit && other.posit)
                || !(self.posit || other.posit),
            num,
            den,
        })
    }

    pub fn div(&self, other: &Rational) -> OpResult<Self> {
        if other.num == 0 {
            return Err(ComputorError::div_by_zero());
        }
        let mut num = read_overflow(self.num.overflowing_mul(other.den))?;
        let mut den = read_overflow(self.den.overflowing_mul(other.num))?;

        simplify_gcd(&mut num, &mut den);
        Ok(Rational {
            posit: num == 0
                || (self.posit && other.posit)
                || !(self.posit || other.posit),
            num,
            den,
        })
    }

    pub fn rem(&self, other: &Rational) -> OpResult<Self> {
        if other.num == 0 {
            return Err(ComputorError::div_by_zero());
        }
        if self < other {
            Ok(*self)
        } else {
            let left = read_overflow(self.num.overflowing_mul(other.den))?;
            let right = read_overflow(other.num.overflowing_mul(self.den))?;
            let mut num = read_overflow(left.overflowing_rem(right))?;
            let mut den = read_overflow(other.den.overflowing_mul(self.den))?;

            simplify_gcd(&mut num, &mut den);
            Ok(Rational {
                posit: num == 0 || self.posit,
                num,
                den,
            })
        }
    }

    pub fn pow(&self, power: i32) -> OpResult<Self> {
        let mut num: u64;
        let mut den: u64;
        let pow: u32;
        match power {
            0 => {
                return Ok(Rational {
                    posit: true,
                    num: 1,
                    den: 1,
                });
            }
            0..=I32_MAX => {
                pow = power as u32;
                num = read_overflow(self.num.overflowing_pow(pow))?;
                den = read_overflow(self.den.overflowing_pow(pow))?;
            }
            I32_MIN..=0 => {
                pow = -power as u32;
                num = read_overflow(self.den.overflowing_pow(pow))?;
                den = read_overflow(self.num.overflowing_pow(pow))?;
            }
        };
        simplify_gcd(&mut num, &mut den);
        Ok(Rational {
            posit: self.posit || pow % 2 == 0,
            num,
            den,
        })
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.posit { "" } else { "- " };
        if self.den != 0 {
            let float_value: f64 = self.num as f64 / self.den as f64;
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
        self.posit == rhs.posit && self.num == rhs.num && self.den == rhs.den
    }
}

impl cmp::PartialOrd for Rational {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        match (self.posit, rhs.posit) {
            (left_sign, right_sign) if left_sign && !right_sign => {
                Some(cmp::Ordering::Greater)
            }
            (left_sign, right_sign) if !left_sign && right_sign => {
                Some(cmp::Ordering::Less)
            }
            (left_sign, right_sign) if left_sign && right_sign => {
                Some((self.num * rhs.den).cmp(&(self.den * rhs.num)))
            }
            (left_sign, right_sign) if !left_sign && !right_sign => {
                Some((rhs.num * self.den).cmp(&(rhs.den * self.num)))
            }
            _ => None,
        }
    }
}

impl ops::Neg for Rational {
    type Output = Rational;

    fn neg(mut self) -> Self::Output {
        self.posit = self.num == 0 || !self.posit;
        self
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
        assert!(zero.posit, "Zero is posite.");
        assert_eq!(zero.num, 0, "Zero numerator is not null");
        assert_eq!(zero.den, 1, "Zero denominator should never be null");
    }

    #[test]
    fn new_float_0() {
        let value = Rational::new(0.0);

        assert!(value.posit, "Float invalid sign");
        assert_eq!(value.num, 0, "Float invalid numerator");
        assert_eq!(value.den, 1, "Float invalid denominator");
    }

    #[test]
    fn new_float_1() {
        let value = Rational::new(-42.42);

        assert!(!value.posit, "Float invalid sign");
        assert_eq!(value.num, 2121, "Float invalid numerator");
        assert_eq!(value.den, 50, "Float invalid denominator");
    }

    #[test]
    fn new_float_2() {
        let value = Rational::new(123.0);

        assert!(value.posit, "Float invalid sign");
        assert_eq!(value.num, 123, "Float invalid numerator");
        assert_eq!(value.den, 1, "Float invalid denominator");
    }

    #[test]
    fn new_float_3() {
        let value = Rational::new(-99999999.9);

        assert!(!value.posit, "Float invalid sign");
        assert_eq!(value.num, 999999999, "Float invalid numerator");
        assert_eq!(value.den, 10, "Float invalid denominator");
    }

    #[test]
    fn new_float_4() {
        let value = Rational::new(111111111.1);

        assert!(value.posit, "Float invalid sign");
        assert_eq!(value.num, 1111111111, "Float invalid numerator");
        assert_eq!(value.den, 10, "Float invalid denominator");
    }

    #[test]
    fn new_float_5() {
        let value = Rational::new(-7.77777777);

        assert!(!value.posit, "Float invalid sign");
        assert_eq!(value.num, 777777777, "Float invalid numerator");
        assert_eq!(value.den, 100000000, "Float invalid den");
    }

    #[test]
    fn new_float_6() {
        let value = Rational::new(3.333333333);

        assert!(value.posit, "Float invalid sign");
        assert_eq!(value.num, 3333333333, "Float invalid numerator");
        assert_eq!(value.den, 1000000000, "Float invalid den");
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

        assert_eq!(zero.add(&pos_big).unwrap(), pos_big);
        assert_eq!(pos_small.add(&zero).unwrap(), pos_small);
        assert_eq!(zero.add(&neg_big).unwrap(), neg_big);
        assert_eq!(neg_small.add(&zero).unwrap(), neg_small);

        assert_eq!(
            pos_small.add(&pos_big).unwrap(),
            Rational::new(12345679.11042)
        );
        assert_eq!(
            neg_small.add(&neg_big).unwrap(),
            Rational::new(-100042.8498)
        );
        assert_eq!(
            pos_small.add(&neg_big).unwrap(),
            Rational::new(-100041.56778)
        );
        assert_eq!(
            neg_small.add(&pos_big).unwrap(),
            Rational::new(12345677.8284)
        );
    }

    #[test]
    fn sub_rational() {
        let zero = Rational::zero();
        let neg_big = Rational::new(-100042.4242);
        let neg_small = Rational::new(-0.4256);
        let pos_big = Rational::new(12345678.254);
        let pos_small = Rational::new(0.85642);

        assert_eq!(zero.sub(&pos_big).unwrap(), Rational::new(-12345678.254));
        assert_eq!(pos_small.sub(&zero).unwrap(), pos_small);
        assert_eq!(zero.sub(&neg_big).unwrap(), Rational::new(100042.4242));
        assert_eq!(neg_small.sub(&zero).unwrap(), neg_small);

        assert_eq!(
            pos_small.sub(&pos_big).unwrap(),
            Rational::new(-12345677.39758)
        );
        assert_eq!(
            neg_small.sub(&neg_big).unwrap(),
            Rational::new(100041.9986)
        );
        assert_eq!(
            pos_small.sub(&neg_big).unwrap(),
            Rational::new(100043.28062)
        );
        assert_eq!(
            neg_small.sub(&pos_big).unwrap(),
            Rational::new(-12345678.6796)
        );
    }

    #[test]
    fn mul_rational() {
        let zero = Rational::zero();
        let neg_big = Rational::new(-100042.4242);
        let neg_small = Rational::new(-0.4256);
        let pos_big = Rational::new(12345678.254);
        let pos_small = Rational::new(0.85642);

        assert_eq!(zero.mul(&pos_big).unwrap(), zero);
        assert_eq!(pos_small.mul(&zero).unwrap(), zero);
        assert_eq!(zero.mul(&neg_big).unwrap(), zero);
        assert_eq!(neg_small.mul(&zero).unwrap(), zero);

        assert_eq!(
            pos_small.mul(&pos_big).unwrap(),
            Rational::new(10573085.77029068)
        );
        assert_eq!(
            neg_small.mul(&neg_big).unwrap(),
            Rational::new(42578.05573952)
        );
        assert_eq!(
            pos_small.mul(&neg_big).unwrap(),
            Rational::new(-85678.332933364)
        );
        assert_eq!(
            neg_small.mul(&pos_big).unwrap(),
            Rational::new(-5254320.6649024)
        );
    }

    #[test]
    fn div_rational() {
        let zero = Rational::zero();
        let one = Rational::new(1.0);
        let three = Rational::new(3.0);

        assert_eq!(zero.div(&one).unwrap(), zero);
        assert_eq!(zero.div(&three).unwrap(), zero);

        let third = one.div(&three).unwrap();
        assert_ne!(third, Rational::new(0.3333333333333333333));
        assert_eq!(third.mul(&three).unwrap(), one);
    }

    #[test]
    fn mod_rational() {
        let zero = Rational::zero();
        let neg_big = Rational::new(-133.33);
        let neg_small = Rational::new(-1.42);
        let pos_big = Rational::new(123.123);
        let pos_small = Rational::new(2.222);

        assert_eq!(pos_big.rem(&pos_big).unwrap(), zero);
        assert_eq!(zero.rem(&pos_big).unwrap(), zero);
        assert_eq!(zero.rem(&neg_big).unwrap(), zero);

        assert_eq!(pos_small.rem(&pos_big).unwrap(), Rational::new(2.222));
        assert_eq!(neg_small.rem(&neg_big).unwrap(), Rational::new(-1.42));
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

        assert_eq!(zero.pow(2).unwrap().get_val(), 0.0);
        assert_eq!(neg_big.pow(2).unwrap().get_val(), 10008486639.81274564);
        assert_eq!(neg_small.pow(2).unwrap().get_val(), 0.18113536);
        assert_eq!(pos_big.pow(2).unwrap().get_val(), 5502206471288.489);
        assert_eq!(pos_small.pow(2).unwrap().get_val(), 0.7334552164);
    }

    #[test]
    fn fith() {
        let zero = Rational::zero();
        let neg = Rational::new(-12.42);
        let pos = Rational::new(53.89);

        assert_eq!(zero.pow(5).unwrap().get_val(), 0.0);
        assert_eq!(neg.pow(5).unwrap().get_val(), -295534.3588067232);
        assert_eq!(pos.pow(5).unwrap().get_val(), 454507357.5715545949);
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
