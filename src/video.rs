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
use crate::{
    asciivideo::run_ascii_video, images::asciify, videotoimage::read_video_and_decode
};
extern crate ffmpeg_next as ffmpeg;

/// ### run_video
/// 
/// just take a video path, decode it an run it in ascii-coded mode
/// 
pub fn run_video(path: &String) -> Result<(), ffmpeg::Error> {
    ffmpeg::init().unwrap();

    let mut scale = *b"@%#*+=-:. "; 
    scale.reverse();
    let img_scale: Vec<char> = String::from_utf8(Vec::from(scale))
        .unwrap_or(String::new())
        .chars()
        .collect();


    match read_video_and_decode(path) {
        Ok((img_vec, time_interval)) => {

            let mut ascii_vec: Vec<String> = Vec::new();

            for image in img_vec {
                let frame: String =  asciify(128, 64, &img_scale, &image).unwrap();
                ascii_vec.push(frame);
            }

            run_ascii_video(ascii_vec, time_interval);
        },
        Err(e) => return Err(e)
    }

    Ok(())
}
