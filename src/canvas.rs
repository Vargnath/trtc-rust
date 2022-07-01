use crate::color::Color;
use std::io::Write;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    fn coordinate_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.coordinate_to_index(x, y);
        self.pixels[index] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[self.coordinate_to_index(x, y)]
    }

    fn scale_component(component: f64) -> u8 {
        (component * 255.0).clamp(0.0, 255.0).round() as u8
    }

    fn pixel_to_rgb(pixel: Color) -> [u8; 3] {
        let red = Self::scale_component(pixel.red);
        let green = Self::scale_component(pixel.green);
        let blue = Self::scale_component(pixel.blue);
        [red, green, blue]
    }

    pub fn to_ppm(&self) -> Vec<u8> {
        let mut result = Vec::new();
        write!(&mut result, "P3\n{} {}\n255\n", self.width, self.height,).unwrap();

        let rows = self.pixels.chunks(self.width);
        for row in rows {
            let mut line = String::new();
            row.iter()
                .flat_map(|pixel| Self::pixel_to_rgb(*pixel))
                .map(|component| component.to_string())
                .for_each(|component| {
                    if line.len() + component.len() + 1 > 70 {
                        writeln!(&mut result, "{}", line).unwrap();
                        line.clear();
                    }
                    if !line.is_empty() {
                        line.push_str(format!(" {}", component).as_str());
                    } else {
                        line = component;
                    }
                });
            if !line.is_empty() {
                writeln!(&mut result, "{}", line).unwrap();
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::color::Color;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        c.pixels
            .iter()
            .for_each(|p| assert_eq!(*p, Color::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let expected = String::from("P3\n5 3\n255\n");
        let header = String::from_utf8(ppm)
            .unwrap()
            .lines()
            .take(3)
            .fold(String::new(), |lhs, rhs| format!("{}{}\n", lhs, rhs));
        assert_eq!(header, expected);
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        let expected = "\
            255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
        let data = String::from_utf8(ppm)
            .unwrap()
            .lines()
            .skip(3)
            .fold(String::new(), |lhs, rhs| format!("{}{}\n", lhs, rhs));
        assert_eq!(data, expected);
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        for pixel in c.pixels.iter_mut() {
            *pixel = Color::new(1.0, 0.8, 0.6);
        }
        let ppm = c.to_ppm();
        let expected = "\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
            153 255 204 153 255 204 153 255 204 153 255 204 153\n\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
            153 255 204 153 255 204 153 255 204 153 255 204 153\n";
        let data = String::from_utf8(ppm)
            .unwrap()
            .lines()
            .skip(3)
            .fold(String::new(), |lhs, rhs| format!("{}{}\n", lhs, rhs));
        assert_eq!(data, expected);
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert_eq!(ppm.last(), Some(&b'\n'));
    }
}
