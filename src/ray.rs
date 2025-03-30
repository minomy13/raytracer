use crate::{matrix::Matrix, tuple::Tuple};

pub mod intersection;

pub struct Ray(Tuple, Tuple);

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Ray(origin, direction)
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.0 + self.1 * t
    }

    pub fn get_origin(&self) -> Tuple {
        self.0
    }

    pub fn get_direction(&self) -> Tuple {
        self.1
    }

    pub fn transform(&self, by: Matrix<4, 4>) -> Self {
        Self(by * self.0, by * self.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, tuple::Tuple};

    use super::Ray;

    #[test]
    fn creating_and_querying_ray() {
        let origin = Tuple::new_point(1.0, 2.0, 3.0);
        let direction = Tuple::new_vec(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.get_origin(), origin);
        assert_eq!(ray.get_direction(), direction)
    }

    #[test]
    fn computing_point_from_distance() {
        let r = Ray::new(
            Tuple::new_point(2.0, 3.0, 4.0),
            Tuple::new_vec(1.0, 0.0, 0.0),
        );
        assert_eq!(r.position(0.0), Tuple::new_point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple::new_point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::new_point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::new_point(4.5, 3.0, 4.0))
    }

    #[test]
    fn translating_ray() {
        let r = Ray::new(
            Tuple::new_point(1.0, 2.0, 3.0),
            Tuple::new_vec(0.0, 1.0, 0.0),
        );
        let m = Matrix::translation_matrix(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
        assert_eq!(r2.get_origin(), Tuple::new_point(4.0, 6.0, 8.0));
        assert_eq!(r2.get_direction(), Tuple::new_vec(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_ray() {
        let r = Ray::new(
            Tuple::new_point(1.0, 2.0, 3.0),
            Tuple::new_vec(0.0, 1.0, 0.0),
        );
        let m = Matrix::scaling_matrix(2.0, 3.0, 4.0);
        let r2 = r.transform(m);
        assert_eq!(r2.get_origin(), Tuple::new_point(2.0, 6.0, 12.0));
        assert_eq!(r2.get_direction(), Tuple::new_vec(0.0, 3.0, 0.0))
    }
}
