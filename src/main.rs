mod color;
mod functions;
mod cli;
mod util;

use std::{collections::HashMap, fs, io::stdout};

use clap::Parser;
use minijinja::{context, Environment, Value};

pub use anyhow::{Result, Error, Context};

// TODO print truecolor to terminal
fn main() -> Result<()> {
    let args = cli::Cli::parse();

    let mut jinja_env = Environment::new();
    functions::load_functions(&mut jinja_env);

    jinja_env.add_global("AA", 4.5f64);
    jinja_env.add_global("AAA", 7.0f64);

    if args.strict {
        jinja_env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);
    }

    let mut ctx = HashMap::<String, Value>::new();

    if args.info {
        // TODO
        return Ok(());
    }

    // evaluate every include tempalate and add it to ctx
    for template_path in args.include {
        let template_str = fs::read_to_string(&template_path)
            .with_context(|| format!("could not read file '{}'", &template_path))?;

        let template = jinja_env.template_from_str(&template_str)
            .with_context(|| format!("invalid template '{}'", &template_path))?;

        let state = template.eval_to_state(context!{})
            .with_context(|| format!("could not evaluate template '{}'", &template_path))?;

        for global_name in state.exports() {
            if let Some(val) = state.lookup(global_name) {
                ctx.insert(global_name.into(), val);
            }
        }
    }

    // TODO parse args passed
    // TODO search the file for %~%BASE64%~% here and use that instead of file itself if it exists
    // TODO ability to extract template from a file with marker

    let file_contents = fs::read_to_string(&args.template)
        .with_context(|| format!("could not read file '{}'", &args.template))?;

    jinja_env.add_template("template", &file_contents)
        .with_context(|| format!("invalid template '{}'", &args.template))?;

    let template = jinja_env.get_template("template")?;

    if let Some(outfile) = args.outfile {
        todo!();
    } else {
        template.render_to_write(&ctx, &mut stdout())?;
        // add a newline just in case
        println!();
    }

    Ok(())
}

