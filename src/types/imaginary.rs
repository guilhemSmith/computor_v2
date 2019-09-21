/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   imaginary.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:46:59 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/21 15:44:29 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::rational::Rational;
use super::OpResult;
use crate::computor::ComputorError;
use std::{cmp, fmt, ops};

#[derive(Eq, Copy, Clone, Debug)]
pub struct Imaginary {
    real: Rational,
    irreal: Rational,
}

impl Imaginary {
    pub fn new(real: f64, irreal: f64) -> Self {
        Imaginary {
            real: Rational::new(real),
            irreal: Rational::new(irreal),
        }
    }

    pub fn get_real(&self) -> Rational {
        self.real
    }

    pub fn is_real(&self) -> bool {
        self.irreal == Rational::zero()
    }

    pub fn is_int(&self) -> bool {
        self.real.is_int() && self.irreal.is_int()
    }

    pub fn add(&self, other: &Imaginary) -> OpResult<Self> {
        let real = self.real.add(&other.real)?;
        let irreal = self.irreal.add(&other.irreal)?;
        let mut res = Imaginary { real, irreal };
        res.real.simplify();
        res.irreal.simplify();
        Ok(res)
    }

    pub fn sub(&self, other: &Imaginary) -> OpResult<Self> {
        let negated = -*other;
        self.add(&negated)
    }

    pub fn mul(&self, other: &Imaginary) -> OpResult<Self> {
        let tmp_left = self.real.mul(&other.real)?;
        let tmp_right = self.irreal.mul(&other.irreal)?;
        let real = tmp_left.sub(&tmp_right)?;
        let tmp_left = self.real.mul(&other.irreal)?;
        let tmp_right = self.irreal.mul(&other.real)?;
        let irreal = tmp_left.add(&tmp_right)?;
        let mut res = Imaginary { real, irreal };
        res.real.simplify();
        res.irreal.simplify();
        Ok(res)
    }

    pub fn div(&self, other: &Imaginary) -> OpResult<Self> {
        let real_prod = self.real.mul(&other.real)?;
        let irreal_prod = self.irreal.mul(&other.irreal)?;
        let real_num = real_prod.add(&irreal_prod)?;
        let real_cross = self.irreal.mul(&other.real)?;
        let irreal_cross = self.real.mul(&other.irreal)?;
        let irreal_num = real_cross.sub(&irreal_cross)?;
        let other_real_square = other.real.pow(2)?;
        let other_irreal_square = other.irreal.pow(2)?;
        let den = other_real_square.add(&other_irreal_square)?;
        let mut res = Imaginary {
            real: real_num.div(&den)?,
            irreal: irreal_num.div(&den)?,
        };
        res.real.simplify();
        res.irreal.simplify();
        Ok(res)
    }

    pub fn rem(&self, other: &Imaginary) -> OpResult<Self> {
        if self.is_real() && other.is_real() {
            let mut res = Imaginary {
                real: self.get_real().rem(&other.get_real())?,
                irreal: Rational::zero(),
            };
            res.real.simplify();
            res.irreal.simplify();
            Ok(res)
        } else {
            Err(ComputorError::mod_with_im())
        }
    }

    pub fn pow(&self, power: i32) -> OpResult<Self> {
        if power < 0 {
            return (Imaginary::new(1.0, 0.0).div(self)?).pow(-power);
        }
        let mut res = Imaginary::new(1.0, 0.0);
        let pow: u32 = power as u32;
        let mut i: u32 = 0;

        while pow > i {
            i += 1;
            res = res.mul(self)?;
        }
        res.real.simplify();
        res.irreal.simplify();
        return Ok(res);
    }
}

impl fmt::Display for Imaginary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.real != Rational::zero() {
            write!(
                f,
                "{}{}",
                self.real,
                if self.irreal != Rational::zero() {
                    format!(
                        " {}i",
                        if self.irreal > Rational::zero() {
                            format!(
                                "+ {}",
                                if self.irreal != Rational::new(1.0) {
                                    format!("{}", self.irreal)
                                } else {
                                    String::new()
                                }
                            )
                        } else {
                            if self.irreal != Rational::new(-1.0) {
                                format!("{}", self.irreal)
                            } else {
                                String::from("- ")
                            }
                        },
                    )
                } else {
                    String::from("")
                }
            )
        } else {
            write!(
                f,
                "{}",
                match self.irreal {
                    zero if zero == Rational::zero() => String::from("0"),
                    one if one == Rational::new(1.0) => String::from("i"),
                    other => format!("{}i", other),
                }
            )
        }
    }
}

