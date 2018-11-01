extern crate exempi;
extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod coord;
mod tagger;
mod tracklog;

use std::fs;
use std::path::Path;

use coord::CoordTagger;
use docopt::Docopt;
use tagger::Tagger;
use tracklog::TracklogTagger;

const USAGE: &str = "
Usage:
  geotag [-t <tracklog> | -c <coords>] [-f <file> | -d <dir>]

Options:
  -t <tracklog>  Use GPX tracklog.
  -c <coords>    GPS Coordinates to apply to all the files. x,y
  -f <file>      Image to geotag. Will locate the XMP side car.
  -d <dir>       Directory to geotag.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_t: Option<String>,
    flag_c: Option<String>,
    flag_f: Option<String>,
    flag_d: Option<String>,
}

fn tag_file(tagger: &Tagger, file: &Path)
{
    

    let _coords = tagger.get_coord_for_file(file);

}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(std::env::args()).deserialize())
        .unwrap_or_else(|e| e.exit());

    let tagger: Box<Tagger>;
    if let Some(tracklog) = args.flag_t {
        tagger = Box::new(TracklogTagger::new(&tracklog));
    } else if let Some(coord) = args.flag_c {
        tagger = Box::new(CoordTagger::new(&coord));
    } else {
        // ERROR
        return;
    }

    //
    if let Some(file) = args.flag_f {
        let path = Path::new(&file);
        tag_file(tagger.as_ref(), &path);
    } else if let Some(dir) = args.flag_d {
        let dir_content = fs::read_dir(dir).unwrap();

        for entry in dir_content {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        tag_file(tagger.as_ref(), &entry.path());
                    }
                }
            }
        }
    }
}
