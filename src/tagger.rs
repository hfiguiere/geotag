
use std::path::Path;

pub trait Tagger {
    fn get_coord_for_file(&self, file: &Path) -> (String, String);

    fn is_ok(&self) -> bool;
}
