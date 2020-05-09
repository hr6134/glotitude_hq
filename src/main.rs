extern crate ctchi;

use ctchi::core::ctchi::{Ctchi, Config};
use ctchi::core::routes::Routes;

fn main() {
    let mut routes = Routes::new();
    routes.add_route("/", "src/static/index.html");
    routes.add_route("/favicon.ico", "src/static/404.html");
    routes.add_route("/blog", "src/static/blog.html");
    routes.add_route("/blog/1", "src/static/blog/first_day.html");
    routes.add_route("/blog/2", "src/static/blog/second_day.html");
    routes.add_route("/404", "src/static/404.html");

    let configuration = Config {
        bind_path: "138.197.210.225:80",
//        bind_path: "127.0.0.1:8080",
//        base_path: "/home/ltoshchev/programming/rust/glotitude_hq/",
        base_path: "/var/www/gl/",
        routes,
    };

    let server = Ctchi::new(configuration);
    server.start();
}