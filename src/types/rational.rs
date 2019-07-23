/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   rational.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:47:05 by gsmith            #+#    #+#             */
/*   Updated: 2019/07/23 11:53:32 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{cmp, fmt, ops};

pub enum RationalParam {
    Float(f64),
    Couple(i64, i64),
    Zero,
}

#[derive(Eq, Ord, Copy, Clone, Debug)]
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
mod constructor {
    use super::{Rational, RationalParam};
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
    fn new_float_0() {
        let value = Rational::new(RationalParam::Float(0.0));

        assert!(value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 0, "Float invalid numerator");
        assert_eq!(value.denominator, 1, "Float invalid denominator");
    }

    #[test]
    fn new_float_1() {
        let value = Rational::new(RationalParam::Float(-42.42));

        assert!(!value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 2121, "Float invalid numerator");
        assert_eq!(value.denominator, 50, "Float invalid denominator");
    }

    #[test]
    fn new_float_2() {
        let value = Rational::new(RationalParam::Float(123.0));

        assert!(value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 123, "Float invalid numerator");
        assert_eq!(value.denominator, 1, "Float invalid denominator");
    }

    #[test]
    fn new_float_3() {
        let value = Rational::new(RationalParam::Float(-99999999.9));

        assert!(!value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 999999999, "Float invalid numerator");
        assert_eq!(value.denominator, 10, "Float invalid denominator");
    }

    #[test]
    fn new_float_4() {
        let value = Rational::new(RationalParam::Float(111111111.1));

        assert!(value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 1111111111, "Float invalid numerator");
        assert_eq!(value.denominator, 10, "Float invalid denominator");
    }

    #[test]
    fn new_float_5() {
        let value = Rational::new(RationalParam::Float(-7.77777777));

        assert!(!value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 777777777, "Float invalid numerator");
        assert_eq!(value.denominator, 100000000, "Float invalid denominator");
    }

    #[test]
    fn new_float_6() {
        let value = Rational::new(RationalParam::Float(3.333333333));

        assert!(value.positiv, "Float invalid sign");
        assert_eq!(value.numerator, 3333333333, "Float invalid numerator");
        assert_eq!(value.denominator, 1000000000, "Float invalid denominator");
    }

    #[test]
    fn new_couple_0() {
        let value = Rational::new(RationalParam::Couple(0, 1));

        assert!(value.positiv, "Couple invalid sign");
        assert_eq!(value.numerator, 0, "Couple invalid numerator");
        assert_eq!(value.denominator, 1, "Couple invalid denominator");
    }

    #[test]
    fn new_couple_1() {
        let value = Rational::new(RationalParam::Couple(-986, -3));

        assert!(value.positiv, "Couple invalid sign");
        assert_eq!(value.numerator, 986, "Couple invalid numerator");
        assert_eq!(value.denominator, 3, "Couple invalid denominator");
    }

    #[test]
    fn new_couple_2() {
        let value = Rational::new(RationalParam::Couple(-1, 3));

        assert!(!value.positiv, "Couple invalid sign");
        assert_eq!(value.numerator, 1, "Couple invalid numerator");
        assert_eq!(value.denominator, 3, "Couple invalid denominator");
    }

}

#[cfg(test)]
mod operator {
    use super::{Rational, RationalParam};

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
    fn mul_rational() {
        let zero = Rational::new(RationalParam::Zero);
        let neg_big = Rational::new(RationalParam::Couple(-133, 4));
        let neg_small = Rational::new(RationalParam::Couple(-1, 2));
        let pos_big = Rational::new(RationalParam::Couple(123, 4));
        let pos_small = Rational::new(RationalParam::Couple(2, 3));

        assert_eq!(zero * pos_big, zero);
        assert_eq!(pos_small * zero, zero);
        assert_eq!(zero * neg_big, zero);
        assert_eq!(neg_small * zero, zero);

        assert_eq!(
            pos_small * pos_big,
            Rational::new(RationalParam::Couple(246, 12))
        );
        assert_eq!(
            neg_small * neg_big,
            Rational::new(RationalParam::Couple(133, 8))
        );
        assert_eq!(
            pos_small * neg_big,
            Rational::new(RationalParam::Couple(-266, 12))
        );
        assert_eq!(
            neg_small * pos_big,
            Rational::new(RationalParam::Couple(-123, 8))
        );
    }

    #[test]
    fn div_rational() {
        let zero = Rational::new(RationalParam::Zero);
        let neg_big = Rational::new(RationalParam::Couple(-133, 4));
        let neg_small = Rational::new(RationalParam::Couple(-1, 2));
        let pos_big = Rational::new(RationalParam::Couple(123, 4));
        let pos_small = Rational::new(RationalParam::Couple(2, 3));

        assert_eq!(zero / pos_big, zero);
        assert_eq!(zero / neg_big, zero);

        assert_eq!(
            pos_small / pos_big,
            Rational::new(RationalParam::Couple(8, 369))
        );
        assert_eq!(
            neg_small / neg_big,
            Rational::new(RationalParam::Couple(4, 266))
        );
        assert_eq!(
            pos_small / neg_big,
            Rational::new(RationalParam::Couple(-8, 399))
        );
        assert_eq!(
            neg_small / pos_big,
            Rational::new(RationalParam::Couple(-4, 246))
        );
    }

    #[test]
    fn cmp_0() {
        let val_a = Rational::new(RationalParam::Zero);
        let val_b = Rational::new(RationalParam::Couple(42, 10));

        assert!(val_a < val_b);
        assert!(val_a <= val_b);
        assert!(!(val_a > val_b));
        assert!(!(val_a >= val_b));
    }

    #[test]
    fn cmp_1() {
        let val_a = Rational::new(RationalParam::Couple(8, 10));
        let val_b = Rational::new(RationalParam::Couple(42, 10));

        assert!(val_a < val_b);
        assert!(val_a <= val_b);
        assert!(!(val_a > val_b));
        assert!(!(val_a >= val_b));
    }

    #[test]
    fn cmp_2() {
        let val_a = Rational::new(RationalParam::Zero);
        let val_b = Rational::new(RationalParam::Couple(-42, 10));

        assert!(val_a > val_b);
        assert!(val_a >= val_b);
        assert!(!(val_a < val_b));
        assert!(!(val_a <= val_b));
    }

    #[test]
    fn cmp_3() {
        let val_a = Rational::new(RationalParam::Couple(8, 10));
        let val_b = Rational::new(RationalParam::Couple(-42, 10));

        assert!(val_a > val_b);
        assert!(val_a >= val_b);
        assert!(!(val_a < val_b));
        assert!(!(val_a <= val_b));
    }

    #[test]
    fn cmp_4() {
        let val_a = Rational::new(RationalParam::Couple(-8, 10));
        let val_b = Rational::new(RationalParam::Couple(-42, 10));

        assert!(val_a > val_b);
        assert!(val_a >= val_b);
        assert!(!(val_a < val_b));
        assert!(!(val_a <= val_b));
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
