//! Module that contains all templating functions

use minijinja::{value::ValueKind, Environment, Error, ErrorKind, Value, value::Object};
use crate::Color;
use std::fmt::{self, Write};

// /// Get contrast ratio between the two colors, the order does not matter
// fn contrast(color1: String, color2: String, standard: Option<String>) -> Result<String, Error> {
//     Ok(color1 + &color2)
// }

// / Converts string to color
// fn color(name: String) -> Result<Color, Error> {
//     Ok("unimplemented".into())
// }

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.a {
            Some(alpha) => write!(f, "0x{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, alpha),
            None => write!(f, "0x{:02x}{:02x}{:02x}", self.r, self.g, self.b),
        }
    }
}

// impl StructObject for color
impl Object for Color {}

fn color_rgba(r: u8, g: u8, b: u8, a: Option<u8>) -> Result<Value, Error> {
    Ok(Value::from_object(Color { r, g, b, a }))
}

// TODO
// 1. color_hsl(h, s, l)
// 2. color_rgba(r, g, b, [a])
//

pub fn load_functions(env: &mut Environment) {
    env.add_function("color_rgba", color_rgba);
}
