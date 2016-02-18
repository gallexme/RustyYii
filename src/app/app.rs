use app::app_config::*;
pub use hyper::server::Listening;
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware};
use router::Router;
pub struct App {
    listener: Listening,
}

struct LogRequests;

impl BeforeMiddleware for LogRequests {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        println!("{}", req.url);
        Ok(())
    }
}
fn iron_spawner(mut chain: Chain) -> Listening {
    // let mut router = router::Router::new();
    // router.add_route("".to_string(), hello_world);
    // router.add_route("hello".to_string(), hello_world);
    let host = "localhost:80";
    println!("Starting HTTP server on http://{}...", host);
    chain.link_before(LogRequests);
    Iron::new(chain).http(host).unwrap()

}

impl App {
    pub fn run(conf: app_config) -> App {
        App { listener: iron_spawner(conf.chain) }
    }
}
