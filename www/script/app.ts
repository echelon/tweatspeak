import $ = require('jquery');

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
    $div.find('.text').html(tweet.text);

    $tweets.append($div);

    $div.on('click', (ev: JQueryEventObject) => {
      let $el = $(ev.target);
      sayText($el.text());
    });
  }
}

function sayText(text: string) {
  // TODO: Do this on the backend.
  let cleaned = text.trim()
      .replace(/&/, ' and ')
      .replace(/%/, ' percent ')
      .replace(/"/, '')
      .replace(/[\.\?!#,]/, ' ');
  console.log(cleaned);
}

$(function() {
  console.log('installing');

  $('form').submit((ev: JQueryEventObject) => {
    let username = $('input').val().trim();
    fetch(username);
    return false;
  });
});

