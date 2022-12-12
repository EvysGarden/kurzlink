use std::process::exit;

use crate::config::Config;
use anyhow::Context;
use clap::{arg, command};

mod config;
mod error;
mod utils;

#[rustfmt::skip::macros(arg)]

fn main() -> anyhow::Result<()> {
    let matches = command!()
        .arg(
            arg!(-c --config <VALUE>)
                .default_value("kurzlink.yml")
                .help("the file used as base for the generated links"),
        )
        .arg(
            arg!(-t --template <VALUE>)
                .default_value("redirect.template")
                .help("the file used as template to generate pages"),
        )
        .arg(arg!(-g --generate).help("generates files based on the template"))
        .arg(arg!(-n --nocheck).help("skips the checks of the config file for validity"))
        .arg(
            arg!(-m --vanitymap <VALUE>)
                .default_value(None)
                .help("generate a vanitymap at <VALUE>"),
        )
        .arg(
            arg!(-o --output <VALUE>)
                .default_value("output")
                .help("the base directory to populate"),
        )
        .get_matches();

    // unwrapping is okay since clap inserts safe defaults
    let template_file = matches.get_one::<String>("template").unwrap();
    let config_file = matches.get_one::<String>("config").unwrap();
    let nocheck_flag = matches.get_one::<bool>("nocheck").unwrap();
    let generate_flag = matches.get_one::<bool>("generate").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();
    let vanity_opt_path = matches.get_one::<String>("vanitymap");

    // get the links
    let config = Config::new(config_file)
        .with_context(|| "config cant be init".to_string())
        .unwrap_or_else(|err| {
            println!("Error: {}", err.root_cause());
            exit(1);
        });

    if !*nocheck_flag {
        config.validate().unwrap_or_else(|err| {
            println!("Error: {}", err.root_cause());
            exit(2);
        });
    }

    if *generate_flag {
        config
            .render_files(output_path, template_file)
            .with_context(|| "Rendering failed files failed".to_string())?
    }

    if let Some(vanity_path) = vanity_opt_path {
        config
            .write_vanity(vanity_path)
            .with_context(|| "Writing the vanitymap failed heroically".to_string())?;
    };
    anyhow::Ok(())
}

// fn handle_errors_in_shortlinks(config: &Config) -> anyhow::Result<()> {
//     if let Err(validation_error) = config.validate() {
//         match &validation_error {
//             ValidationError::DuplicateSources(v) => bail!("Found duplicate sources: {:?}", v),
//             ValidationError::DuplicateDestinations(v) => {
//                 bail!("Found duplicate destinations: {:?}", v)
//             }
//             ValidationError::NetworkError(v) => bail!("Network error: {:?}", v),
//         }
//     } else {
//         anyhow::Result::Ok(())
//     }
// }
