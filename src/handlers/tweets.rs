// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use iron::Handler;
use iron::mime::Mime;
use iron::prelude::IronResult;
use iron::prelude::Request;
use iron::prelude::Response;
use iron::status;
use router::Router;
use rustc_serialize::json;
use twitter::TwitterMediator;

pub struct TweetHandler {
  mediator: TwitterMediator,
}

impl TweetHandler {
  pub fn new(mediator: TwitterMediator) -> TweetHandler {
    TweetHandler {
      mediator: mediator,
    }
  }
}

impl Handler for TweetHandler {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {

    let username = req.extensions.get::<Router>().unwrap() // FIXME
        .find("username")
        .unwrap_or("undeclared")
        .to_string();

    let tweets = self.mediator.get_timeline(&username, 50).unwrap(); // FIXME
    let response = json::encode(&tweets).unwrap(); // FIXME
    let mime_type = "application/json".parse::<Mime>().unwrap(); // FIXME

    Ok(Response::with((mime_type, status::Ok, response)))
  }
}

/*impl From<TwitterError> for IronError {
  fn from(error: TwitterError) -> IronError {
    IronError::new()
  }
}*/

