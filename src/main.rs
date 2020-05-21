extern crate ctchi;

use ctchi::core::app::{Ctchi, Config};
use ctchi::core::routes::{Routes, Route};

use ctchi_codegen::static_page;

#[static_page("/", "index.html")]
fn index() -> Route {}

#[static_page("/blog", "blog.html")]
fn blog() -> Route {}

#[static_page("/blog/1", "blog/first_day.html")]
fn blog1() -> Route {}

#[static_page("/blog/2", "blog/second_day.html")]
fn blog2() -> Route {}

#[static_page("/blog/3", "blog/third_day.html")]
fn blog3() -> Route {}

#[static_page("/blog/4", "blog/forth_day.html")]
fn blog4() -> Route {}

#[static_page("/blog/5", "blog/fifth_day.html")]
fn blog5() -> Route {}

#[static_page("/blog/6", "blog/sixth_day.html")]
fn blog6() -> Route {}

#[static_page("/blog/7", "blog/seventh_day.html")]
fn blog7() -> Route {}

#[static_page("/blog/8", "blog/8.html")]
fn blog8() -> Route {}

#[static_page("/blog/9", "blog/9.html")]
fn blog9() -> Route {}

fn main() {
    let mut routes = Routes::new();
    routes.add_route(index());
    routes.add_route(blog());
    routes.add_route(blog1());
    routes.add_route(blog2());
    routes.add_route(blog3());
    routes.add_route(blog4());
    routes.add_route(blog5());
    routes.add_route(blog6());
    routes.add_route(blog7());
    routes.add_route(blog8());
    routes.add_route(blog9());

    let configuration = Config::new();

    let server = Ctchi::new(configuration, routes);
    server.start();
}