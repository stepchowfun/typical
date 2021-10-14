#![deny(clippy::all, clippy::pedantic, warnings)]

mod types;

use {
    std::{
        fs::{remove_file, File},
        io::{self, BufReader, BufWriter},
    },
    types::{
        types::{SendEmailRequestIn, SendEmailRequestOut},
        Deserialize, Serialize,
    },
};

const FILE_PATH: &str = "/tmp/request";

fn write_to_file() -> io::Result<()> {
    let request = SendEmailRequestOut {
        to: "typical@example.com".to_owned(),
        subject: "I love Typical!".to_owned(),
        body: "It makes serialization easy and safe.".to_owned(),
    };

    let mut file = BufWriter::new(File::create(FILE_PATH)?);
    request.serialize(&mut file)
}

fn read_from_file() -> io::Result<()> {
    let mut file = BufReader::new(File::open(FILE_PATH)?);
    let request = SendEmailRequestIn::deserialize(&mut file)?;

    println!("to: {}", request.to);
    println!("subject: {}", request.subject);
    println!("body: {}", request.body);

    remove_file(FILE_PATH)
}

fn main() -> io::Result<()> {
    write_to_file()?;
    read_from_file()
}
