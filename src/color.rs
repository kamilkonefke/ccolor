pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color {r, g, b}
    }

    pub fn get_red(&self) -> u8 {
        self.r
    }

    pub fn get_green(&self) -> u8 {
        self.g
    }

    pub fn get_blue(&self) -> u8 {
        self.b
    }

    pub fn from_hex(hex: &str) -> Color {
        Color {
            r: u8::from_str_radix(&hex[0..2], 16).expect("err"),
            g: u8::from_str_radix(&hex[2..4], 16).expect("err"),
            b: u8::from_str_radix(&hex[4..6], 16).expect("err"),
        }
    }
}