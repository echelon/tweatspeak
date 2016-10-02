// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use egg_mode::Token;
use egg_mode::tweet;
use egg_mode;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use toml;

pub type TwitterError = egg_mode::error::Error;

#[derive(Clone, RustcDecodable)]
pub struct TwitterSecrets {
  pub api_key: String,
  pub api_secret: String,
  pub access_token_key: String,
  pub access_token_secret: String,
}

impl TwitterSecrets {
  /// Read the secrets from a TOML file.
  pub fn read_toml_file(filename: &str) -> Result<TwitterSecrets, Error> {
    let mut file = try!(File::open(filename));
    let mut buf = String::new();
    try!(file.read_to_string(&mut buf));

    toml::Parser::new(&buf).parse()
        .and_then(|p| toml::decode(toml::Value::Table(p)))
        .ok_or(Error::new(ErrorKind::Other, ""))
  }

  pub fn api_token(&self) -> Token {
    Token::new(self.api_key.clone(), self.api_secret.clone())
  }

  pub fn access_token(&self) -> Token {
    Token::new(self.access_token_key.clone(), self.access_token_secret.clone())
  }
}

#[derive(Clone, Debug, RustcEncodable)]
pub struct Tweet {
  pub avatar: String,
  pub created_at: String, // TODO: Make this a datetime.
  pub name: String,
  pub text: String,
  pub username: String,
}

#[derive(Clone)]
pub struct TwitterMediator {
  secrets: TwitterSecrets,
}

impl TwitterMediator {
  /// CTOR.
  pub fn new(secrets: TwitterSecrets) -> TwitterMediator {
    TwitterMediator {
      secrets: secrets.clone(),
    }
  }

  /// Fetch the timeline for a given user.
  pub fn get_timeline(&self, username: &str, count: i32) ->
      Result<Vec<Tweet>, TwitterError> {

    let api_token = &self.secrets.api_token();
    let access_token = &self.secrets.access_token();
    let mut timeline = tweet::user_timeline(username, true, true, &api_token,
                                            &access_token)
        .with_page_size(count);

    let mut tweets = Vec::new();

    for tweet in &try!(timeline.start()).response {
      println!("{:?}", tweet);
      let t = Tweet {
        avatar: tweet.user.profile_image_url.clone(),
        created_at: tweet.created_at.clone(),
        name: tweet.user.name.clone(),
        text: tweet.text.clone(),
        username: tweet.user.screen_name.clone(),
      };
      tweets.push(t);
    }

    Ok(tweets)
  }
}

