import $ = require('jquery');
import 'buzz';

window.$ = $;

function fetch(username: string) {
  console.log('Getting page for ', username);
  let url = '/tweets/user/' + username;
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
      sayText(sentence);
    });
  }
}

function getAudioServerBaseUrl(): string {
  let val = $('meta[name=audio_server_base_url]').attr('content');
  return val;
}

function sayText(sentence: string) {
  // TODO: Do this on the backend.
  let encoded = encodeURIComponent(sentence),
      speaker = 'trump',
      base = getAudioServerBaseUrl(),
      url = base
          + '/speak'
          + '?v=' + speaker
          + '&s=' + encoded
          + '&vol=3';

  let sound = new buzz.sound(url);
  window.sound = sound;
  sound.play();
}

$(function() {
  console.log('installing');

  $('form').submit((ev: JQueryEventObject) => {
    let username = $('input').val().trim();
    fetch(username);
    return false;
  });
});

