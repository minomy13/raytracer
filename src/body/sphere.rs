use uuid::Uuid;

use crate::{
    material::Material,
    matrix::Matrix,
    ray::{intersection::Intersection, Ray},
    tuple::{Position, Tuple},
};

use super::Body;

#[derive(Clone, Copy)]
pub struct Sphere {
    id: Uuid,
    transformation: Matrix<4, 4>,
    material: Material,
}

impl Body for Sphere {
    fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transformation: Matrix::identity_matrix(),
            material: Material::default(),
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<[Intersection; 2]> {
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

    fn normal_at(&self, point: Tuple) -> Tuple {
        // convert to object space
        let point = self.transformation.inverse() * point;
        // left side converts back to world space
        let world_space_normal =
            self.transformation.inverse().transpose() * (point - Tuple::point_origin()).normalize();
        // WORKAROUND! page 82
        Tuple::from([
            world_space_normal[Position::X],
            world_space_normal[Position::Y],
            world_space_normal[Position::Z],
            0.0,
        ])
        .normalize()
    }

    fn get_material(&self) -> crate::material::Material {
        self.material
    }

    fn set_material(&self, material: Material) -> Self {
        Self { material, ..*self }
    }
}

impl Sphere {
    fn discriminant(&self, ray: &Ray) -> f64 {
        let sphere_to_ray = ray.get_origin() - Tuple::point_origin();
        let a = ray.get_direction().dot(ray.get_direction());
        let b = 2f64 * ray.get_direction().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1f64;

        b.powf(2f64) - 4f64 * a * c
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.discriminant(ray) >= 0f64
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        body::Body,
        material::Material,
        matrix::{transformation::Axis, Matrix},
        ray::Ray,
        tuple::Tuple,
    };

    use super::Sphere;

    // checking length in every test because of potential future switch to vec

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vec(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();
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
        let xs = s.intersect(&r).unwrap();
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
        let xs = s.intersect(&r);
        assert!(xs.is_none())
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Tuple::point_origin(), Tuple::new_vec(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();
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
        let xs = s.intersect(&r).unwrap();
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
        let xs = s.intersect(&r).unwrap();
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
        let xs = s.intersect(&r);
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
        let xs = s.intersect(&r);
        assert!(xs.is_none())
    }

    #[test]
    fn normal_on_sphere_on_point_on_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::new_point(1.0, 0.0, 0.0));
        assert_eq!(n, Tuple::new_vec(1.0, 0.0, 0.0))
    }

    #[test]
    fn normal_on_sphere_on_point_on_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::new_point(0.0, 1.0, 0.0));
        assert_eq!(n, Tuple::new_vec(0.0, 1.0, 0.0))
    }

    #[test]
    fn normal_on_sphere_on_point_on_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::new_point(0.0, 0.0, 1.0));
        assert_eq!(n, Tuple::new_vec(0.0, 0.0, 1.0))
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::new_point(
            3f64.sqrt() / 3.0,
            03f64.sqrt() / 3.0,
            3f64.sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            Tuple::new_vec(3f64.sqrt() / 3.0, 3f64.sqrt() / 3.0, 3f64.sqrt() / 3.0)
        )
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::new_point(1.0, 0.0, 0.0));
        assert_eq!(n, n.normalize())
    }

    #[test]
    fn computing_normal_of_translated_sphere() {
        let s = Sphere::new().transform(Matrix::translation_matrix(0.0, 1.0, 0.0));
        let n = s.normal_at(Tuple::new_point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Tuple::new_vec(0.0, 0.70711, -0.70711))
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let s = Sphere::new()
            .transform(Matrix::scaling_matrix(1.0, 0.5, 1.0).rotate(Axis::Z, PI / 5.0));
        let n = s.normal_at(Tuple::new_point(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0));
        assert_eq!(n, Tuple::new_vec(0.0, 0.97014, -0.24254))
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new();
        let m = s.get_material();
        assert_eq!(m, Material::default())
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let m = Material::default().set_ambient(1.0);
        let s = Sphere::new().set_material(m);
        assert_eq!(s.get_material(), m)
    }
}
