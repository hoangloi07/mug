use ::std::fs::File;
use ::std::io::BufReader;
use rodio::Decoder;
use rodio::OutputStreamHandle;

use crate::playlist;

pub struct Msink {
    pub sink: Option<rodio::Sink>,
}
impl Msink {
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        let sink = rodio::Sink::try_new(&stream_handle)
            .expect("Error while open sink");
        Msink { sink: Some(sink) }
    }
    pub fn play_append(&mut self, filename: &str) {
        match &mut self.sink {
            Some(sink) => {
                let filek = File::open(filename).unwrap();
                let file = BufReader::new(filek);
                let source = Decoder::new(file).unwrap();
                sink.append(source);
            }
            None => {
                println!("Can't connect to sink!")
            }
        }
    }
    pub fn playlist_load(
        &mut self,
        playlist: playlist::Playlist,
        first: usize,
        last: usize,
    ) {
        match &self.sink {
            Some(_) => {
                let pl = playlist.list.unwrap();
                for i in first..last {
                    self.play_append(&pl[i].path);
                }
            }
            None => {
                println!("Can't connect to sink!")
            }
        }
    }
    pub fn playlist_next(&mut self, mut playlist: playlist::Playlist) {
        match &self.sink {
            Some(sink) => {
                sink.skip_one();
                match playlist.next_track() {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
            None => {
                println!("Can't connect to sink!")
            }
        }
    }
    pub fn playlist_previous(&mut self, playlist: playlist::Playlist) {
        match &self.sink {
            Some(sink) => {
                sink.skip_one();
                sink.stop();
                let cur = playlist.current.unwrap();
                match playlist.list {
                    Some(ref pl) if !pl.is_empty() => {
                        let end = pl.len() - 1;
                        self.playlist_load(playlist, cur, end);
                    }
                    Some(_) => {
                        println!("playlist is fucking empty")
                    }
                    None => {
                        println!("playlist is not implemented!")
                    }
                }
            }
            None => {
                println!("Can't connect to sink!")
            }
        }
    }
}