impl cmp::PartialEq for Imaginary {
    fn eq(&self, rhs: &Self) -> bool {
        self.real == rhs.real && self.irreal == rhs.irreal
    }
}

impl ops::Neg for Imaginary {
    type Output = Imaginary;

    fn neg(mut self) -> Self::Output {
        self.real = -self.real;
        self.irreal = -self.irreal;
        self
    }
}

#[cfg(test)]
mod operator {
    use super::Imaginary;

    #[test]
    fn add_imaginary() {
        let zero = Imaginary::new(0.0, 0.0);
        let real_1 = Imaginary::new(42.0, 0.0);
        let real_2 = Imaginary::new(-13.00001456, 0.0);
        let irreal_1 = Imaginary::new(0.0, 81.0987);
        let irreal_2 = Imaginary::new(0.0, 50.0);
        let complex_1 = Imaginary::new(42.0, 50.0);
        let complex_2 = Imaginary::new(-42.0, -90.0);

        assert_eq!(real_1.add(&zero).unwrap(), real_1);
        assert_eq!(zero.add(&real_2).unwrap(), real_2);
        assert_eq!(irreal_1.add(&zero).unwrap(), irreal_1);
        assert_eq!(zero.add(&irreal_2).unwrap(), irreal_2);
        assert_eq!(complex_1.add(&zero).unwrap(), complex_1);
        assert_eq!(zero.add(&complex_2).unwrap(), complex_2);
        assert_eq!(real_1.add(&irreal_2).unwrap(), complex_1);
    }

    #[test]
    fn sub_imaginary() {
        let zero = Imaginary::new(0.0, 0.0);
        let real_1 = Imaginary::new(42.0, 0.0);
        let real_2 = Imaginary::new(-13.00001456, 0.0);
        let irreal_1 = Imaginary::new(0.0, 81.0987);
        let irreal_2 = Imaginary::new(0.0, 50.0);
        let complex_1 = Imaginary::new(42.0, 50.0);
        let complex_2 = Imaginary::new(-42.0, 50.0);

        assert_eq!(real_1.sub(&zero).unwrap(), real_1);
        assert_eq!(
            real_1.sub(&real_2).unwrap(),
            Imaginary::new(42.0 + 13.00001456, 0.0)
        );
        assert_eq!(irreal_1.sub(&zero).unwrap(), irreal_1);
        assert_eq!(complex_1.sub(&zero).unwrap(), complex_1);
        assert_eq!(irreal_2.sub(&real_1).unwrap(), complex_2);
    }

    #[test]
    fn mul_imaginary() {
        let zero = Imaginary::new(0.0, 0.0);
        let complex_1 = Imaginary::new(42.0, 50.0);
        let complex_2 = Imaginary::new(2.0, 3.0);
        let complex_3 = Imaginary::new(3.0, 2.0);
        let complex_4 = Imaginary::new(0.0, 13.0);

        assert_eq!(complex_1.mul(&zero).unwrap(), zero);
        assert_eq!(zero.mul(&complex_2).unwrap(), zero);
        assert_eq!(complex_2.mul(&complex_3).unwrap(), complex_4);
    }

    #[test]
    fn div_imaginary() {
        let zero = Imaginary::new(0.0, 0.0);
        let complex_2 = Imaginary::new(2.0, 3.0);
        let complex_3 = Imaginary::new(3.0, 2.0);
        let complex_4 = Imaginary::new(13.0, 0.0);
        let complex_5 = Imaginary::new(12.0, 5.0).div(&complex_4).unwrap();

        assert_eq!(zero.div(&complex_2).unwrap(), zero);
        assert_eq!(complex_2.div(&complex_3).unwrap(), complex_5);
    }
}

#[cfg(test)]
mod pow {
    use super::Imaginary;

    #[test]
    fn square() {
        let zero = Imaginary::new(0.0, 0.0);
        let raw = Imaginary::new(4.0, -6.2);
        let res = Imaginary::new(-22.44, -49.6);

        assert_eq!(zero.pow(2).unwrap(), zero);
        assert_eq!(raw.pow(2).unwrap(), res);
    }

    #[test]
    fn third() {
        let zero = Imaginary::new(0.0, 0.0);
        let raw = Imaginary::new(8.2, -5.0);
        let res = Imaginary::new(-63.632, -883.6);

        assert_eq!(zero.pow(3).unwrap(), zero);
        assert_eq!(raw.pow(3).unwrap(), res);
    }
}
