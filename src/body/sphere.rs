use uuid::Uuid;

use crate::{
    matrix::Matrix,
    ray::{intersection::Intersection, Ray},
    tuple::Tuple,
};

use super::Body;

#[derive(Clone, Copy)]
pub struct Sphere {
    id: Uuid,
    transformation: Matrix<4, 4>,
}

impl Body for Sphere {
    fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transformation: Matrix::identity_matrix(),
        }
    }

    fn intersect(&self, ray: Ray) -> Option<[Intersection; 2]> {
        let ray = ray.transform(self.transformation.inverse());

        if !self.intersects(&ray) {
            return None;
        }

        // TODO: prevent duplication with `discriminant()`
        let sphere_to_ray = ray.get_origin() - Tuple::point_origin();
        let a = ray.get_direction().dot(ray.get_direction());
        let b = 2f64 * ray.get_direction().dot(sphere_to_ray);

        Some([
            Intersection::new((-b - self.discriminant(&ray).sqrt()) / (2f64 * a), self.id),
            Intersection::new((-b + self.discriminant(&ray).sqrt()) / (2f64 * a), self.id),
        ])
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_transformation(&self) -> crate::matrix::Matrix<4, 4> {
        self.transformation
    }

    fn transform(&self, by: crate::matrix::Matrix<4, 4>) -> Self {
        Self {
            transformation: self.transformation * by,
            ..*self
        }
    }
}

impl Sphere {
    fn discriminant(&self, ray: &Ray) -> f64 {
        let sphere_to_ray = ray.get_origin() - Tuple::point_origin(); // 0 0 -5
        let a = ray.get_direction().dot(ray.get_direction()); // 1
        let b = 2f64 * ray.get_direction().dot(sphere_to_ray); // -10
        let c = sphere_to_ray.dot(sphere_to_ray) - 1f64; // 23

        b.powf(2f64) - 4f64 * a * c // 100 - 4 * 1 * 23 = 8
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.discriminant(ray) >= 0f64
    }
}

#[cfg(test)]
mod tests {
    use crate::{body::Body, matrix::Matrix, ray::Ray, tuple::Tuple};

    use super::Sphere;

    // checking length in every test because of potential future switch to vec

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vec(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_t(), 4.0);
        assert_eq!(xs[1].get_t(), 6.0)
    }

    #[test]
    fn ray_intersects_sphere_as_tangent() {
        let r = Ray::new(
            Tuple::new_point(0.0, 1.0, -5.0),
            Tuple::new_vec(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_t(), 5.0);
        assert_eq!(xs[1].get_t(), 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 2.0, -5.0),
            Tuple::new_vec(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert!(xs.is_none())
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Tuple::point_origin(), Tuple::new_vec(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_t(), -1.0);
        assert_eq!(xs[1].get_t(), 1.0)
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vec(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_t(), -6.0);
        assert_eq!(xs[1].get_t(), -4.0)
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vec(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_object_id(), s.id);
        assert_eq!(xs[1].get_object_id(), s.id)
    }

    #[test]
    fn spheres_default_transformation() {
        let s = Sphere::new();
        assert_eq!(s.get_transformation(), Matrix::identity_matrix())
    }

    #[test]
    fn changing_tests_transformation() {
        let t = Matrix::translation_matrix(2.0, 3.0, 4.0);
        let s = Sphere::new().transform(t);
        assert_eq!(s.get_transformation(), t)
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vec(0.0, 0.0, 1.0),
        );
        let s = Sphere::new().transform(Matrix::scaling_matrix(2.0, 2.0, 2.0));
        let xs = s.intersect(r);
        assert_eq!(xs.unwrap().len(), 2);
        assert_eq!(xs.unwrap()[0].get_t(), 3.0);
        assert_eq!(xs.unwrap()[1].get_t(), 7.0)
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vec(0.0, 0.0, 1.0),
        );
        let s = Sphere::new().transform(Matrix::translation_matrix(5.0, 0.0, 0.0));
        let xs = s.intersect(r);
        assert!(xs.is_none())
    }
}
