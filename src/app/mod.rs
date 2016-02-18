
pub mod app;
pub mod app_config;

// RustyYii Konfiguration
// use std::thread::{JoinHandle, spawn, sleep};
// use std::time;
// use iron::prelude::*;
//
//
// pub use iron::error::HttpResult;
// pub use hyper::server::Listening;


#[cfg(test)]
mod app_tests {
    use super::*;

    use iron::prelude::*;
    use iron::status;
    use hyper::header::{Headers, ContentType};
    use hyper::mime::{Mime, TopLevel, SubLevel};

    fn hello_world(req: &mut Request) -> IronResult<Response> {
        let out = String::new();

        let mut response = Response::with((status::Ok,
                                           format!("
                           <form \
                                                    action=\"\">
                            \
                                                    First name:<br>
                            \
                                                    <input type=\"text\" name=\"firstname\"><br>
                            \
                                                    Last name:<br>
                            \
                                                    <input type=\"text\" name=\"lastname\">
                            \
                                                    <input type=\"submit\" value=\"Submit\">
                                                    \
                                                    </form>
                           Hello \
                                                    World: {:?}<br> headers:{}<br>",
                                                   req.url.clone().query,
                                                   req.headers.clone())));
        let mut headers = Headers::new();

        headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
        response.headers = headers;
        Ok(response)

    }
    #[test]
    fn it_works() {
        println!("Hello Test");
        let mut router = router::Router::new();
        router.add_controller("".to_string(), hello_world);
        let appConfig = app_config::app_config { router: router };
        let app = app::App::run(appConfig);
        // println!("{:?}", app.listener);
    }
}
