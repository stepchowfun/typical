mod types;

use {
    std::{
        fs::{remove_file, File},
        io::{self, BufReader, BufWriter},
    },
    types::{
        types::{
            SendEmailRequestIn, SendEmailRequestOut, SendEmailResponseIn, SendEmailResponseOut,
        },
        Deserialize, Serialize,
    },
};

const REQUEST_FILE_PATH: &str = "/tmp/request";
const RESPONSE_FILE_PATH: &str = "/tmp/response";

fn write_to_file() -> io::Result<()> {
    let request_message = SendEmailRequestOut {
        to: "typical@example.com".to_owned(),
        subject: "I love Typical!".to_owned(),
        body: "It makes serialization easy and safe.".to_owned(),
    };

    let response_message = SendEmailResponseOut::Error("Example error".to_string());

    let request_file = BufWriter::new(File::create(REQUEST_FILE_PATH)?);
    request_message.serialize(request_file)?;

    let response_file = BufWriter::new(File::create(RESPONSE_FILE_PATH)?);
    response_message.serialize(response_file)
}

fn read_from_file() -> io::Result<()> {
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
    write_to_file()?;
    read_from_file()?;
    remove_file(REQUEST_FILE_PATH)?;
    remove_file(RESPONSE_FILE_PATH)
}
