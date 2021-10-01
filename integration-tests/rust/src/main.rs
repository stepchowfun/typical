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
        u_required: true,
        v_required: vec![0, 42, 255],
        w_required: PI,
        x_required: "Hello, World!".to_owned(),
        y_required: u64::MAX,
        z_required: UnitOut {},

        u_unstable: true,
        v_unstable: vec![0, 42, 255],
        w_unstable: PI,
        x_unstable: "Hello, World!".to_owned(),
        y_unstable: u64::MAX,
        z_unstable: UnitOut {},

        u_optional: None,
        v_optional: None,
        w_optional: None,
        x_optional: None,
        y_optional: None,
        z_optional: None,
    })?;

    println!();

    round_trip::<FooOut, FooIn>(FooOut {
        u_required: true,
        v_required: vec![0, 42, 255],
        w_required: PI,
        x_required: "Hello, World!".to_owned(),
        y_required: u64::MAX,
        z_required: UnitOut {},

        u_unstable: true,
        v_unstable: vec![0, 42, 255],
        w_unstable: PI,
        x_unstable: "Hello, World!".to_owned(),
        y_unstable: u64::MAX,
        z_unstable: UnitOut {},

        u_optional: Some(true),
        v_optional: Some(vec![0, 42, 255]),
        w_optional: Some(PI),
        x_optional: Some("Hello, World!".to_owned()),
        y_optional: Some(u64::MAX),
        z_optional: Some(UnitOut {}),
    })?;

    println!();

    round_trip::<BarOut, BarIn>(BarOut::URequired(true))?;
    round_trip::<BarOut, BarIn>(BarOut::VRequired(vec![0, 42, 255]))?;
    round_trip::<BarOut, BarIn>(BarOut::WRequired(PI))?;
    round_trip::<BarOut, BarIn>(BarOut::XRequired("Hello, World!".to_owned()))?;
    round_trip::<BarOut, BarIn>(BarOut::YRequired(u64::MAX))?;
    round_trip::<BarOut, BarIn>(BarOut::ZRequired(UnitOut {}))?;

    let fallback = BarOut::URequired(true);

    round_trip::<BarOut, BarIn>(BarOut::UUnstable(true, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::VUnstable(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;
    round_trip::<BarOut, BarIn>(BarOut::WUnstable(PI, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::XUnstable(
        "Hello, World!".to_owned(),
        Box::new(fallback.clone()),
    ))?;
    round_trip::<BarOut, BarIn>(BarOut::YUnstable(u64::MAX, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::ZUnstable(UnitOut {}, Box::new(fallback.clone())))?;

    round_trip::<BarOut, BarIn>(BarOut::UOptional(true, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::VOptional(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;
    round_trip::<BarOut, BarIn>(BarOut::WOptional(PI, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::XOptional(
        "Hello, World!".to_owned(),
        Box::new(fallback.clone()),
    ))?;
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
