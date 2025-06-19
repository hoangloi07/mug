use lofty::file::TaggedFileExt;
#[allow(unused_imports)]
use lofty::tag::{Accessor, Tag};
use mug::PROP;

// #[derive(Clone)]
pub struct Track {
    tagged_file: lofty::file::TaggedFile,
    // pub tag: Option<&'a mut lofty::tag::Tag>,
    pub path: String,
}
impl Track {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let tagged_file = lofty::probe::read_from_path(path)?;
        Ok(Self {
            tagged_file,
            path: path.to_string(),
        })
    }
    pub fn tag_mut(&mut self) -> Option<&mut lofty::tag::Tag> {
        self.tagged_file
            .tag_mut(self.tagged_file.primary_tag_type())
    }
    pub fn tag(&self) -> Option<&lofty::tag::Tag> {
        self.tagged_file.tag(self.tagged_file.primary_tag_type())
    }
    pub fn edit(
        &mut self,
        prop: mug::PROP,
        edit_to: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tag = self.tag_mut().ok_or("No tag available")?;

        match prop {
            PROP::ARTIST => tag.set_artist(edit_to),
            PROP::TITLE => tag.set_title(edit_to),
            PROP::YEAR => {
                let year = edit_to.trim().parse()?;
                tag.set_year(year);
            }
            PROP::ALBUM => tag.set_album(edit_to),
            PROP::GENRE => tag.set_genre(edit_to),
            PROP::TRACK => {
                let track = edit_to.trim().parse()?;
                tag.set_track(track);
            }
        }

        Ok(())
    }
}
