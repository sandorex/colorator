//! Module that contains all templating functions

use minijinja::{Environment, Error, ErrorKind, State, Value, value::{ Kwargs, ValueKind, from_args }, value::Rest};
use crate::color::Color;
use crate::util;

/// Create color object from either RGBA or hex string
fn color(args: Rest<Value>) -> Result<Value, Error> {
    let arg = args.get(0)
        .ok_or_else(|| Error::new(ErrorKind::MissingArgument, "no arguments provided"))?;

    match arg.kind() {
        ValueKind::Number => {
            let (r, g, b, a): (u8, u8, u8, Option<u8>) = from_args(args.as_slice())?;

            Ok(Value::from_object(Color { r, g, b, a }))
        },
        ValueKind::String => {
            let (string,): (&str,) = from_args(args.as_slice())?;

            match TryInto::<Color>::try_into(string) {
                Ok(x) => Ok(Value::from_object(x)),
                Err(_) => Err(Error::new(ErrorKind::InvalidOperation, "invalid color string")),
            }
        },
        _ => Err(Error::new(ErrorKind::MissingArgument, "invalid arguments")),
    }
}

/// Embedds the whole template inside the template for possible resue
fn embed(state: &State) -> Result<String, Error> {
    match state.get_template(state.name()) {
        Ok(x) => Ok(util::create_embed_marker(x.source())),
        // it's probably a temporary included template so ignore the error
        Err(_) => Ok("".into()),
    }
}

/// Warns with custom message, optionally with a condition
fn warn(msg: String, cond: Option<bool>) -> Result<String, Error> {
    // TODO i do not know how to get line number here
    // warn if no condition or condition is true
    if cond.unwrap_or(true) {
        eprintln!("warning: {}", msg)
    }

    Ok("".into())
}

/// Fails with custom message, optionally with a condition
fn fail(msg: String, cond: Option<bool>) -> Result<String, Error> {
    // fail if no condition or condition is true
    if cond.unwrap_or(true) {
        return Err(Error::new(ErrorKind::InvalidOperation, format!("fail: {}", msg)));
    }

    Ok("".into())
}

/// Print to console, colors can be changed using fg, bg keyword. Both hexadecimal strings and
/// color object are supported
/// Only works in truecolor terminals
fn print(msg: String, options: Kwargs) -> Result<String, Error> {
    let mut full_msg: String = msg;

    // set bg optionally
    if let Ok(Some(x)) = options.get::<Option<Value>>("bg") {
        match TryInto::<Color>::try_into(x) {
            Ok(bg_color) => {
                full_msg = bg_color.bg(&full_msg);
            },
            Err(_) => return Err(Error::new(ErrorKind::InvalidOperation, "bg color can be either a hex string or a color object")),
        }
    }

    // set fg optionally
    if let Ok(Some(x)) = options.get::<Option<Value>>("fg") {
        match TryInto::<Color>::try_into(x) {
            Ok(fg_color) => {
                full_msg = fg_color.fg(&full_msg);
            },
            Err(_) => return Err(Error::new(ErrorKind::InvalidOperation, "fg color can be either a hex string or a color object")),
        }
    }

    // print to stderr so piping works
    eprintln!("{}", full_msg);

    Ok("".into())
}

// TODO add timestamp function

pub fn load_functions(env: &mut Environment) {
    env.add_function("color", color);
    env.add_function("embed", embed);
    env.add_function("warn", warn);
    env.add_function("fail", fail);
    env.add_function("print", print);
}
