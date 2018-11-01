use std::path::Path;

use tagger::Tagger;

pub struct TracklogTagger {}

impl TracklogTagger {
    pub fn new(_path: &str) -> TracklogTagger {
        TracklogTagger {}
    }
}

impl Tagger for TracklogTagger {
    fn get_coord_for_file(&self, _file: &Path) -> (String, String) {
        (String::default(), String::default())
    }

    fn is_ok(&self) -> bool {
        false
    }
}
