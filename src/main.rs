use std::fs;
use std::fs::File;
use std::io::{self, Read, Write, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    match fs::create_dir("./output") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("{}", "directory exists or created");

    let mut file = File::create("./output/f.mid")?;

    // let buf = BufReader::new(io::stdin()).bytes().collect::<io::Result<Vec<u8>>>()?;
    let buf = "hello world";

    file.write_all(buf.as_bytes())?;
    file.flush()?;
    Ok(())
}
