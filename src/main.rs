#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use rocket_contrib::Template;
use std::collections::HashMap;
use serde::Serialize;



#[derive(Serialize)]
struct TemplateContext {
    name: String
}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}\n", age, name)
}

#[get("/aaa")]
fn index() -> Template {
    let context = TemplateContext {
        name: "name".to_string()
    };
    Template::render("index", &context)
}

fn main() {
  rocket::ignite()
      .mount("/hello", routes![hello])
      .mount("/", routes![index])
      .attach(Template::fairing())
      .launch();
}
