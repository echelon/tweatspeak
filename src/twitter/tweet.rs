// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, Debug, Eq, PartialEq, RustcEncodable)]
pub struct Tweet {
  pub avatar: String,
  pub created_at: String,
  pub created_datetime: DateTime<FixedOffset>,
  pub id: i64,
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

impl Hash for Tweet {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}

