use std::io::Read;
use std::fs::File;

use markdown::Options;
use rocket::{response::status::NotFound, serde::Serialize};
use rocket_dyn_templates::{context, Template};
#[macro_use] extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Project<'a> {
    name: &'a str,
    description: &'a str
}

#[get("/")]
fn index() -> Template {
    let projects = [
        Project { name: "mockpass", description: "A mock SingPass / CorpPass / MyInfo server for dev purposes" },
        Project { name: "MyInfo API on Rails", description: "MyInfo API wrappers for Rails" },
        Project { name: "is_uen", description: "Simple gem to check whether a UEN has a valid format and date" },
    ];

    Template::render("index", context! { projects })
}

#[get("/posts/<slug>")]
fn post(slug: &str) -> Result<Template, NotFound<String>> {
    let article_path = format!("./posts/{}.md", slug);
    let article = File::open(article_path);
    let mut content = String::new();

    match article {
        Ok(mut file) =>  {
            file.read_to_string(&mut content).unwrap();
            let content =  &markdown::to_html_with_options(&content, &Options::gfm()).expect(".md should be valid");
            Ok(Template::render("post", context! { content }))
        }
        Err(e) => Err(NotFound(e.to_string()))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, post])
}
