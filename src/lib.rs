// Copyright 2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::str::FromStr;

pub struct WorldFile {
    pub x_scale: f64,
    pub y_skew: f64,
    pub x_skew: f64,
    pub y_scale: f64,
    pub x_coord: f64,
    pub y_coord: f64,
}

impl FromStr for WorldFile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut lines = s.lines_any();

        // size of pixel in x direction (x_scale)
        let x_scale: f64 = match lines.next() {
            Some(s) => match FromStr::from_str(s) {
                Ok(i) => i,
                Err(_) => return Err(()),
            },
            None => return Err(()),
        };

        // rotation term for row (y_skew)
        let y_skew: f64 = match lines.next() {
            Some(s) => match FromStr::from_str(s) {
                Ok(i) => i,
                Err(_) => return Err(()),
            },
            None => return Err(()),
        };

        // rotation term for column (x_skew)
        let x_skew: f64 = match lines.next() {
            Some(s) => match FromStr::from_str(s) {
                Ok(i) => i,
                Err(_) => return Err(()),
            },
            None => return Err(()),
        };

        // size of pixel in y direction (y_scale)
        let y_scale: f64 = match lines.next() {
            Some(s) => match FromStr::from_str(s) {
                Ok(i) => i,
                Err(_) => return Err(()),
            },
            None => return Err(()),
        };

        // x coordinate of centre of upper left pixel in map units (x_coord)
        let x_coord: f64 = match lines.next() {
            Some(s) => match FromStr::from_str(s) {
                Ok(i) => i,
                Err(_) => return Err(()),
            },
            None => return Err(()),
        };

        // y coordinate of centre of upper left pixel in map units (y_coord)
        let y_coord: f64 = match lines.next() {
            Some(s) => match FromStr::from_str(s) {
                Ok(i) => i,
                Err(_) => return Err(()),
            },
            None => return Err(()),
        };

        Ok(WorldFile {
            x_scale: x_scale,
            y_skew: y_skew,
            x_skew: x_skew,
            y_scale: y_scale,
            x_coord: x_coord,
            y_coord: y_coord,
        })
    }
}
