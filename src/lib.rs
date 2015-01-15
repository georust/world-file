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
    x_scale: f64,
    y_skew: f64,
    x_skew: f64,
    y_scale: f64,
    x_coord: f64,
    y_coord: f64,
}

impl FromStr for WorldFile {
    fn from_str(s: &str) -> Option<Self> {
        let mut lines = s.lines_any();

        // size of pixel in x direction (x_scale)
        let x_scale: f64 = match lines.next() {
            Some(s) => match FromStr::from_str(s) {
                Some(i) => i,
                None => return None,
            },
            None => return None,
        };

        // rotation term for row (y_skew)
        let y_skew: f64 = match lines.next() {
            Some(s) => match FromStr::from_str(s) {
                Some(i) => i,
                None => return None,
            },
            None => return None,
        };

        // rotation term for column (x_skew)
        let x_skew: f64 = match lines.next() {
            Some(s) => match FromStr::from_str(s) {
                Some(i) => i,
                None => return None,
            },
            None => return None,
        };

        // size of pixel in y direction (y_scale)
        let y_scale: f64 = match lines.next() {
            Some(s) => match FromStr::from_str(s) {
                Some(i) => i,
                None => return None,
            },
            None => return None,
        };

        // x coordinate of centre of upper left pixel in map units (x_coord)
        let x_coord: f64 = match lines.next() {
            Some(s) => match FromStr::from_str(s) {
                Some(i) => i,
                None => return None,
            },
            None => return None,
        };

        // y coordinate of centre of upper left pixel in map units (y_coord)
        let y_coord: f64 = match lines.next() {
            Some(s) => match FromStr::from_str(s) {
                Some(i) => i,
                None => return None,
            },
            None => return None,
        };

        Some(WorldFile {
            x_scale: x_scale,
            y_skew: y_skew,
            x_skew: x_skew,
            y_scale: y_scale,
            x_coord: x_coord,
            y_coord: y_coord,
        })
    }
}
