mod types;

use {
    std::{
        fs::{File, remove_file},
        io::{self, BufReader, BufWriter, Write},
    },
    types::{
        Deserialize, Serialize,
        types::{
            SendEmailRequestIn, SendEmailRequestOut, SendEmailResponseIn, SendEmailResponseOut,
        },
    },
};

const REQUEST_FILE_PATH: &str = "/tmp/request";
const RESPONSE_FILE_PATH: &str = "/tmp/response";

fn write_to_files() -> io::Result<()> {
    let request_message = SendEmailRequestOut {
        to: "typical@example.com".to_owned(),
        subject: "I love Typical!".to_owned(),
        body: "It makes serialization easy and safe.".to_owned(),
    };

    let response_message = SendEmailResponseOut::Error("Example error".to_string());

    let mut request_file = BufWriter::new(File::create(REQUEST_FILE_PATH)?);
    request_message.serialize(&mut request_file)?;
    request_file.flush()?;

    let mut response_file = BufWriter::new(File::create(RESPONSE_FILE_PATH)?);
    response_message.serialize(&mut response_file)?;
    response_file.flush()
}

fn read_from_files() -> io::Result<()> {
    let request_file = BufReader::new(File::open(REQUEST_FILE_PATH)?);
    let request_message = SendEmailRequestIn::deserialize(request_file)?;

    let response_file = BufReader::new(File::open(RESPONSE_FILE_PATH)?);
    let response_message = SendEmailResponseIn::deserialize(response_file)?;

    println!("to: {}", request_message.to);
    println!("subject: {}", request_message.subject);
    println!("body: {}", request_message.body);

    match response_message {
        SendEmailResponseIn::Success => println!("The email was sent!"),
        SendEmailResponseIn::Error(message) => println!("An error occurred: {message}"),
    }

    Ok(())
}

fn main() -> io::Result<()> {
    write_to_files()?;
    read_from_files()?;
    remove_file(REQUEST_FILE_PATH)?;
    remove_file(RESPONSE_FILE_PATH)
}
