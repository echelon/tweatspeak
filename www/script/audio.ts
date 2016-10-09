import 'buzz';

export default class AudioPlayer {
  sound: any;

  constructor() {
    this.sound = null;
  }

  sayText(sentence: string) {
    let encoded = encodeURIComponent(sentence),
        speaker = 'trump',
        baseUrl = this.getAudioServerBaseUrl(),
        url = baseUrl
            + '/speak'
            + '?v=' + speaker
            + '&s=' + encoded
            + '&vol=3';

    if (this.sound != null) {
      this.sound.stop();
    }

    console.log(`Say: ${sentence}`);
    this.sound = new buzz.sound(url);
    this.sound.play();
  }

  getAudioServerBaseUrl(): string {
    let val = $('meta[name=audio_server_base_url]').attr('content');
    return val;
  }
}

