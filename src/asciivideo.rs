use std::time;




pub fn run_ascii_video(ascii_vec: Vec<String>, time_interval: f32) -> () {
  for ascii_frame in ascii_vec {
    println!("{}", ascii_frame);
    let ten_millis = time::Duration::from_millis(time_interval as u64);
    std::thread::sleep(ten_millis);
    print!("{}[2J", 27 as char);
  }
}