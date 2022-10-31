use crate::{config::Config, error::ValidationError};
use clap::{arg, command};

mod config;
mod error;
mod utils;
mod templating;


fn main(){
    let matches = command!()
        .arg(arg!(-f --configfile <VALUE>).default_value("kurzlink.yml").help("the file used as base for the generated links"))
        .arg(arg!(-t --templatefile <VALUE>).default_value("gitlab_redirect_page.template").help("the file used as template to generate pages"))
        .arg(arg!(-g --generate).help("genrates files defined by the "))
        .arg(arg!(-n --nocheck).help("skips the checks of the base file for validity"))
        .arg(arg!(--debug).help("starts a normal run but prints the result instead of writing them to files"))
        .get_matches();

    // unwrapping is okay since clap inserts safe defaults
    let template_path = matches.get_one::<String>("templatefile").unwrap();
    let config_file = matches.get_one::<String>("configfile").unwrap();
    let nocheck_flag = matches.get_one::<bool>("nocheck").unwrap();
    let generate_flag = matches.get_one::<bool>("generate").unwrap();
    let debug = matches.get_one::<bool>("debug").unwrap();
    let links = Config::new(config_file).expect("Invalid shortlink yaml file");

    if !*nocheck_flag {
        handle_errors_in_shortlinks(&links);
    }

    // generate a file for every shortlink
    if *generate_flag || *debug {
        for link in links.shortlinks {
            for link_source in link.sources{
                let rendered_template = dbg!(templating::print_kurzlink_page_from_template(&link.destination, template_path).expect("could not generate tepmlate(s)"));
                if !*debug {
                    templating::write_html(dbg!(rendered_template), link_source).expect("couldnt write a file")
                }
            }
        }
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
