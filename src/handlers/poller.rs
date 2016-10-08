// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use iron::Handler;
use iron::mime::Mime;
use iron::prelude::IronResult;
use iron::prelude::Request;
use iron::prelude::Response;
use iron::status;
use rustc_serialize::json;
use std::sync::Arc;
use twitter::poller::TwitterPoller;

pub struct PollerHandler {
  poller: Arc<TwitterPoller>,
}

impl PollerHandler {
  pub fn new(poller: Arc<TwitterPoller>) -> PollerHandler {
    PollerHandler { poller: poller }
  }
}

impl Handler for PollerHandler {
  fn handle(&self, _req: &mut Request) -> IronResult<Response> {
    let tweets = self.poller.get_tweets().unwrap(); // FIXME
    let response = json::encode(&tweets).unwrap(); // FIXME
    let mime_type = "application/json".parse::<Mime>().unwrap(); // FIXME

    Ok(Response::with((mime_type, status::Ok, response)))
  }
}

