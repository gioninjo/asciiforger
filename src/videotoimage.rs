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
extern crate ffmpeg_next as ffmpeg;
use ffmpeg::format::context::Input;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use ffmpeg::Rational;
use image::*;



///
/// ### read_video_and_decode
/// 
/// Transform a video into a dynamic images vector
/// 
pub fn read_video_and_decode(path: &String) -> Result<(Vec<DynamicImage>, f32), ffmpeg::Error> {
    match input(path) {
      Ok(res) => {
        let ictx = res;

        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input.index();

        let avg_frame_rate: Rational = input.avg_frame_rate();
        let time_interval_between_frames = get_time_between_frames_millis(avg_frame_rate);

        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        let decoder = context_decoder.decoder().video()?;

        let scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        match get_img_vec_from_video(ictx, scaler, video_stream_index, decoder) {
            Ok(img_vec) => Ok((img_vec, time_interval_between_frames)),
            Err(e) => return Err(e)
        }

      },
      Err(e) => return Err(e)
    }
}

fn get_img_from_frame(frame: &Video) -> Result<DynamicImage, ffmpeg::Error> {

    // insert header bytes for image file
    let mut image_in_byte_vec = Vec::from(
        format!("P6\n{} {}\n255\n", frame.width(), frame.height())
            .as_bytes()
    );

    image_in_byte_vec.extend_from_slice(frame.data(0));

    let buffer = image_in_byte_vec.as_slice();

    let image: DynamicImage = load_from_memory(buffer).unwrap();

    Ok(image)
}

fn get_img_vec_from_video(
    mut ictx: Input,
    mut scaler: Context,
    video_stream_index: usize,
    mut decoder: ffmpeg::codec::decoder::Video
) -> Result<Vec<image::DynamicImage>, ffmpeg::Error> {
    let mut frame_index = 0;

    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> Result<Vec<DynamicImage>, ffmpeg::Error> {
            let mut decoded = Video::empty();
            let mut img_vec: Vec<DynamicImage> = Vec::new();

            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;
                let img_frame = match get_img_from_frame(&rgb_frame) {
                    Ok(img) => img,
                    Err(e) => return Err(e),
                };
                img_vec.push(img_frame);
                frame_index += 1;
            }
            Ok(img_vec)
        };

    let mut images: Vec<DynamicImage> = Vec::new();

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            let mut new_img_vec = match receive_and_process_decoded_frames(&mut decoder) {
                Ok(img_vec) => img_vec,
                Err(e) => return Err(e),
            };
            images.append(&mut new_img_vec);
        }
    }
    decoder.send_eof()?;

    let mut new_img_vec = match receive_and_process_decoded_frames(&mut decoder) {
        Ok(img_vec) => img_vec,
        Err(e) => return Err(e),
    };
    images.append(&mut new_img_vec);

    Ok(images)
}

fn get_time_between_frames_millis(frame_rate: Rational) -> f32 {
    let fps: f32 = (frame_rate.numerator() as f32) / (frame_rate.denominator() as f32);

    let time_between_frames_seconds  = 1.000 / fps;
    let time_between_frames_millis = time_between_frames_seconds * 1000.0;

    time_between_frames_millis
}