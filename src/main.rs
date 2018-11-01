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
  geotag [-t <tracklog> | -c <coords>] [--overwrite] <files>...

Options:
  -t <tracklog>  Use GPX tracklog.
  -c <coords>    GPS Coordinates to apply to all the files. x,y
  --overwrite    Overwrite existing geotag
  <files>        Image(s) to geotag. Will locate the XMP side car.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_t: Option<String>,
    flag_c: Option<String>,
    flag_overwrite: bool,
    arg_files: Vec<String>,
}

#[derive(Debug)]
enum GeoTagError {
    XmpError(exempi::Error),
    IoError(std::io::Error),
}

impl From<std::io::Error> for GeoTagError {
    fn from(err: std::io::Error) -> Self {
        GeoTagError::IoError(err)
    }
}

impl From<exempi::Error> for GeoTagError {
    fn from(err: exempi::Error) -> Self {
        GeoTagError::XmpError(err)
    }
}


type Result<T> = std::result::Result<T, GeoTagError>;

fn tag_file(tagger: &Tagger, file: &Path, overwrite: bool) -> Result<bool>
{
    if let Some(stem) = file.file_stem() {
        let mut xmp_file = file.with_file_name(stem);
        xmp_file.set_extension("xmp");

        let mut xmp = if xmp_file.exists() {
            let mut buf: Vec<u8> = vec![];
            let mut file = File::open(xmp_file.clone())?;
            let _r = file.read_to_end(&mut buf)?;

            Xmp::from_buffer(&buf)?
        } else {
            Xmp::new()
        };
        if !overwrite {
            if xmp.has_property("http://ns.adobe.com/exif/1.0/",
                                "GPSLatitude") ||
                xmp.has_property("http://ns.adobe.com/exif/1.0/",
                                 "GPSLongitude") {
                // already there, skip
                return Ok(false);
            }
        }
        let coords = tagger.get_coord_for_file(file);

        xmp.set_property(
            "http://ns.adobe.com/exif/1.0/", "GPSLatitude", &coords.0,
            exempi::PropFlags::empty())?;
        xmp.set_property(
            "http://ns.adobe.com/exif/1.0/", "GPSLongitude", &coords.1,
            exempi::PropFlags::empty())?;
        xmp.set_property(
            "http://ns.adobe.com/exif/1.0/", "GPSVersionID", "2.2.0.0",
            exempi::PropFlags::empty())?;
        xmp.set_property(
            "http://ns.adobe.com/exif/1.0/", "GPSAltitude", "0/10000",
            exempi::PropFlags::empty())?;

        let buf = xmp.serialize_and_format(
            exempi::SERIAL_OMITPACKETWRAPPER | exempi::SERIAL_USECOMPACTFORMAT,
            0, "\n", " ", 1)?;
        let mut file = File::create(xmp_file)?;
        let written = file.write(buf.to_str().as_bytes())?;
        if written < buf.len() {
            println!("Short write: {} of {} bytes written.",
                     written, buf.len());
        }
    }
    Ok(true)
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

    if !tagger.is_ok() {
        return;
    }
    //
    let current_dir = env::current_dir().ok().unwrap();
    for file in args.arg_files {

        let mut path = current_dir.clone();
        path.push(file);

        let r = tag_file(tagger.as_ref(), &path, args.flag_overwrite);
        println!("{:?}", r);
    }
}
