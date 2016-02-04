use std::thread::{JoinHandle, spawn, sleep};
use std::time;
use iron::prelude::*;

pub use iron::error::HttpResult;
pub use hyper::server::Listening;

pub mod router;
pub struct App {
    listener: Listening, // Nur das was drin ist
}
fn iron_spawner() -> Listening {
    let mut router = router::Router::new();
    router.add_route("".to_string(),
                     |_: &mut Request| Ok(Response::with("Hello world !")));
    router.add_route("hello".to_string(),
                     |_: &mut Request| Ok(Response::with("Hello world !")));
    let host = "localhost:80";
    println!("Starting HTTP server on http://{}...", host);
    Iron::new(router).http(host).unwrap()

}
impl App {
    pub fn run() -> App {


        App { listener: iron_spawner() }
    }
}
#[cfg(test)]
mod app_tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("Hello Test");
        let app = App::run();
        println!("{:?}", app.listener);
    }
}
