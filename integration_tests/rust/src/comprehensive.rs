use {
    crate::types::{
        comprehensive::{
            bar::{BarIn, BarOut},
            foo::{FooIn, FooOut},
            main::{
                EmptyStructIn, EmptyStructOut, FooAndBarIn, FooAndBarOut, FooOrBarIn, FooOrBarOut,
            },
        },
        Deserialize, Serialize,
    },
    std::{
        f64::consts::{E, PI},
        fmt::Debug,
        io::{self, Error, ErrorKind},
    },
};

#[allow(clippy::too_many_lines)]
pub fn run() -> io::Result<()> {
    round_trip_match::<FooOut, FooIn>(FooOut {
        p_required: vec![(), (), ()],
        q_required: vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
        r_required: vec![i64::MIN, 0, i64::MAX],
        s_required: vec![
            vec!["Hello".to_owned(), "World".to_owned()],
            vec!["Hello".to_owned(), "Earth".to_owned()],
            vec!["Hello".to_owned(), "Planet".to_owned()],
        ],
        t_required: true,
        u_required: vec![0, 42, 255],
        v_required: PI,
        w_required: i64::MAX,
        x_required: "Hello, World!".to_owned(),
        y_required: u64::MAX,
        z_required: (),

        p_unstable: vec![(), (), ()],
        q_unstable: vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
        r_unstable: vec![i64::MIN, 0, i64::MAX],
        s_unstable: vec![
            vec!["Hello".to_owned(), "World".to_owned()],
            vec!["Hello".to_owned(), "Earth".to_owned()],
            vec!["Hello".to_owned(), "Planet".to_owned()],
        ],
        t_unstable: true,
        u_unstable: vec![0, 42, 255],
        v_unstable: PI,
        w_unstable: i64::MAX,
        x_unstable: "Hello, World!".to_owned(),
        y_unstable: u64::MAX,
        z_unstable: (),

        p_optional: None,
        q_optional: None,
        r_optional: None,
        s_optional: None,
        t_optional: None,
        u_optional: None,
        v_optional: None,
        w_optional: None,
        x_optional: None,
        y_optional: None,
        z_optional: None,
    })?;

    println!();

    round_trip_match::<FooOut, FooIn>(FooOut {
        p_required: vec![(), (), ()],
        q_required: vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
        r_required: vec![i64::MIN, 0, i64::MAX],
        s_required: vec![
            vec!["Hello".to_owned(), "World".to_owned()],
            vec!["Hello".to_owned(), "Earth".to_owned()],
            vec!["Hello".to_owned(), "Planet".to_owned()],
        ],
        t_required: true,
        u_required: vec![0, 42, 255],
        v_required: PI,
        w_required: i64::MAX,
        x_required: "Hello, World!".to_owned(),
        y_required: u64::MAX,
        z_required: (),

        p_unstable: vec![(), (), ()],
        q_unstable: vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
        r_unstable: vec![i64::MIN, 0, i64::MAX],
        s_unstable: vec![
            vec!["Hello".to_owned(), "World".to_owned()],
            vec!["Hello".to_owned(), "Earth".to_owned()],
            vec!["Hello".to_owned(), "Planet".to_owned()],
        ],
        t_unstable: true,
        u_unstable: vec![0, 42, 255],
        v_unstable: PI,
        w_unstable: i64::MAX,
        x_unstable: "Hello, World!".to_owned(),
        y_unstable: u64::MAX,
        z_unstable: (),

        p_optional: Some(vec![(), (), ()]),
        q_optional: Some(vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN]),
        r_optional: Some(vec![i64::MIN, 0, i64::MAX]),
        s_optional: Some(vec![
            vec!["Hello".to_owned(), "World".to_owned()],
            vec!["Hello".to_owned(), "Earth".to_owned()],
            vec!["Hello".to_owned(), "Planet".to_owned()],
        ]),
        t_optional: Some(true),
        u_optional: Some(vec![0, 42, 255]),
        v_optional: Some(PI),
        w_optional: Some(i64::MAX),
        x_optional: Some("Hello, World!".to_owned()),
        y_optional: Some(u64::MAX),
        z_optional: Some(()),
    })?;

    println!();

    round_trip_match::<BarOut, BarIn>(BarOut::PRequired(vec![]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::PRequired(vec![()]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::PRequired(vec![(), ()]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::PRequired(vec![(), (), ()]))?;

    round_trip_match::<BarOut, BarIn>(BarOut::QRequired(vec![]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::QRequired(vec![f64::NEG_INFINITY]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::QRequired(vec![f64::NEG_INFINITY, f64::INFINITY]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::QRequired(vec![
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NAN,
    ]))?;

    round_trip_match::<BarOut, BarIn>(BarOut::RRequired(vec![]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::RRequired(vec![i64::MIN]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::RRequired(vec![i64::MIN, 0]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::RRequired(vec![i64::MIN, 0, i64::MAX]))?;

    round_trip_match::<BarOut, BarIn>(BarOut::SRequired(vec![]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::SRequired(vec![vec![]]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::SRequired(vec![vec![], vec![]]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::SRequired(vec![vec![], vec![], vec![]]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::SRequired(vec![vec![
        "Hello".to_owned(),
        "World".to_owned(),
    ]]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::SRequired(vec![
        vec!["Hello".to_owned(), "World".to_owned()],
        vec!["Hello".to_owned(), "Earth".to_owned()],
    ]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::SRequired(vec![
        vec!["Hello".to_owned(), "World".to_owned()],
        vec!["Hello".to_owned(), "Earth".to_owned()],
        vec!["Hello".to_owned(), "Planet".to_owned()],
    ]))?;

    round_trip_match::<BarOut, BarIn>(BarOut::TRequired(false))?;
    round_trip_match::<BarOut, BarIn>(BarOut::TRequired(true))?;

    round_trip_match::<BarOut, BarIn>(BarOut::URequired(vec![]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::URequired(vec![0]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::URequired(vec![0, 42]))?;
    round_trip_match::<BarOut, BarIn>(BarOut::URequired(vec![0, 42, 255]))?;

    round_trip_match::<BarOut, BarIn>(BarOut::VRequired(0.0_f64))?;
    round_trip_match::<BarOut, BarIn>(BarOut::VRequired(E))?;
    round_trip_match::<BarOut, BarIn>(BarOut::VRequired(PI))?;
    round_trip_match::<BarOut, BarIn>(BarOut::VRequired(f64::EPSILON))?;
    round_trip_match::<BarOut, BarIn>(BarOut::VRequired(f64::INFINITY))?;
    round_trip_match::<BarOut, BarIn>(BarOut::VRequired(f64::MAX))?;
    round_trip_match::<BarOut, BarIn>(BarOut::VRequired(f64::MIN))?;
    round_trip_match::<BarOut, BarIn>(BarOut::VRequired(f64::MIN_POSITIVE))?;
    round_trip_match::<BarOut, BarIn>(BarOut::VRequired(f64::NAN))?;
    round_trip_match::<BarOut, BarIn>(BarOut::VRequired(f64::NEG_INFINITY))?;

    for i in i64::MIN..=i64::MIN + 1000 {
        round_trip_match::<BarOut, BarIn>(BarOut::WRequired(i))?;
    }
    for i in -1000_i64..=1000_i64 {
        round_trip_match::<BarOut, BarIn>(BarOut::WRequired(i))?;
    }
    for i in i64::MAX - 1000..=i64::MAX {
        round_trip_match::<BarOut, BarIn>(BarOut::WRequired(i))?;
    }

    round_trip_match::<BarOut, BarIn>(BarOut::XRequired("".to_owned()))?;
    round_trip_match::<BarOut, BarIn>(BarOut::XRequired("=8 bytes".to_owned()))?;
    round_trip_match::<BarOut, BarIn>(BarOut::XRequired("Hello, World!".to_owned()))?;

    for i in u64::MIN..=u64::MIN + 1000 {
        round_trip_match::<BarOut, BarIn>(BarOut::YRequired(i))?;
    }
    for i in u64::MAX / 2 - 1000..=u64::MAX / 2 + 1000 {
        round_trip_match::<BarOut, BarIn>(BarOut::YRequired(i))?;
    }
    for i in u64::MAX - 1000..=u64::MAX {
        round_trip_match::<BarOut, BarIn>(BarOut::YRequired(i))?;
    }

    round_trip_match::<BarOut, BarIn>(BarOut::ZRequired)?;

    let fallback = BarOut::TRequired(true);

    round_trip_match::<BarOut, BarIn>(BarOut::PUnstable(
        vec![(), (), ()],
        Box::new(fallback.clone()),
    ))?;
    round_trip_match::<BarOut, BarIn>(BarOut::QUnstable(
        vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
        Box::new(fallback.clone()),
    ))?;
    round_trip_match::<BarOut, BarIn>(BarOut::RUnstable(
        vec![i64::MIN, 0, i64::MAX],
        Box::new(fallback.clone()),
    ))?;
    round_trip_match::<BarOut, BarIn>(BarOut::SUnstable(
        vec![
            vec!["Hello".to_owned(), "World".to_owned()],
            vec!["Hello".to_owned(), "Earth".to_owned()],
            vec!["Hello".to_owned(), "Planet".to_owned()],
        ],
        Box::new(fallback.clone()),
    ))?;
    round_trip_match::<BarOut, BarIn>(BarOut::TUnstable(true, Box::new(fallback.clone())))?;
    round_trip_match::<BarOut, BarIn>(BarOut::UUnstable(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;
    round_trip_match::<BarOut, BarIn>(BarOut::VUnstable(PI, Box::new(fallback.clone())))?;
    round_trip_match::<BarOut, BarIn>(BarOut::WUnstable(i64::MAX, Box::new(fallback.clone())))?;
    round_trip_match::<BarOut, BarIn>(BarOut::XUnstable(
        "Hello, World!".to_owned(),
        Box::new(fallback.clone()),
    ))?;
    round_trip_match::<BarOut, BarIn>(BarOut::YUnstable(u64::MAX, Box::new(fallback.clone())))?;
    round_trip_match::<BarOut, BarIn>(BarOut::ZUnstable(Box::new(fallback.clone())))?;

    round_trip_match::<BarOut, BarIn>(BarOut::POptional(
        vec![(), (), ()],
        Box::new(fallback.clone()),
    ))?;
    round_trip_match::<BarOut, BarIn>(BarOut::QOptional(
        vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
        Box::new(fallback.clone()),
    ))?;
    round_trip_match::<BarOut, BarIn>(BarOut::ROptional(
        vec![i64::MIN, 0, i64::MAX],
        Box::new(fallback.clone()),
    ))?;
    round_trip_match::<BarOut, BarIn>(BarOut::SOptional(
        vec![
            vec!["Hello".to_owned(), "World".to_owned()],
            vec!["Hello".to_owned(), "Earth".to_owned()],
            vec!["Hello".to_owned(), "Planet".to_owned()],
        ],
        Box::new(fallback.clone()),
    ))?;
    round_trip_match::<BarOut, BarIn>(BarOut::TOptional(true, Box::new(fallback.clone())))?;
    round_trip_match::<BarOut, BarIn>(BarOut::UOptional(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;
    round_trip_match::<BarOut, BarIn>(BarOut::VOptional(PI, Box::new(fallback.clone())))?;
    round_trip_match::<BarOut, BarIn>(BarOut::WOptional(i64::MAX, Box::new(fallback.clone())))?;
    round_trip_match::<BarOut, BarIn>(BarOut::XOptional(
        "Hello, World!".to_owned(),
        Box::new(fallback.clone()),
    ))?;
    round_trip_match::<BarOut, BarIn>(BarOut::YOptional(u64::MAX, Box::new(fallback.clone())))?;
    round_trip_match::<BarOut, BarIn>(BarOut::ZOptional(Box::new(fallback)))?;

    println!();

    round_trip_match::<FooAndBarOut, FooAndBarIn>(FooAndBarOut {
        x: FooOut {
            p_required: vec![(), (), ()],
            q_required: vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
            r_required: vec![i64::MIN, 0, i64::MAX],
            s_required: vec![
                vec!["Hello".to_owned(), "World".to_owned()],
                vec!["Hello".to_owned(), "Earth".to_owned()],
                vec!["Hello".to_owned(), "Planet".to_owned()],
            ],
            t_required: true,
            u_required: vec![0, 42, 255],
            v_required: PI,
            w_required: i64::MAX,
            x_required: "Hello, World!".to_owned(),
            y_required: u64::MAX,
            z_required: (),

            p_unstable: vec![(), (), ()],
            q_unstable: vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
            r_unstable: vec![i64::MIN, 0, i64::MAX],
            s_unstable: vec![
                vec!["Hello".to_owned(), "World".to_owned()],
                vec!["Hello".to_owned(), "Earth".to_owned()],
                vec!["Hello".to_owned(), "Planet".to_owned()],
            ],
            t_unstable: true,
            u_unstable: vec![0, 42, 255],
            v_unstable: PI,
            w_unstable: i64::MAX,
            x_unstable: "Hello, World!".to_owned(),
            y_unstable: u64::MAX,
            z_unstable: (),

            p_optional: None,
            q_optional: None,
            r_optional: None,
            s_optional: None,
            t_optional: None,
            u_optional: None,
            v_optional: None,
            w_optional: None,
            x_optional: None,
            y_optional: None,
            z_optional: None,
        },
        y: BarOut::TRequired(true),
    })?;

    println!();

    round_trip_match::<FooOrBarOut, FooOrBarIn>(FooOrBarOut::Y(BarOut::TRequired(true)))?;

    println!();

    round_trip_match::<EmptyStructOut, EmptyStructIn>(EmptyStructOut {})?;

    Ok(())
}

fn round_trip_match<T: Debug + Serialize, U: Debug + Deserialize + From<T>>(
    x: T,
) -> io::Result<()> {
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