use std::{fs, path::Path};

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
async fn post(slug: &str) -> Result<Template, NotFound<String>> {
    let path = Path::new("posts").join(slug);
    fs::read_to_string(&path)
        .map(|content| Template::render("post", context! { content }))
        .map_err(|e| NotFound(e.to_string())
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, post])
}
