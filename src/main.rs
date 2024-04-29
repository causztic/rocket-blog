use std::{fs::File, io::Read};

use indexmap::IndexMap;
use rand::Rng;
use markdown::Options;
use rocket::{serde::Serialize, serde::ser::SerializeStruct, Request, http::Status};
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

struct Post<'a> {
    title: &'a str,
    date: &'a str
}

impl Serialize for Post<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer {
        let mut item = serializer.serialize_struct("Post", 2)?;
        item.serialize_field("title", &self.title)?;
        item.serialize_field("date", &self.date)?;
        item.end()
    }
}

type Posts<'a> = IndexMap<&'a str, Post<'a>>;

fn get_posts<'a>() -> Posts<'a> {
    let mut posts: Posts = IndexMap::new();
    posts.insert("devise-jwt-session-hybrid", Post { title: "Devise JWT with Sessions Hybrid", date: "2019-09-20" });
    posts.insert("the-phoenix-project-thoughts", Post { title: "Phoenix", date: "2020-02-26" });
    posts.insert("distinct-on-subquery-with-order", Post { title: "DISTINCT ON subquery with correct ordering", date: "2020-10-14" });
    posts.insert("nextjs-new-design", Post { title: "New Design with Next.js - from development to deployment", date: "2021-05-02" });
    posts.insert("writing-objectively-better-specs", Post { title: "Writing Objectively Better Specs", date: "2021-05-10" });
    posts.insert("react-input-with-gboard-emoji-bug", Post { title: "Fixing a weird React input bug with Gboard emoji suggestions", date: "2021-08-20" });
    posts.insert("upgrading-i18n-webpack-plugin", Post { title: "How I upgraded a Webpack 4 plugin to Webpack 5", date: "2021-10-13" });
    posts.insert("rails-gotcha-with-options", Post { title: "Ruby on Rails Gotcha - Object#with_options", date: "2022-03-30" });
    posts.insert("using-loops-in-rspec-tests", Post { title: "Using loops in RSpec", date: "2022-07-14" });
    posts.insert("value-objects-in-ruby", Post { title: "Value Objects in Ruby", date: "2022-08-28" });
    posts.insert("no-active-record-callbacks", Post { title: "DISTINCT ON subquery with correct ordering", date: "2022-09-27" });
    posts.insert("monkey-patching-in-ruby", Post { title: "Monkey Patching in Ruby", date: "2022-10-25" });
    posts
}

#[get("/posts/<slug>")]
fn post(slug: &str) -> Option<Template> {
    let posts = get_posts();
    if !posts.contains_key(slug) { return None }

    let article_path = format!("./posts/{}.md", slug);
    let article = File::open(article_path);
    let mut content = String::new();

    match article {
        Ok(mut file) => {
            let post = posts.get(slug).unwrap();
            file.read_to_string(&mut content).unwrap();
            let content =  &markdown::to_html_with_options(&content, &Options::gfm()).expect(".md should be valid");
            Some(Template::render("post", context! { content, date: post.date, title: post.title }))
        }
        Err(_) => None
    }
}

#[get("/posts")]
fn posts() -> Template {
    let posts: Vec<_> = get_posts().into_iter().rev().collect();
    Template::render("posts", context! { posts })
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
