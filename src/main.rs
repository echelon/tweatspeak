// Copyright (c) 2016 Brandon Thomas <bt@brand.io>

extern crate egg_mode;
extern crate rustc_serialize;
extern crate toml;

pub mod twitter;

use egg_mode::tweet;

fn main() {
  let secrets = twitter::TwitterSecrets::read_toml_file("./twitter_secrets.toml").unwrap();

  let consumer_token = egg_mode::Token::new(secrets.api_key, secrets.api_secret);

  let access_token = egg_mode::Token::new(
    secrets.access_token_key,
    secrets.access_token_secret
  );

  let mut timeline = tweet::user_timeline("echelon", true, true, &consumer_token, &access_token)
      .with_page_size(100);

  let mut i = 0;
  for tweet in &timeline.start().unwrap().response {
    println!("{} <@{}> {}", i, tweet.user.screen_name, tweet.text);
    i += 1;
  }
}

