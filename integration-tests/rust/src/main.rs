#![deny(clippy::all, clippy::pedantic, warnings)]

mod types;

use {
    std::{
        f64::consts::PI,
        fmt::Debug,
        io::{self, Error, ErrorKind},
    },
    types::{
        basic::unit::UnitOut,
        main::{BarIn, BarOut, FooIn, FooOut},
        Deserialize, Serialize,
    },
};

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
    out_to_in::<FooOut, FooIn>(FooOut {
        x: true,
        y: vec![0, 42, 255],
        z: UnitOut {},
    })?;
    println!();
    out_to_in::<BarOut, BarIn>(BarOut::X(true))?;

    Ok(())
}

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

fn out_to_in<T: Debug + Serialize, U: Debug + Deserialize + From<T>>(x: T) -> io::Result<()> {
    println!("Value to be serialized: {:?}", x);

    let mut buffer = Vec::<u8>::new();
    x.serialize(&mut buffer)?;
    println!("Bytes from serialization: {:?}", buffer);

    let y = U::deserialize(&mut buffer.as_slice())?;
    println!("Value deserialized from those bytes: {:?}", y);

    if format!("{:?}", y) == format!("{:?}", U::from(x)) {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Mismatch!"))
    }
}
