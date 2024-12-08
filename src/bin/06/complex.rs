use std::{cmp::Ordering, ops};

#[derive(Clone, Debug, PartialEq)]
pub struct Complex {
    pub im: i64,
    pub re: i64,
}

pub struct ComplexBuilder {
    real: Option<i64>,
    imag: Option<i64>,
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Complex {
    pub fn new(re: i64, im: i64) -> Complex {
        Complex { re, im }
    }

    pub fn builder() -> ComplexBuilder {
        ComplexBuilder::default()
    }

    pub fn direction(&self) -> Direction {
        match self {
            Complex{ re:1, im: 0} => Direction::North,
            Complex{ re:0, im: 1} => Direction::East,
            Complex{ re:-1, im: 0} => Direction::South,
            Complex{ re:0, im: -1} => Direction::West,
            _ => panic!("Only takes Complex numbers that be reduced to the four cardinal directions"),
        }
    }
}

impl ops::Add for Complex {
    type Output = Self;
    fn add(self, z: Complex) -> Complex {
        Complex {
            re: self.re + z.re,
            im: self.im + z.im,
        }
    }
}

impl ops::AddAssign for Complex {
    fn add_assign(&mut self, z: Complex) {
        *self = Complex {
            re: self.re + z.re,
            im: self.im + z.im,
        }
    }
}

impl ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, z: Complex) -> Complex {
        Complex {
            re: (self.re * z.re) - (self.im * z.im),
            im: (self.re * z.im) + (self.im * z.re),
        }
    }
}

impl ops::MulAssign for Complex {
    fn mul_assign(&mut self, z: Self) {
        *self = Self {
            re: (self.re * z.re) - (self.im * z.im),
            im: (self.re * z.im) + (self.im * z.re),
        };
    }
}

impl ComplexBuilder {
    fn new() -> Self {
        Self {
            real: None,
            imag: None,
        }
    }

    fn default() -> Self {
        Self::new()
    }

    pub fn real(self, real: i64) -> Self {
        Self {
            real: Some(real),
            ..self
        }
    }

    pub fn imaginary(self, imaginary: i64) -> Self {
        Self {
            imag: Some(imaginary),
            ..self
        }
    }

    pub fn build(self) -> Complex {
        Complex {
            re: self.real.map_or(0, |r| r),
            im: self.imag.map_or(0, |i| i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        assert_eq!(1, 2 - 1);
    }

    #[test]
    fn test_builder() {
        let complex_1_0 = Complex::builder().real(1).build();
        assert_eq!(complex_1_0, Complex::new(1, 0));
    }

    #[test]
    fn add_test() {
        let mut z_2_4 = Complex::new(2, 4);
        let z_1_m9 = Complex::new(1, -9);
        assert_eq!(Complex { re: 3, im: -5 }, z_2_4.clone() + z_1_m9.clone());
        z_2_4 += z_1_m9;
        assert_eq!(Complex { re: 3, im: -5 }, z_2_4);
    }

    #[test]
    fn mul_test() {
        let mut z_2_4 = Complex::new(2, 4);
        let z_1_m9 = Complex::new(1, -9);
        assert_eq!(Complex { re: 38, im: -14 }, z_2_4.clone() * z_1_m9.clone());
        z_2_4 *= z_1_m9;
        assert_eq!(Complex { re: 38, im: -14 }, z_2_4);
    }

    #[test]
    fn direction_change() {
        let mut dir = Complex::new(1, 0);
        assert_eq!(dir, Complex { re: 1, im: 0 });
        dir *= Complex::new(0, 1);
        assert_eq!(dir, Complex { re: 0, im: 1 });
        dir *= Complex::new(0, 1);
        assert_eq!(dir, Complex { re: -1, im: 0 });
        dir *= Complex::new(0, 1);
        assert_eq!(dir, Complex { re: 0, im: -1 });
    }
}
