#![deny(warnings)]

mod types;

use {
    std::{
        f64::consts::PI,
        fmt::Debug,
        io::{self, Error, ErrorKind},
    },
    types::{Deserialize, Serialize},
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

#[allow(clippy::float_cmp, clippy::shadow_unrelated)]
fn main() -> io::Result<()> {
    round_trip::<f64>(&PI)?;
    round_trip::<bool>(&false)?;
    round_trip::<bool>(&true)?;
    Ok(())
}
