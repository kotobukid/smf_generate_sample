use std::fs;
use std::fs::File;
use std::io::{self, Read, Write, BufReader};
use std::panic::resume_unwind;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    match fs::create_dir("./output") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("{}", "directory exists or created");

    let mut file = File::create("./output/f.mid")?;

    // let buf = BufReader::new(io::stdin()).bytes().collect::<io::Result<Vec<u8>>>()?;
    let buf = "MThd";

    file.write_all(buf.as_bytes())?;

    let buf2: i32 = 6;
    file.write_all(&buf2.to_be_bytes());

    let file_format: i16 = 1;
    file.write_all(&file_format.to_be_bytes());

    let track_amount: i16 = 2;
    file.write_all(&track_amount.to_be_bytes());

    let resolution: i16 = 960;
    file.write_all(&resolution.to_be_bytes());

    let start_track: &str = "MTrk";
    file.write_all(start_track.as_bytes());

    let track_length: i32 = 23;
    file.write_all(&track_length.to_be_bytes());

    let before_start_track_title: i16 = 255;    // 00FFxx   を表現したいが今回は分割して記述
    file.write_all(&before_start_track_title.to_be_bytes());
    let before_start_track_title2: i8 = 3;
    file.write_all(&before_start_track_title2.to_be_bytes());


    let track_title: &str = "Track1";
    let track_title_length: i8 = track_title.as_bytes().len() as i8;
    file.write_all(&track_title_length.to_be_bytes());
    file.write_all(&track_title.as_bytes());

    file.flush()?;
    Ok(())
}
