use super::Matrix;
use std::{ops, usize};

use crate::tuple::{Position, Tuple};

// multiplication
impl<const R: usize, const C: usize, const P: usize> ops::Mul<Matrix<C, P>> for Matrix<R, C> {
    type Output = Matrix<R, P>;

    fn mul(self, rhs: Matrix<C, P>) -> Self::Output {
        self.multiply(rhs)
    }
}

// multiplication (ref)
impl<const R: usize, const C: usize, const P: usize> ops::Mul<&Matrix<C, P>> for &Matrix<R, C> {
    type Output = Matrix<R, P>;

    fn mul(self, rhs: &Matrix<C, P>) -> Self::Output {
        self.multiply(rhs)
    }
}

// multiplication matrix by tuple
impl<const R: usize> ops::Mul<Tuple> for Matrix<R, 4> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let result = self.multiply(Matrix([
            [rhs[Position::X]],
            [rhs[Position::Y]],
            [rhs[Position::Z]],
            [rhs[Position::Kind]],
        ]));
        Tuple::from([result[0][0], result[1][0], result[2][0], result[3][0]])
    }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    fn multiply<const P: usize, T>(&self, rhs: T) -> Matrix<R, P>
    where
        T: AsRef<Matrix<C, P>>,
    {
        let mut matrix = Matrix::zero();

        for r in 0..R {
            for c in 0..P {
                let mut result = 0.0;
                for i in 0..C {
                    result += self[r][i] * rhs.as_ref()[i][c];
                }
                matrix[r][c] = result;
            }
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        matrix::Matrix,
        tuple::{Tuple, TupleKind},
    };

    #[test]
    fn multiplying_two_matrices() {
        let m = Matrix::<4, 4>([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let x = Matrix::<4, 4>([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let result = Matrix::<4, 4>([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        let r = m * x;
        assert!(r == result)
    }

    #[test]
    fn multiplying_two_non_square_matrices() {
        let m = Matrix::<2, 3>([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let x = Matrix::<3, 2>([[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]);
        let result = Matrix::<2, 2>([[58.0, 64.0], [139.0, 154.0]]);
        let r = m * x;
        assert!(r == result)
    }

    #[test]
    fn multiply_by_tuple() {
        let m = Matrix::<4, 4>([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Tuple::new(TupleKind::Point, 1.0, 2.0, 3.0);
        assert!((m * b).equals(Tuple::new(TupleKind::Point, 18.0, 24.0, 33.0)))
    }

    #[test]
    fn multiply_identity_matrix_by_tuple() {
        let a = Tuple::from([1.0, 2.0, 3.0, 4.0]);
        assert!(Matrix::identity_matrix() * a == a)
    }
}
