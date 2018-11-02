/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

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
