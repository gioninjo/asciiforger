use std::fs;

use asciiforger::{
    images::asciify, videotoimage::read_video_and_decode,
};
extern crate ffmpeg_next as ffmpeg;
use image::open;

#[test]
fn asciify_compare() {
    let desired_width: u32 = 128;
    let desired_height: u32 = 64;
    let scale = *b"@%#*+=-:. ";

    let img_scale: Vec<char> = String::from_utf8(Vec::from(scale))
        .unwrap_or(String::new())
        .chars()
        .collect();

    let img = open("tests/res/monster.jpg").expect("Failed to open image at {file_path}");

    let test_string = fs::read_to_string("tests/res/test_string")
        .expect("Should have been able to read the file");

    let ascii_string = asciify(desired_width, desired_height, &img_scale, &img).unwrap();

    println!("{}", test_string.len());
    println!("{}", ascii_string.len());

    assert_eq!(ascii_string, test_string);
}

#[test]
fn video_decoding_test() {
    ffmpeg::init().unwrap();

    const TEST_STRING_PATH: &str = "tests/res/videotest";
    const TEST_VIDEO_PATH: &str = "tests/res/video.avi";

    let mut scale = *b"@%#*+=-:. ";
    scale.reverse();
    let img_scale: Vec<char> = String::from_utf8(Vec::from(scale))
        .unwrap_or(String::new())
        .chars()
        .collect();
    println!("reading test file...");
    let test_string = fs::read_to_string(&String::from(TEST_STRING_PATH)).unwrap();
    println!("done :)");

    let (img_vec, _time_interval) = read_video_and_decode(&String::from(TEST_VIDEO_PATH)).unwrap();
    let mut ascii_vec: Vec<String> = Vec::new();

    for image in img_vec {
        let frame: String = asciify(128, 64, &img_scale, &image).unwrap();
        ascii_vec.push(frame);
    }

    let mut video_string = String::new();
    for ascii_frame in ascii_vec {
        video_string.push_str(&ascii_frame);
        video_string.push_str(&format!("{}[2J", 27 as char));
    }
    println!("video_string {}", video_string.len());
    println!("test_string {}", test_string.len());

    assert_eq!(video_string.len(), test_string.len());
    println!("o");
}
