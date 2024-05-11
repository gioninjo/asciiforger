//  SPDX-License-Identifier: GPL-3.0-only
/*  Build tool with support for git tags, wrapping make.
 *  Copyright (C) 2024  gioninjo
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, version 3 of the License.
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use std::time;

///
/// ### run_ascii_video
/// 
/// take an ascii vector and a time interval, then print the vector like a video on the output,
/// with time_interval as the time between a frame and the other
pub fn run_ascii_video(ascii_vec: Vec<String>, time_interval: f32) -> () {
  for ascii_frame in ascii_vec {
    println!("{}", ascii_frame);
    let ten_millis = time::Duration::from_millis(time_interval as u64);
    std::thread::sleep(ten_millis);
    print!("{}[2J", 27 as char);
  }
}