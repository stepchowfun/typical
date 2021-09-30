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
    round_trip::<FooOut, FooIn>(FooOut {
        v_required: true,
        w_required: vec![0, 42, 255],
        x_required: PI,
        y_required: u64::MAX,
        z_required: UnitOut {},

        v_unstable: true,
        w_unstable: vec![0, 42, 255],
        x_unstable: PI,
        y_unstable: u64::MAX,
        z_unstable: UnitOut {},

        v_optional: None,
        w_optional: None,
        x_optional: None,
        y_optional: None,
        z_optional: None,
    })?;

    println!();

    round_trip::<FooOut, FooIn>(FooOut {
        v_required: true,
        w_required: vec![0, 42, 255],
        x_required: PI,
        y_required: u64::MAX,
        z_required: UnitOut {},

        v_unstable: true,
        w_unstable: vec![0, 42, 255],
        x_unstable: PI,
        y_unstable: u64::MAX,
        z_unstable: UnitOut {},

        v_optional: Some(true),
        w_optional: Some(vec![0, 42, 255]),
        x_optional: Some(PI),
        y_optional: Some(u64::MAX),
        z_optional: Some(UnitOut {}),
    })?;

    println!();

    round_trip::<BarOut, BarIn>(BarOut::VRequired(true))?;
    round_trip::<BarOut, BarIn>(BarOut::WRequired(vec![0, 42, 255]))?;
    round_trip::<BarOut, BarIn>(BarOut::XRequired(PI))?;
    round_trip::<BarOut, BarIn>(BarOut::YRequired(u64::MAX))?;
    round_trip::<BarOut, BarIn>(BarOut::ZRequired(UnitOut {}))?;

    let fallback = BarOut::VRequired(true);

    round_trip::<BarOut, BarIn>(BarOut::VUnstable(true, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::WUnstable(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;
    round_trip::<BarOut, BarIn>(BarOut::XUnstable(PI, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::YUnstable(u64::MAX, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::ZUnstable(UnitOut {}, Box::new(fallback.clone())))?;

    round_trip::<BarOut, BarIn>(BarOut::VOptional(true, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::WOptional(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;
    round_trip::<BarOut, BarIn>(BarOut::XOptional(PI, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::YOptional(u64::MAX, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::ZOptional(UnitOut {}, Box::new(fallback)))?;

    Ok(())
}

fn round_trip<T: Debug + Serialize, U: Debug + Deserialize + From<T>>(x: T) -> io::Result<()> {
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
