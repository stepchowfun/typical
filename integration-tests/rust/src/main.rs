#![deny(clippy::all, clippy::pedantic, warnings)]

mod types;

use {
    std::{
        f64::consts::PI,
        fmt::Debug,
        io::{self, Error, ErrorKind},
    },
    types::{
        main::{BazIn, BazOut, QuxIn, QuxOut},
        Deserialize, Serialize,
    },
};

fn round_trip<T: Debug + Deserialize + PartialEq + Serialize>(x: &T) -> io::Result<()> {
    println!("Value to be serialized: {:?}", x);

    let mut buffer = Vec::<u8>::new();
    x.serialize(&mut buffer)?;
    println!("Bytes from serialization: {:?}", buffer);

    let y = T::deserialize(&mut buffer.as_slice())?;
    println!("Value deserialized from those bytes: {:?}", y);

    if y == *x {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Mismatch!"))
    }
}

fn main() -> io::Result<()> {
    round_trip::<bool>(&false)?;
    println!();
    round_trip::<bool>(&true)?;
    println!();
    round_trip::<u64>(&u64::MIN)?;
    println!();
    round_trip::<u64>(&u64::MAX)?;
    println!();
    round_trip::<f64>(&PI)?;

    println!();

    let message = BazOut {
        x: true,
        y: 42,
        z: PI,
    };
    let mut buffer = Vec::<u8>::new();
    message.serialize(&mut buffer)?;
    println!("Struct size: {:?}", message.size());
    println!("Struct bytes: {:?}", buffer);
    let message = BazIn::deserialize(&mut buffer.as_slice())?;
    println!("Deserialized struct: {:?}", message);

    println!();

    let message = QuxOut::X(true);
    let mut buffer = Vec::<u8>::new();
    message.serialize(&mut buffer)?;
    println!("Choice size: {:?}", message.size());
    println!("Choice bytes: {:?}", buffer);
    let message = QuxIn::deserialize(&mut buffer.as_slice())?;
    println!("Deserialized choice: {:?}", message);

    Ok(())
}
