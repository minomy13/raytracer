use crate::tuple::{Position, Tuple};

use super::Matrix;

pub enum Axis {
    X,
    Y,
    Z,
}

impl Matrix<4, 4> {
    // TODO: consider switching to variadic macro
    pub fn translation_matrix(x: f64, y: f64, z: f64) -> Self {
        let mut result = Matrix::identity_matrix();
        result[0][3] = x;
        result[1][3] = y;
        result[2][3] = z;
        result
    }

    pub fn scaling_matrix(x: f64, y: f64, z: f64) -> Self {
        let mut result = Matrix::identity_matrix();
        result[0][0] = x;
        result[1][1] = y;
        result[2][2] = z;
        result
    }

    pub fn rotation_matrix(axis: Axis, rad: f64) -> Self {
        let mut result = Matrix::identity_matrix();

        match axis {
            Axis::X => {
                result[1][1] = rad.cos();
                result[1][2] = -rad.sin();
                result[2][1] = rad.sin();
                result[2][2] = rad.cos();
            }
            Axis::Y => {
                result[0][0] = rad.cos();
                result[0][2] = rad.sin();
                result[2][0] = -rad.sin();
                result[2][2] = rad.cos();
            }
            Axis::Z => {
                result[0][0] = rad.cos();
                result[0][1] = -rad.sin();
                result[1][0] = rad.sin();
                result[1][1] = rad.cos();
            }
        }

        result
    }

    pub fn shearing_matrix(xpy: f64, xpz: f64, ypx: f64, ypz: f64, zpx: f64, zpy: f64) -> Self {
        let mut result = Matrix::identity_matrix();

        result[0][1] = xpy;
        result[0][2] = xpz;
        result[1][0] = ypx;
        result[1][2] = ypz;
        result[2][0] = zpx;
        result[2][1] = zpy;

        result
    }

