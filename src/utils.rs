use core::fmt;
use hyper::{Client, Uri};
use std::{
    error::Error,
    fs,
    future::{self, Future},
    path::Path,
    str::FromStr,
    time::Duration,
};
use tokio::runtime::Builder;
use yaml_rust::{Yaml, YamlLoader};

type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct BadHttpStatusError(pub hyper::StatusCode);

impl fmt::Display for BadHttpStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Got status code {}", self.0)
    }
}

impl Error for BadHttpStatusError {}

pub fn yaml_from_file(path: &Path) -> Result<Yaml, Box<dyn Error>> {
    let config_str = fs::read_to_string(path)?;
    let config_yml = YamlLoader::load_from_str(config_str.as_str())?;

    Ok(config_yml[0].clone())
}

pub async fn check_url(url: &'static str) -> DynResult<()> {
    let client = Client::new();
    let result = client.get(Uri::from_str(url)?).await?;
    println!("Got Status code: {}", result.status());
    if result.status().is_success() {
        Ok(())
    } else {
        Err(Box::new(BadHttpStatusError(result.status())))
    }
}

pub fn check_urls(urls: Vec<&str>, timeout: Duration) -> Result<(), Box<dyn Error>> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("url-checker")
        .enable_time()
        .build()
        .unwrap();

    let _eg = runtime.enter();

    for &url in &urls {
        println!("Checking {}", url);
        let timeout = tokio::time::timeout(
            timeout,
            check_url(Box::leak(url.to_owned().into_boxed_str())),
        );

        runtime.spawn(timeout);
    }

    runtime.shutdown_timeout(timeout * urls.len().try_into().unwrap() * 2);

    Ok(())
}
