use crate::{config::Config, error::ValidationError};
use clap::{arg, command};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

mod config;
mod error;
mod templating;
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
            arg!(-p --print)
                .help("starts a normal run but prints the result instead of writing them to files"),
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
    let print_flag = matches.get_one::<bool>("print").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();
    let vanity_opt_path = matches.get_one::<String>("vanitymap");

    // get the links
    let links = Config::new(config_file).expect("Invalid shortlink yaml file");

    if !*nocheck_flag {
        handle_errors_in_shortlinks(&links);
    }

    // generate a file for every shortlink
    if *generate_flag || *print_flag {
        fs::create_dir(output_path).ok();

        for link in &links.shortlinks {
            for link_source in &link.sources {
                let rendered_template =
                    templating::render_redirect_html(&link.destination, Path::new(template_file))
                        .expect("could not generate tepmlate(s)");
                if !*print_flag {
                    templating::write_html(Path::new(output_path), &link_source, &rendered_template)
                        .expect("couldnt write a file")
                }
            }
        }
    }

    if let Some(vanity_path) = vanity_opt_path {
        let vanitymap_json = links.generate_vanitymap().to_string();
        let mut vanity_file = File::create(vanity_path).unwrap();
        write!(vanity_file, "{vanitymap_json}").unwrap();
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
