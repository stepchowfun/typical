use {
    crate::{
        round_trip::check_match,
        types::comprehensive::{
            bar::{BarIn, BarOut},
            foo::{FooIn, FooOut},
            main::{
                EmptyStructIn, EmptyStructOut, FooAndBarIn, FooAndBarOut, FooOrBarIn, FooOrBarOut,
            },
        },
    },
    std::{
        f64::consts::{E, PI},
        io,
    },
};

#[allow(clippy::too_many_lines)]
pub fn run() -> io::Result<()> {
    check_match::<FooOut, FooIn>(FooOut {
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

        p_asymmetric: vec![(), (), ()],
        q_asymmetric: vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
        r_asymmetric: vec![i64::MIN, 0, i64::MAX],
        s_asymmetric: vec![
            vec!["Hello".to_owned(), "World".to_owned()],
            vec!["Hello".to_owned(), "Earth".to_owned()],
            vec!["Hello".to_owned(), "Planet".to_owned()],
        ],
        t_asymmetric: true,
        u_asymmetric: vec![0, 42, 255],
        v_asymmetric: PI,
        w_asymmetric: i64::MAX,
        x_asymmetric: "Hello, World!".to_owned(),
        y_asymmetric: u64::MAX,
        z_asymmetric: (),

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

    check_match::<FooOut, FooIn>(FooOut {
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

        p_asymmetric: vec![(), (), ()],
        q_asymmetric: vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
        r_asymmetric: vec![i64::MIN, 0, i64::MAX],
        s_asymmetric: vec![
            vec!["Hello".to_owned(), "World".to_owned()],
            vec!["Hello".to_owned(), "Earth".to_owned()],
            vec!["Hello".to_owned(), "Planet".to_owned()],
        ],
        t_asymmetric: true,
        u_asymmetric: vec![0, 42, 255],
        v_asymmetric: PI,
        w_asymmetric: i64::MAX,
        x_asymmetric: "Hello, World!".to_owned(),
        y_asymmetric: u64::MAX,
        z_asymmetric: (),

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

    check_match::<BarOut, BarIn>(BarOut::PRequired(vec![]))?;
    check_match::<BarOut, BarIn>(BarOut::PRequired(vec![()]))?;
    check_match::<BarOut, BarIn>(BarOut::PRequired(vec![(), ()]))?;
    check_match::<BarOut, BarIn>(BarOut::PRequired(vec![(), (), ()]))?;

    check_match::<BarOut, BarIn>(BarOut::QRequired(vec![]))?;
    check_match::<BarOut, BarIn>(BarOut::QRequired(vec![f64::NEG_INFINITY]))?;
    check_match::<BarOut, BarIn>(BarOut::QRequired(vec![f64::NEG_INFINITY, f64::INFINITY]))?;
    check_match::<BarOut, BarIn>(BarOut::QRequired(vec![
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NAN,
    ]))?;

    check_match::<BarOut, BarIn>(BarOut::RRequired(vec![]))?;
    check_match::<BarOut, BarIn>(BarOut::RRequired(vec![i64::MIN]))?;
    check_match::<BarOut, BarIn>(BarOut::RRequired(vec![i64::MIN, 0]))?;
    check_match::<BarOut, BarIn>(BarOut::RRequired(vec![i64::MIN, 0, i64::MAX]))?;

    check_match::<BarOut, BarIn>(BarOut::SRequired(vec![]))?;
    check_match::<BarOut, BarIn>(BarOut::SRequired(vec![vec![]]))?;
    check_match::<BarOut, BarIn>(BarOut::SRequired(vec![vec![], vec![]]))?;
    check_match::<BarOut, BarIn>(BarOut::SRequired(vec![vec![], vec![], vec![]]))?;
    check_match::<BarOut, BarIn>(BarOut::SRequired(vec![vec![
        "Hello".to_owned(),
        "World".to_owned(),
    ]]))?;
    check_match::<BarOut, BarIn>(BarOut::SRequired(vec![
        vec!["Hello".to_owned(), "World".to_owned()],
        vec!["Hello".to_owned(), "Earth".to_owned()],
    ]))?;
    check_match::<BarOut, BarIn>(BarOut::SRequired(vec![
        vec!["Hello".to_owned(), "World".to_owned()],
        vec!["Hello".to_owned(), "Earth".to_owned()],
        vec!["Hello".to_owned(), "Planet".to_owned()],
    ]))?;

    check_match::<BarOut, BarIn>(BarOut::TRequired(false))?;
    check_match::<BarOut, BarIn>(BarOut::TRequired(true))?;

    check_match::<BarOut, BarIn>(BarOut::URequired(vec![]))?;
    check_match::<BarOut, BarIn>(BarOut::URequired(vec![0]))?;
    check_match::<BarOut, BarIn>(BarOut::URequired(vec![0, 42]))?;
    check_match::<BarOut, BarIn>(BarOut::URequired(vec![0, 42, 255]))?;

    check_match::<BarOut, BarIn>(BarOut::VRequired(0.0_f64))?;
    check_match::<BarOut, BarIn>(BarOut::VRequired(E))?;
    check_match::<BarOut, BarIn>(BarOut::VRequired(PI))?;
    check_match::<BarOut, BarIn>(BarOut::VRequired(f64::EPSILON))?;
    check_match::<BarOut, BarIn>(BarOut::VRequired(f64::INFINITY))?;
    check_match::<BarOut, BarIn>(BarOut::VRequired(f64::MAX))?;
    check_match::<BarOut, BarIn>(BarOut::VRequired(f64::MIN))?;
    check_match::<BarOut, BarIn>(BarOut::VRequired(f64::MIN_POSITIVE))?;
    check_match::<BarOut, BarIn>(BarOut::VRequired(f64::NAN))?;
    check_match::<BarOut, BarIn>(BarOut::VRequired(f64::NEG_INFINITY))?;

    for i in i64::MIN..=i64::MIN + 1000 {
        check_match::<BarOut, BarIn>(BarOut::WRequired(i))?;
    }
    for i in -1000_i64..=1000_i64 {
        check_match::<BarOut, BarIn>(BarOut::WRequired(i))?;
    }
    for i in i64::MAX - 1000..=i64::MAX {
        check_match::<BarOut, BarIn>(BarOut::WRequired(i))?;
    }

    check_match::<BarOut, BarIn>(BarOut::XRequired("".to_owned()))?;
    check_match::<BarOut, BarIn>(BarOut::XRequired("=8 bytes".to_owned()))?;
    check_match::<BarOut, BarIn>(BarOut::XRequired("Hello, World!".to_owned()))?;

    for i in u64::MIN..=u64::MIN + 1000 {
        check_match::<BarOut, BarIn>(BarOut::YRequired(i))?;
    }
    for i in u64::MAX / 2 - 1000..=u64::MAX / 2 + 1000 {
        check_match::<BarOut, BarIn>(BarOut::YRequired(i))?;
    }
    for i in u64::MAX - 1000..=u64::MAX {
        check_match::<BarOut, BarIn>(BarOut::YRequired(i))?;
    }

    check_match::<BarOut, BarIn>(BarOut::ZRequired)?;

    let fallback = BarOut::TRequired(true);

    check_match::<BarOut, BarIn>(BarOut::PAsymmetric(
        vec![(), (), ()],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::QAsymmetric(
        vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::RAsymmetric(
        vec![i64::MIN, 0, i64::MAX],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::SAsymmetric(
        vec![
            vec!["Hello".to_owned(), "World".to_owned()],
            vec!["Hello".to_owned(), "Earth".to_owned()],
            vec!["Hello".to_owned(), "Planet".to_owned()],
        ],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::TAsymmetric(true, Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::UAsymmetric(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::VAsymmetric(PI, Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::WAsymmetric(i64::MAX, Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::XAsymmetric(
        "Hello, World!".to_owned(),
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::YAsymmetric(u64::MAX, Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::ZAsymmetric(Box::new(fallback.clone())))?;

    check_match::<BarOut, BarIn>(BarOut::POptional(
        vec![(), (), ()],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::QOptional(
        vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::ROptional(
        vec![i64::MIN, 0, i64::MAX],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::SOptional(
        vec![
            vec!["Hello".to_owned(), "World".to_owned()],
            vec!["Hello".to_owned(), "Earth".to_owned()],
            vec!["Hello".to_owned(), "Planet".to_owned()],
        ],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::TOptional(true, Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::UOptional(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::VOptional(PI, Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::WOptional(i64::MAX, Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::XOptional(
        "Hello, World!".to_owned(),
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::YOptional(u64::MAX, Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::ZOptional(Box::new(fallback)))?;

    println!();

    check_match::<FooAndBarOut, FooAndBarIn>(FooAndBarOut {
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

            p_asymmetric: vec![(), (), ()],
            q_asymmetric: vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
            r_asymmetric: vec![i64::MIN, 0, i64::MAX],
            s_asymmetric: vec![
                vec!["Hello".to_owned(), "World".to_owned()],
                vec!["Hello".to_owned(), "Earth".to_owned()],
                vec!["Hello".to_owned(), "Planet".to_owned()],
            ],
            t_asymmetric: true,
            u_asymmetric: vec![0, 42, 255],
            v_asymmetric: PI,
            w_asymmetric: i64::MAX,
            x_asymmetric: "Hello, World!".to_owned(),
            y_asymmetric: u64::MAX,
            z_asymmetric: (),

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

    check_match::<FooOrBarOut, FooOrBarIn>(FooOrBarOut::Y(BarOut::TRequired(true)))?;

    println!();

    check_match::<EmptyStructOut, EmptyStructIn>(EmptyStructOut {})?;

    Ok(())
}
