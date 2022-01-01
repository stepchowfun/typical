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
    std::{f64::consts::PI, io},
};

const F64_TEST_VALUES: &[f64] = &[
    0.0,
    PI,
    f64::EPSILON,
    f64::INFINITY,
    f64::MAX,
    f64::MIN,
    f64::MIN_POSITIVE,
    f64::NAN,
    f64::NEG_INFINITY,
];

const U64_TEST_VALUES: &[u64] = &[
    u64::MIN,
    127,
    128,
    16_511,
    16_512,
    2_113_663,
    2_113_664,
    270_549_119,
    270_549_120,
    34_630_287_487,
    34_630_287_488,
    4_432_676_798_591,
    4_432_676_798_592,
    567_382_630_219_903,
    567_382_630_219_904,
    72_624_976_668_147_839,
    72_624_976_668_147_840,
    u64::MAX,
];

const S64_TEST_VALUES: &[i64] = &[
    0,
    -64,
    64,
    -8_256,
    8_256,
    -1_056_832,
    1_056_832,
    -135_274_560,
    135_274_560,
    -17_315_143_744,
    17_315_143_744,
    -2_216_338_399_296,
    2_216_338_399_296,
    -283_691_315_109_952,
    283_691_315_109_952,
    -36_312_488_334_073_920,
    36_312_488_334_073_920,
    i64::MIN,
    i64::MAX,
];

