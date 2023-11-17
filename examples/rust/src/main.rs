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

const FILE_PATH: &str = "/tmp/message";

fn write_to_file() -> io::Result<()> {
    let message = SendEmailRequestOut {
        to: "typical@example.com".to_owned(),
        subject: "I love Typical!".to_owned(),
        body: "It makes serialization easy and safe.".to_owned(),
    };

    let file = BufWriter::new(File::create(FILE_PATH)?);
    message.serialize(file)
}

fn read_from_file() -> io::Result<()> {
    let file = BufReader::new(File::open(FILE_PATH)?);
    let message = SendEmailRequestIn::deserialize(file)?;

    println!("to: {}", message.to);
    println!("subject: {}", message.subject);
    println!("body: {}", message.body);

    Ok(())
}

fn main() -> io::Result<()> {
    write_to_file()?;
    read_from_file()?;
    remove_file(FILE_PATH)
}
