use minijinja::{context, Environment};
use std::fs;
use std::fs::File;
use std::io::Write;

use crate::utils::BoxError;

pub fn render_redirect_html(destination: &str, template_path: &str) -> Result<String, BoxError> {
    let mut env = Environment::new();
    let template: &str = &fs::read_to_string(template_path)?;
    env.add_template("redirect", template.as_ref())?;
    let tmpl = env.get_template("redirect")?;
    Ok(tmpl.render(context!(redirect_uri => destination))?)
}

pub fn write_html(basepath: &str, source: &str, html: &str) -> Result<(), BoxError> {
    let dirpath = format!("{basepath}/{source}");
    fs::create_dir(dirpath)?;

    let filepath = format!("{basepath}/{source}/index.html");
    let mut output = File::create(filepath)?;

    write!(output, "{html}")?;
    Ok(())
}

#[cfg(test)]
mod tmp_tests {
    use crate::templating::{render_redirect_html, write_html};
    use crate::Config;
    use std::fs;

    #[test]
    fn test_render() {
        let links = Config::new("kurzlink.yml").expect("Invalid shortlink yaml file");
        let link_to_print = links.shortlinks.get(2).unwrap();
        let rendered_template =
            render_redirect_html(link_to_print.sources.get(0).unwrap(), "redirect.template")
                .unwrap();
        dbg!("{}", rendered_template);
        //assert!(rendered_template.contains(&link_to_print.destination));
    }
    #[test]
    fn test_file_writing() {
        fs::create_dir("testbase").unwrap();
        write_html("testbase", "link", "content").unwrap();
        let metadata = fs::metadata("testbase/link/index.html").unwrap();
        assert!(metadata.is_file());
        // cleanup
        fs::remove_file("testbase/link/index.html").unwrap();
        fs::remove_dir("testbase/link").unwrap();
        fs::remove_dir("testbase").unwrap();
    }
}
