//! Module that contains all templating functions

use minijinja::{Environment, Error, ErrorKind, State, Value, value::Rest};
use crate::color::Color;
use crate::util;

/// Create color object from HEX string, optionally with Alpha channel
fn color_hex(hex: String) -> Result<Value, Error> {
    match TryInto::<Color>::try_into(hex.as_str()) {
        Ok(x) => Ok(Value::from_object(x)),
        Err(_) => Err(Error::new(ErrorKind::InvalidOperation, "invalid color string")),
    }
}

/// Create color object from Red Green Blue and optionally Alpha channels (8 bit)
fn color_rgb(r: u8, g: u8, b: u8, a: Option<u8>) -> Result<Value, Error> {
    Ok(Value::from_object(Color { r, g, b, a }))
}

/// Embedds the whole template inside the template for possible resue
fn embed(state: &State) -> Result<String, Error> {
    match state.get_template(state.name()) {
        Ok(x) => Ok(util::create_embed_marker(x.source())),
        // it's probably a temporary included template so ignore the error
        Err(_) => Ok("".into()),
    }
}

fn fail_if(condition: bool, msg: Option<String>) -> Result<String, Error> {
    let message = match msg {
        Some(x) => format!("failed with following msg: {}", x),
        None => "assert has failed".into(),
    };

    match condition {
        // there is no vague error type
        true => Err(Error::new(ErrorKind::InvalidOperation, message)),
        false => Ok("".into()),
    }
}

/// Embedds the whole template inside the template for possible resue
fn warn_if(condition: bool, msg: String) -> Result<String, Error> {
    // TODO i do not know how to get line number here
    if condition {
        println!("warn: {}", msg)
    }

    Ok("".into())
}

/// Fails with custom message
fn fail(msg: String) -> Result<String, Error> {
    Err(Error::new(ErrorKind::InvalidOperation, format!("fail: {}", msg)))
}

// TODO add timestamp function

pub fn load_functions(env: &mut Environment) {
    env.add_function("color_hex", color_hex);
    env.add_function("color_rgb", color_rgb);
    env.add_function("embed", embed);
    env.add_function("fail_if", fail_if);
    env.add_function("warn_if", warn_if);
    env.add_function("fail", fail);
}
