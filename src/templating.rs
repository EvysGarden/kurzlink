use std::error::Error;
use std::io::Write;
use minijinja::{Environment, context};
use crate::config::shortlink;
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn print_kurzlink_page_from_template<P: AsRef<Path>>(link: &shortlink::Shortlink, template_path: P)->Result<String,Box<dyn Error>>{
    let mut env = Environment::new();
    let template :&str = &fs::read_to_string(template_path)?;
    env.add_template("gitlab_pages_kurzlink", template.as_ref())?;
    let tmpl = env.get_template("gitlab_pages_kurzlink")?;
    Ok(tmpl.render(context!(link => link))?)
}

// kein plan ob das worked
pub fn write_html<P: AsRef<str>>(text_to_write: P,filename: P )->Result<(),Box<dyn std::error::Error>>{
    let path = format!("out/{}.html", filename.as_ref());
    let mut output = File::create(path)?;
    write!(output, "{}", text_to_write.as_ref());
    Ok(())
}


#[cfg(test)]
mod tmp_tests {
    use std::path::Path;
    use crate::templating::{print_kurzlink_page_from_template, write_html};
    use crate::Config;
    use std::fs;

    #[test]
    fn test_render() {
        let links = Config::new("kurzlink.yml").expect("Invalid shortlink yaml file");
        let link_to_print = links.shortlinks.get(2).unwrap();
        let rendered_template= print_kurzlink_page_from_template(&link_to_print,"gitlab_redirect_page.template").unwrap();
        print!("{rendered_template}");
        assert!(rendered_template.contains(&link_to_print.destination));
    }
    #[test]
    fn test_file_writing() {
        write_html("test","test").unwrap();
        let metadata = fs::metadata("out/test.html").unwrap();
        assert!(metadata.is_file());
        fs::remove_file("test.html");
    }
}
