use anyhow::Context;
use minijinja::{context, Environment};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::url::AbsoluteUrl;

pub fn render_redirect_html(
    destination: &AbsoluteUrl,
    template_path: impl AsRef<Path>,
) -> anyhow::Result<String> {
    let mut env = Environment::new();
    let template: &str = &fs::read_to_string(template_path)?;
    env.add_template("redirect", template.as_ref())?;
    let tmpl = env.get_template("redirect")?;
    Ok(tmpl.render(context!(redirect_uri => destination.inner()))?)
}

pub fn write_html(base_path: impl AsRef<Path>, html: &str) -> anyhow::Result<()> {
    if !base_path.as_ref().exists() {
        fs::create_dir_all(&base_path).with_context(|| {
            format!("files already present or invalid character in file: {}", base_path.as_ref().to_str().unwrap_or("INVALID PATH"))
        })?;
    };

    let filepath = base_path.as_ref().join("index.html");
    let mut output = File::create(filepath)
        .with_context(|| format!("files already present or invalid character in file: {}", base_path.as_ref().to_str().unwrap_or("INVALID PATH")))?;

    write!(output, "{html}").with_context(|| {
        format!("file unable to be written, exists or contains invalid character: {}", base_path.as_ref().to_str().unwrap_or("INVALID PATH"))
    })?;
    Ok(())
}

#[cfg(test)]
mod tmp_tests {
    use crate::{
        config::templating::{render_redirect_html, write_html},
        Config,
    };
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_render() {
        let links = Config::new("kurzlink.yml").expect("Invalid shortlink yaml file");
        let link_to_print = links.shortlinks.get(2).unwrap();
        let _rendered_template =
            render_redirect_html(&link_to_print.destination, Path::new("redirect.template"))
                .unwrap();
        dbg!("{rendered_template}");
    }
    #[test]
    fn test_file_writing() {
        fs::create_dir("testbase").unwrap();
        write_html(Path::new("testbase").join("link"), "content").unwrap();
        let metadata = fs::metadata("testbase/link/index.html").unwrap();
        assert!(metadata.is_file());
        // cleanup
        fs::remove_file("testbase/link/index.html").unwrap();
        fs::remove_dir("testbase/link").unwrap();
        fs::remove_dir("testbase").unwrap();
    }
}
