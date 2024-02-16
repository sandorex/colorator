mod functions;

use serde::{Serialize, Deserialize};
use minijinja::{Environment, context};
use std::fs;

pub use anyhow::{Result, Error, Context};

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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: Option<u8>,
}

fn read_color_scheme(path: &str) -> Result<ColorScheme> {
    let file = fs::File::open(path)?;

    serde_yaml::from_reader::<_, ColorScheme>(file)
        .with_context(|| format!("failed to parse {} as a ColorScheme", path))
}

// TODO add dev command that will watch template and rebuild it whenever it changes
// TODO print truecolor to terminal
fn main() -> Result<()> {
    let mut env: Environment = Environment::new();

    functions::load_functions(&mut env);

    // env.add_global("version", "0.1.0");

    let scheme = read_color_scheme("test.yml")?;
    for template in scheme.templates {
        env.add_template_owned::<String, String>(template.name.into(), template.template.into())?;
    }

    println!("{}", env.get_template("theme.lua").unwrap().render(context! { test => "yes" })?);

    Ok(())
}

