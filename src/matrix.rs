use std::{cmp, ops, usize};

use crate::utils::float_eq;

mod multiplication;
pub mod transformation;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize>([[f64; C]; R]);

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

impl<const R: usize, const C: usize> AsRef<Matrix<R, C>> for Matrix<R, C> {
    fn as_ref(&self) -> &Matrix<R, C> {
        &self
    }
}

// scalar division
// TESTME
impl<const R: usize, const C: usize> ops::Div<f64> for Matrix<R, C> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let mut result = Matrix::zero();

        for r in 0..R {
            for c in 0..C {
                result[r][c] = self[r][c] / rhs;
            }
        }

        result
    }
}

// equality check
impl<const R: usize, const C: usize> cmp::PartialEq for Matrix<R, C> {
    fn eq(&self, other: &Self) -> bool {
        for r in 0..R {
            for c in 0..C {
                if !float_eq(self[r][c], other[r][c]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Matrix<2, 2> {
    fn determinant(&self) -> f64 {
        (self[0][0] * self[1][1]) - (self[0][1] * self[1][0])
    }
}

// TODO: use generic constant expressions asap, currently available with `unstable` feature enabled
impl Matrix<3, 3> {
    #[cfg(not(feature = "unstable"))]
    pub fn submatrix(&self, row: usize, column: usize) -> Matrix<2, 2> {
        let mut result = Matrix::zero();

        let mut rskip: usize = 0;
        for r in 0..3 {
            if r == row {
                rskip = 1;
                continue;
            }

            let mut cskip: usize = 0;
            for c in 0..3 {
                if c == column {
                    cskip = 1;
                    continue;
                }
                result[r - rskip][c - cskip] = self[r][c];
            }
        }

        result
    }

    fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    fn cofactor(&self, row: usize, column: usize) -> f64 {
        let result = self.minor(row, column);
        if (row + column) % 2 != 0 {
            return -result;
        } else {
            return result;
        }
    }

    fn determinant(&self) -> f64 {
        let row = self[0];
        let mut result = 0.0;
        for column in 0..row.len() {
            result += row[column] * self.cofactor(0, column)
        }
        result
    }

    pub fn is_invertible(&self) -> bool {
        // TODO: consider `float_eq()` function
        self.determinant() != 0.0
    }
}

// TODO: use generic constant expressions asap, currently available with `unstable` feature enabled
impl Matrix<4, 4> {
    #[cfg(not(feature = "unstable"))]
    pub fn submatrix(&self, row: usize, column: usize) -> Matrix<3, 3> {
        let mut result = Matrix::zero();

        let mut rskip: usize = 0;
        for r in 0..4 {
            if r == row {
                rskip = 1;
                continue;
            }

            let mut cskip: usize = 0;
            for c in 0..4 {
                if c == column {
                    cskip = 1;
                    continue;
                }
                result[r - rskip][c - cskip] = self[r][c];
            }
        }

        result
    }

    fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    fn cofactor(&self, row: usize, column: usize) -> f64 {
        let result = self.minor(row, column);
        if (row + column) % 2 != 0 {
            return -result;
        } else {
            return result;
        }
    }

    fn determinant(&self) -> f64 {
        let row = self[0];
        let mut result = 0.0;
        for column in 0..row.len() {
            result += row[column] * self.cofactor(0, column)
        }
        result
    }

    pub fn is_invertible(&self) -> bool {
        // TODO: consider `float_eq()` function
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Self {
        let mut result = Matrix::zero();

        // creating matrix of cofactors
        for r in 0..4 {
            for c in 0..4 {
                result[r][c] = self.cofactor(r, c);
            }
        }

        // TODO: consider combining operations
        result.transpose() / self.determinant()
    }
}

// implementations for square matrix
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

    pub fn transpose(&self) -> Matrix<C, R> {
        let mut result = Matrix::zero();

        for r in 0..R {
            for c in 0..C {
                result[c][r] = self[r][c]
            }
        }

        result
    }

    #[cfg(feature = "unstable")]
    fn submatrix(&self, row: usize, column: usize) -> Matrix<{ R - 1 }, { C - 1 }> {
        let mut result = Matrix::zero();

        let mut rskip: usize = 0;
        for r in 0..R {
            if r == row {
                rskip = 1;
                continue;
            }

            let mut cskip: usize = 0;
            for c in 0..C {
                if c == column {
                    cskip = 1;
                    continue;
                }
                result[r - rskip][c - cskip] = self[r][c];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{assert_f64_eq, float_eq};

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
        assert!(m.submatrix(0, 2) == s)
    }

    #[test]
    fn submatrix_of_4x4_is_3x3() {
        let m = Matrix([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let s = Matrix([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);
        assert!(m.submatrix(2, 1) == s)
    }

    #[test]
    fn minor_of_3x3_matrix() {
        let m = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        // TODO: use `assert_eq!` if possible
        assert!(m.minor(1, 0) == 25.0)
    }

    #[test]
    fn cofactor_of_3x3_matrix() {
        let m = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0)
    }

    #[test]
    fn calculate_determinant_of_3x3_matrix() {
        let m = Matrix([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0)
    }

    #[test]
    fn calculate_determinant_of_4x4_matrix() {
        let m = Matrix([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0)
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn can_create_submatrix_from_10x10() {
        let m = Matrix::<10, 10>::zero().submatrix(1, 1);
        assert_eq!(m, Matrix::<9, 9>::zero())
    }

    #[test]
    fn test_invertible_for_invertability() {
        let m = Matrix([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(m.determinant(), -2120.0);
        assert!(m.is_invertible())
    }

    #[test]
    fn test_non_invertible_for_invertability() {
        let m = Matrix([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(m.determinant(), 0.0);
        assert!(!m.is_invertible())
    }

    #[test]
    fn calculate_inverse_of_matrix() {
        let m = Matrix([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let b_result = Matrix([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        let b = m.inverse();

        assert_f64_eq!(m.determinant(), 532.0);
        assert_f64_eq!(m.cofactor(2, 3), -160.0);
        assert_f64_eq!(b[3][2], -160.0 / 532.0);
        assert_f64_eq!(m.cofactor(3, 2), 105.0);
        assert_f64_eq!(b[2][3], 105.0 / 532.0);
        assert_eq!(b, b_result)
    }

    #[test]
    fn calulate_inverse_of_matrix_2() {
        let m = Matrix([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let b = Matrix([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert_eq!(m.inverse(), b)
    }

    #[test]
    fn calulate_inverse_of_matrix_3() {
        let m = Matrix([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let b = Matrix([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        assert_eq!(m.inverse(), b)
    }

    #[test]
    fn multiplying_product_by_inverse() {
        let m = Matrix([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let c = m * b;
        assert_eq!(c * b.inverse(), m)
    }
}
