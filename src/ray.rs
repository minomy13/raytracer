use crate::tuple::Tuple;

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
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

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
}
