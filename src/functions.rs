//! Module that contains all templating functions

use minijinja::{Environment, Error, ErrorKind, State, Value};
use crate::color::Color;
use crate::util;

fn color_hex(hex: String) -> Result<Value, Error> {
    match TryInto::<Color>::try_into(hex.as_str()) {
        Ok(x) => Ok(Value::from_object(x)),
        Err(_) => Err(Error::new(ErrorKind::InvalidOperation, "invalid color string")),
    }
}

fn color_rgb(r: u8, g: u8, b: u8, a: Option<u8>) -> Result<Value, Error> {
    Ok(Value::from_object(Color { r, g, b, a }))
}

/// Embedds the whole template inside the template for possible resue
fn embed(state: &State) -> Result<String, Error> {
    match state.get_template(state.name()) {
        // if it does not exist in state then its a template from string so ignore it
        Ok(x) => Ok(util::create_embed_marker(x.source())),
        Err(_) => Ok("".into()),
    }
}

pub fn load_functions(env: &mut Environment) {
    env.add_function("color_hex", color_hex);
    env.add_function("color_rgb", color_rgb);
    env.add_function("embed", embed);
}
