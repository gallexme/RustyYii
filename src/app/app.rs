use app::router;
use app::app_config::*;
pub use hyper::server::Listening;
use iron::prelude::*;
pub struct App {
    listener: Listening,
}
fn iron_spawner(router: router::Router) -> Listening {
    // let mut router = router::Router::new();
    // router.add_route("".to_string(), hello_world);
    // router.add_route("hello".to_string(), hello_world);
    let host = "localhost:80";
    println!("Starting HTTP server on http://{}...", host);
    Iron::new(router).http(host).unwrap()

}

impl App {
    pub fn run(conf: app_config) -> App {
        App { listener: iron_spawner(conf.router) }
    }
}