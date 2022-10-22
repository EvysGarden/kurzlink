use crate::config::Config;
use std::{path::Path};

mod config;
mod utils;

fn main() {
    let config = Config::new(Path::new("kurzlink.yml"));
    // println!("{:?}", config);
    config.check_links();
}
