/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::path::Path;
use std::str::FromStr;

use tagger::Tagger;

#[cfg(test)]
fn dd_to_dms(dd: f64, is_lat: bool) -> String {
    let reference: char = if dd.is_sign_positive() {
        if is_lat {
            'N'
        } else {
            'E'
        }
    } else if is_lat {
        'S'
    } else {
        'W'
    };

    let dd = dd.abs();
    let d = dd.trunc() as i32;
    let ms = dd.fract() * 60_f64;
    let m = ms.trunc() as i32;
    let s = (ms.fract() * 60_f64) as i32;

    format!("{},{},{}{}", d, m, s, reference)
}

fn dd_to_dmfract(dd: f64, is_lat: bool) -> String {
    let reference: char = if dd.is_sign_positive() {
        if is_lat {
            'N'
        } else {
            'E'
        }
    } else if is_lat {
        'S'
    } else {
        'W'
    };

    let dd = dd.abs();
    let d = dd.trunc() as i32;
    let ms = dd.fract() * 60_f64;

    format!("{},{:.6}{}", d, ms, reference)
}

#[derive(Default)]
pub struct CoordTagger {
    lat: String,
    long: String,
}

impl CoordTagger {
    // assume the argument is dd,dd
    // dd is decimal degrees, with sign.
    pub fn new(arg: &str) -> CoordTagger {
        // parse

        let v: Vec<&str> = arg.split(',').collect();
        if v.len() == 2 {
            if let Ok(lat) = f64::from_str(v[0]) {
                if lat.abs() > 90_f64 {
                    return CoordTagger::default();
                }
                if let Ok(long) = f64::from_str(v[1]) {
                    if long.abs() >= 180_f64 {
                        return CoordTagger::default();
                    }
                    let lat = dd_to_dmfract(lat, true);
                    let long = dd_to_dmfract(long, false);
                    return CoordTagger { lat, long };
                }
            }
        }
        CoordTagger::default()
    }
}

impl Tagger for CoordTagger {
    fn get_coord_for_file(&self, _file: &Path) -> (String, String) {
        (self.lat.clone(), self.long.clone())
    }

    fn is_ok(&self) -> bool {
        !self.long.is_empty() && !self.lat.is_empty()
    }
}

#[cfg(test)]
#[test]
fn test_dd_to_dms() {
    assert_eq!(dd_to_dms(45.5135219, true), "45,30,48N");
    assert_eq!(dd_to_dms(-73.5718842, false), "73,34,18W");

    assert_eq!(dd_to_dmfract(45.520514, true), "45,31.230840N");
    assert_eq!(dd_to_dmfract(-73.582707, false), "73,34.962420W");
}

#[cfg(test)]
#[test]
fn test_parsing() {
    let coord = CoordTagger::new("101.23,-223.10");
    assert!(!coord.is_ok());
    let coord = CoordTagger::new("12.34,-45.54");
    assert!(coord.is_ok());

    let coords = coord.get_coord_for_file(&Path::new(""));
    assert_eq!(
        coords,
        ("12,20.400000N".to_owned(), "45,32.400000W".to_owned())
    );

    // Invalid data
    let coord = CoordTagger::new("12.34/-45.54");
    assert!(!coord.is_ok());

    let coords = coord.get_coord_for_file(&Path::new(""));
    assert_eq!(coords, (String::default(), String::default()));
}
