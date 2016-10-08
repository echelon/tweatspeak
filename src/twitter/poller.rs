// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use config::Config;
use std::collections::LinkedList;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;
use twitter::client::TwitterClient;
use twitter::tweet::Tweet;

pub struct TwitterPoller {
  sleep_duration: Duration,
  tweets: Arc<RwLock<LinkedList<Tweet>>>,
  twitter_client: TwitterClient,
  usernames: Vec<String>,
}

impl TwitterPoller {

  pub fn new(client: TwitterClient, config: &Config) -> TwitterPoller {
    TwitterPoller {
      sleep_duration: Duration::new(30, 0), // 30 seconds
      twitter_client: client,
      usernames: config.twitter_handles.clone().unwrap(), // TODO FIX
      tweets: Arc::new(RwLock::new(LinkedList::new())),
    }
  }

  pub fn poll(&self) -> ! {
    loop {
      for username in &self.usernames {
        match self.twitter_client.get_timeline(username, 40) {
          Err(_) => {},
          Ok(tweets) => {
            for tweet in tweets {
              println!("Tweet: {:?}", tweet);
            }
          },
        }

        // Don't spam the Twitter API!
        sleep(self.sleep_duration);
      }
    }
  }
}

