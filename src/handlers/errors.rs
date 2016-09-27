// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use iron::AfterMiddleware;
use iron::prelude::IronError;
use iron::prelude::IronResult;
use iron::prelude::Request;
use iron::prelude::Response;

/// Error-handling filter.
pub struct ErrorFilter;

impl AfterMiddleware for ErrorFilter {
  fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
    //error!("Error has occurred! Error: {}", err.error); - TODO Logging.

    // TODO: Return JSON if request was JSON, otherwise HTML.
    match err.response.status {
      Some(status) => {
        Ok(Response::with((status, err.error.description())))
      },
      _ => Err(err)
    }
  }
}


