use std::fs;

use asciiforger::asciify;
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

    let test_string =
        fs::read_to_string("tests/res/test_string").expect("Should have been able to read the file");

    let ascii_string = asciify(desired_width, desired_height, &img_scale, &img).unwrap();

    println!("{}", test_string.len());
    println!("{}", ascii_string.len());

    assert_eq!(ascii_string, test_string);
}
