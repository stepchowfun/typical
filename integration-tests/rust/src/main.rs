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
        t_required: true,
        u_required: vec![0, 42, 255],
        v_required: PI,
        w_required: i64::MIN,
        x_required: "Hello, World!".to_owned(),
        y_required: u64::MAX,
        z_required: UnitOut {},

        t_unstable: true,
        u_unstable: vec![0, 42, 255],
        v_unstable: PI,
        w_unstable: i64::MIN,
        x_unstable: "Hello, World!".to_owned(),
        y_unstable: u64::MAX,
        z_unstable: UnitOut {},

        t_optional: None,
        u_optional: None,
        v_optional: None,
        w_optional: None,
        x_optional: None,
        y_optional: None,
        z_optional: None,
    })?;

    println!();

    round_trip::<FooOut, FooIn>(FooOut {
        t_required: true,
        u_required: vec![0, 42, 255],
        v_required: PI,
        w_required: i64::MIN,
        x_required: "Hello, World!".to_owned(),
        y_required: u64::MAX,
        z_required: UnitOut {},

        t_unstable: true,
        u_unstable: vec![0, 42, 255],
        v_unstable: PI,
        w_unstable: i64::MIN,
        x_unstable: "Hello, World!".to_owned(),
        y_unstable: u64::MAX,
        z_unstable: UnitOut {},

        t_optional: Some(true),
        u_optional: Some(vec![0, 42, 255]),
        v_optional: Some(PI),
        w_optional: Some(i64::MIN),
        x_optional: Some("Hello, World!".to_owned()),
        y_optional: Some(u64::MAX),
        z_optional: Some(UnitOut {}),
    })?;

    println!();

    round_trip::<BarOut, BarIn>(BarOut::TRequired(true))?;
    round_trip::<BarOut, BarIn>(BarOut::URequired(vec![0, 42, 255]))?;
    round_trip::<BarOut, BarIn>(BarOut::VRequired(PI))?;
    round_trip::<BarOut, BarIn>(BarOut::WRequired(i64::MIN))?;
    round_trip::<BarOut, BarIn>(BarOut::XRequired("Hello, World!".to_owned()))?;
    round_trip::<BarOut, BarIn>(BarOut::YRequired(u64::MAX))?;
    round_trip::<BarOut, BarIn>(BarOut::ZRequired(UnitOut {}))?;

    let fallback = BarOut::TRequired(true);

    round_trip::<BarOut, BarIn>(BarOut::TUnstable(true, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::UUnstable(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;
    round_trip::<BarOut, BarIn>(BarOut::VUnstable(PI, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::WUnstable(i64::MIN, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::XUnstable(
        "Hello, World!".to_owned(),
        Box::new(fallback.clone()),
    ))?;
    round_trip::<BarOut, BarIn>(BarOut::YUnstable(u64::MAX, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::ZUnstable(UnitOut {}, Box::new(fallback.clone())))?;

    round_trip::<BarOut, BarIn>(BarOut::TOptional(true, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::UOptional(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;
    round_trip::<BarOut, BarIn>(BarOut::VOptional(PI, Box::new(fallback.clone())))?;
    round_trip::<BarOut, BarIn>(BarOut::WOptional(i64::MIN, Box::new(fallback.clone())))?;
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
