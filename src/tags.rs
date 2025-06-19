use lofty::tag::Accessor;

pub
struct TagInfo {
    pub artist: Option<String>,
    pub title : Option<String>, 
    pub album : Option<String>,
    pub genre : Option<String>,
    pub year  : Option<u32>,
    pub track : Option<u32>
}

impl TagInfo {
    pub fn new() -> Self {
        TagInfo {
            artist: None, title : None, year  : None, album : None, genre : None, track : None,
        }
    }
    pub fn get_info(&self, tag: lofty::tag::Tag) -> Self {
        let artist = match tag.artist() {
            Some(str) => Some(String::from(str)),
            None => None
        };
        let title = match tag.title() {
            Some(str) => Some(String::from(str)),
            None => None
        };
        let album = match tag.album() {
            Some(str) => Some(String::from(str)),
            None => None
        };
        let genre = match tag.genre() {
            Some(str) => Some(String::from(str)),
            None => None
        };
        let year  = tag.year();
        let track = tag.track();
        TagInfo {
            artist,title,year,album,genre,track
        }
    }
}
