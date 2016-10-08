// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq, RustcEncodable)]
pub struct Tweet {
  pub avatar: String,
  pub created_at: String,
  pub created_datetime: DateTime<FixedOffset>,
  pub name: String,
  pub text: String,
  pub username: String,
}

impl Ord for Tweet {
  fn cmp(&self, other: &Tweet) -> Ordering {
    self.created_datetime.cmp(&other.created_datetime)
  }
}

impl PartialOrd for Tweet {
  fn partial_cmp(&self, other: &Tweet) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

