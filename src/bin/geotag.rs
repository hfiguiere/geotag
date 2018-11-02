/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate docopt;
extern crate geotag;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::env;

use docopt::Docopt;
use geotag::{tag_file, CoordTagger, Tagger, TracklogTagger};

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
