/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   imaginary.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:46:59 by gsmith            #+#    #+#             */
/*   Updated: 2019/09/16 13:58:17 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::rational::Rational;
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

    pub fn pow(&self, power: i32) -> Imaginary {
        if power == 0 {
            return Imaginary::new(1.0, 0.0);
        }
        if power < 0 {
            return (Imaginary::new(1.0, 0.0) / *self).pow(-power);
        }

        let mut real = Rational::zero();
        let mut irreal = Rational::zero();

        let pow: u32 = power as u32;
        let mut i: u32 = 0;

        while pow >= i {
            let sign = Rational::new(if i % 4 < 2 { 1.0 } else { -1.0 });
            let new_val = self.real.pow((pow - i) as i32)
                * self.irreal.pow(i as i32)
                * sign;
            let new_coef = Rational::new(pascal_num(pow, i) as f64);
            if i % 2 == 0 {
                real = real + new_coef * new_val;
            } else {
                irreal = irreal + new_coef * new_val;
            }
            i += 1;
        }
        real.simplify();
        irreal.simplify();
        return Imaginary { real, irreal };
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
}

fn pascal_num(p: u32, n: u32) -> u32 {
    if p == 0 || n == 0 || n == p {
        return 1;
    }
    pascal_num(p - 1, n) + pascal_num(p - 1, n - 1)
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

impl ops::Add<Imaginary> for Imaginary {
    type Output = Imaginary;

    fn add(self, rhs: Imaginary) -> Imaginary {
        Imaginary {
            real: self.real + rhs.real,
            irreal: self.irreal + rhs.irreal,
        }
    }
}

impl ops::Sub<Imaginary> for Imaginary {
    type Output = Imaginary;

    fn sub(self, rhs: Imaginary) -> Imaginary {
        Imaginary {
            real: self.real - rhs.real,
            irreal: self.irreal - rhs.irreal,
        }
    }
}

impl ops::Mul<Imaginary> for Imaginary {
    type Output = Imaginary;

    fn mul(self, rhs: Imaginary) -> Imaginary {
        Imaginary {
            real: self.real * rhs.real - self.irreal * rhs.irreal,
            irreal: self.real * rhs.irreal + self.irreal * rhs.real,
        }
    }
}

impl ops::Div<Imaginary> for Imaginary {
    type Output = Imaginary;

    fn div(self, rhs: Imaginary) -> Imaginary {
        let den = rhs.real.pow(2) + rhs.irreal.pow(2);
        Imaginary {
            real: (self.real * rhs.real + self.irreal * rhs.irreal) / den,
            irreal: (self.irreal * rhs.real - self.real * rhs.irreal) / den,
        }
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

        assert_eq!(real_1 + zero, real_1);
        assert_eq!(zero + real_2, real_2);
        assert_eq!(irreal_1 + zero, irreal_1);
        assert_eq!(zero + irreal_2, irreal_2);
        assert_eq!(complex_1 + zero, complex_1);
        assert_eq!(zero + complex_2, complex_2);
        assert_eq!(real_1 + irreal_2, complex_1);
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

        assert_eq!(real_1 - zero, real_1);
        assert_eq!(real_1 - real_2, Imaginary::new(42.0 + 13.00001456, 0.0));
        assert_eq!(irreal_1 - zero, irreal_1);
        assert_eq!(complex_1 - zero, complex_1);
        assert_eq!(irreal_2 - real_1, complex_2);
    }

    #[test]
    fn mul_imaginary() {
        let zero = Imaginary::new(0.0, 0.0);
        let complex_1 = Imaginary::new(42.0, 50.0);
        let complex_2 = Imaginary::new(2.0, 3.0);
        let complex_3 = Imaginary::new(3.0, 2.0);
        let complex_4 = Imaginary::new(0.0, 13.0);

        assert_eq!(complex_1 * zero, zero);
        assert_eq!(zero * complex_2, zero);
        assert_eq!(complex_2 * complex_3, complex_4);
    }

    #[test]
    fn div_imaginary() {
        let zero = Imaginary::new(0.0, 0.0);
        let complex_2 = Imaginary::new(2.0, 3.0);
        let complex_3 = Imaginary::new(3.0, 2.0);
        let complex_4 = Imaginary::new(12.0, 5.0) / Imaginary::new(13.0, 0.0);

        assert_eq!(zero / complex_2, zero);
        assert_eq!(complex_2 / complex_3, complex_4);
    }
}

#[cfg(test)]
mod pow {
    use super::pascal_num;
    use super::Imaginary;

    #[test]
    fn square() {
        let zero = Imaginary::new(0.0, 0.0);
        let raw = Imaginary::new(4.0, -6.2);
        let res = Imaginary::new(-22.44, -49.6);

        assert_eq!(zero.pow(2), zero);
        assert_eq!(raw.pow(2), res);
    }

    #[test]
    fn third() {
        let zero = Imaginary::new(0.0, 0.0);
        let raw = Imaginary::new(8.2, -5.0);
        let res = Imaginary::new(-63.632, -883.6);

        assert_eq!(zero.pow(3), zero);
        assert_eq!(raw.pow(3), res);
    }

    #[test]
    fn pascal() {
        assert_eq!(pascal_num(0, 0), 1);
        assert_eq!(pascal_num(1, 0), 1);
        assert_eq!(pascal_num(1, 1), 1);
        assert_eq!(pascal_num(2, 0), 1);
        assert_eq!(pascal_num(2, 1), 2);
        assert_eq!(pascal_num(2, 2), 1);
        assert_eq!(pascal_num(3, 0), 1);
        assert_eq!(pascal_num(3, 1), 3);
        assert_eq!(pascal_num(3, 2), 3);
        assert_eq!(pascal_num(3, 3), 1);
        assert_eq!(pascal_num(4, 0), 1);
        assert_eq!(pascal_num(4, 1), 4);
        assert_eq!(pascal_num(4, 2), 6);
        assert_eq!(pascal_num(4, 3), 4);
        assert_eq!(pascal_num(4, 4), 1);
    }
}
