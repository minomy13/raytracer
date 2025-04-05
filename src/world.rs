use crate::{
    body::{sphere::Sphere, Body},
    color::Color,
    light::{Light, PointLight},
    material::Material,
    matrix::Matrix,
    ray::{intersection::Intersection, Ray},
    tuple::Tuple,
};

pub struct World {
    lights: Vec<Box<dyn Light>>,
    objects: Vec<Box<dyn Body>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            lights: vec![],
            objects: vec![],
        }
    }

    pub fn default() -> Self {
        Self {
            lights: vec![Box::new(PointLight::new(
                Tuple::new_point(-10.0, 10.0, -10.0),
                Color::white(),
            ))],
            objects: vec![
                Box::new(
                    Sphere::new().set_material(
                        Material::default()
                            .set_color(Color::new(0.8, 1.0, 0.6))
                            .set_diffuse(0.7)
                            .set_specular(0.2),
                    ),
                ),
                Box::new(Sphere::new().transform(Matrix::scaling_matrix(0.5, 0.5, 0.5))),
            ],
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections = vec![];

        for object in self.get_objects() {
            if let Some(intersection) = object.intersect(&ray) {
                intersections.extend(intersection);
            }
        }

        intersections.sort_by(|a, b| a.get_t().total_cmp(&b.get_t()));
        intersections
    }

    pub fn get_objects(&self) -> Vec<&dyn Body> {
        self.objects.iter().map(|elm| elm.as_ref()).collect()
    }

    pub fn get_lights(&self) -> Vec<&dyn Light> {
        self.lights.iter().map(|elm| elm.as_ref()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        body::{sphere::Sphere, Body},
        color::Color,
        light::{Light, PointLight},
        material::Material,
        matrix::Matrix,
        ray::Ray,
        tuple::Tuple,
    };

    use super::World;

    #[test]
    fn creating_a_world() {
        let w = World::new();
        assert_eq!(w.get_objects().len(), 0);
        assert_eq!(w.get_lights().len(), 0);
    }

    #[test]
    fn the_default_world() {
        let light = PointLight::new(Tuple::new_point(-10.0, 10.0, -10.0), Color::white());
        let sphere1 = Sphere::new().set_material(
            Material::default()
                .set_color(Color::new(0.8, 1.0, 0.6))
                .set_diffuse(0.7)
                .set_specular(0.2),
        );
        let sphere2 = Sphere::new().transform(Matrix::scaling_matrix(0.5, 0.5, 0.5));

        // TODO: consider checking type
        let world = World::default();

        assert_eq!(world.get_lights().len(), 1);
        let world_lights = world.get_lights();
        let l = world_lights.first().unwrap();
        assert!(
            (l.get_position() == light.get_position())
                && (l.get_intensity() == light.get_intensity())
        );

        let obj = world.get_objects();
        assert_eq!(obj.len(), 2);
        let obj1 = obj[0];
        let obj2 = obj[1];
        assert!(
            (obj1.get_material() == sphere1.get_material())
                && (obj1.get_transformation() == sphere1.get_transformation())
        );
        assert!(
            (obj2.get_material() == sphere2.get_material())
                && (obj2.get_transformation() == sphere2.get_transformation())
        )
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vec(0.0, 0.0, 1.0),
        );
        let xs = w.intersect(r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].get_t(), 4.0);
        assert_eq!(xs[1].get_t(), 4.5);
        assert_eq!(xs[2].get_t(), 5.5);
        assert_eq!(xs[3].get_t(), 6.0)
    }
}
