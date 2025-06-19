pub mod playlist;
pub mod sock;
pub mod tags;
pub mod track;

use ::rodio::{Decoder, OutputStream, Sink};
use ::std::fs::File;
use ::std::io::BufReader;
use console::Term;
#[allow(unused_imports)]
use lofty::file::{AudioFile, FileType, TaggedFileExt};
#[allow(unused_imports)]
use lofty::tag::{Accessor, Tag};
use std::time::Duration;
use track::Track;
pub fn play_append(filename: &str, sink: &Sink) {
    let filek = File::open(filename).unwrap();
    let file = BufReader::new(filek);

    let source = Decoder::new(file).unwrap();
    sink.append(source);
    // (())
}

pub fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    let path = "HowtoDisapearCompletely.flac";
    let path1 = "Pink+White.flac";
    // let cl = Cow::Borrowed(&fprop);
    // fp.set_artist(String::from("RadioCat"));
    // println!("{}", fp.artist().unwrap());
    let t = Track::new(path).unwrap();
    let t1 = Track::new(path1).unwrap();
    let mut k: playlist::Playlist = playlist::Playlist {
        list: Some(vec![t, t1]),
        next: Some(1),
        current: Some(0),
        previous: None,
    };
    // k.remove_from_list(0);
    // loop {

    sink.play();
    if let Some(list) = k.list.take() {
        k.list = Some(list);
        let term = Term::stdout();
        while let Some(curr_idx) = k.current {
            if let Some(ref tracks) = k.list {
                let current_track = &tracks[curr_idx];

                play_append(&tracks[curr_idx].path, &sink);

                match current_track.tag().and_then(|t| t.title()) {
                    Some(title) => {
                        println!("Now playing: {}", title);
                        loop {
                            let key = term.read_char().unwrap();
                            println!("You pressed: {}", key);
                            match key {
                                ' ' => {
                                    if sink.is_paused() {
                                        sink.play();
                                    } else {
                                        sink.pause();
                                    }
                                }
                                'n' => {
                                    sink.skip_one();

                                    match k.next_track() {
                                        Ok(_) => {}
                                        Err(_) => {}
                                    }
                                    break;
                                }
                                'p' => {
                                    sink.stop();
                                    // sink.clear();
                                    k.current = Some(curr_idx - 1);
                                    // println!("{}", k.current.unwrap());
                                    break;
                                }
                                'l' => {
                                    match sink.try_seek(Duration::from_secs(5))
                                    {
                                        Ok(_) => {}
                                        Err(_) => match k.next_track() {
                                            Ok(_) => {}
                                            Err(_) => {}
                                        },
                                    }
                                }
                                // 'b' =>
                                _ => {}
                            }
                        }
                    }
                    None => println!("Now playing: untitled"),
                }
                // println!("{}", k.current.unwrap());
            }
        }
        println!("End of playlist")
    }
    // }
    //
    // play_append("Pink+White.flac", &sink);
    // play_append("HowtoDisapearCompletely.flac", &sink);
    //
    // sink.play();
    //
    // }
}
