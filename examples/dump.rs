use std::env;
use std::fs;
use std::error::Error;
use gtfs_rt::FeedMessage;
use prost::Message;

fn main() -> Result<(), Box<Error>> {
    let fname = env::args().nth(1).ok_or("pass path to gtfs-rt protobuf file")?;
    let data = fs::read(fname)?;
    let message = FeedMessage::decode(data)?;
    println!("{:#?}", message);

    Ok(())
}
