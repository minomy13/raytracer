use uuid::Uuid;

use crate::{
    ray::{intersection::Intersection, Ray},
    tuple::Tuple,
};

use super::Body;

#[derive(Clone, Copy)]
pub struct Sphere {
    id: Uuid,
}

impl Body for Sphere {
    fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    fn intersect(&self, ray: Ray) -> Option<[Intersection; 2]> {
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
    use crate::{body::Body, ray::Ray, tuple::Tuple};

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
}
