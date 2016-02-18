use std::collections::HashMap;
use iron::Handler;
use iron::status;
use iron::prelude::*;
pub struct Controller {
    actions: HashMap<String, Box<Handler>>,
}
impl Controller {
    pub fn new() -> Controller {
        Controller { actions: HashMap::new() }
    }
    pub fn add_action<H>(&mut self, path: String, handler: H)
        where H: Handler
    {
        self.actions.insert(path, Box::new(handler));
    }
}
impl Handler for Controller {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path.clone();
        let key = path.first().unwrap();
        match self.actions.get(key) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}
