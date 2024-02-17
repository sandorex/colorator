//! Module that contains all templating functions

use minijinja::{value::ValueKind, Environment, Error, ErrorKind, Value};
use crate::color::Color;

// /// Get contrast ratio between the two colors, the order does not matter
// fn contrast(color1: String, color2: String, standard: Option<String>) -> Result<String, Error> {
//     Ok(color1 + &color2)
// }

// / Converts string to color
// fn color(name: String) -> Result<Color, Error> {
//     Ok("unimplemented".into())
// }

fn color_hex(hex: String) -> Result<Value, Error> {
    match TryInto::<Color>::try_into(hex.as_str()) {
        Ok(x) => Ok(Value::from_object(x)),
        Err(_) => Err(Error::new(ErrorKind::InvalidOperation, "invalid color string")),
    }
}

fn color_rgb(r: u8, g: u8, b: u8, a: Option<u8>) -> Result<Value, Error> {
    Ok(Value::from_object(Color { r, g, b, a }))
}

// TODO
// 1. color_hsl(h, s, l)
// 2. color_rgba(r, g, b, [a])
//

pub fn load_functions(env: &mut Environment) {
    env.add_function("color_hex", color_hex);
    env.add_function("color_rgb", color_rgb);
}
