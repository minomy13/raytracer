use crate::utils;
use std::{cmp, ops};

pub enum TupleKind {
    Point,
    Vector,
}

pub enum Position {
    X,
    Y,
    Z,
    Kind,
}

#[derive(Clone, Copy, Debug)]
pub struct Tuple(f64, f64, f64, f64);

impl ops::Add for Tuple {
    type Output = Tuple;
    fn add(self, rhs: Self) -> Self::Output {
        Tuple(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;
    fn sub(self, rhs: Self) -> Self::Output {
        Tuple(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: f64) -> Self::Output {
        Tuple(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, rhs: f64) -> Self::Output {
        Tuple(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Self::Output {
        Tuple(-self.0, -self.1, -self.2, -self.3)
    }
}

impl ops::Index<Position> for Tuple {
    type Output = f64;
    fn index(&self, index: Position) -> &Self::Output {
        match index {
            Position::X => {
                return &self.0;
            }
            Position::Y => {
                return &self.1;
            }
            Position::Z => {
                return &self.2;
            }
            Position::Kind => {
                return &self.3;
            }
        }
    }
}

impl cmp::PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.equals(*other)
    }
}

impl Tuple {
    pub fn new(kind: TupleKind, x: f64, y: f64, z: f64) -> Self {
        let kind: f64 = match kind {
            TupleKind::Point => 1.0,
            TupleKind::Vector => 0.0,
        };
        Self(x, y, z, kind)
    }

    // TESTME
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self::new(TupleKind::Point, x, y, z)
    }

    // TESTME
    pub fn new_vec(x: f64, y: f64, z: f64) -> Self {
        Self::new(TupleKind::Vector, x, y, z)
    }

    // TESTME
    pub fn from(arr: [f64; 4]) -> Tuple {
        Tuple(arr[0], arr[1], arr[2], arr[3])
    }

    // TODO: use operator overloading
    pub fn equals(&self, tuple: Tuple) -> bool {
        utils::float_eq(self.0, tuple.0)
            && utils::float_eq(self.1, tuple.1)
            && utils::float_eq(self.2, tuple.2)
            && utils::float_eq(self.3, tuple.3)
    }

    pub fn magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2) + self.3.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        Tuple(
            self.0 / self.magnitude(),
            self.1 / self.magnitude(),
            self.2 / self.magnitude(),
            self.3 / self.magnitude(),
        )
    }

    pub fn dot(&self, tuple: Tuple) -> f64 {
        if self.3 != 0.0 || tuple.3 != 0.0 {
            panic!("Dot product can only be calculated from vectors.")
        }
        self.0 * tuple.0 + self.1 * tuple.1 + self.2 * tuple.2
    }

    pub fn cross(&self, tuple: Tuple) -> Tuple {
        if self.3 != 0.0 || tuple.3 != 0.0 {
            panic!("Cross product can only be calculated from vectors.")
        }
        Tuple(
            self.1 * tuple.2 - self.2 * tuple.1,
            self.2 * tuple.0 - self.0 * tuple.2,
            self.0 * tuple.1 - self.1 * tuple.0,
            0.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_creates_tuples_with_w1() {
        let tuple: Tuple = Tuple::new(TupleKind::Point, 4.2, -3.8, 23.7);
        assert_eq!(tuple.3, 1.0)
    }

    #[test]
    fn point_creates_tuples_with_w0() {
        let tuple: Tuple = Tuple::new(TupleKind::Vector, 4.2, -3.8, 23.7);
        assert_eq!(tuple.3, 0.0)
    }

    #[test]
    fn tuple_equals_works() {
        let tuple_a: Tuple = Tuple(4.2, 3.2, 5.2, 1.0);
        let tuple_b: Tuple = tuple_a.clone();
        let tuple_c: Tuple = Tuple(6.3, 6.3, 7.4, 0.0);

        assert!(tuple_a.equals(tuple_b));
        assert!(!tuple_b.equals(tuple_c));
    }

    #[test]
    fn adding_tuples_works() {
        let tuple_a: Tuple = Tuple(3.0, -2.0, 5.0, 1.0);
        let tuple_b: Tuple = Tuple(-2.0, 3.0, 1.0, 0.0);
        assert!((tuple_a + tuple_b).equals(Tuple(1.0, 1.0, 6.0, 1.0)));
    }

    #[test]
    fn subtracting_two_points() {
        let tuple_a: Tuple = Tuple::new(TupleKind::Point, 3.0, 2.0, 1.0);
        let tuple_b: Tuple = Tuple::new(TupleKind::Point, 5.0, 6.0, 7.0);
        assert!((tuple_a - tuple_b).equals(Tuple::new(TupleKind::Vector, -2.0, -4.0, -6.0)));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let tuple_a: Tuple = Tuple::new(TupleKind::Point, 3.0, 2.0, 1.0);
        let tuple_b: Tuple = Tuple::new(TupleKind::Vector, 5.0, 6.0, 7.0);
        assert!((tuple_a - tuple_b).equals(Tuple::new(TupleKind::Point, -2.0, -4.0, -6.0)));
    }

    #[test]
    fn subtracting_two_vectors() {
        let tuple_a: Tuple = Tuple::new(TupleKind::Vector, 3.0, 2.0, 1.0);
        let tuple_b: Tuple = Tuple::new(TupleKind::Vector, 5.0, 6.0, 7.0);
        assert!((tuple_a - tuple_b).equals(Tuple::new(TupleKind::Vector, -2.0, -4.0, -6.0)));
    }

    #[test]
    fn negating_tuple() {
        assert!(
            (-Tuple::new(TupleKind::Vector, -1.0, 45.0, 32.0)).equals(Tuple::new(
                TupleKind::Vector,
                1.0,
                -45.0,
                -32.0
            ))
        );
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        assert!((Tuple(1.0, -2.0, 3.0, -4.0) * 3.5).equals(Tuple(3.5, -7.0, 10.5, -14.0)));
    }

    #[test]
    fn multiplying_tuple_by_fracture() {
        assert!((Tuple(1.0, -2.0, 3.0, -4.0) * 0.5).equals(Tuple(0.5, -1.0, 1.5, -2.0)));
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        assert!((Tuple(1.0, -2.0, 3.0, -4.0) / 2.0).equals(Tuple(0.5, -1.0, 1.5, -2.0)));
    }

    #[test]
    fn computing_magnitude_of_vector() {
        assert_eq!(
            Tuple::new(TupleKind::Vector, 1.0, 0.0, 0.0).magnitude(),
            1.0
        );
        assert_eq!(
            Tuple::new(TupleKind::Vector, 0.0, 1.0, 0.0).magnitude(),
            1.0
        );
        assert_eq!(
            Tuple::new(TupleKind::Vector, 0.0, 0.0, 1.0).magnitude(),
            1.0
        );
        assert_eq!(
            Tuple::new(TupleKind::Vector, 1.0, 2.0, 3.0).magnitude(),
            14.0_f64.sqrt()
        );
        assert_eq!(
            Tuple::new(TupleKind::Vector, -1.0, -2.0, -3.0).magnitude(),
            14.0_f64.sqrt()
        );
    }

    #[test]
    fn normalizing_vectors() {
        assert!(Tuple::new(TupleKind::Vector, 4.0, 0.0, 0.0)
            .normalize()
            .equals(Tuple::new(TupleKind::Vector, 1.0, 0.0, 0.0)));
        assert!(Tuple::new(TupleKind::Vector, 1.0, 2.0, 3.0)
            .normalize()
            .equals(Tuple::new(
                TupleKind::Vector,
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )));
    }

    #[test]
    fn magnitude_of_normalized_is_1() {
        assert_eq!(
            Tuple::new(TupleKind::Vector, 1.0, 2.0, 3.0)
                .normalize()
                .magnitude(),
            1.0
        );
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::new(TupleKind::Vector, 1.0, 2.0, 3.0);
        let b = Tuple::new(TupleKind::Vector, 2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::new(TupleKind::Vector, 1.0, 2.0, 3.0);
        let b = Tuple::new(TupleKind::Vector, 2.0, 3.0, 4.0);
        assert!(a
            .cross(b)
            .equals(Tuple::new(TupleKind::Vector, -1.0, 2.0, -1.0)));
        assert!(b
            .cross(a)
            .equals(Tuple::new(TupleKind::Vector, 1.0, -2.0, 1.0)));
    }
}
