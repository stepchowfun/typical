#![deny(clippy::all, clippy::pedantic, warnings)]

mod types;

use {
    std::{f64::consts::PI, io, time::Instant},
    types::{
        types::{ChoiceOut, MessageIn, MessageOut, StructIn, StructOut},
        Deserialize, Serialize,
    },
};

const PATHOLOGICAL_ITERATIONS: usize = 300_000;
const MASSIVE_STRING_SIZE: usize = 800_000_000;

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

#[allow(clippy::cast_precision_loss)]
fn benchmark<T: Serialize, U: Deserialize>(message: &T, iterations: usize) -> io::Result<()> {
    let message_size = message.size();
    let mut buffer = Vec::<u8>::new();
    buffer.reserve(message_size);

    println!("Message size: {message_size}");

    let serialization_instant = Instant::now();

    for _ in 0..iterations {
        message.serialize(&mut buffer)?;
    }

    let serialization_duration = serialization_instant.elapsed();

    println!("Wrote {} bytes.", buffer.len());
    println!("Serialization duration: {serialization_duration:?}");
    println!(
        "Serialization rate: {} bytes/second",
        (buffer.len() as f64) / serialization_duration.as_secs_f64()
    );

    let deserialization_instant = Instant::now();

    for i in 0..iterations {
        let offset = message_size * i;
        U::deserialize(&buffer[offset..offset + message_size])?;
    }

    let deserialization_duration = deserialization_instant.elapsed();

    println!("Deserialization duration: {deserialization_duration:?}");
    println!(
        "Deserialization rate: {} bytes/second",
        (buffer.len() as f64) / deserialization_duration.as_secs_f64()
    );

    Ok(())
}

#[allow(clippy::too_many_lines)]
fn main() -> io::Result<()> {
    println!("Massive message test.");

    benchmark::<StructOut, StructIn>(
        &StructOut {
            x: "a".repeat(MASSIVE_STRING_SIZE),
        },
        1,
    )?;

    println!();
    println!("Pathological message test.");

    benchmark::<MessageOut, MessageIn>(
        &MessageOut {
            a: (),
            b: PI,
            c: u64::MAX,
            d: i64::MAX,
            e: true,
            f: vec![0, 42, 255],
            g: "Hello, World!".to_owned(),
            h: StructOut {
                x: "Hello, World!".to_owned(),
            },
            i: ChoiceOut::X("Hello, World!".to_owned()),
            j: vec![(), (), ()],
            k: F64_TEST_VALUES.to_owned(),
            l: U64_TEST_VALUES.to_owned(),
            m: S64_TEST_VALUES.to_owned(),
            n: vec![false, true, false],
            o: vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
            p: vec![
                String::new(),
                "=8 bytes".to_owned(),
                "Hello, World!".to_owned(),
            ],
            q: vec![
                StructOut {
                    x: "Hello, World!".to_owned(),
                },
                StructOut {
                    x: "Hello, World!".to_owned(),
                },
                StructOut {
                    x: "Hello, World!".to_owned(),
                },
            ],
            r: vec![
                ChoiceOut::X("Hello, World!".to_owned()),
                ChoiceOut::X("Hello, World!".to_owned()),
                ChoiceOut::X("Hello, World!".to_owned()),
            ],
            s: vec![vec![], vec![()], vec![(), ()], vec![(), (), ()]],
            t: vec![
                vec![],
                vec![0.0],
                vec![0.0, PI],
                vec![0.0, PI, f64::EPSILON],
                F64_TEST_VALUES.to_owned(),
            ],
            u: vec![
                vec![],
                vec![u64::MIN],
                vec![u64::MIN, 256],
                vec![u64::MIN, 256, u64::MAX],
                U64_TEST_VALUES.to_owned(),
            ],
            v: vec![
                vec![],
                vec![i64::MIN],
                vec![i64::MIN, 0],
                vec![i64::MIN, 0, i64::MAX],
                S64_TEST_VALUES.to_owned(),
            ],
            w: vec![
                vec![],
                vec![false],
                vec![false, true],
                vec![false, true, false],
            ],
            x: vec![
                vec![],
                vec![vec![]],
                vec![vec![], vec![0, 42, 255]],
                vec![vec![], vec![0, 42, 255], vec![7, 6, 5, 4, 3, 2, 1, 0]],
            ],
            y: vec![
                vec![String::new()],
                vec![String::new(), "=8 bytes".to_owned()],
                vec![
                    String::new(),
                    "=8 bytes".to_owned(),
                    "Hello, World!".to_owned(),
                ],
            ],
            z: vec![
                vec![],
                vec![StructOut {
                    x: "Hello, World!".to_owned(),
                }],
                vec![
                    StructOut {
                        x: "Hello, World!".to_owned(),
                    },
                    StructOut {
                        x: "Hello, World!".to_owned(),
                    },
                ],
                vec![
                    StructOut {
                        x: "Hello, World!".to_owned(),
                    },
                    StructOut {
                        x: "Hello, World!".to_owned(),
                    },
                    StructOut {
                        x: "Hello, World!".to_owned(),
                    },
                ],
            ],
            aa: vec![
                vec![],
                vec![ChoiceOut::X("Hello, World!".to_owned())],
                vec![
                    ChoiceOut::X("Hello, World!".to_owned()),
                    ChoiceOut::X("Hello, World!".to_owned()),
                ],
                vec![
                    ChoiceOut::X("Hello, World!".to_owned()),
                    ChoiceOut::X("Hello, World!".to_owned()),
                    ChoiceOut::X("Hello, World!".to_owned()),
                ],
            ],
        },
        PATHOLOGICAL_ITERATIONS,
    )
}
