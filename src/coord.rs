
use std::path::Path;

use tagger::Tagger;

pub struct CoordTagger {
    long: String,
    lat: String,
}


impl CoordTagger {
    pub fn new(arg: &str) -> CoordTagger {
        // parse

        let v: Vec<&str> = arg.split(',').collect();
        if v.len() == 2 {
            return CoordTagger{long: v[0].to_string(), lat: v[1].to_string()};
        }
        CoordTagger{long: String::default(), lat: String::default()}
    }
}

impl Tagger for CoordTagger {

    fn get_coord_for_file(&self, _file: &Path) -> (String, String) {
        (self.long.clone(), self.lat.clone())
    }
}


#[cfg(test)]
#[test]
fn test_parsing() {
    let coord = CoordTagger::new("12.34,-45.54");

    let coords = coord.get_coord_for_file(&Path::new(""));
    assert_eq!(coords, ("12.34".to_owned(), "-45.54".to_owned()));

    // Invalid data
    let coord = CoordTagger::new("12.34/-45.54");

    let coords = coord.get_coord_for_file(&Path::new(""));
    assert_eq!(coords, (String::default(), String::default()));
}
