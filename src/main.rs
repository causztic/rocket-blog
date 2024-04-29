use std::{fs, fs::File, io::Read};

use rand::Rng;
use markdown::Options;
use rocket::{serde::Serialize, Request, http::Status};
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
        Project { name: "myinfo-rails", description: "MyInfo API wrappers for Rails" },
        Project { name: "is_uen", description: "Simple gem to check whether a UEN has a valid format and date" }
    ];

    let hobbies = [
        "play the piano", 
        "draw", 
        "brew filter coffee", 
        "make espresso",
        "collect CDs",
        "collect keyboards",
    ];
    let random_hobby = hobbies[rand::thread_rng().gen_range(0..hobbies.len())];
    Template::render("index", context! { projects, random_hobby })
}

#[get("/posts/<slug>")]
fn post(slug: &str) -> Option<Template> {
    let article_path = format!("./posts/{}.md", slug);
    let article = File::open(article_path);
    let mut content = String::new();

    match article {
        Ok(mut file) =>  {
            file.read_to_string(&mut content).unwrap();
            let content =  &markdown::to_html_with_options(&content, &Options::gfm()).expect(".md should be valid");
            Some(Template::render("post", context! { content }))
        }
        Err(_) => None
    }
}

#[get("/posts")]
fn posts() -> Template {
    let articles: Vec<String> = fs::read_dir("./posts").unwrap().map(|post| post.unwrap().path().as_path().file_stem().unwrap().to_os_string().into_string().unwrap()).collect();
    Template::render("posts", context! { articles })
}

#[catch(default)]
fn default_catcher(status: Status, _: &Request) -> Template {
    Template::render("error", context! { status })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, post, posts])
        .register("/", catchers![default_catcher])
}
