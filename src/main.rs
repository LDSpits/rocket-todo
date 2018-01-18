#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::response::NamedFile;
use rocket::response::status::NotFound;

use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = std::env::current_dir().unwrap().join("assets").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {}", path.display())))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files])
        .launch();
}
