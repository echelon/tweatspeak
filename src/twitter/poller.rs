// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use config::Config;
use std::collections::HashSet;
use std::sync::RwLockReadGuard;
use std::sync::Arc;
use std::sync::PoisonError;
use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;
use twitter::client::TwitterClient;
use twitter::tweet::Tweet;

pub type LockReadError<'a> = PoisonError<RwLockReadGuard<'a, Vec<Tweet>>>;

pub struct TwitterPoller {
  max_length: usize,
  sleep_duration: Duration,
  tweets: Arc<RwLock<Vec<Tweet>>>, // TODO: Better data structures.
  twitter_client: TwitterClient,
  usernames: Vec<String>,
}

impl TwitterPoller {

  pub fn new(client: TwitterClient, config: &Config) -> TwitterPoller {
    TwitterPoller {
      max_length: 50,
      sleep_duration: Duration::new(30, 0), // 30 seconds
      twitter_client: client,
      usernames: config.twitter_handles.clone().unwrap(), // TODO FIX
      tweets: Arc::new(RwLock::new(Vec::new())),
    }
  }

  /// Return a copy of the current tweets to the caller.
  pub fn get_tweets(&self) -> Result<Vec<Tweet>, LockReadError> {
    let existing = try!(self.tweets.read());
    Ok(existing.clone())
  }

  /// Meant to be launched in a separate thread.
  /// This will poll Twitter for new Tweets and insert them into the
  /// local feed.
  pub fn poll(&self) -> ! {
    loop {
      for username in &self.usernames {
        match self.twitter_client.get_timeline(username, 40) {
          Err(err) => {
            warn!("Error querying twitter handle '{}': {:?}", username, err);
          },
          Ok(tweets) => {
            self.update_tweets(tweets);
          },
        }

        // Don't spam the Twitter API!
        sleep(self.sleep_duration);
      }
    }
  }

  // TODO: This is all kinds of inefficient, but only happens on a
  // single thread, so isn't world-ending. I'm also not care about
  // any read/write races here.
  fn update_tweets(&self, tweets: Vec<Tweet>) {
    let existing = match self.tweets.read() {
      Err(_) => {
        warn!("Error updating tweets.");
        return;
      },
      Ok(existing) => { existing.clone() },
    };

    // FIXME: Forgive me. I'm a terrible person.
    let mut set = HashSet::new();

    // De-dup.
    for tweet in existing { set.insert(tweet); }
    for tweet in tweets { set.insert(tweet); }

    let mut sorted : Vec<Tweet> = Vec::new();
    for tweet in set.drain() { sorted.push(tweet); }
    sorted.sort_by(|a, b| a.cmp(b).reverse());

    // Truncate.
    sorted.truncate(self.max_length);

    // Replace.
    match self.tweets.write() {
      Err(_) => {
        // warn!("cannot write lock on tweets");
        return;
      },
      Ok(mut lock) => {
        *lock = sorted;
      }
    }
  }
}