    pub fn view_transform_matrix(from: Tuple, to: Tuple, up: Tuple) -> Self {
        let forward = (to - from).normalize();
        let left = forward.cross(up.normalize());
        let true_up = left.cross(forward);

        let mut orientation = Matrix::identity_matrix();
        orientation[0][0] = left[Position::X];
        orientation[0][1] = left[Position::Y];
        orientation[0][2] = left[Position::Z];
        orientation[1][0] = true_up[Position::X];
        orientation[1][1] = true_up[Position::Y];
        orientation[1][2] = true_up[Position::Z];
        orientation[2][0] = -forward[Position::X];
        orientation[2][1] = -forward[Position::Y];
        orientation[2][2] = -forward[Position::Z];

        orientation
            * Matrix::translation_matrix(-from[Position::X], -from[Position::Y], -from[Position::Z])
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Self {
        // TODO: check style, consider deref
        self * &Self::translation_matrix(x, y, z)
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Self {
        // TODO: check style, consider deref
        self * &Self::scaling_matrix(x, y, z)
    }

    pub fn rotate(&self, axis: Axis, rad: f64) -> Self {
        self * &Self::rotation_matrix(axis, rad)
    }

    pub fn shear(&self, xpy: f64, xpz: f64, ypx: f64, ypz: f64, zpx: f64, zpy: f64) -> Self {
        self * &Self::shearing_matrix(xpy, xpz, ypx, ypz, zpx, zpy)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{vector, point, 
        matrix::Matrix,
        tuple::Tuple,
    };

    use super::Axis;

    // TRASLATE
    #[test]
    fn multiply_by_translation_matrix() {
        let transform = super::Matrix::translation_matrix(5.0, -3.0, 2.0);
        let p = Tuple::new_point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, Tuple::new_point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiply_by_inverse_of_translation_matrix() {
        let transform = super::Matrix::translation_matrix(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Tuple::new_point(-3.0, 4.0, 5.0);
        assert_eq!(inv * p, Tuple::new_point(-8.0, 7.0, 3.0))
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = super::Matrix::translation_matrix(5.0, -3.0, 2.0);
        let v = Tuple::new_vec(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v)
    }

    // SCALE
    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = super::Matrix::scaling_matrix(2.0, 3.0, 4.0);
        let p = Tuple::new_point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Tuple::new_point(-8.0, 18.0, 32.0))
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = super::Matrix::scaling_matrix(2.0, 3.0, 4.0);
        let v = Tuple::new_vec(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, Tuple::new_vec(-8.0, 18.0, 32.0))
    }

    #[test]
    fn multiply_by_inverse_of_scaling_matrix() {
        let transform = super::Matrix::scaling_matrix(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Tuple::new_vec(-4.0, 6.0, 8.0);
        assert_eq!(inv * v, Tuple::new_vec(-2.0, 2.0, 2.0))
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = super::Matrix::scaling_matrix(-1.0, 1.0, 1.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::new_point(-2.0, 3.0, 4.0))
    }

    // ROTATE
    #[test]
    fn rotating_point_around_x_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_matrix(Axis::X, PI / 4.0);
        let full_quarter = Matrix::rotation_matrix(Axis::X, PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Tuple::new_point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::new_point(0.0, 0.0, 1.0))
    }

    #[test]
    fn rotating_by_inverse_rotation_matrix_rotates_opposite_direction() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_matrix(Axis::X, PI / 4.0);
        let inv = half_quarter.inverse();
        assert_eq!(
            inv * p,
            Tuple::new_point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0)
        )
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = Tuple::new_point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_matrix(Axis::Y, PI / 4.0);
        let full_quarter = Matrix::rotation_matrix(Axis::Y, PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Tuple::new_point(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::new_point(1.0, 0.0, 0.0))
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_matrix(Axis::Z, PI / 4.0);
        let full_quarter = Matrix::rotation_matrix(Axis::Z, PI / 2.0);
        assert_eq!(
            half_quarter * p,
            Tuple::new_point(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Tuple::new_point(-1.0, 0.0, 0.0))
    }

    #[test]
    fn shearing_moves_coordinates_in_proportion_to_others() {
        let p = Tuple::new_point(2.0, 3.0, 4.0);

        let transform_xpy = Matrix::shearing_matrix(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(transform_xpy * p, Tuple::new_point(5.0, 3.0, 4.0));

        let transform_xpy = Matrix::shearing_matrix(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(transform_xpy * p, Tuple::new_point(6.0, 3.0, 4.0));

        let transform_xpy = Matrix::shearing_matrix(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(transform_xpy * p, Tuple::new_point(2.0, 5.0, 4.0));

        let transform_xpy = Matrix::shearing_matrix(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(transform_xpy * p, Tuple::new_point(2.0, 7.0, 4.0));

        let transform_xpy = Matrix::shearing_matrix(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(transform_xpy * p, Tuple::new_point(2.0, 3.0, 6.0));

        let transform_xpy = Matrix::shearing_matrix(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(transform_xpy * p, Tuple::new_point(2.0, 3.0, 7.0));
    }

    #[test]
    fn transformation_matrix_for_default_orientation() {
        let from = Tuple::point_origin();
        let to = point!(0, 0, -1);
        let up = vector!(0, 1, 0);
        let t = Matrix::view_transform_matrix(from, to, up);
        assert_eq!(t, Matrix::identity_matrix())
    }

    #[test]
    fn view_transformation_matrix_looking_positive_z_direction() {
        let from = Tuple::point_origin();
        let to = point!(0, 0, 1);
        let up = vector!(0, 1, 0);
        let t = Matrix::view_transform_matrix(from, to, up);
        assert_eq!(t, Matrix::scaling_matrix(-1.0, 1.0, -1.0))
    }

    #[test]
    fn view_transformation_moves_world() {
        let from = point!(0, 0, 8);
        let to = Tuple::point_origin();
        let up = vector!(0, 1, 0);
        let t = Matrix::view_transform_matrix(from, to, up);
        assert_eq!(t, Matrix::translation_matrix(0.0, 0.0, -8.0))
    }

    #[test]
    fn arbitary_view_transformation() {
        let from = point!(1, 3, 2);
        let to = point!(4, -2, 8);
        let up = vector!(1, 1, 0);
        let t = Matrix::view_transform_matrix(from, to, up);
        let result = Matrix::<4, 4>([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000],
        ]);
        assert_eq!(t, result)
    }
}
