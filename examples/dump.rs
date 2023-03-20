use gtfs_rt::FeedMessage;
use prost::Message;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let fname = env::args()
        .nth(1)
        .ok_or("pass path to gtfs-rt protobuf file")?;
    let data = fs::read(fname)?;
    let message = FeedMessage::decode(data.as_slice())?;
    println!("{:#?}", message);

    Ok(())
}
