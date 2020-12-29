#![feature(concat_idents)]
#[macro_use]
extern crate ctchi;

use std::collections::HashMap;

use ctchi::core::app::Ctchi;
use ctchi::core::routes::{Routes, Route};

use ctchi_codegen::route;

#[route("/")]
fn index() -> String {
    render!("index.html")
}

#[route("/blog/")]
fn blog_list() -> String {
    render!("blog.html")
}


#[route("/blog/ctchi/{id}/")]
fn blog(id: &str) -> String {
    let page = &format!("blog/ctchi/{}.html", id);
    render!(page)
}

#[route("/blog/clean/{name}/")]
fn blog_clean(name: &str) -> String {
    let page = &format!("blog/clean/{}.html", name);
    render!(page)
}

fn main() {
    let mut routes = Routes::new();
    routes.add_route(routes!(index)());
    routes.add_route(routes!(blog_list)());
    routes.add_route(routes!(blog_clean)());
    routes.add_route(routes!(blog)());

    let server = Ctchi::new(routes);
    server.start();
}