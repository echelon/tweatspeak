// Copyright (c) 2016 Brandon Thomas <bt@brand.io>

use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use toml;

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
}

