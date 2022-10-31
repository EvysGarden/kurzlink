use std::error::Error;
use minijinja::{Environment, context};
use crate::config::shortlink;
use std::fs;
use std::path::Path;

pub fn print_kurzlink_page_from_template<P: AsRef<Path>>(link: &shortlink::Shortlink, template_path: P)->Result<String,Box<dyn Error>>{
    let mut env = Environment::new();
    let template :&str = &fs::read_to_string(template_path)?;
    env.add_template("gitlab_pages_kurzlink", template.as_ref())?;
    let tmpl = env.get_template("gitlab_pages_kurzlink")?;
    Ok(tmpl.render(context!(link => link))?)
}


#[cfg(test)]
mod tmp_tests {
    use std::path::Path;
    use crate::templating::print_kurzlink_page_from_template;
    use crate::Config;

    #[test]
    fn it_works() {
        let links = Config::new("kurzlink.yml").expect("Invalid shortlink yaml file");
        let link_to_print = links.shortlinks.get(2).unwrap();
        let rendered_template= print_kurzlink_page_from_template(&link_to_print,"gitlab_redirect_page.template").unwrap();
        print!("{rendered_template}");
        assert!(rendered_template.contains(&link_to_print.destination));
    }
}
