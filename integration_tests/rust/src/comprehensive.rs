use {
    crate::{
        assertions::assert_round_trip,
        types::{
            comprehensive::types::{BarIn, BarOut, FooIn, FooOut, LocalStructOut},
            degenerate::types::EmptyStructOut,
        },
    },
    std::{f64::consts::PI, io},
};

const F64_TEST_VALUES: &[f64] = &[
    0.0,
    -0.0,
    PI,
    f64::EPSILON,
    f64::INFINITY,
    f64::MAX,
    f64::MIN,
    5e-324_f64,        // Smallest positive (subnormal) value
    f64::MIN_POSITIVE, // Smallest possible normal value
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
    assert_round_trip::<FooOut, FooIn>(&FooOut {
        a_required: (),
        b_required: PI,
        c_required: u64::MAX,
        d_required: i64::MAX,
        e_required: true,
        f_required: vec![0, 42, 255],
        g_required: "Hello, \u{5e78}\u{798f}!".to_owned(),
        h_required: LocalStructOut {},
        i_required: EmptyStructOut {},
        j_required: vec![(), (), ()],
        k_required: F64_TEST_VALUES.to_owned(),
        l_required: U64_TEST_VALUES.to_owned(),
        m_required: S64_TEST_VALUES.to_owned(),
        n_required: vec![false, true, false],
        o_required: vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        p_required: vec![
            String::new(),
            "=8 bytes".to_owned(),
            "Hello, \u{5e78}\u{798f}!".to_owned(),
        ],
        q_required: vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        r_required: vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        s_required: vec![vec![], vec![()], vec![(), ()], vec![(), (), ()]],
        t_required: vec![
            vec![],
            vec![0.0_f64],
            vec![0.0_f64, PI],
            vec![0.0_f64, PI, f64::EPSILON],
            F64_TEST_VALUES.to_owned(),
        ],
        u_required: vec![
            vec![],
            vec![u64::MIN],
            vec![u64::MIN, 256],
            vec![u64::MIN, 256, u64::MAX],
            U64_TEST_VALUES.to_owned(),
        ],
        v_required: vec![
            vec![],
            vec![i64::MIN],
            vec![i64::MIN, 0],
            vec![i64::MIN, 0, i64::MAX],
            S64_TEST_VALUES.to_owned(),
        ],
        w_required: vec![
            vec![],
            vec![false],
            vec![false, true],
            vec![false, true, false],
        ],
        x_required: vec![
            vec![],
            vec![vec![]],
            vec![vec![], vec![0, 42, 255]],
            vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        ],
        y_required: vec![
            vec![String::new()],
            vec![String::new(), "=8 bytes".to_owned()],
            vec![
                String::new(),
                "=8 bytes".to_owned(),
                "Hello, \u{5e78}\u{798f}!".to_owned(),
            ],
        ],
        z_required: vec![
            vec![],
            vec![LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        ],
        aa_required: vec![
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
        g_asymmetric: "Hello, \u{5e78}\u{798f}!".to_owned(),
        h_asymmetric: LocalStructOut {},
        i_asymmetric: EmptyStructOut {},
        j_asymmetric: vec![(), (), ()],
        k_asymmetric: F64_TEST_VALUES.to_owned(),
        l_asymmetric: U64_TEST_VALUES.to_owned(),
        m_asymmetric: S64_TEST_VALUES.to_owned(),
        n_asymmetric: vec![false, true, false],
        o_asymmetric: vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        p_asymmetric: vec![
            String::new(),
            "=8 bytes".to_owned(),
            "Hello, \u{5e78}\u{798f}!".to_owned(),
        ],
        q_asymmetric: vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        r_asymmetric: vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        s_asymmetric: vec![vec![], vec![()], vec![(), ()], vec![(), (), ()]],
        t_asymmetric: vec![
            vec![],
            vec![0.0_f64],
            vec![0.0_f64, PI],
            vec![0.0_f64, PI, f64::EPSILON],
            F64_TEST_VALUES.to_owned(),
        ],
        u_asymmetric: vec![
            vec![],
            vec![u64::MIN],
            vec![u64::MIN, 256],
            vec![u64::MIN, 256, u64::MAX],
            U64_TEST_VALUES.to_owned(),
        ],
        v_asymmetric: vec![
            vec![],
            vec![i64::MIN],
            vec![i64::MIN, 0],
            vec![i64::MIN, 0, i64::MAX],
            S64_TEST_VALUES.to_owned(),
        ],
        w_asymmetric: vec![
            vec![],
            vec![false],
            vec![false, true],
            vec![false, true, false],
        ],
        x_asymmetric: vec![
            vec![],
            vec![vec![]],
            vec![vec![], vec![0, 42, 255]],
            vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        ],
        y_asymmetric: vec![
            vec![String::new()],
            vec![String::new(), "=8 bytes".to_owned()],
            vec![
                String::new(),
                "=8 bytes".to_owned(),
                "Hello, \u{5e78}\u{798f}!".to_owned(),
            ],
        ],
        z_asymmetric: vec![
            vec![],
            vec![LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        ],
        aa_asymmetric: vec![
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
        aa_optional: None,
    })?;

    println!();

    assert_round_trip::<FooOut, FooIn>(&FooOut {
        a_required: (),
        b_required: PI,
        c_required: u64::MAX,
        d_required: i64::MAX,
        e_required: true,
        f_required: vec![0, 42, 255],
        g_required: "Hello, \u{5e78}\u{798f}!".to_owned(),
        h_required: LocalStructOut {},
        i_required: EmptyStructOut {},
        j_required: vec![(), (), ()],
        k_required: F64_TEST_VALUES.to_owned(),
        l_required: U64_TEST_VALUES.to_owned(),
        m_required: S64_TEST_VALUES.to_owned(),
        n_required: vec![false, true, false],
        o_required: vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        p_required: vec![
            String::new(),
            "=8 bytes".to_owned(),
            "Hello, \u{5e78}\u{798f}!".to_owned(),
        ],
        q_required: vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        r_required: vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        s_required: vec![vec![], vec![()], vec![(), ()], vec![(), (), ()]],
        t_required: vec![
            vec![],
            vec![0.0_f64],
            vec![0.0_f64, PI],
            vec![0.0_f64, PI, f64::EPSILON],
            F64_TEST_VALUES.to_owned(),
        ],
        u_required: vec![
            vec![],
            vec![u64::MIN],
            vec![u64::MIN, 256],
            vec![u64::MIN, 256, u64::MAX],
            U64_TEST_VALUES.to_owned(),
        ],
        v_required: vec![
            vec![],
            vec![i64::MIN],
            vec![i64::MIN, 0],
            vec![i64::MIN, 0, i64::MAX],
            S64_TEST_VALUES.to_owned(),
        ],
        w_required: vec![
            vec![],
            vec![false],
            vec![false, true],
            vec![false, true, false],
        ],
        x_required: vec![
            vec![],
            vec![vec![]],
            vec![vec![], vec![0, 42, 255]],
            vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        ],
        y_required: vec![
            vec![String::new()],
            vec![String::new(), "=8 bytes".to_owned()],
            vec![
                String::new(),
                "=8 bytes".to_owned(),
                "Hello, \u{5e78}\u{798f}!".to_owned(),
            ],
        ],
        z_required: vec![
            vec![],
            vec![LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        ],
        aa_required: vec![
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
        g_asymmetric: "Hello, \u{5e78}\u{798f}!".to_owned(),
        h_asymmetric: LocalStructOut {},
        i_asymmetric: EmptyStructOut {},
        j_asymmetric: vec![(), (), ()],
        k_asymmetric: F64_TEST_VALUES.to_owned(),
        l_asymmetric: U64_TEST_VALUES.to_owned(),
        m_asymmetric: S64_TEST_VALUES.to_owned(),
        n_asymmetric: vec![false, true, false],
        o_asymmetric: vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        p_asymmetric: vec![
            String::new(),
            "=8 bytes".to_owned(),
            "Hello, \u{5e78}\u{798f}!".to_owned(),
        ],
        q_asymmetric: vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        r_asymmetric: vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        s_asymmetric: vec![vec![], vec![()], vec![(), ()], vec![(), (), ()]],
        t_asymmetric: vec![
            vec![],
            vec![0.0_f64],
            vec![0.0_f64, PI],
            vec![0.0_f64, PI, f64::EPSILON],
            F64_TEST_VALUES.to_owned(),
        ],
        u_asymmetric: vec![
            vec![],
            vec![u64::MIN],
            vec![u64::MIN, 256],
            vec![u64::MIN, 256, u64::MAX],
            U64_TEST_VALUES.to_owned(),
        ],
        v_asymmetric: vec![
            vec![],
            vec![i64::MIN],
            vec![i64::MIN, 0],
            vec![i64::MIN, 0, i64::MAX],
            S64_TEST_VALUES.to_owned(),
        ],
        w_asymmetric: vec![
            vec![],
            vec![false],
            vec![false, true],
            vec![false, true, false],
        ],
        x_asymmetric: vec![
            vec![],
            vec![vec![]],
            vec![vec![], vec![0, 42, 255]],
            vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        ],
        y_asymmetric: vec![
            vec![String::new()],
            vec![String::new(), "=8 bytes".to_owned()],
            vec![
                String::new(),
                "=8 bytes".to_owned(),
                "Hello, \u{5e78}\u{798f}!".to_owned(),
            ],
        ],
        z_asymmetric: vec![
            vec![],
            vec![LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        ],
        aa_asymmetric: vec![
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
        g_optional: Some("Hello, \u{5e78}\u{798f}!".to_owned()),
        h_optional: Some(LocalStructOut {}),
        i_optional: Some(EmptyStructOut {}),
        j_optional: Some(vec![(), (), ()]),
        k_optional: Some(F64_TEST_VALUES.to_owned()),
        l_optional: Some(U64_TEST_VALUES.to_owned()),
        m_optional: Some(S64_TEST_VALUES.to_owned()),
        n_optional: Some(vec![false, true, false]),
        o_optional: Some(vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]]),
        p_optional: Some(vec![
            String::new(),
            "=8 bytes".to_owned(),
            "Hello, \u{5e78}\u{798f}!".to_owned(),
        ]),
        q_optional: Some(vec![
            LocalStructOut {},
            LocalStructOut {},
            LocalStructOut {},
        ]),
        r_optional: Some(vec![
            EmptyStructOut {},
            EmptyStructOut {},
            EmptyStructOut {},
        ]),
        s_optional: Some(vec![vec![], vec![()], vec![(), ()], vec![(), (), ()]]),
        t_optional: Some(vec![
            vec![],
            vec![0.0_f64],
            vec![0.0_f64, PI],
            vec![0.0_f64, PI, f64::EPSILON],
            F64_TEST_VALUES.to_owned(),
        ]),
        u_optional: Some(vec![
            vec![],
            vec![u64::MIN],
            vec![u64::MIN, 256],
            vec![u64::MIN, 256, u64::MAX],
            U64_TEST_VALUES.to_owned(),
        ]),
        v_optional: Some(vec![
            vec![],
            vec![i64::MIN],
            vec![i64::MIN, 0],
            vec![i64::MIN, 0, i64::MAX],
            S64_TEST_VALUES.to_owned(),
        ]),
        w_optional: Some(vec![
            vec![],
            vec![false],
            vec![false, true],
            vec![false, true, false],
        ]),
        x_optional: Some(vec![
            vec![],
            vec![vec![]],
            vec![vec![], vec![0, 42, 255]],
            vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        ]),
        y_optional: Some(vec![
            vec![String::new()],
            vec![String::new(), "=8 bytes".to_owned()],
            vec![
                String::new(),
                "=8 bytes".to_owned(),
                "Hello, \u{5e78}\u{798f}!".to_owned(),
            ],
        ]),
        z_optional: Some(vec![
            vec![],
            vec![LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        ]),
        aa_optional: Some(vec![
            vec![],
            vec![EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        ]),
    })?;

    println!();

    assert_round_trip::<BarOut, BarIn>(&BarOut::ARequired)?;

    for &value in F64_TEST_VALUES {
        assert_round_trip::<BarOut, BarIn>(&BarOut::BRequired(value))?;
    }

    for &value in U64_TEST_VALUES {
        assert_round_trip::<BarOut, BarIn>(&BarOut::CRequired(value))?;
    }

    for &value in S64_TEST_VALUES {
        assert_round_trip::<BarOut, BarIn>(&BarOut::DRequired(value))?;
    }

    assert_round_trip::<BarOut, BarIn>(&BarOut::ERequired(false))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ERequired(true))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::FRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::FRequired(vec![0]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::FRequired(vec![0, 42]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::FRequired(vec![0, 42, 255]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::GRequired(String::new()))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::GRequired("=8 bytes".to_owned()))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::GRequired("Hello, \u{5e78}\u{798f}!".to_owned()))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::HRequired(LocalStructOut {}))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::IRequired(EmptyStructOut {}))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::JRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::JRequired(vec![()]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::JRequired(vec![(), ()]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::JRequired(vec![(), (), ()]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::KRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::KRequired(vec![0.0_f64]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::KRequired(vec![0.0_f64, PI]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::KRequired(vec![0.0_f64, PI, f64::EPSILON]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::KRequired(F64_TEST_VALUES.to_owned()))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::LRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::LRequired(vec![u64::MIN]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::LRequired(vec![u64::MIN, 256]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::LRequired(vec![u64::MIN, 256, u64::MAX]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::LRequired(U64_TEST_VALUES.to_owned()))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::MRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::MRequired(vec![i64::MIN]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::MRequired(vec![i64::MIN, 0]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::MRequired(vec![i64::MIN, 0, i64::MAX]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::MRequired(S64_TEST_VALUES.to_owned()))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::NRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::NRequired(vec![false]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::NRequired(vec![false, true]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::NRequired(vec![false, true, false]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::ORequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ORequired(vec![vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ORequired(vec![vec![], vec![0, 42, 255]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ORequired(vec![
        vec![],
        vec![0, 42, 255],
        vec![7, 6, 5, 4, 3, 2, 1, 0],
    ]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::PRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::PRequired(vec![String::new()]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::PRequired(vec![
        String::new(),
        "=8 bytes".to_owned(),
    ]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::PRequired(vec![
        String::new(),
        "=8 bytes".to_owned(),
        "Hello, \u{5e78}\u{798f}!".to_owned(),
    ]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::QRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::QRequired(vec![LocalStructOut {}]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::QRequired(vec![
        LocalStructOut {},
        LocalStructOut {},
    ]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::QRequired(vec![
        LocalStructOut {},
        LocalStructOut {},
        LocalStructOut {},
    ]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::RRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::RRequired(vec![EmptyStructOut {}]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::RRequired(vec![
        EmptyStructOut {},
        EmptyStructOut {},
    ]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::RRequired(vec![
        EmptyStructOut {},
        EmptyStructOut {},
        EmptyStructOut {},
    ]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::SRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SRequired(vec![vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SRequired(vec![vec![()]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SRequired(vec![vec![], vec![], vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SRequired(vec![vec![(), (), ()]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SRequired(vec![
        vec![],
        vec![()],
        vec![(), ()],
        vec![(), (), ()],
    ]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::TRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TRequired(vec![vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TRequired(vec![vec![0.0_f64]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TRequired(vec![vec![], vec![], vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TRequired(vec![F64_TEST_VALUES.to_owned()]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TRequired(vec![
        vec![],
        vec![0.0_f64],
        vec![0.0_f64, PI],
        vec![0.0_f64, PI, f64::EPSILON],
    ]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::URequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::URequired(vec![vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::URequired(vec![vec![u64::MIN]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::URequired(vec![vec![], vec![], vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::URequired(vec![U64_TEST_VALUES.to_owned()]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::URequired(vec![
        vec![],
        vec![u64::MIN],
        vec![u64::MIN, 256],
        vec![u64::MIN, 256, u64::MAX],
    ]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::VRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VRequired(vec![vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VRequired(vec![vec![i64::MIN]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VRequired(vec![vec![], vec![], vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VRequired(vec![S64_TEST_VALUES.to_owned()]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VRequired(vec![
        vec![],
        vec![i64::MIN],
        vec![i64::MIN, 0],
        vec![i64::MIN, 0, i64::MAX],
    ]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::WRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WRequired(vec![vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WRequired(vec![vec![false]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WRequired(vec![vec![], vec![], vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WRequired(vec![vec![false, true, false]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WRequired(vec![
        vec![],
        vec![false],
        vec![false, true],
        vec![false, true, false],
    ]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::XRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XRequired(vec![vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XRequired(vec![vec![vec![]]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XRequired(vec![vec![], vec![], vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XRequired(vec![vec![
        vec![],
        vec![0, 42, 255],
        vec![7, 6, 5, 4, 3, 2, 1, 0],
    ]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XRequired(vec![
        vec![],
        vec![vec![]],
        vec![vec![], vec![0, 42, 255]],
        vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
    ]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::YRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YRequired(vec![vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YRequired(vec![vec![String::new()]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YRequired(vec![vec![], vec![], vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YRequired(vec![vec![
        String::new(),
        "=8 bytes".to_owned(),
        "Hello, \u{5e78}\u{798f}!".to_owned(),
    ]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YRequired(vec![
        vec![],
        vec![String::new()],
        vec![String::new(), "=8 bytes".to_owned()],
        vec![
            String::new(),
            "=8 bytes".to_owned(),
            "Hello, \u{5e78}\u{798f}!".to_owned(),
        ],
    ]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::ZRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZRequired(vec![vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZRequired(vec![vec![LocalStructOut {}]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZRequired(vec![vec![], vec![], vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZRequired(vec![vec![
        LocalStructOut {},
        LocalStructOut {},
        LocalStructOut {},
    ]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZRequired(vec![
        vec![],
        vec![LocalStructOut {}],
        vec![LocalStructOut {}, LocalStructOut {}],
        vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
    ]))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::AaRequired(vec![]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaRequired(vec![vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaRequired(vec![vec![EmptyStructOut {}]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaRequired(vec![vec![], vec![], vec![]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaRequired(vec![vec![
        EmptyStructOut {},
        EmptyStructOut {},
        EmptyStructOut {},
    ]]))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaRequired(vec![
        vec![],
        vec![EmptyStructOut {}],
        vec![EmptyStructOut {}, EmptyStructOut {}],
        vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
    ]))?;

    let fallback = BarOut::ARequired;

    assert_round_trip::<BarOut, BarIn>(&BarOut::AAsymmetric(Box::new(fallback.clone())))?;

    for &value in F64_TEST_VALUES {
        assert_round_trip::<BarOut, BarIn>(&BarOut::BAsymmetric(
            value,
            Box::new(fallback.clone()),
        ))?;
    }

    for &value in U64_TEST_VALUES {
        assert_round_trip::<BarOut, BarIn>(&BarOut::CAsymmetric(
            value,
            Box::new(fallback.clone()),
        ))?;
    }

    for &value in S64_TEST_VALUES {
        assert_round_trip::<BarOut, BarIn>(&BarOut::DAsymmetric(
            value,
            Box::new(fallback.clone()),
        ))?;
    }

    assert_round_trip::<BarOut, BarIn>(&BarOut::EAsymmetric(false, Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::EAsymmetric(true, Box::new(fallback.clone())))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::FAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::FAsymmetric(vec![0], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::FAsymmetric(
        vec![0, 42],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::FAsymmetric(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::GAsymmetric(
        String::new(),
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::GAsymmetric(
        "=8 bytes".to_owned(),
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::GAsymmetric(
        "Hello, \u{5e78}\u{798f}!".to_owned(),
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::HAsymmetric(
        LocalStructOut {},
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::IAsymmetric(
        EmptyStructOut {},
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::JAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::JAsymmetric(vec![()], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::JAsymmetric(
        vec![(), ()],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::JAsymmetric(
        vec![(), (), ()],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::KAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::KAsymmetric(
        vec![0.0_f64],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::KAsymmetric(
        vec![0.0_f64, PI],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::KAsymmetric(
        vec![0.0_f64, PI, f64::EPSILON],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::KAsymmetric(
        F64_TEST_VALUES.to_owned(),
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::LAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::LAsymmetric(
        vec![u64::MIN],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::LAsymmetric(
        vec![u64::MIN, 256],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::LAsymmetric(
        vec![u64::MIN, 256, u64::MAX],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::LAsymmetric(
        U64_TEST_VALUES.to_owned(),
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::MAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::MAsymmetric(
        vec![i64::MIN],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::MAsymmetric(
        vec![i64::MIN, 0],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::MAsymmetric(
        vec![i64::MIN, 0, i64::MAX],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::MAsymmetric(
        S64_TEST_VALUES.to_owned(),
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::NAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::NAsymmetric(
        vec![false],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::NAsymmetric(
        vec![false, true],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::NAsymmetric(
        vec![false, true, false],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::OAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::OAsymmetric(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::OAsymmetric(
        vec![vec![], vec![0, 42, 255]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::OAsymmetric(
        vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::PAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::PAsymmetric(
        vec![String::new()],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::PAsymmetric(
        vec![String::new(), "=8 bytes".to_owned()],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::PAsymmetric(
        vec![
            String::new(),
            "=8 bytes".to_owned(),
            "Hello, \u{5e78}\u{798f}!".to_owned(),
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::QAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::QAsymmetric(
        vec![LocalStructOut {}],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::QAsymmetric(
        vec![LocalStructOut {}, LocalStructOut {}],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::QAsymmetric(
        vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::RAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::RAsymmetric(
        vec![EmptyStructOut {}],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::RAsymmetric(
        vec![EmptyStructOut {}, EmptyStructOut {}],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::RAsymmetric(
        vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::SAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SAsymmetric(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SAsymmetric(
        vec![vec![()]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SAsymmetric(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SAsymmetric(
        vec![vec![(), (), ()]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SAsymmetric(
        vec![vec![], vec![()], vec![(), ()], vec![(), (), ()]],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::TAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TAsymmetric(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TAsymmetric(
        vec![vec![0.0_f64]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TAsymmetric(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TAsymmetric(
        vec![F64_TEST_VALUES.to_owned()],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TAsymmetric(
        vec![
            vec![],
            vec![0.0_f64],
            vec![0.0_f64, PI],
            vec![0.0_f64, PI, f64::EPSILON],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::UAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::UAsymmetric(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::UAsymmetric(
        vec![vec![u64::MIN]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::UAsymmetric(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::UAsymmetric(
        vec![U64_TEST_VALUES.to_owned()],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::UAsymmetric(
        vec![
            vec![],
            vec![u64::MIN],
            vec![u64::MIN, 256],
            vec![u64::MIN, 256, u64::MAX],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::VAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VAsymmetric(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VAsymmetric(
        vec![vec![i64::MIN]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VAsymmetric(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VAsymmetric(
        vec![S64_TEST_VALUES.to_owned()],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VAsymmetric(
        vec![
            vec![],
            vec![i64::MIN],
            vec![i64::MIN, 0],
            vec![i64::MIN, 0, i64::MAX],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::WAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WAsymmetric(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WAsymmetric(
        vec![vec![false]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WAsymmetric(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WAsymmetric(
        vec![vec![false, true, false]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WAsymmetric(
        vec![
            vec![],
            vec![false],
            vec![false, true],
            vec![false, true, false],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::XAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XAsymmetric(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XAsymmetric(
        vec![vec![vec![]]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XAsymmetric(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XAsymmetric(
        vec![vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XAsymmetric(
        vec![
            vec![],
            vec![vec![]],
            vec![vec![], vec![0, 42, 255]],
            vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::YAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YAsymmetric(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YAsymmetric(
        vec![vec![String::new()]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YAsymmetric(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YAsymmetric(
        vec![vec![
            String::new(),
            "=8 bytes".to_owned(),
            "Hello, \u{5e78}\u{798f}!".to_owned(),
        ]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YAsymmetric(
        vec![
            vec![],
            vec![String::new()],
            vec![String::new(), "=8 bytes".to_owned()],
            vec![
                String::new(),
                "=8 bytes".to_owned(),
                "Hello, \u{5e78}\u{798f}!".to_owned(),
            ],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::ZAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZAsymmetric(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZAsymmetric(
        vec![vec![LocalStructOut {}]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZAsymmetric(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZAsymmetric(
        vec![vec![
            LocalStructOut {},
            LocalStructOut {},
            LocalStructOut {},
        ]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZAsymmetric(
        vec![
            vec![],
            vec![LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::AaAsymmetric(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaAsymmetric(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaAsymmetric(
        vec![vec![EmptyStructOut {}]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaAsymmetric(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaAsymmetric(
        vec![vec![
            EmptyStructOut {},
            EmptyStructOut {},
            EmptyStructOut {},
        ]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaAsymmetric(
        vec![
            vec![],
            vec![EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::AOptional(Box::new(fallback.clone())))?;

    for &value in F64_TEST_VALUES {
        assert_round_trip::<BarOut, BarIn>(&BarOut::BOptional(value, Box::new(fallback.clone())))?;
    }

    for &value in U64_TEST_VALUES {
        assert_round_trip::<BarOut, BarIn>(&BarOut::COptional(value, Box::new(fallback.clone())))?;
    }

    for &value in S64_TEST_VALUES {
        assert_round_trip::<BarOut, BarIn>(&BarOut::DOptional(value, Box::new(fallback.clone())))?;
    }

    assert_round_trip::<BarOut, BarIn>(&BarOut::EOptional(false, Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::EOptional(true, Box::new(fallback.clone())))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::FOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::FOptional(vec![0], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::FOptional(
        vec![0, 42],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::FOptional(
        vec![0, 42, 255],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::GOptional(
        String::new(),
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::GOptional(
        "=8 bytes".to_owned(),
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::GOptional(
        "Hello, \u{5e78}\u{798f}!".to_owned(),
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::HOptional(
        LocalStructOut {},
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::IOptional(
        EmptyStructOut {},
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::JOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::JOptional(vec![()], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::JOptional(
        vec![(), ()],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::JOptional(
        vec![(), (), ()],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::KOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::KOptional(
        vec![0.0_f64],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::KOptional(
        vec![0.0_f64, PI],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::KOptional(
        vec![0.0_f64, PI, f64::EPSILON],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::KOptional(
        F64_TEST_VALUES.to_owned(),
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::LOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::LOptional(
        vec![u64::MIN],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::LOptional(
        vec![u64::MIN, 256],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::LOptional(
        vec![u64::MIN, 256, u64::MAX],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::LOptional(
        U64_TEST_VALUES.to_owned(),
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::MOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::MOptional(
        vec![i64::MIN],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::MOptional(
        vec![i64::MIN, 0],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::MOptional(
        vec![i64::MIN, 0, i64::MAX],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::MOptional(
        S64_TEST_VALUES.to_owned(),
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::NOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::NOptional(
        vec![false],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::NOptional(
        vec![false, true],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::NOptional(
        vec![false, true, false],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::OOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::OOptional(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::OOptional(
        vec![vec![], vec![0, 42, 255]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::OOptional(
        vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::POptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::POptional(
        vec![String::new()],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::POptional(
        vec![String::new(), "=8 bytes".to_owned()],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::POptional(
        vec![
            String::new(),
            "=8 bytes".to_owned(),
            "Hello, \u{5e78}\u{798f}!".to_owned(),
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::QOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::QOptional(
        vec![LocalStructOut {}],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::QOptional(
        vec![LocalStructOut {}, LocalStructOut {}],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::QOptional(
        vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::ROptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ROptional(
        vec![EmptyStructOut {}],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ROptional(
        vec![EmptyStructOut {}, EmptyStructOut {}],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ROptional(
        vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::SOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SOptional(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SOptional(
        vec![vec![()]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SOptional(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SOptional(
        vec![vec![(), (), ()]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::SOptional(
        vec![vec![], vec![()], vec![(), ()], vec![(), (), ()]],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::TOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TOptional(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TOptional(
        vec![vec![0.0_f64]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TOptional(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TOptional(
        vec![F64_TEST_VALUES.to_owned()],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::TOptional(
        vec![
            vec![],
            vec![0.0_f64],
            vec![0.0_f64, PI],
            vec![0.0_f64, PI, f64::EPSILON],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::UOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::UOptional(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::UOptional(
        vec![vec![u64::MIN]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::UOptional(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::UOptional(
        vec![U64_TEST_VALUES.to_owned()],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::UOptional(
        vec![
            vec![],
            vec![u64::MIN],
            vec![u64::MIN, 256],
            vec![u64::MIN, 256, u64::MAX],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::VOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VOptional(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VOptional(
        vec![vec![i64::MIN]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VOptional(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VOptional(
        vec![S64_TEST_VALUES.to_owned()],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::VOptional(
        vec![
            vec![],
            vec![i64::MIN],
            vec![i64::MIN, 0],
            vec![i64::MIN, 0, i64::MAX],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::WOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WOptional(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WOptional(
        vec![vec![false]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WOptional(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WOptional(
        vec![vec![false, true, false]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::WOptional(
        vec![
            vec![],
            vec![false],
            vec![false, true],
            vec![false, true, false],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::XOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XOptional(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XOptional(
        vec![vec![vec![]]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XOptional(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XOptional(
        vec![vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::XOptional(
        vec![
            vec![],
            vec![vec![]],
            vec![vec![], vec![0, 42, 255]],
            vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::YOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YOptional(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YOptional(
        vec![vec![String::new()]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YOptional(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YOptional(
        vec![vec![
            String::new(),
            "=8 bytes".to_owned(),
            "Hello, \u{5e78}\u{798f}!".to_owned(),
        ]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::YOptional(
        vec![
            vec![],
            vec![String::new()],
            vec![String::new(), "=8 bytes".to_owned()],
            vec![
                String::new(),
                "=8 bytes".to_owned(),
                "Hello, \u{5e78}\u{798f}!".to_owned(),
            ],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::ZOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZOptional(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZOptional(
        vec![vec![LocalStructOut {}]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZOptional(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZOptional(
        vec![vec![
            LocalStructOut {},
            LocalStructOut {},
            LocalStructOut {},
        ]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::ZOptional(
        vec![
            vec![],
            vec![LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}],
            vec![LocalStructOut {}, LocalStructOut {}, LocalStructOut {}],
        ],
        Box::new(fallback.clone()),
    ))?;

    assert_round_trip::<BarOut, BarIn>(&BarOut::AaOptional(vec![], Box::new(fallback.clone())))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaOptional(
        vec![vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaOptional(
        vec![vec![EmptyStructOut {}]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaOptional(
        vec![vec![], vec![], vec![]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaOptional(
        vec![vec![
            EmptyStructOut {},
            EmptyStructOut {},
            EmptyStructOut {},
        ]],
        Box::new(fallback.clone()),
    ))?;
    assert_round_trip::<BarOut, BarIn>(&BarOut::AaOptional(
        vec![
            vec![],
            vec![EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}],
            vec![EmptyStructOut {}, EmptyStructOut {}, EmptyStructOut {}],
        ],
        Box::new(fallback),
    ))?;

    Ok(())
}
