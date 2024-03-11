mod color;
mod functions;
mod cli;
mod util;

use std::{collections::HashMap, io::stdout};

use clap::Parser;
use minijinja::{context, Environment, Value};

pub use anyhow::{Result, Error, Context, anyhow};

fn cmd_extract(args: &cli::Cli) -> Result<()> {
    let template_str = util::safe_file_to_string(&args.template)
        .with_context(|| format!("could not read file '{}'", &args.template))?;

    let marker = util::find_embed_marker(&template_str)
        .with_context(|| format!("could not find marker in '{}'", &args.template))?;

    let extracted_template = util::read_embed_marker(&marker)
        .with_context(|| format!("could not decode marker in '{}'", &args.template))?;

    print!("{}", extracted_template);

    // ensure there is an newline character
    if extracted_template.chars().last().unwrap_or(' ') != '\n' {
        println!();
    }

    Ok(())
}

fn create_jinja_env<'a>() -> Environment<'a> {
    let mut jinja_env = Environment::new();

    // load functions
    functions::load_functions(&mut jinja_env);

    // load vars
    jinja_env.add_global("AA", 4.5f64);
    jinja_env.add_global("AAA", 7.0f64);

    jinja_env
}

fn main() -> Result<()> {
    let args = cli::Cli::parse();

    // no need for context when extracting
    if args.extract {
        return cmd_extract(&args);
    }

    let mut jinja_env = create_jinja_env();

    if args.strict {
        jinja_env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);
    }

    let mut ctx = HashMap::<String, Value>::new();

    // evaluate every include tempalate and add it to ctx
    for template_path in args.include {
        let template_str = util::safe_file_to_string(&template_path)
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

    let file_contents = util::safe_file_to_string(&args.template)
        .with_context(|| format!("could not read file '{}'", &args.template))?;

    jinja_env.add_template("template", &file_contents)
        .with_context(|| format!("invalid template '{}'", &args.template))?;

    let template = jinja_env.get_template("template")?;

    if let Some(outfile) = args.outfile {
        let mut file = std::fs::File::create(outfile)?;
        template.render_to_write(&ctx, &mut file)?;
    } else {
        let result = template.render(&ctx)?;

        // a separator to indicate where printing from template stops
        eprintln!("---- TEMPLATE ----");

        print!("{}", result);

        // ensure there is a newline character
        if result.chars().last().unwrap_or(' ') != '\n' {
            println!();
        }
    }

    Ok(())
}

