use std::{cmp, ops, usize};

use crate::{
    tuple::{Position, Tuple},
    utils::float_eq,
};

macro_rules! define_matrix {
    ($r:expr, $c:expr) => {
        Matrix<{$r - 1}, {$c - 1}>
    };
}

#[derive(Debug, Clone, Copy)]
struct Matrix<const R: usize, const C: usize>([[f64; C]; R]);

impl<const R: usize, const C: usize> ops::Index<usize> for Matrix<R, C> {
    type Output = [f64; C];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const R: usize, const C: usize> ops::IndexMut<usize> for Matrix<R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const R: usize, const C: usize, const P: usize> ops::Mul<Matrix<C, P>> for Matrix<R, C> {
    type Output = Matrix<R, P>;

    fn mul(self, rhs: Matrix<C, P>) -> Self::Output {
        self.multiply(rhs)
    }
}

impl<const R: usize, const C: usize, const P: usize> ops::Mul<&Matrix<C, P>> for &Matrix<R, C> {
    type Output = Matrix<R, P>;

    fn mul(self, rhs: &Matrix<C, P>) -> Self::Output {
        self.multiply(rhs)
    }
}

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

impl<const R: usize, const C: usize> cmp::PartialEq for Matrix<R, C> {
    fn eq(&self, other: &Self) -> bool {
        for r in 0..R {
            for c in 0..C {
                if !(float_eq(self[r][c], other[r][c])) {
                    return false;
                }
            }
        }
        true
    }
}

impl<const R: usize, const C: usize> AsRef<Matrix<R, C>> for Matrix<R, C> {
    fn as_ref(&self) -> &Matrix<R, C> {
        &self
    }
}

impl Matrix<2, 2> {
    fn determinant(&self) -> f64 {
        (self[0][0] * self[1][1]) - (self[0][1] * self[1][0])
    }
}

impl<const R: usize> Matrix<R, R> {
    pub fn identity_matrix() -> Matrix<R, R> {
        let mut matrix = Matrix::zero();

        for i in 0..R {
            for j in 0..R {
                if i == j {
                    matrix[i][j] = 1.0
                }
            }
        }

        matrix
    }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn zero() -> Self {
        Matrix([[0.0; C]; R])
    }

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

    fn transpose(&self) -> Matrix<C, R> {
        let mut result = Matrix::zero();

        for r in 0..R {
            for c in 0..C {
                result[c][r] = self[r][c]
            }
        }

        result
    }

    fn submatrix(&self, row: usize, column: usize) -> define_matrix!(R, C) {
        let result = Matrix::zero();



        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tuple::{Tuple, TupleKind},
        utils::float_eq,
    };

    use super::Matrix;

    #[test]
    fn matrix_2x2_representable() {
        // TODO: create variadic macro
        let m = Matrix::<2, 2>([[-3.0, 5.0], [1.0, -2.0]]);
        assert!(float_eq(m[0][0], -3.0));
        assert!(float_eq(m[0][1], 5.0));
        assert!(float_eq(m[1][0], 1.0));
        assert!(float_eq(m[1][1], -2.0))
    }

    #[test]
    fn matrix_3x3_representable() {
        // TODO: create variadic macro
        let m = Matrix::<3, 3>([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        assert!(float_eq(m[0][0], -3.0));
        assert!(float_eq(m[1][1], -2.0));
        assert!(float_eq(m[2][2], 1.0))
    }

    #[test]
    fn matrix_equality_with_identical_and_different_matrices() {
        let m = Matrix::<4, 4>([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let n = Matrix::<4, 4>([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let o = Matrix::<4, 4>([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 6.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert!(m == n);
        assert!(m != o)
    }

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
    fn multiply_by_identity_matrix() {
        let m = Matrix::<4, 4>([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        assert!(&m * &Matrix::identity_matrix() == m)
    }

    #[test]
    fn multiply_identity_matrix_by_tuple() {
        let a = Tuple::from([1.0, 2.0, 3.0, 4.0]);
        assert!(Matrix::identity_matrix() * a == a)
    }

    #[test]
    fn transposing_a_matrix() {
        let m = Matrix::<4, 4>([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let transposed = Matrix::<4, 4>([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert!(m.transpose() == transposed)
    }

    #[test]
    fn transposing_identity_matrix() {
        assert!(Matrix::<4, 4>::identity_matrix().transpose() == Matrix::identity_matrix())
    }

    #[test]
    fn determinant_2x2_matrix() {
        let m = Matrix([[1.0, 5.0], [-3.0, 2.0]]);
        assert!(m.determinant() == 17.0)
    }

    #[test]
    fn submatrix_of_3x3_is_2x2() {
        let m = Matrix([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let s = Matrix([[-3.0, 2.0], [0.0, 6.0]]);
        assert!()
    }
}
