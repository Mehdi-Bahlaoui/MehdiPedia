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
      { id: 'line-1',  dur: 9,    text: '', annotations: [] },
      { id: 'line-2',  dur: 1.5,  text: '(Okay, shee~', annotations: [] },
      { id: 'line-3',  dur: 2.3,    text: 'Survivors', annotations: ['ann-1'] },
      { id: 'line-4',  dur: 2.5,  text: 'Clueless by the Zillions', annotations: [] },
      { id: 'line-5',  dur: 1.75,    text: 'They all act like in diapers', annotations: [] },
      { id: 'line-6',  dur: 1,    text: 'They show you smiles', annotations: [] },
      { id: 'line-7',  dur: 3.25,  text: 'but would switch for an affiliate', annotations: [] },
            { id: 'line-7-1',  dur: 0.5,  text: '', annotations: [] },

      { id: 'line-8',  dur: 1.5,  text: 'Survivors', annotations: [] },
      { id: 'line-9',  dur: 3,  text: 'Would fake death with vermillion', annotations: [] },
      { id: 'line-10', dur: 1.8,  text: 'Really got saved this time', annotations: [] },
      { id: 'line-11', dur: 3,    text: 'so I believe we should vote Aya for rebellion', annotations: [] },
      { id: 'line-12', dur: 1.5,  text: 'Mmmmh..', annotations: [] },
      { id: 'line-13', dur: 0.50,  text: 'yeah!', annotations: [] },
      { id: 'line-13-1', dur: 0.2,  text: '', annotations: [] },

      { id: 'line-14', dur: 2.75,    text: 'When I drop a new song I check Instagram for likes', annotations: [] },
      { id: 'line-15', dur: 2.5,    text: "But I don't fancy having social media", annotations: [] },
      { id: 'line-16', dur: 2.5,    text: "it's just for personal reasons that I can't cite", annotations: [] },
      { id: 'line-17', dur: 2.75, text: "If you're liking this check my Wikipedia", annotations: [] },
      { id: 'line-19', dur: 2.75,  text: 'Vice-président GDE does shit he likes', annotations: [] },
      { id: 'line-20', dur: 2.8,  text: "He might be shy but he'll speaks his mind immedia-", annotations: [] },
      { id: 'line-21', dur: 0.28,  text: '-ately', annotations: [] },
      { id: 'line-22', dur: 2.5,  text: 'And if he grows his hair to not look alike', annotations: [] },
      { id: 'line-23', dur: 2,    text: 'His friends are the type to change a @#$%&', annotations: [] },
      { id: 'line-24', dur: 0.7,    text: 'to Sicilia', annotations: [] },
      { id: 'line-25', dur: 1,    text: 'um Sicilia', annotations: [] },
      { id: 'line-26', dur: 2,    text: 'or all the way to Siberia', annotations: [] },
      { id: 'line-27', dur: 1,    text: 'Not tryna be rude', annotations: [] },
      { id: 'line-28', dur: 0.7,    text: "you're like Hamlet", annotations: [] },

      { id: 'line-29', dur: 1,    text: "she's Ophelia", annotations: [] },
      { id: 'line-30', dur: 1.85,    text: 'When I went to Marrakech to film', annotations: [] },
      { id: 'line-31', dur: 1,  text: 'she was furious', annotations: [] },
      { id: 'line-32', dur: 1.75,  text: 'Martin Luther King the type of @#$%&', annotations: [] },
      { id: 'line-33', dur: 1.25,  text: 'who would beat her up', annotations: [] },
      // { id: 'line-33-1', dur: 0.2,  text: '', annotations: [] },

      { id: 'line-34', dur: 0.5,    text: 'Sicilia', annotations: [] },
      { id: 'line-35', dur: 2,    text: 'or all the way to Siberia', annotations: [] },
      { id: 'line-36', dur: 1,    text: 'Not tryna be rude', annotations: [] },
      { id: 'line-37', dur: 0.7,    text: "you're like Hamlet", annotations: [] },
      { id: 'line-38', dur: 1,    text: "she's Ophelia", annotations: [] },
      { id: 'line-39', dur: 1.8,    text: 'When I went to Marrakech to film', annotations: [] },
      { id: 'line-40', dur: 1,  text: 'she was furious', annotations: [] },
      { id: 'line-41', dur: 1.75,  text: "Let's see if she'll mention me now", annotations: [] },
      { id: 'line-42', dur: 1.5,  text: 'with no fear in her', annotations: [] },
            { id: 'line-42-1', dur: 1.5,  text: '', annotations: [] },

      { id: 'line-43', dur: 2.5,  text: 'with no fear in her~', annotations: [] },
      { id: 'line-44', dur: 3.65,  text: "She's not serious~ (Oh) ", annotations: [] },
 
      { id: 'line-45', dur: 10.9,     text: "Lmouchkil l7a9i9i dyal l'2insania, dyal bnadem, houa fach kisewel chkoun ana?", annotations: [] },
      { id: 'line-45-1', dur: 3,     text: "fach chkoun ghadi ijaweb?", annotations: [] },

      { id: 'line-46', dur: 1.6,     text: "This time, it's all about me", annotations: [] },
      { id: 'line-47', dur: 0.2, text: "(mouchkila)", annotations: []},
      { id: 'line-48', dur: 0.6,     text: 'You can wait,', annotations: [] },

      { id: 'line-48-2', dur: 0.8,     text: 'you are next-time', annotations: [] },


      { id: 'line-49', dur: 1,   text: "don't wanna be seen,", annotations: [] },
      { id: 'line-50', dur: 0.8,     text: "with a homie", annotations: [] },
            { id: 'line-50-1', dur: 1,     text: "who's a waste man", annotations: [] },

      { id: 'line-51', dur: 2.8,  text: "I'm cleaning up the scene with the fakes and the yes-man", annotations: [] },
      { id: 'line-52', dur: 2.5,   text: "it don't matter the time or the place, man", annotations: [] },

      { id: 'line-53', dur: 3,     text: 'This time, I get up one knew in the morning just to pray five', annotations: [] },
      { id: 'line-54', dur: 2.5,   text: 'Ait l\'Archine is a @#$%& you can count on', annotations: [] },
      { id: 'line-55', dur: 2.75,  text: 'Shout-out to the delegates last year for the best time', annotations: [] },
      { id: 'line-56', dur: 2,     text: 'even tho lmadrassa is a mess,', annotations: [] },
      { id: 'line-57', dur: 0.5,     text: 'sooo', annotations: [] },

      { id: 'line-58', dur: 2.8,     text: "Clashed the ADE 'bout 3 times! they seem dead right?", annotations: [] },
      { id: 'line-59', dur: 2.7,  text: 'Now they all up in my free time, for a headstart', annotations: [] },
      { id: 'line-60', dur: 2.7,     text: "Can't put the blame on the whole team, when there is none", annotations: [] },
      { id: 'line-61', dur: 2.5,   text: "No longer in their position, I mean that's life", annotations: [] },

      { id: 'line-62', dur: 2.9,     text: "Clashed the ADE 'bout 3 times! they seem dead right?", annotations: [] },
      { id: 'line-63', dur: 1.3,     text: '(OH noo), @#$%&', annotations: [] },
      { id: 'line-65', dur: 1.3,     text: '(Oh noo)', annotations: [] },
      { id: 'line-66', dur: 0.8,     text: '(Oh noo)', annotations: [] },
      { id: 'line-67', dur: 1.25,  text: 'yeah (nah nah)', annotations: [] },

      { id: 'line-69', dur: 1.5,     text: 'like 3 times just to act right!', annotations: [] },
            { id: 'line-69-1', dur: 0.6,     text: '@#$%&', annotations: [] },

      { id: 'line-70', dur: 2,     text: '3 times just to doo sum...', annotations: [] },

      { id: 'line-71', dur: 0.35,  text: 'yeah', annotations: [] },
      { id: 'line-72', dur: 1.85,   text: 'I\'ve been feeling down, can you doo sum?', annotations: [] },
      { id: 'line-73', dur: 0.85,   text: '(3 times! yeah )', annotations: [] },
      { id: 'line-74', dur: 1,  text: "When I'm not around", annotations: [] },
      { id: 'line-75', dur: 0.85,   text: 'and you doo sum', annotations: [] },
      { id: 'line-76', dur: 0.9,   text: '(3 times! yeah )', annotations: [] },

      { id: 'line-77', dur: 1.35,  text: 'Never wanna get back', annotations: [] },
      { id: 'line-78', dur: 1.35,  text: 'this wil be a set-back', annotations: [] },
      { id: 'line-79', dur: 2,   text: 'And if you wanna get back I just moove on', annotations: [] },

      { id: 'line-80', dur: 2.5,   text: 'Do you wanna add the 4th time, just to ..', annotations: [] },
      { id: 'line-81', dur: 3,     text: "Do you wanna add the 4th time you won't wake up anymoore!", annotations: [] },
      { id: 'line-82', dur: 2.35,   text: 'Do you wanna add the 4th time, just to ..', annotations: [] },

      { id: 'line-83', dur: 1.3,   text: "I don't think so tho,", annotations: [] },
      { id: 'line-84', dur: 2,     text: 'so I will spare your liife!', annotations: [] },

      { id: 'gap-1', dur: 0.8, text: '', annotations: [] },

      { id: 'line-85', dur: 3.5,     text: '(De7ekni khoun gal lia ADE is bullshit hhhh)', annotations: [] },
      { id: 'line-85-1', dur: 3,     text: '(wtff hhhh)', annotations: [] },

      { id: 'line-86', dur: 2.5,   text: 'Layhdik a sa7bi rak men lADE!', annotations: [] },
            { id: 'line-86-2', dur: 1.5,   text: 'shit!', annotations: [] },
      { id: 'gap-2', dur: 6, text: '', annotations: [] },

      { id: 'line-87', dur: 2,     text: 'Wa lADE fi9o m3ana', annotations: [] },
      { id: 'line-88', dur: 2,  text: 'waaaa makainch De7ek', annotations: [] },
      { id: 'line-89', dur: 2,     text: '9te3 laydir lkhir', annotations: [] },
            { id: 'line-90', dur: 1,     text: '', annotations: [] },




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
