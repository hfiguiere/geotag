extern crate exempi;
extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod coord;
mod tagger;
mod tracklog;

use std::env;
use std::fs::{File};
use std::io::{Read,Write};
use std::path::Path;

use exempi::Xmp;

use coord::CoordTagger;
use docopt::Docopt;
use tagger::Tagger;
use tracklog::TracklogTagger;

const USAGE: &str = "
Usage:
  geotag [-t <tracklog> | -c <coords>] <files>...

Options:
  -t <tracklog>  Use GPX tracklog.
  -c <coords>    GPS Coordinates to apply to all the files. x,y
  <files>        Image(s) to geotag. Will locate the XMP side car.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_t: Option<String>,
    flag_c: Option<String>,
    arg_files: Vec<String>,
}

fn tag_file(tagger: &Tagger, file: &Path)
{
    if let Some(stem) = file.file_stem() {
        let mut xmp_file = file.with_file_name(stem);
        xmp_file.set_extension("xmp");

        let mut xmp;
        if xmp_file.exists() {
            let mut buf: Vec<u8> = vec![];
            if let Ok(mut file) = File::open(xmp_file.clone()) {
                let r = file.read_to_end(&mut buf);
            }
            xmp = Xmp::from_buffer(&buf).unwrap();
        } else {
            xmp = Xmp::new();
        }
        let mut props = exempi::PropFlags::empty();
        let result = xmp.get_property(
            "http://ns.adobe.com/exif/1.0/",
            "GPSLatitude", &mut props);
        if result.is_ok() {
            // already there, skip
            // XXX allow overriding
            return;
        }
        let mut props = exempi::PropFlags::empty();
        let result = xmp.get_property(
            "http://ns.adobe.com/exif/1.0/", "GPSLongitude", &mut props);
        if result.is_ok() {
            // already there, skip
            // XXX allow overriding
            return;
        }

        let coords = tagger.get_coord_for_file(file);

        let result = xmp.set_property(
            "http://ns.adobe.com/exif/1.0/", "GPSLatitude", &coords.0,
            exempi::PropFlags::empty());
        let result = xmp.set_property(
            "http://ns.adobe.com/exif/1.0/", "GPSLongitude", &coords.1,
            exempi::PropFlags::empty());

        let result = xmp.serialize_and_format(
            exempi::SerialFlags::empty(), 0, "\n", " ", 1);
        if let Ok(buf) = result {
            if let Ok(mut file) = File::create(xmp_file) {
                file.write(buf.to_str().as_bytes());
            }
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(std::env::args()).deserialize())
        .unwrap_or_else(|e| e.exit());

    exempi::init();

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
    let current_dir = env::current_dir().ok().unwrap();
    for file in args.arg_files {

        let mut path = current_dir.clone();
        path.push(file);

        tag_file(tagger.as_ref(), &path);
    }
}
