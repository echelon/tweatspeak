// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use chrono::datetime::DateTime;
use egg_mode::Token;
use egg_mode::tweet;
use egg_mode;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use toml;
use twitter::tweet::Tweet;

/// Twitter 'created_at' timestamp format.
static TIMESTAMP_FORMAT: &'static str = "%a %b %d %H:%M:%S %z %Y";

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

#[derive(Clone)]
pub struct TwitterClient {
  secrets: TwitterSecrets,
}

impl TwitterClient {
  /// CTOR.
  pub fn new(secrets: TwitterSecrets) -> TwitterClient {
    TwitterClient {
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
      // Format: Fri Oct 07 02:18:06 +0000 2016
      // Fri (%a) Oct (%b) 07 (%d) 02 (%H) :18 (%M) :06 (%S) +0000 (%z) 2016 (%Y)
      let datetime = DateTime::parse_from_str(&tweet.created_at,
                                              TIMESTAMP_FORMAT);
      if datetime.is_err() {
        //warn!("Could not parse date: {}", tweet.created_at);
        println!("Can't format date: `{}`", tweet.created_at);
        continue;
      }

      let t = Tweet {
        avatar: tweet.user.profile_image_url.clone(),
        created_at: tweet.created_at.clone(),
        created_datetime: datetime.unwrap(),
        id: tweet.id,
        name: tweet.user.name.clone(),
        text: tweet.text.clone(),
        username: tweet.user.screen_name.clone(),
      };
      tweets.push(t);
    }

    Ok(tweets)
  }
}

#[cfg(test)]
mod tests {
  use chrono::datetime::DateTime;
  use super::TIMESTAMP_FORMAT;

  #[test]
  fn test_twitter_date_format() {
    let timestamp = "Fri Oct 07 02:18:06 +0000 2016";
    let datetime = DateTime::parse_from_str(timestamp, TIMESTAMP_FORMAT);
    assert_eq!(true, datetime.is_ok());
  }
}


