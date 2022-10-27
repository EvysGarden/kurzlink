use crate::{config::Config, error::ValidationError};
use std::{path::Path, process::exit};

mod config;
mod error;
mod utils;

fn main() {
    let config = Config::new(Path::new("kurzlink.yml")).unwrap_or_else(|v| {
        println!("Invalid config file: {:?}", v);
        exit(-1);
    });
    // println!("{:?}", config);
    if let Err(validation_error) = config.validate() {
        match &validation_error {
            ValidationError::DuplicateSources(v) => println!("Found duplicate sources: {:?}", v),
            ValidationError::DuplicateDestinations(v) => {
                println!("Found duplicate destinations: {:?}", v)
            }
            ValidationError::NetworkError(v) => println!("Network error: {}", v),
        }
        exit(validation_error.error_code());
    }
}
