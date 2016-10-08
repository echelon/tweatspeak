// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use resolve::hostname;
use rustc_serialize::json::Json;
use rustc_serialize::json;
use std::convert::From;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io;
use toml;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Config {
  /// Base URL where we fetch audio files, eg. 'http://server:1000'.
  pub audio_server_base_url: Option<String>,
  pub twitter_handles: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum ConfigError {
  /// Wraps an IO error.
  IoError { cause: io::Error },
  /// TOML parsing error.
  TomlError,
  /// Configs absent.
  TomlMissing,
}

impl Config {
  /// Static CTOR.
  /// Read a [default] set of configs and merge with a ["$hostname"] set.
  pub fn read(filename: &str) -> Result<Config, ConfigError> {
    let contents = try!(read_file(filename));

    let table = match toml::Parser::new(&contents).parse() {
      None => { return Err(ConfigError::TomlError); },
      Some(table) => table,
    };

    let system_hostname = hostname::get_hostname().ok();

    let maybe_host = if system_hostname.is_some() {
      table.get(&system_hostname.unwrap())
          .and_then(|t| toml::decode::<Config>(t.clone()))
    } else {
      None
    };

    let maybe_default = table.get("default")
        .and_then(|t| toml::decode::<Config>(t.clone()));

    if maybe_default.is_none() {
      if maybe_host.is_none() {
        return Err(ConfigError::TomlMissing);
      } else {
        return Ok(maybe_host.unwrap());
      }
    }

    let default = maybe_default.unwrap();

    if maybe_host.is_some() {
      let host = maybe_host.unwrap();
      Ok(host.merge(default))
    } else {
      Ok(default)
    }
  }

  /// Merge another config object, keeping current values where they
  /// exist, and overriding Optional values where they do not.
  pub fn merge(&self, other: Config) -> Config {
    Config {
      audio_server_base_url: self.audio_server_base_url
          .clone()
          .or(other.audio_server_base_url.clone()),
      twitter_handles: self.twitter_handles
          .clone()
          .or(other.twitter_handles.clone()),
    }
  }
}

/// Necessary trait for binding Handlebars variables.
impl json::ToJson for Config {
  fn to_json(&self) -> Json {
    // FIXME: There's probably a more elegant way to do this than
    // encoding as a JSON string literal, then parsing the string.
    let encoded = json::encode(&self).unwrap_or("{}".to_string());
    Json::from_str(&encoded).unwrap_or(Json::Null)
  }
}

impl From<io::Error> for ConfigError {
  fn from(error: io::Error) -> ConfigError {
    ConfigError::IoError { cause: error }
  }
}

fn read_file(filename: &str) -> Result<String, Error> {
  let mut file = try!(File::open(filename));
  let mut buf = String::new();
  try!(file.read_to_string(&mut buf));
  Ok(buf)
}

