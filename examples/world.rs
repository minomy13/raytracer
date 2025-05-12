use std::f64::consts::PI;

use raytracer::body::plane::Plane;
use raytracer::{
    body::{sphere::Sphere, Body},
    camera::Camera,
    color::Color,
    light::{Light, PointLight},
    material::Material,
    matrix::Matrix,
    point,
    tuple::Tuple,
    vector,
    world::World,
};

fn main() {
    let floor = Plane::new().set_material(
        Material::default()
            .set_color(Color::new(1.0, 0.9, 0.9))
            .set_specular(0.0),
    );

    let middle = Sphere::new()
        .transform(Matrix::translation_matrix(-0.5, 1.0, 0.5))
        .set_material(
            Material::default()
                .set_color(Color::new(0.1, 1.0, 0.5))
                .set_diffuse(0.7)
                .set_specular(0.3),
        );
    let right = Sphere::new()
        .transform(Matrix::translation_matrix(1.5, 0.5, -0.5).scale(0.5, 0.5, 0.5))
        .set_material(
            Material::default()
                .set_color(Color::new(0.5, 1.0, 0.1))
                .set_diffuse(0.7)
                .set_specular(0.3),
        );
    let left = Sphere::new()
        .transform(Matrix::translation_matrix(-1.5, 0.33, -0.75).scale(0.33, 0.33, 0.33))
        .set_material(
            Material::default()
                .set_color(Color::new(1.0, 0.8, 0.1))
                .set_diffuse(0.7)
                .set_specular(0.3),
        );

    // TODO: find better style!
    let mut world = World::new();
    let world = world.add_light(PointLight::new(point!(-10, 10, -10), Color::white()));
    let world = world
        .add_object(floor)
        .add_object(middle)
        .add_object(right)
        .add_object(left);

    let mut c = Camera::new(500, 250, PI / 3.0);
    let c = c.transform(Matrix::view_transform_matrix(
        point!(0, 1.5, -5),
        point!(0, 1, 0),
        vector!(0, 1, 0),
    ));
    let _ = c.render(world).save();
}