#[allow(clippy::too_many_lines)]
pub fn run() -> io::Result<()> {
    check_match::<FooOut, FooIn>(FooOut {
        a_required: (),
        b_required: PI,
        c_required: u64::MAX,
        d_required: i64::MAX,
        e_required: true,
        f_required: vec![0, 42, 255],
        g_required: "Hello, World!".to_owned(),
        h_required: vec![(), (), ()],
        i_required: F64_TEST_VALUES.to_owned(),
        j_required: U64_TEST_VALUES.to_owned(),
        k_required: S64_TEST_VALUES.to_owned(),
        l_required: vec![false, true, false],
        m_required: vec![vec![0, 42, 255], vec![1, 43, 254], vec![2, 44, 253]],
        n_required: vec![
            "".to_owned(),
            "=8 bytes".to_owned(),
            "Hello, World!".to_owned(),
        ],
        o_required: vec![
            vec![],
            vec![EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        ],

        a_asymmetric: (),
        b_asymmetric: PI,
        c_asymmetric: u64::MAX,
        d_asymmetric: i64::MAX,
        e_asymmetric: true,
        f_asymmetric: vec![0, 42, 255],
        g_asymmetric: "Hello, World!".to_owned(),
        h_asymmetric: vec![(), (), ()],
        i_asymmetric: F64_TEST_VALUES.to_owned(),
        j_asymmetric: U64_TEST_VALUES.to_owned(),
        k_asymmetric: S64_TEST_VALUES.to_owned(),
        l_asymmetric: vec![false, true, false],
        m_asymmetric: vec![vec![0, 42, 255], vec![1, 43, 254], vec![2, 44, 253]],
        n_asymmetric: vec![
            "".to_owned(),
            "=8 bytes".to_owned(),
            "Hello, World!".to_owned(),
        ],
        o_asymmetric: vec![
            vec![],
            vec![EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        ],

        a_optional: None,
        b_optional: None,
        c_optional: None,
        d_optional: None,
        e_optional: None,
        f_optional: None,
        g_optional: None,
        h_optional: None,
        i_optional: None,
        j_optional: None,
        k_optional: None,
        l_optional: None,
        m_optional: None,
        n_optional: None,
        o_optional: None,
    })?;

    println!();

    check_match::<FooOut, FooIn>(FooOut {
        a_required: (),
        b_required: PI,
        c_required: u64::MAX,
        d_required: i64::MAX,
        e_required: true,
        f_required: vec![0, 42, 255],
        g_required: "Hello, World!".to_owned(),
        h_required: vec![(), (), ()],
        i_required: vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
        j_required: U64_TEST_VALUES.to_owned(),
        k_required: S64_TEST_VALUES.to_owned(),
        l_required: vec![false, true, false],
        m_required: vec![vec![0, 42, 255], vec![1, 43, 254], vec![2, 44, 253]],
        n_required: vec![
            "".to_owned(),
            "=8 bytes".to_owned(),
            "Hello, World!".to_owned(),
        ],
        o_required: vec![
            vec![],
            vec![EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        ],

        a_asymmetric: (),
        b_asymmetric: PI,
        c_asymmetric: u64::MAX,
        d_asymmetric: i64::MAX,
        e_asymmetric: true,
        f_asymmetric: vec![0, 42, 255],
        g_asymmetric: "Hello, World!".to_owned(),
        h_asymmetric: vec![(), (), ()],
        i_asymmetric: vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN],
        j_asymmetric: U64_TEST_VALUES.to_owned(),
        k_asymmetric: S64_TEST_VALUES.to_owned(),
        l_asymmetric: vec![false, true, false],
        m_asymmetric: vec![vec![0, 42, 255], vec![1, 43, 254], vec![2, 44, 253]],
        n_asymmetric: vec![
            "".to_owned(),
            "=8 bytes".to_owned(),
            "Hello, World!".to_owned(),
        ],
        o_asymmetric: vec![
            vec![],
            vec![EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        ],

        a_optional: Some(()),
        b_optional: Some(PI),
        c_optional: Some(u64::MAX),
        d_optional: Some(i64::MAX),
        e_optional: Some(true),
        f_optional: Some(vec![0, 42, 255]),
        g_optional: Some("Hello, World!".to_owned()),
        h_optional: Some(vec![(), (), ()]),
        i_optional: Some(vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN]),
        j_optional: Some(U64_TEST_VALUES.to_owned()),
        k_optional: Some(S64_TEST_VALUES.to_owned()),
        l_optional: Some(vec![false, true, false]),
        m_optional: Some(vec![vec![0, 42, 255], vec![1, 43, 254], vec![2, 44, 253]]),
        n_optional: Some(vec![
            "".to_owned(),
            "=8 bytes".to_owned(),
            "Hello, World!".to_owned(),
        ]),
        o_optional: Some(vec![
            vec![],
            vec![EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        ]),
    })?;

    println!();

    check_match::<BarOut, BarIn>(BarOut::ARequired)?;

    for &value in F64_TEST_VALUES {
        check_match::<BarOut, BarIn>(BarOut::BRequired(value))?;
    }

    for &value in U64_TEST_VALUES {
        check_match::<BarOut, BarIn>(BarOut::CRequired(value))?;
    }

    for &value in S64_TEST_VALUES {
        check_match::<BarOut, BarIn>(BarOut::DRequired(value))?;
    }

    check_match::<BarOut, BarIn>(BarOut::ERequired(false))?;
    check_match::<BarOut, BarIn>(BarOut::ERequired(true))?;

    check_match::<BarOut, BarIn>(BarOut::FRequired(vec![]))?;
    check_match::<BarOut, BarIn>(BarOut::FRequired(vec![0]))?;
    check_match::<BarOut, BarIn>(BarOut::FRequired(vec![0, 42]))?;
    check_match::<BarOut, BarIn>(BarOut::FRequired(vec![0, 42, 255]))?;

    check_match::<BarOut, BarIn>(BarOut::GRequired("".to_owned()))?;
    check_match::<BarOut, BarIn>(BarOut::GRequired("=8 bytes".to_owned()))?;
    check_match::<BarOut, BarIn>(BarOut::GRequired("Hello, World!".to_owned()))?;

    check_match::<BarOut, BarIn>(BarOut::HRequired(vec![]))?;
    check_match::<BarOut, BarIn>(BarOut::HRequired(vec![()]))?;
    check_match::<BarOut, BarIn>(BarOut::HRequired(vec![(), ()]))?;
    check_match::<BarOut, BarIn>(BarOut::HRequired(vec![(), (), ()]))?;

    check_match::<BarOut, BarIn>(BarOut::IRequired(F64_TEST_VALUES.to_owned()))?;

    check_match::<BarOut, BarIn>(BarOut::JRequired(U64_TEST_VALUES.to_owned()))?;

    check_match::<BarOut, BarIn>(BarOut::KRequired(S64_TEST_VALUES.to_owned()))?;

    check_match::<BarOut, BarIn>(BarOut::LRequired(vec![]))?;
    check_match::<BarOut, BarIn>(BarOut::LRequired(vec![false]))?;
    check_match::<BarOut, BarIn>(BarOut::LRequired(vec![false, true]))?;
    check_match::<BarOut, BarIn>(BarOut::LRequired(vec![false, true, false]))?;

    check_match::<BarOut, BarIn>(BarOut::MRequired(vec![]))?;
    check_match::<BarOut, BarIn>(BarOut::MRequired(vec![vec![0, 45, 255]]))?;
    check_match::<BarOut, BarIn>(BarOut::MRequired(vec![vec![0, 45, 255], vec![1, 43, 254]]))?;
    check_match::<BarOut, BarIn>(BarOut::MRequired(vec![
        vec![0, 45, 255],
        vec![1, 43, 254],
        vec![2, 44, 253],
    ]))?;

    check_match::<BarOut, BarIn>(BarOut::NRequired(vec![]))?;
    check_match::<BarOut, BarIn>(BarOut::NRequired(vec!["".to_owned()]))?;
    check_match::<BarOut, BarIn>(BarOut::NRequired(vec![
        "".to_owned(),
        "=8 bytes".to_owned(),
    ]))?;
    check_match::<BarOut, BarIn>(BarOut::NRequired(vec![
        "".to_owned(),
        "=8 bytes".to_owned(),
        "Hello, World!".to_owned(),
    ]))?;

    check_match::<BarOut, BarIn>(BarOut::ORequired(vec![]))?;
    check_match::<BarOut, BarIn>(BarOut::ORequired(vec![vec![]]))?;
    check_match::<BarOut, BarIn>(BarOut::ORequired(vec![vec![], vec![]]))?;
    check_match::<BarOut, BarIn>(BarOut::ORequired(vec![vec![], vec![], vec![]]))?;
    check_match::<BarOut, BarIn>(BarOut::ORequired(vec![vec![EmptyStructOut {}]]))?;
    check_match::<BarOut, BarIn>(BarOut::ORequired(vec![
        vec![EmptyStructOut {}],
        vec![EmptyStructOut {}, EmptyStructOut {}],
    ]))?;
    check_match::<BarOut, BarIn>(BarOut::ORequired(vec![
        vec![EmptyStructOut {}],
        vec![EmptyStructOut {}, EmptyStructOut {}],
        vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
    ]))?;

    let fallback = BarOut::ARequired;

    check_match::<BarOut, BarIn>(BarOut::AAsymmetric(Box::new(fallback.clone())))?;

    for &value in F64_TEST_VALUES {
        check_match::<BarOut, BarIn>(BarOut::BAsymmetric(value, Box::new(fallback.clone())))?;
    }

    for &value in U64_TEST_VALUES {
        check_match::<BarOut, BarIn>(BarOut::CAsymmetric(value, Box::new(fallback.clone())))?;
    }

    for &value in S64_TEST_VALUES {
        check_match::<BarOut, BarIn>(BarOut::DAsymmetric(value, Box::new(fallback.clone())))?;
    }

    check_match::<BarOut, BarIn>(BarOut::EAsymmetric(false, Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::EAsymmetric(true, Box::new(fallback.clone())))?;

    check_match::<BarOut, BarIn>(BarOut::FAsymmetric(vec![], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::FAsymmetric(vec![0], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::FAsymmetric(vec![0, 42], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::FAsymmetric(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::GAsymmetric(
        "".to_owned(),
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::GAsymmetric(
        "=8 bytes".to_owned(),
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::GAsymmetric(
        "Hello, World!".to_owned(),
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::HAsymmetric(vec![], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::HAsymmetric(vec![()], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::HAsymmetric(
        vec![(), ()],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::HAsymmetric(
        vec![(), (), ()],
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::IAsymmetric(
        F64_TEST_VALUES.to_owned(),
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::JAsymmetric(
        U64_TEST_VALUES.to_owned(),
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::KAsymmetric(
        S64_TEST_VALUES.to_owned(),
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::LAsymmetric(vec![], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::LAsymmetric(vec![false], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::LAsymmetric(
        vec![false, true],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::LAsymmetric(
        vec![false, true, false],
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::MAsymmetric(vec![], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::MAsymmetric(
        vec![vec![0, 45, 255]],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::MAsymmetric(
        vec![vec![0, 45, 255], vec![1, 43, 254]],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::MAsymmetric(
        vec![vec![0, 45, 255], vec![1, 43, 254], vec![2, 44, 253]],
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::NAsymmetric(vec![], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::NAsymmetric(
        vec!["".to_owned()],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::NAsymmetric(
        vec!["".to_owned(), "=8 bytes".to_owned()],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::NAsymmetric(
        vec![
            "".to_owned(),
            "=8 bytes".to_owned(),
            "Hello, World!".to_owned(),
        ],
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::OAsymmetric(vec![], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::OAsymmetric(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::OAsymmetric(
        vec![vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::OAsymmetric(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::OAsymmetric(
        vec![vec![EmptyStructOut {}]],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::OAsymmetric(
        vec![
            vec![EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}],
        ],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::OAsymmetric(
        vec![
            vec![EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        ],
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::AOptional(Box::new(fallback.clone())))?;

    for &value in F64_TEST_VALUES {
        check_match::<BarOut, BarIn>(BarOut::BOptional(value, Box::new(fallback.clone())))?;
    }

    for &value in U64_TEST_VALUES {
        check_match::<BarOut, BarIn>(BarOut::COptional(value, Box::new(fallback.clone())))?;
    }

    for &value in S64_TEST_VALUES {
        check_match::<BarOut, BarIn>(BarOut::DOptional(value, Box::new(fallback.clone())))?;
    }

    check_match::<BarOut, BarIn>(BarOut::EOptional(false, Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::EOptional(true, Box::new(fallback.clone())))?;

    check_match::<BarOut, BarIn>(BarOut::FOptional(vec![], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::FOptional(vec![0], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::FOptional(vec![0, 42], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::FOptional(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::GOptional("".to_owned(), Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::GOptional(
        "=8 bytes".to_owned(),
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::GOptional(
        "Hello, World!".to_owned(),
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::HOptional(vec![], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::HOptional(vec![()], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::HOptional(vec![(), ()], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::HOptional(
        vec![(), (), ()],
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::IOptional(
        F64_TEST_VALUES.to_owned(),
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::JOptional(
        U64_TEST_VALUES.to_owned(),
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::KOptional(
        S64_TEST_VALUES.to_owned(),
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::LOptional(vec![], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::LOptional(vec![false], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::LOptional(
        vec![false, true],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::LOptional(
        vec![false, true, false],
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::MOptional(vec![], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::MOptional(
        vec![vec![0, 45, 255]],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::MOptional(
        vec![vec![0, 45, 255], vec![1, 43, 254]],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::MOptional(
        vec![vec![0, 45, 255], vec![1, 43, 254], vec![2, 44, 253]],
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::NOptional(vec![], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::NOptional(
        vec!["".to_owned()],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::NOptional(
        vec!["".to_owned(), "=8 bytes".to_owned()],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::NOptional(
        vec![
            "".to_owned(),
            "=8 bytes".to_owned(),
            "Hello, World!".to_owned(),
        ],
        Box::new(fallback.clone()),
    ))?;

    check_match::<BarOut, BarIn>(BarOut::OOptional(vec![], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::OOptional(vec![vec![]], Box::new(fallback.clone())))?;
    check_match::<BarOut, BarIn>(BarOut::OOptional(
        vec![vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::OOptional(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::OOptional(
        vec![vec![EmptyStructOut {}]],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::OOptional(
        vec![
            vec![EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}],
        ],
        Box::new(fallback.clone()),
    ))?;
    check_match::<BarOut, BarIn>(BarOut::OOptional(
        vec![
            vec![],
            vec![EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        ],
        Box::new(fallback),
    ))?;

    println!();

    check_match::<FooAndBarOut, FooAndBarIn>(FooAndBarOut {
        x: FooOut {
            a_required: (),
            b_required: PI,
            c_required: u64::MAX,
            d_required: i64::MAX,
            e_required: true,
            f_required: vec![0, 42, 255],
            g_required: "Hello, World!".to_owned(),
            h_required: vec![(), (), ()],
            i_required: F64_TEST_VALUES.to_owned(),
            j_required: U64_TEST_VALUES.to_owned(),
            k_required: S64_TEST_VALUES.to_owned(),
            l_required: vec![false, true, false],
            m_required: vec![vec![0, 42, 255], vec![1, 43, 254], vec![2, 44, 253]],
            n_required: vec![
                "".to_owned(),
                "=8 bytes".to_owned(),
                "Hello, World!".to_owned(),
            ],
            o_required: vec![
                vec![],
                vec![EmptyStructOut {}],
                vec![EmptyStructOut {}, EmptyStructOut {}],
                vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
            ],

            a_asymmetric: (),
            b_asymmetric: PI,
            c_asymmetric: u64::MAX,
            d_asymmetric: i64::MAX,
            e_asymmetric: true,
            f_asymmetric: vec![0, 42, 255],
            g_asymmetric: "Hello, World!".to_owned(),
            h_asymmetric: vec![(), (), ()],
            i_asymmetric: F64_TEST_VALUES.to_owned(),
            j_asymmetric: U64_TEST_VALUES.to_owned(),
            k_asymmetric: S64_TEST_VALUES.to_owned(),
            l_asymmetric: vec![false, true, false],
            m_asymmetric: vec![vec![0, 42, 255], vec![1, 43, 254], vec![2, 44, 253]],
            n_asymmetric: vec![
                "".to_owned(),
                "=8 bytes".to_owned(),
                "Hello, World!".to_owned(),
            ],
            o_asymmetric: vec![
                vec![],
                vec![EmptyStructOut {}],
                vec![EmptyStructOut {}, EmptyStructOut {}],
                vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
            ],

            a_optional: None,
            b_optional: None,
            c_optional: None,
            d_optional: None,
            e_optional: None,
            f_optional: None,
            g_optional: None,
            h_optional: None,
            i_optional: None,
            j_optional: None,
            k_optional: None,
            l_optional: None,
            m_optional: None,
            n_optional: None,
            o_optional: None,
        },
        y: BarOut::ARequired,
    })?;

    println!();

    check_match::<FooOrBarOut, FooOrBarIn>(FooOrBarOut::Y(BarOut::ARequired))?;

    println!();

    check_match::<EmptyStructOut, EmptyStructIn>(EmptyStructOut {})?;

    Ok(())
}
