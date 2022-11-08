use crate::{config::Config, error::ValidationError};
use clap::{arg, command};
use std::path::Path;

mod config;
mod error;
mod utils;

#[rustfmt::skip::macros(arg)]

fn main() {
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
    let config = Config::new(config_file).expect("Invalid shortlink yaml file");

    if !*nocheck_flag {
        handle_errors_in_shortlinks(&config);
    }

    if *generate_flag {
        config
            .render_files(output_path, Path::new(template_file))
            .expect("couldn't generate files");
    }

    if let Some(vanity_path) = vanity_opt_path {
        config
            .write_vanity(vanity_path)
            .expect("couldn't write vanity json");
    }
}

fn handle_errors_in_shortlinks(config: &Config) {
    if let Err(validation_error) = config.validate() {
        match &validation_error {
            ValidationError::DuplicateSources(v) => panic!("Found duplicate sources: {:?}", v),
            ValidationError::DuplicateDestinations(v) => {
                panic!("Found duplicate destinations: {:?}", v)
            }
            ValidationError::NetworkError(v) => panic!("Network error: {:?}", v),
        }
    }
}
