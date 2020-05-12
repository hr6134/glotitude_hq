extern crate ctchi;

use ctchi::core::ctchi::{Ctchi, Config};
use ctchi::core::routes::Routes;

fn main() {
    let mut routes = Routes::new();
    routes.add_route("/", "static/index.html");
    routes.add_route("/favicon.ico", "static/404.html");
    routes.add_route("/blog", "static/blog.html");
    routes.add_route("/blog/1", "static/blog/first_day.html");
    routes.add_route("/blog/2", "static/blog/second_day.html");
    routes.add_route("/blog/3", "static/blog/third_day.html");
    routes.add_route("/blog/4", "static/blog/forth_day.html");
//    routes.add_route("/404", "static/404.html");

    let configuration = Config {
        bind_path: "138.197.210.225:80",
//        bind_path: "127.0.0.1:8080",
//        base_path: "/home/ltoshchev/programming/rust/glotitude_hq/src/",
        static_uri_pref: "/static/",
        base_path: "/var/www/gl/src/",
        routes,
    };

    let server = Ctchi::new(configuration);
    server.start();
}