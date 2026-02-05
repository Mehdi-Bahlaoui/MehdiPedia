// Music Player Database
// Structured with lyrics and annotations

const musicData = {
  'welcome-to-rap-battle': {
    id: 'welcome-to-rap-battle',
    title: 'Welcome to Rap-Battle',
    artist: 'Mehdi Bahlaoui',
    album: 'Singles',
    releaseDate: '2026',
    audioUrl: 'assets/1/Welcome to Rap-Battle.mp3',
    videoUrl: 'assets/1/Welcome to Rap-Battle.mp4',
    coverArt: '../../Images/boss.jpg',
    duration: 123,

    // Lyrics: each line has a dur (duration in seconds)
    // startTime is computed automatically: previous line's start + previous line's dur
    // Change one dur and everything after it shifts accordingly
    lyrics: [
      { id: 'line-1',  dur: 9,    text: '[Intro]', annotations: [] },
      { id: 'line-2',  dur: 1.5,  text: '(Okay, shee~', annotations: [] },
      { id: 'line-3',  dur: 2.3,    text: 'Survivors', annotations: ['ann-1'] },
      { id: 'line-4',  dur: 2.5,  text: 'Clueless by the Zillions', annotations: [] },
      { id: 'line-5',  dur: 1.75,    text: 'They all act like in diapers', annotations: [] },
      { id: 'line-6',  dur: 1,    text: 'They show you smiles', annotations: [] },
      { id: 'line-7',  dur: 3.75,  text: 'but would switch for an affiliate', annotations: [] },
      { id: 'line-8',  dur: 1.5,  text: 'Survivors', annotations: [] },
      { id: 'line-9',  dur: 3,  text: 'Would fake death with vermillion', annotations: [] },
      { id: 'line-10', dur: 2,  text: 'Really got saved this time', annotations: [] },
      { id: 'line-11', dur: 3,    text: 'so I believe we should vote Aya for rebellion', annotations: [] },
      { id: 'line-12', dur: 1.3,  text: 'Mmmmh..', annotations: [] },
      { id: 'line-13', dur: 0.75,  text: 'yeah!', annotations: [] },
      { id: 'line-14', dur: 2.75,    text: 'When I drop a new song I check Instagram for likes', annotations: [] },
      { id: 'line-15', dur: 2.5,    text: "But I don't fancy having social media", annotations: [] },
      { id: 'line-16', dur: 2.5,    text: "it's just for personal reasons that I can't cite", annotations: [] },
      { id: 'line-17', dur: 2.75, text: "If you're liking this check my Wikipedia", annotations: [] },
      { id: 'line-19', dur: 2.75,  text: 'Vice-président GDE does shit he likes', annotations: [] },
      { id: 'line-20', dur: 2.8,  text: "He might be shy but he'll speaks his mind immedia-", annotations: [] },
      { id: 'line-21', dur: 0.28,  text: '-ately', annotations: [] },
      { id: 'line-22', dur: 2.5,  text: 'And if he grows his hair to not look alike', annotations: [] },
      { id: 'line-23', dur: 2,    text: 'His friends are the type to change a @#$%&', annotations: [] },
      { id: 'line-24', dur: 0.5,    text: 'to Sicilia', annotations: [] },
      { id: 'line-25', dur: 1,    text: 'um Sicilia', annotations: [] },
      { id: 'line-26', dur: 2,    text: 'or all the way to Siberia', annotations: [] },
      { id: 'line-27', dur: 1,    text: 'Not tryna be rude', annotations: [] },
      { id: 'line-28', dur: 0.7,    text: "you're like Hamlet", annotations: [] },

      { id: 'line-29', dur: 1,    text: "she's Ophelia", annotations: [] },
      { id: 'line-30', dur: 1.85,    text: 'When I went to Marrakech to film', annotations: [] },
      { id: 'line-31', dur: 1,  text: 'she was furious', annotations: [] },
      { id: 'line-32', dur: 1.75,  text: 'Martin Luther King the type of @#$%&', annotations: [] },
      { id: 'line-33', dur: 1.25,  text: 'who would beat her up', annotations: [] },
      { id: 'line-34', dur: 0.7,    text: 'Sicilia', annotations: [] },
      { id: 'line-35', dur: 2,    text: 'or all the way to Siberia', annotations: [] },
      { id: 'line-36', dur: 1,    text: 'Not tryna be rude', annotations: [] },
      { id: 'line-37', dur: 0.7,    text: "you're like Hamlet", annotations: [] },
      { id: 'line-38', dur: 1,    text: "she's Ophelia", annotations: [] },
      { id: 'line-39', dur: 1.8,    text: 'When I went to Marrakech to film', annotations: [] },
      { id: 'line-40', dur: 1,  text: 'she was furious', annotations: [] },
      { id: 'line-41', dur: 1.75,  text: "Let's see if she'll mention me now", annotations: [] },
      { id: 'line-42', dur: 3,  text: 'with no fear in her', annotations: [] },
      { id: 'line-43', dur: 2.5,  text: 'with no fear in her~', annotations: [] },
      { id: 'line-44', dur: 3.65,  text: "She's not serious~", annotations: [] },
      { id: 'line-45', dur: 1.5,  text: 'Oh', annotations: [] },
    ],

    // Sample annotations - examples showing different styles
    // You can edit the content field with your actual annotations
    annotations: {
      'ann-1': {
        id: 'ann-1',
        lineId: 'line-3',
        author: 'Mehdi Bahlaoui',
        date: '2026-01-28',
        verified: false,
        content: 'This opening line refers to a game that that I played with some of my friends, where the players are called survivors, and the rule of the game requires that one person should be voted out each round.</p>',
        votes: 0
      },
      // 'ann-2': {
      //   id: 'ann-2',
      //   lineId: 'line-2',
      //   author: 'Mehdi Bahlaoui',
      //   date: '2026-01-28',
      //   verified: true,
      //   content: '<p>Here I transition into the main theme, establishing the narrative voice and perspective.</p>',
      //   votes: 0
      // },
      'ann-3': {
        id: 'ann-3',
        lineId: 'line-4',
        author: 'Mehdi Bahlaoui',
        date: '2026-01-28',
        verified: false,
        content: '<p>This line is a reference to classic hip-hop. The wordplay here is clever and multilayered. JUst kidding hh</p>',
        votes: 0
      }
    }
  },

  // 'rapbattle-ensamr': {
  //   id: 'rapbattle-ensamr',
  //   title: 'This video was brought to you by- RapBattle EnsamR',
  //   artist: 'Mehdi Bahlaoui',
  //   album: 'Singles',
  //   releaseDate: '2026',
  //   audioUrl: '../assets/2/This video was brought to you by- RapBattle EnsamR.mp3',
  //   videoUrl: '../assets/2/This video was brought to you by- RapBattle EnsamR.mp4',
  //   coverArt: '../images/boss.jpg',
  //   duration: 0,
  //   lyrics: [],
  //   annotations: {}
  // }
};
