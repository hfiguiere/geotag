/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::path::Path;

pub trait Tagger {
    fn get_coord_for_file(&self, file: &Path) -> (String, String);

    fn is_ok(&self) -> bool;
}
