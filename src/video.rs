use crate::{
    asciivideo::run_ascii_video, images::asciify, videotoimage::read_video_and_decode
};

extern crate ffmpeg_next as ffmpeg;

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
