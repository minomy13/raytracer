use raytracer::{
    body::{sphere::Sphere, Body},
    canvas::Canvas,
    color::Color,
    light::{Light, PointLight},
    material::Material,
    matrix::Matrix,
    ray::{intersection::Intersection, Ray},
    tuple::Tuple,
};

fn main() {
    let ray_origin = Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 600;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half_wall = wall_size / 2f64;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let material = Material::default().set_color(Color::new(1.0, 0.0, 0.0));
    let shape = Sphere::new()
        .transform(Matrix::scaling_matrix(1.0, 1.0, 1.0))
        .set_material(material);

    let light_pos = Tuple::new_point(-10.0, 10.0, -10.0);
    let light_color = Color::white();
    let light = PointLight::new(light_pos, light_color);

    for y in 0..canvas_pixels {
        let world_y = half_wall - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half_wall + pixel_size * x as f64;

            let position = Tuple::new_point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            if let Some(xs) = shape.intersect(&r) {
                let mut v = xs.to_vec();
                if let Some(hit) = Intersection::find_hit(&mut v) {
                    let hitpoint = r.position(hit.get_t());
                    let color = shape.get_material().lighting(
                        &light,
                        hitpoint,
                        -r.get_direction(),
                        shape.normal_at(hitpoint),
                    );
                    canvas.write_pixel(x, y, color);
                }
            }
        }
    }

    let _ = canvas.save();
}
