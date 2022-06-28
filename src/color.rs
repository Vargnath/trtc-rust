#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::float_eq;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert!(float_eq(c.red, -0.5));
        assert!(float_eq(c.green, 0.4));
        assert!(float_eq(c.blue, 1.7));
    }
}
