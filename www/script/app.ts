import $ = require('jquery');
//import 'buzz';

import AudioPlayer from "./audio";

window.$ = $;
let audio = new AudioPlayer();

function fetch(username: string) {
  console.log('Getting page for ', username);
  let url = '/tweets/user/' + username;
  $.get(url, (data: any) => {
    setTweets(data);
  });
}

function fetchPoller() {
  console.info('Getting feed');
  let url = '/poller';
  $.get(url, (data: any) => {
    setTweets(data);
  });
}

function setTweets(tweets: [any]) {
  let $tweets = $('#tweets');
  $tweets.empty();

  for (let tweet of tweets) {
    let klass = 'tweet-template';
    let $div = $('.' + klass).clone().removeClass(klass);

    $div.find('.username').html(tweet.username);
    $div.find('.name').html(tweet.name);
    $div.find('.text').html(tweet.text);
    $div.find('.avatar').css('background-image', `url(${tweet.avatar})`);

    $tweets.append($div);

    $div.on('click', (ev: JQueryEventObject) => {
      let $el= $(ev.target),
          $tweet = $el.closest('.tweet'),
          $text = $tweet.find('.text'),
          sentence = $text.text().trim();
      audio.sayText(sentence);
    });
  }
}

function installForm() {
  $('form').submit((ev: JQueryEventObject) => {
    let username = $('input').val().trim();
    fetch(username);
    return false;
  });
}

$(function() {
  console.info('Installing');
  $(() => {
    fetchPoller();
  });
});

