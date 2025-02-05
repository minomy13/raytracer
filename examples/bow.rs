use raytracer::{
    canvas::Canvas,
    color::Color,
    tuple::{Tuple, TupleKind},
};

const WIDTH: usize = 800;
const HEIGHT: usize = 400;

#[derive(Clone, Copy)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Clone, Copy)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(environment: Environment, projectile: Projectile) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let start = Tuple::new(TupleKind::Point, 0.0, 1.0, 0.0);
    let velocity = Tuple::new(TupleKind::Vector, 1.0, 1.8, 0.0).normalize() * 11.25;

    let mut projectile = Projectile {
        position: start,
        velocity: velocity,
    };

    let environment = Environment {
        gravity: Tuple::new(TupleKind::Vector, 0.0, -0.1, 0.0),
        wind: Tuple::new(TupleKind::Vector, -0.01, 0.0, 0.0),
    };

    while (projectile.position.0.round() as usize) < WIDTH {
        println!(
            "{} at {}",
            (projectile.position.1.round() as usize).clamp(0, HEIGHT - 1),
            (projectile.position.0.round() as usize).clamp(0, WIDTH - 1)
        );
        canvas.write_pixel(
            projectile.position.0.round() as usize,
            (projectile.position.1.round() as usize).clamp(0, HEIGHT - 1),
            Color::new(0.0, 1.0, 0.0),
        );
        projectile = tick(environment, projectile);
    }

    println!("{:?}", canvas.save())
}
