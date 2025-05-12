use crate::body::Body;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::intersection::Intersection;
use crate::ray::Ray;
use crate::tuple::{Position, Tuple};
use crate::utils::EPSILON;
use crate::vector;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Plane {
    id: Uuid,
    transformation: Matrix<4, 4>,
    material: Material,
}

impl Body for Plane {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            id: Uuid::new_v4(),
            transformation: Matrix::identity_matrix(),
            material: Material::default(),
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<[Intersection; 2]> {
        if ray.get_direction()[Position::Y].abs() < EPSILON {
            return None;
        }

        let t = -ray.get_origin()[Position::Y] / ray.get_direction()[Position::Y];
        Some([Intersection::new(t, self); 2]) // TODO: change to array slice
    }

    fn transform(&self, by: Matrix<4, 4>) -> Self
    where
        Self: Sized,
    {
        Self {
            transformation: self.transformation * by,
            ..*self
        }
    }

    fn normal_at(&self, point: Tuple) -> Tuple {
        vector!(0, 1, 0)
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_transformation(&self) -> Matrix<4, 4> {
        self.transformation
    }

    fn get_material(&self) -> Material {
        self.material
    }

    fn set_material(&self, material: Material) -> Self
    where
        Self: Sized,
    {
        Self { material, ..*self }
    }
}

#[cfg(test)]
mod tests {
    use crate::body::plane::Plane;
    use crate::body::Body;
    use crate::ray::Ray;
    use crate::tuple::Tuple;
    use crate::{point, vector};

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let plane = Plane::new();
        let normal1 = plane.normal_at(point!(0, 0, 0));
        let normal2 = plane.normal_at(point!(10, 0, -10));
        let normal3 = plane.normal_at(point!(-5, 0, 150));
        assert_eq!(normal1, vector!(0, 1, 0));
        assert_eq!(normal2, vector!(0, 1, 0));
        assert_eq!(normal3, vector!(0, 1, 0));
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let plane = Plane::new();
        let ray = Ray::new(point!(0, 10, 0), vector!(0, 0, 1));
        let xs = plane.intersect(&ray);
        assert!(xs.is_none());
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let plane = Plane::new();
        let ray = Ray::new(point!(0, 0, 0), vector!(0, 0, 1));
        let xs = plane.intersect(&ray);
        assert!(xs.is_none());
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let plane = Plane::new();
        let ray = Ray::new(point!(0, 1, 0), vector!(0, -1, 0));
        let xs = plane.intersect(&ray);
        let xs = xs.unwrap();
        assert_eq!(xs.len(), 2); // TODO: has to be 1, not possible with fixed size array
        assert_eq!(xs[0].get_t(), 1.0);
        assert_eq!(xs[0].get_object().get_id(), plane.get_id());
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let plane = Plane::new();
        let ray = Ray::new(point!(0, -1, 0), vector!(0, 1, 0));
        let xs = plane.intersect(&ray);
        let xs = xs.unwrap();
        assert_eq!(xs.len(), 2); // TODO: has to be 1, not possible with fixed size array
        assert_eq!(xs[0].get_t(), 1.0);
        assert_eq!(xs[0].get_object().get_id(), plane.get_id());
    }
}
