/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate exempi;

mod coord;
mod tagger;
mod tracklog;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use exempi::Xmp;

pub use coord::CoordTagger;
pub use tagger::Tagger;
pub use tracklog::TracklogTagger;

/// Geotag error: either IO or XMP.
#[derive(Debug)]
pub enum GeoTagError {
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

/// Tag a file using tagger.
///
/// Return a result with bool value indicating if the file was
/// tagged or not. A false value indicate the geotag wasn't overwritten.
pub fn tag_file(tagger: &Tagger, file: &Path, overwrite: bool) -> Result<bool> {
    exempi::init();

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
        if !overwrite
            && (xmp.has_property("http://ns.adobe.com/exif/1.0/", "GPSLatitude")
                || xmp.has_property("http://ns.adobe.com/exif/1.0/", "GPSLongitude"))
        {
            // already there, skip
            return Ok(false);
        }
        let coords = tagger.get_coord_for_file(file);

        xmp.set_property(
            "http://ns.adobe.com/exif/1.0/",
            "GPSLatitude",
            &coords.0,
            exempi::PropFlags::empty(),
        )?;
        xmp.set_property(
            "http://ns.adobe.com/exif/1.0/",
            "GPSLongitude",
            &coords.1,
            exempi::PropFlags::empty(),
        )?;
        xmp.set_property(
            "http://ns.adobe.com/exif/1.0/",
            "GPSVersionID",
            "2.2.0.0",
            exempi::PropFlags::empty(),
        )?;
        xmp.set_property(
            "http://ns.adobe.com/exif/1.0/",
            "GPSAltitude",
            "0/10000",
            exempi::PropFlags::empty(),
        )?;

        let buf = xmp.serialize_and_format(
            exempi::SERIAL_OMITPACKETWRAPPER | exempi::SERIAL_USECOMPACTFORMAT,
            0,
            "\n",
            " ",
            1,
        )?;
        let mut file = File::create(xmp_file)?;
        let written = file.write(buf.to_str().as_bytes())?;
        if written < buf.len() {
            println!("Short write: {} of {} bytes written.", written, buf.len());
        }
    }
    Ok(true)
}
