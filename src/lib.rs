#![allow(unused)]
pub const SOCKET_PATH: &str = "/tmp/mug.sock";
pub const TCP_PORT: &str = "127.0.0.1:8080";

pub enum PROP {
    ARTIST,
    TITLE,
    YEAR,
    ALBUM,
    GENRE,
    TRACK,
}

pub enum Signal {
    PLAY,
    PAUSE,
    STOP,
    NEXT,
    PREVIOUS,
}
