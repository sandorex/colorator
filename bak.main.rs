use serde::{Serialize, Deserialize};
use std::fs;
use std::collections::HashMap;
use tera;

pub use anyhow::{Result, Error, Context};

// enum Color<'a> {
//     // Hex(&'a str),
//     RGB(u8, u8, u8),
//     RGBA(u8, u8, u8, u8),
//     HSL(u8, u8, u8),
//     Literal(&'a str),
// }

/// Constraints for the color
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct EnsureOpts {
    /// Ensure contrast with color meets AA standard
    contrast_aa: Option<Vec<String>>,

    /// Ensure contrast with color meets AAA standard
    contrast_aaa: Option<Vec<String>>,
}

/// Definition of a color with optional default value
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ColorVar {
    name: String,
    default: Option<String>,
    ensure: Option<EnsureOpts>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Template {
    /// Template name, also used as filename when outputting the file
    name: String,

    /// Template of the file
    template: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ColorScheme {
    /// Name of the color scheme
    name: String,

    /// Colors defined in the color scheme
    colors: Vec<ColorVar>,

    /// Templates for files to generate
    templates: Vec<Template>
}

fn read_color_scheme(path: &str) -> Result<ColorScheme> {
    let file = fs::File::open(path)?;

    serde_yaml::from_reader::<_, ColorScheme>(file)
        .with_context(|| format!("failed to parse {} as a ColorScheme", path))
}

fn fn_contrast() -> impl tera::Function {
    Box::new(move |args: &HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
        let color1 = match args.get("color1") {
            Some(tera::Value::String(x)) => x,
            _ => return Err("invalid type for color1".into()),
        };

        let color2 = match args.get("color2") {
            Some(tera::Value::String(x)) => x,
            _ => return Err("invalid type for color2".into()),
        };

        match args.get("standard") {
            Some(tera::Value::String(standard)) => {
                Ok(tera::Value::String(format!("{} {} {}", color1, color2, standard)))
            },
            None => {
                Ok(tera::Value::String(format!("{} {}", color1, color2)))
            },
            _ => return Err("invalid type for standard".into()),
        }
    })
}

// TODO add dev command that will watch template and rebuild it whenever it changes
// TODO print truecolor to terminal
fn main() -> Result<()> {
    let scheme: ColorScheme = read_color_scheme("test.yml")?;
    let mut tera: tera::Tera = tera::Tera::default();

    tera.register_function("contrast", fn_contrast());

    for template in scheme.templates {
        tera.add_raw_template(&template.name, &template.template)?;
    }

    let mut ctx = tera::Context::new();
    ctx.insert("name", "fuck");

    let rendered = tera.render("theme.lua", &ctx)?;
    println!("got: {}", rendered);
    // let result = tera::Tera

    // println!("got: {:#?}", read_color_scheme("test.yml"));

    Ok(())
}

