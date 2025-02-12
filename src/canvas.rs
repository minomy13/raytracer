use std::{fs::File, io::Write};

use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); width]; height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.validate_coordinates(x, y);
        self.pixels[y][x] = color;
    }

    // TODO: better use result?
    fn validate_coordinates(&self, x: usize, y: usize) {
        if !(x < self.width) {
            panic!("Failed accessing pixel. x coordinate {x} at y {y} out of canvas bounds.");
        }
        if !(y < self.height) {
            panic!("Failed writing pixel. y coordinate {y} at x {x} out of canvas bounds.");
        }
    }

    fn construct_ppm(&self) -> String {
        let mut ppm = String::new();

        // ppm header; identifier, width and height, maximum color value
        ppm += &format!("P3\n{} {}\n255\n", self.width, self.height);

        for l in self.pixels.clone() {
            let mut line = String::new();
            for c in l {
                // cap line length to 70
                // TODO: unwrap unsafe?
                if line.split("\n").last().unwrap().len() + 13 >= 70 {
                    line += "\n"
                };

                let c = c.as_8bit();
                line += &format!("{} {} {} ", c.0, c.1, c.2);
            }
            ppm += &format!("{}\n", line.trim())
        }

        ppm
    }

    // TODO: check why having to use whole lib path
    pub fn save(&self) -> std::io::Result<()> {
        let path = format!(
            "render-{}.ppm",
            chrono::Local::now().to_rfc3339().replace(":", "-")
        );
        println!("Saving to file {path} now ...");
        let mut file = File::create(path)?;
        file.write_all(self.construct_ppm().as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Canvas {
        fn read_pixel(&self, x: usize, y: usize) -> Color {
            self.validate_coordinates(x, y);
            self.pixels[y][x]
        }
    }

    #[test]
    fn creating_canvas() {
        let width = 10;
        let height = 20;

        let mut canvas = Canvas::new(width, height);

        for y in 0..height {
            for x in 0..width {
                canvas.write_pixel(x, y, Color::new(0.0, 0.0, 0.0));
            }
        }

        for y in 0..height {
            for x in 0..width {
                assert!(canvas.read_pixel(x, y) == Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red);

        assert!(canvas.read_pixel(2, 3) == red);
    }

    #[test]
    fn constructing_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);

        let ppm = canvas.construct_ppm();
        assert_eq!(
            ppm,
            "P3\n5 3\n255\n\
            255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n"
        )
    }

    #[test]
    fn no_line_longer_than_70() {
        let width = 10;
        let height = 2;
        let mut canvas = Canvas::new(width, height);
        for y in 0..height {
            for x in 0..width {
                canvas.write_pixel(x, y, Color::new(1.0, 0.8, 0.6));
            }
        }

        for line in canvas.construct_ppm().split("\n") {
            assert!(!(line.len() > 70))
        }
    }
}
