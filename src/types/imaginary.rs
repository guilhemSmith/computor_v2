/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   imaginary.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/07/09 10:46:59 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/13 12:54:02 by gsmith           ###   ########.fr       */
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

    pub fn zero() -> Self {
        Imaginary {
            real: Rational::zero(),
            irreal: Rational::zero(),
        }
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
                                if self.irreal != Rational::new(1.0)
                                {
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
                    one if one == Rational::new(1.0) => {
                        String::from("i")
                    }
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
        let zero = Imaginary::zero();
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
        let zero = Imaginary::zero();
        let real_1 = Imaginary::new(42.0, 0.0);
        let real_2 = Imaginary::new(-13.00001456, 0.0);
        let irreal_1 = Imaginary::new(0.0, 81.0987);
        let irreal_2 = Imaginary::new(0.0, 50.0);
        let complex_1 = Imaginary::new(42.0, 50.0);
        let complex_2 = Imaginary::new(-42.0, 50.0);

        assert_eq!(real_1 - zero, real_1);
        assert_eq!(
            real_1 - real_2,
            Imaginary::new(42.0 + 13.00001456, 0.0)
        );
        assert_eq!(irreal_1 - zero, irreal_1);
        assert_eq!(complex_1 - zero, complex_1);
        assert_eq!(irreal_2 - real_1, complex_2);
    }

    #[test]
    fn mul_imaginary() {
        let zero = Imaginary::zero();
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
        let zero = Imaginary::zero();
        let complex_2 = Imaginary::new(2.0, 3.0);
        let complex_3 = Imaginary::new(3.0, 2.0);
        let complex_4 = Imaginary::new(12.0, 5.0) / Imaginary::new(13.0, 0.0);

        assert_eq!(zero / complex_2, zero);
        assert_eq!(complex_2 / complex_3, complex_4);
    }

    // #[test]
    // fn cmp_0() {}

    // #[test]
    // fn cmp_1() {}

    // #[test]
    // fn cmp_2() {}

    // #[test]
    // fn cmp_3() {}

    // #[test]
    // fn cmp_4() {}
}
