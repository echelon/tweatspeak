// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

extern crate egg_mode;
extern crate handlebars_iron;
extern crate iron;
extern crate mount;
extern crate resolve;
extern crate router;
extern crate rustc_serialize;
extern crate staticfile;
extern crate toml;

pub mod config;
pub mod handlers;
pub mod twitter;

use config::Config;
use handlebars_iron::DirectorySource;
use handlebars_iron::HandlebarsEngine;
use handlebars_iron::Template;
use handlers::errors::ErrorFilter;
use handlers::tweets::TweetHandler;
use iron::Iron;
use iron::Set;
use iron::middleware::Chain;
use iron::prelude::Request;
use iron::prelude::Response;
use iron::status;
use mount::Mount;
use router::Router;
use staticfile::Static;
use std::path::Path;
use twitter::TwitterMediator;
use twitter::TwitterSecrets;

fn init_server(configs: Config, twitter_mediator: TwitterMediator) {
  let mut mount = Mount::new();
  // Index
  let mut index_chain = Chain::new(move |_: &mut Request| {
    let mut response = Response::new();
    response.set_mut(Template::new("index", configs.clone())).set_mut(status::Ok);
    Ok(response)
  });

  let mut hbs = HandlebarsEngine::new();
  hbs.add(Box::new(DirectorySource::new("./www/templates/", ".hbs")));
  hbs.reload().unwrap();

  index_chain.link_after(hbs);

  let mut index_router = Router::new();
  index_router.get("/", index_chain, "index_handler");
  mount.mount("/", index_router);

  // Assets
  let file_handler = Static::new(Path::new("www/"));
  let mut file_chain = Chain::new(file_handler);
  file_chain.link_after(ErrorFilter);
  mount.mount("/assets", file_chain);

  // Twitter Endpoint
  let mut tweet_router = Router::new();
  let twitter_handler = TweetHandler::new(twitter_mediator);
  let mut chain = Chain::new(twitter_handler);
  chain.link_after(ErrorFilter);
  tweet_router.get("/user/:username", chain, "tweet_handler");
  mount.mount("/tweets", tweet_router);

  Iron::new(mount).http("127.0.0.1:3000").unwrap();
}


fn main() {
  let configs = Config::read("./configs.toml").unwrap();
  let secrets = TwitterSecrets::read_toml_file("./twitter_secrets.toml")
      .unwrap();

  let twitter_mediator = TwitterMediator::new(secrets);

  init_server(configs, twitter_mediator);
}

