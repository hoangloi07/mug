use crate::track::Track;
/// Playlist ///

pub struct Playlist {
    pub list: Option<Vec<Track>>,
    pub next: Option<usize>,
    pub previous: Option<usize>,
    pub current: Option<usize>,
}
impl Playlist {
    pub
    fn new() -> Self {
        Playlist {
            list: None,
            next: None,
            current: None,
            previous: None,
        }
    }
    pub fn next_track(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match &mut self.list {
            Some(tracks) if !tracks.is_empty() => {
                let next_pos = match self.current {
                    Some(current) if current < tracks.len() - 1 => current + 1,
                    Some(_) => 0,
                    None => 0,
                };
                self.previous = self.current;
                if next_pos == 0 {
                    self.current = None
                } else {
                    self.current = Some(next_pos)
                }
                self.next = if next_pos < tracks.len() - 1 {
                    Some(next_pos + 1)
                } else {
                    None
                };

                Ok(())
            },
            Some(_) => Err("Playlist is empty".into()),
            None => Err("No playlist loaded".into()),
        }
    }
    pub
    fn add_to_list(&mut self, track: Track) {
        match &mut self.list {
            Some(vec) => vec.push(track),
            None => self.list = Some(vec![track]),
        }
    }
    pub
    fn remove_from_list(&mut self, index: usize) -> Option<Track> {
        match &mut self.list {
            Some(vec) => {
                // Check if index is out of bounds
                if index >= vec.len() {
                    return None;
                }
                // Remove the track
                let removed_track = vec.swap_remove(index);
                // Update state if we're removing the current track
                if Some(index) == self.current {
                    if vec.is_empty() {
                        // Playlist is now empty
                        self.current = None;
                        self.previous = None;
                        self.next = None;
                    } else if index >= vec.len() {
                        // Removed the last track
                        self.current = Some(vec.len() - 1);
                        self.next = None;
                        self.previous = if vec.len() > 1 {
                            Some(vec.len() - 2)
                        } else {
                            None
                        };
                    } else {
                        // Still tracks remaining after current
                        self.next = if index + 1 < vec.len() {
                            Some(index + 1)
                        } else {
                            None
                        };
                        // Previous stays the same unless we removed the first track
                        if index == 0 {
                            self.previous = None;
                        }
                    }
                }
                // Update all references greater than the removed index
                if let Some(ref mut curr) = self.current {
                    if *curr > index {
                        *curr -= 1;
                    }
                }
                if let Some(ref mut prev) = self.previous {
                    if *prev > index {
                        *prev -= 1;
                    }
                }
                if let Some(ref mut next) = self.next {
                    if *next > index {
                        *next -= 1;
                    }
                }
                Some(removed_track)
            }
            None => None,
        }
        // pub fn
    }
}
