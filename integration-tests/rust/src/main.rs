#![deny(clippy::all, clippy::pedantic, warnings)]

#[macro_use]
mod types;

use {
    std::{
        f64::consts::PI,
        fmt::Debug,
        io::{self, Error, ErrorKind, Read, Write},
        mem::size_of,
    },
    types::{Deserialize, Serialize},
};

fn round_trip_message<T: Debug + Deserialize + PartialEq + Serialize>(x: &T) -> io::Result<()> {
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

fn round_trip_varint(x: u64) -> io::Result<()> {
    println!("Value to be serialized: {:?}", x);

    let mut buffer = Vec::<u8>::new();
    write_varint!(&mut buffer, x, u64)?;
    println!("Bytes from serialization: {:?}", buffer);

    let y = read_varint!(&mut buffer.as_slice(), u64)?;
    println!("Value deserialized from those bytes: {:?}", y);

    if y == x {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Mismatch!"))
    }
}

#[allow(clippy::float_cmp, clippy::shadow_unrelated)]
fn main() -> io::Result<()> {
    round_trip_message::<f64>(&PI)?;
    println!();
    round_trip_message::<bool>(&false)?;
    println!();
    round_trip_message::<bool>(&true)?;
    println!();
    round_trip_varint(u64::MIN)?;
    println!();
    round_trip_varint(u64::MAX)?;

    Ok(())
}
