/* eslint-disable @typescript-eslint/no-magic-numbers -- Allowed just for this file. */

import { Bar, Foo } from '../generated/comprehensive/types';
import { assertMatch, assertRoundTrip } from './assertions';

const u64Min = 0n;
const u64Max = 18_446_744_073_709_551_615n;
const s64Min = -9_223_372_036_854_775_808n;
const s64Max = 9_223_372_036_854_775_807n;

const f64TestValues: number[] = [
  0.0,
  -0.0,
  Math.PI,
  Number.EPSILON,
  Number.POSITIVE_INFINITY,
  Number.MAX_VALUE,
  -Number.MAX_VALUE,
  Number.MIN_VALUE, // Smallest possible (subnormal) value
  2.2250738585072014e-308, // Smallest positive normal value
  Number.NaN,
  Number.NEGATIVE_INFINITY,
];

const u64TestValues: bigint[] = [
  u64Min,
  127n,
  128n,
  16_511n,
  16_512n,
  2_113_663n,
  2_113_664n,
  270_549_119n,
  270_549_120n,
  34_630_287_487n,
  34_630_287_488n,
  4_432_676_798_591n,
  4_432_676_798_592n,
  567_382_630_219_903n,
  567_382_630_219_904n,
  72_624_976_668_147_839n,
  72_624_976_668_147_840n,
  u64Max,
];

const s64TestValues: bigint[] = [
  0n,
  -64n,
  64n,
  -8_256n,
  8_256n,
  -1_056_832n,
  1_056_832n,
  -135_274_560n,
  135_274_560n,
  -17_315_143_744n,
  17_315_143_744n,
  -2_216_338_399_296n,
  2_216_338_399_296n,
  -283_691_315_109_952n,
  283_691_315_109_952n,
  -36_312_488_334_073_920n,
  36_312_488_334_073_920n,
  s64Min,
  s64Max,
];

export default function run(): void {
  assertRoundTrip(Foo.size, Foo.serialize, Foo.deserialize, {
    aRequired: null,
    bRequired: Math.PI,
    cRequired: u64Max,
    dRequired: s64Max,
    eRequired: true,
    fRequired: new Uint8Array([0, 42, 255]).buffer,
    gRequired: 'Hello, 幸福!',
    hRequired: {},
    iRequired: {},
    jRequired: [null, null, null],
    kRequired: f64TestValues,
    lRequired: u64TestValues,
    mRequired: s64TestValues,
    nRequired: [false, true, false],
    oRequired: [
      new Uint8Array([]).buffer,
      new Uint8Array([0, 42, 255]).buffer,
      new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
    ],
    pRequired: ['', '=8 bytes', 'Hello, 幸福!'],
    qRequired: [{}, {}, {}],
    rRequired: [{}, {}, {}],
    sRequired: [[], [null], [null, null], [null, null, null]],
    tRequired: [
      [],
      [0.0],
      [0.0, Math.PI],
      [0.0, Math.PI, Number.EPSILON],
      f64TestValues,
    ],
    uRequired: [
      [],
      [u64Min],
      [u64Min, 256n],
      [u64Min, 256n, u64Max],
      u64TestValues,
    ],
    vRequired: [
      [],
      [s64Min],
      [s64Min, 0n],
      [s64Min, 0n, s64Max],
      s64TestValues,
    ],
    wRequired: [[], [false], [false, true], [false, true, false]],
    xRequired: [
      [],
      [new Uint8Array([]).buffer],
      [new Uint8Array([]).buffer, new Uint8Array([0, 42, 255]).buffer],
      [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    ],
    yRequired: [[''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, 幸福!']],
    zRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
    aaRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
    aAsymmetric: null,
    bAsymmetric: Math.PI,
    cAsymmetric: u64Max,
    dAsymmetric: s64Max,
    eAsymmetric: true,
    fAsymmetric: new Uint8Array([0, 42, 255]).buffer,
    gAsymmetric: 'Hello, 幸福!',
    hAsymmetric: {},
    iAsymmetric: {},
    jAsymmetric: [null, null, null],
    kAsymmetric: f64TestValues,
    lAsymmetric: u64TestValues,
    mAsymmetric: s64TestValues,
    nAsymmetric: [false, true, false],
    oAsymmetric: [
      new Uint8Array([]).buffer,
      new Uint8Array([0, 42, 255]).buffer,
      new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
    ],
    pAsymmetric: ['', '=8 bytes', 'Hello, 幸福!'],
    qAsymmetric: [{}, {}, {}],
    rAsymmetric: [{}, {}, {}],
    sAsymmetric: [[], [null], [null, null], [null, null, null]],
    tAsymmetric: [
      [],
      [0.0],
      [0.0, Math.PI],
      [0.0, Math.PI, Number.EPSILON],
      f64TestValues,
    ],
    uAsymmetric: [
      [],
      [u64Min],
      [u64Min, 256n],
      [u64Min, 256n, u64Max],
      u64TestValues,
    ],
    vAsymmetric: [
      [],
      [s64Min],
      [s64Min, 0n],
      [s64Min, 0n, s64Max],
      s64TestValues,
    ],
    wAsymmetric: [[], [false], [false, true], [false, true, false]],
    xAsymmetric: [
      [],
      [new Uint8Array([]).buffer],
      [new Uint8Array([]).buffer, new Uint8Array([0, 42, 255]).buffer],
      [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    ],
    yAsymmetric: [[''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, 幸福!']],
    zAsymmetric: [[], [{}], [{}, {}], [{}, {}, {}]],
    aaAsymmetric: [[], [{}], [{}, {}], [{}, {}, {}]],
    aOptional: undefined,
    bOptional: undefined,
    cOptional: undefined,
    dOptional: undefined,
    eOptional: undefined,
    fOptional: undefined,
    gOptional: undefined,
    hOptional: undefined,
    iOptional: undefined,
    jOptional: undefined,
    kOptional: undefined,
    lOptional: undefined,
    mOptional: undefined,
    nOptional: undefined,
    oOptional: undefined,
    pOptional: undefined,
    qOptional: undefined,
    rOptional: undefined,
    sOptional: undefined,
    tOptional: undefined,
    uOptional: undefined,
    vOptional: undefined,
    wOptional: undefined,
    xOptional: undefined,
    yOptional: undefined,
    zOptional: undefined,
    aaOptional: undefined,
  });

  // eslint-disable-next-line no-console -- Allow us to separate the test groups with a line break.
  console.log();

  assertRoundTrip(Foo.size, Foo.serialize, Foo.deserialize, {
    aRequired: null,
    bRequired: Math.PI,
    cRequired: u64Max,
    dRequired: s64Max,
    eRequired: true,
    fRequired: new Uint8Array([0, 42, 255]).buffer,
    gRequired: 'Hello, 幸福!',
    hRequired: {},
    iRequired: {},
    jRequired: [null, null, null],
    kRequired: f64TestValues,
    lRequired: u64TestValues,
    mRequired: s64TestValues,
    nRequired: [false, true, false],
    oRequired: [
      new Uint8Array([]).buffer,
      new Uint8Array([0, 42, 255]).buffer,
      new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
    ],
    pRequired: ['', '=8 bytes', 'Hello, 幸福!'],
    qRequired: [{}, {}, {}],
    rRequired: [{}, {}, {}],
    sRequired: [[], [null], [null, null], [null, null, null]],
    tRequired: [
      [],
      [0.0],
      [0.0, Math.PI],
      [0.0, Math.PI, Number.EPSILON],
      f64TestValues,
    ],
    uRequired: [
      [],
      [u64Min],
      [u64Min, 256n],
      [u64Min, 256n, u64Max],
      u64TestValues,
    ],
    vRequired: [
      [],
      [s64Min],
      [s64Min, 0n],
      [s64Min, 0n, s64Max],
      s64TestValues,
    ],
    wRequired: [[], [false], [false, true], [false, true, false]],
    xRequired: [
      [],
      [new Uint8Array([]).buffer],
      [new Uint8Array([]).buffer, new Uint8Array([0, 42, 255]).buffer],
      [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    ],
    yRequired: [[''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, 幸福!']],
    zRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
    aaRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
    aAsymmetric: null,
    bAsymmetric: Math.PI,
    cAsymmetric: u64Max,
    dAsymmetric: s64Max,
    eAsymmetric: true,
    fAsymmetric: new Uint8Array([0, 42, 255]).buffer,
    gAsymmetric: 'Hello, 幸福!',
    hAsymmetric: {},
    iAsymmetric: {},
    jAsymmetric: [null, null, null],
    kAsymmetric: f64TestValues,
    lAsymmetric: u64TestValues,
    mAsymmetric: s64TestValues,
    nAsymmetric: [false, true, false],
    oAsymmetric: [
      new Uint8Array([]).buffer,
      new Uint8Array([0, 42, 255]).buffer,
      new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
    ],
    pAsymmetric: ['', '=8 bytes', 'Hello, 幸福!'],
    qAsymmetric: [{}, {}, {}],
    rAsymmetric: [{}, {}, {}],
    sAsymmetric: [[], [null], [null, null], [null, null, null]],
    tAsymmetric: [
      [],
      [0.0],
      [0.0, Math.PI],
      [0.0, Math.PI, Number.EPSILON],
      f64TestValues,
    ],
    uAsymmetric: [
      [],
      [u64Min],
      [u64Min, 256n],
      [u64Min, 256n, u64Max],
      u64TestValues,
    ],
    vAsymmetric: [
      [],
      [s64Min],
      [s64Min, 0n],
      [s64Min, 0n, s64Max],
      s64TestValues,
    ],
    wAsymmetric: [[], [false], [false, true], [false, true, false]],
    xAsymmetric: [
      [],
      [new Uint8Array([]).buffer],
      [new Uint8Array([]).buffer, new Uint8Array([0, 42, 255]).buffer],
      [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    ],
    yAsymmetric: [[''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, 幸福!']],
    zAsymmetric: [[], [{}], [{}, {}], [{}, {}, {}]],
    aaAsymmetric: [[], [{}], [{}, {}], [{}, {}, {}]],
    aOptional: null,
    bOptional: Math.PI,
    cOptional: u64Max,
    dOptional: s64Max,
    eOptional: true,
    fOptional: new Uint8Array([0, 42, 255]).buffer,
    gOptional: 'Hello, 幸福!',
    hOptional: {},
    iOptional: {},
    jOptional: [null, null, null],
    kOptional: f64TestValues,
    lOptional: u64TestValues,
    mOptional: s64TestValues,
    nOptional: [false, true, false],
    oOptional: [
      new Uint8Array([]).buffer,
      new Uint8Array([0, 42, 255]).buffer,
      new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
    ],
    pOptional: ['', '=8 bytes', 'Hello, 幸福!'],
    qOptional: [{}, {}, {}],
    rOptional: [{}, {}, {}],
    sOptional: [[], [null], [null, null], [null, null, null]],
    tOptional: [
      [],
      [0.0],
      [0.0, Math.PI],
      [0.0, Math.PI, Number.EPSILON],
      f64TestValues,
    ],
    uOptional: [
      [],
      [u64Min],
      [u64Min, 256n],
      [u64Min, 256n, u64Max],
      u64TestValues,
    ],
    vOptional: [
      [],
      [s64Min],
      [s64Min, 0n],
      [s64Min, 0n, s64Max],
      s64TestValues,
    ],
    wOptional: [[], [false], [false, true], [false, true, false]],
    xOptional: [
      [],
      [new Uint8Array([]).buffer],
      [new Uint8Array([]).buffer, new Uint8Array([0, 42, 255]).buffer],
      [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    ],
    yOptional: [[''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, 幸福!']],
    zOptional: [[], [{}], [{}, {}], [{}, {}, {}]],
    aaOptional: [[], [{}], [{}, {}], [{}, {}, {}]],
  });

  // eslint-disable-next-line no-console -- Allow us to separate the test groups with a line break.
  console.log();

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aRequired',
    aRequired: null,
  });

  f64TestValues.forEach((bRequired) => {
    assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
      $field: 'bRequired',
      bRequired,
    });
  });

  u64TestValues.forEach((cRequired) => {
    assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
      $field: 'cRequired',
      cRequired,
    });
  });

  s64TestValues.forEach((dRequired) => {
    assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
      $field: 'dRequired',
      dRequired,
    });
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'eRequired',
    eRequired: false,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'eRequired',
    eRequired: true,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'fRequired',
    fRequired: new Uint8Array([]).buffer,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'fRequired',
    fRequired: new Uint8Array([0]).buffer,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'fRequired',
    fRequired: new Uint8Array([0, 42]).buffer,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'fRequired',
    fRequired: new Uint8Array([0, 42, 255]).buffer,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'gRequired',
    gRequired: '',
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'gRequired',
    gRequired: '=8 bytes',
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'gRequired',
    gRequired: 'Hello, 幸福!',
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'hRequired',
    hRequired: {},
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'iRequired',
    iRequired: {},
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'jRequired',
    jRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'jRequired',
    jRequired: [null],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'jRequired',
    jRequired: [null, null],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'jRequired',
    jRequired: [null, null, null],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'kRequired',
    kRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'kRequired',
    kRequired: [0.0],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'kRequired',
    kRequired: [0.0, Math.PI],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'kRequired',
    kRequired: [0.0, Math.PI, Number.EPSILON],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'kRequired',
    kRequired: f64TestValues,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'lRequired',
    lRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'lRequired',
    lRequired: [u64Min],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'lRequired',
    lRequired: [u64Min, 256n],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'lRequired',
    lRequired: [u64Min, 256n, u64Max],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'lRequired',
    lRequired: u64TestValues,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'mRequired',
    mRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'mRequired',
    mRequired: [s64Min],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'mRequired',
    mRequired: [s64Min, 0n],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'mRequired',
    mRequired: [s64Min, 0n, s64Max],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'mRequired',
    mRequired: s64TestValues,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'nRequired',
    nRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'nRequired',
    nRequired: [false],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'nRequired',
    nRequired: [false, true],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'nRequired',
    nRequired: [false, true, false],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'oRequired',
    oRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'oRequired',
    oRequired: [new Uint8Array([]).buffer],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'oRequired',
    oRequired: [new Uint8Array([]).buffer, new Uint8Array([0, 42, 255]).buffer],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'oRequired',
    oRequired: [
      new Uint8Array([]).buffer,
      new Uint8Array([0, 42, 255]).buffer,
      new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
    ],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'pRequired',
    pRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'pRequired',
    pRequired: [''],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'pRequired',
    pRequired: ['', '=8 bytes'],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'pRequired',
    pRequired: ['', '=8 bytes', 'Hello, 幸福!'],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'qRequired',
    qRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'qRequired',
    qRequired: [{}],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'qRequired',
    qRequired: [{}, {}],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'qRequired',
    qRequired: [{}, {}, {}],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'rRequired',
    rRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'rRequired',
    rRequired: [{}],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'rRequired',
    rRequired: [{}, {}],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'rRequired',
    rRequired: [{}, {}, {}],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'sRequired',
    sRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'sRequired',
    sRequired: [[]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'sRequired',
    sRequired: [[null]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'sRequired',
    sRequired: [[], [], []],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'sRequired',
    sRequired: [[null, null, null]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'sRequired',
    sRequired: [[], [null], [null, null], [null, null, null]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'tRequired',
    tRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'tRequired',
    tRequired: [[]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'tRequired',
    tRequired: [[0.0]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'tRequired',
    tRequired: [[], [], []],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'tRequired',
    tRequired: [f64TestValues],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'tRequired',
    tRequired: [[], [0.0], [0.0, Math.PI], [0.0, Math.PI, Number.EPSILON]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'uRequired',
    uRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'uRequired',
    uRequired: [[]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'uRequired',
    uRequired: [[u64Min]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'uRequired',
    uRequired: [[], [], []],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'uRequired',
    uRequired: [u64TestValues],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'uRequired',
    uRequired: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'vRequired',
    vRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'vRequired',
    vRequired: [[]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'vRequired',
    vRequired: [[s64Min]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'vRequired',
    vRequired: [[], [], []],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'vRequired',
    vRequired: [s64TestValues],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'vRequired',
    vRequired: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'wRequired',
    wRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'wRequired',
    wRequired: [[]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'wRequired',
    wRequired: [[false]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'wRequired',
    wRequired: [[], [], []],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'wRequired',
    wRequired: [[false, true, false]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'wRequired',
    wRequired: [[], [false], [false, true], [false, true, false]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'xRequired',
    xRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'xRequired',
    xRequired: [[]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'xRequired',
    xRequired: [[new Uint8Array([]).buffer]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'xRequired',
    xRequired: [[], [], []],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'xRequired',
    xRequired: [
      [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    ],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'xRequired',
    xRequired: [
      [],
      [new Uint8Array([]).buffer],
      [new Uint8Array([]).buffer, new Uint8Array([0, 42, 255]).buffer],
      [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    ],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'yRequired',
    yRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'yRequired',
    yRequired: [[]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'yRequired',
    yRequired: [['']],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'yRequired',
    yRequired: [[], [], []],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'yRequired',
    yRequired: [['', '=8 bytes', 'Hello, 幸福!']],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'yRequired',
    yRequired: [[], [''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, 幸福!']],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'zRequired',
    zRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'zRequired',
    zRequired: [[]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'zRequired',
    zRequired: [[{}]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'zRequired',
    zRequired: [[], [], []],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'zRequired',
    zRequired: [[{}, {}, {}]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'zRequired',
    zRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aaRequired',
    aaRequired: [],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aaRequired',
    aaRequired: [[]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aaRequired',
    aaRequired: [[{}]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aaRequired',
    aaRequired: [[], [], []],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aaRequired',
    aaRequired: [[{}, {}, {}]],
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aaRequired',
    aaRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
  });

  const $fallback = { $field: 'aRequired', aRequired: null };

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'aAsymmetric',
      aAsymmetric: null,
      $fallback,
    },
    {
      $field: 'aAsymmetric',
      aAsymmetric: null,
    },
  );

  f64TestValues.forEach((bAsymmetric) => {
    assertMatch(
      Bar.size,
      Bar.serialize,
      Bar.deserialize,
      {
        $field: 'bAsymmetric',
        bAsymmetric,
        $fallback,
      },
      {
        $field: 'bAsymmetric',
        bAsymmetric,
      },
    );
  });

  u64TestValues.forEach((cAsymmetric) => {
    assertMatch(
      Bar.size,
      Bar.serialize,
      Bar.deserialize,
      {
        $field: 'cAsymmetric',
        cAsymmetric,
        $fallback,
      },
      {
        $field: 'cAsymmetric',
        cAsymmetric,
      },
    );
  });

  s64TestValues.forEach((dAsymmetric) => {
    assertMatch(
      Bar.size,
      Bar.serialize,
      Bar.deserialize,
      {
        $field: 'dAsymmetric',
        dAsymmetric,
        $fallback,
      },
      {
        $field: 'dAsymmetric',
        dAsymmetric,
      },
    );
  });

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'eAsymmetric',
      eAsymmetric: false,
      $fallback,
    },
    {
      $field: 'eAsymmetric',
      eAsymmetric: false,
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'eAsymmetric',
      eAsymmetric: true,
      $fallback,
    },
    {
      $field: 'eAsymmetric',
      eAsymmetric: true,
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'fAsymmetric',
      fAsymmetric: new Uint8Array([]).buffer,
      $fallback,
    },
    {
      $field: 'fAsymmetric',
      fAsymmetric: new Uint8Array([]).buffer,
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'fAsymmetric',
      fAsymmetric: new Uint8Array([0]).buffer,
      $fallback,
    },
    {
      $field: 'fAsymmetric',
      fAsymmetric: new Uint8Array([0]).buffer,
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'fAsymmetric',
      fAsymmetric: new Uint8Array([0, 42]).buffer,
      $fallback,
    },
    {
      $field: 'fAsymmetric',
      fAsymmetric: new Uint8Array([0, 42]).buffer,
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'fAsymmetric',
      fAsymmetric: new Uint8Array([0, 42, 255]).buffer,
      $fallback,
    },
    {
      $field: 'fAsymmetric',
      fAsymmetric: new Uint8Array([0, 42, 255]).buffer,
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'gAsymmetric',
      gAsymmetric: '',
      $fallback,
    },
    {
      $field: 'gAsymmetric',
      gAsymmetric: '',
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'gAsymmetric',
      gAsymmetric: '=8 bytes',
      $fallback,
    },
    {
      $field: 'gAsymmetric',
      gAsymmetric: '=8 bytes',
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'gAsymmetric',
      gAsymmetric: 'Hello, 幸福!',
      $fallback,
    },
    {
      $field: 'gAsymmetric',
      gAsymmetric: 'Hello, 幸福!',
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'hAsymmetric',
      hAsymmetric: {},
      $fallback,
    },
    {
      $field: 'hAsymmetric',
      hAsymmetric: {},
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'iAsymmetric',
      iAsymmetric: {},
      $fallback,
    },
    {
      $field: 'iAsymmetric',
      iAsymmetric: {},
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'jAsymmetric',
      jAsymmetric: [],
      $fallback,
    },
    {
      $field: 'jAsymmetric',
      jAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'jAsymmetric',
      jAsymmetric: [null],
      $fallback,
    },
    {
      $field: 'jAsymmetric',
      jAsymmetric: [null],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'jAsymmetric',
      jAsymmetric: [null, null],
      $fallback,
    },
    {
      $field: 'jAsymmetric',
      jAsymmetric: [null, null],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'jAsymmetric',
      jAsymmetric: [null, null, null],
      $fallback,
    },
    {
      $field: 'jAsymmetric',
      jAsymmetric: [null, null, null],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'kAsymmetric',
      kAsymmetric: [],
      $fallback,
    },
    {
      $field: 'kAsymmetric',
      kAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'kAsymmetric',
      kAsymmetric: [0.0],
      $fallback,
    },
    {
      $field: 'kAsymmetric',
      kAsymmetric: [0.0],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'kAsymmetric',
      kAsymmetric: [0.0, Math.PI],
      $fallback,
    },
    {
      $field: 'kAsymmetric',
      kAsymmetric: [0.0, Math.PI],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'kAsymmetric',
      kAsymmetric: [0.0, Math.PI, Number.EPSILON],
      $fallback,
    },
    {
      $field: 'kAsymmetric',
      kAsymmetric: [0.0, Math.PI, Number.EPSILON],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'kAsymmetric',
      kAsymmetric: f64TestValues,
      $fallback,
    },
    {
      $field: 'kAsymmetric',
      kAsymmetric: f64TestValues,
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'lAsymmetric',
      lAsymmetric: [],
      $fallback,
    },
    {
      $field: 'lAsymmetric',
      lAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'lAsymmetric',
      lAsymmetric: [u64Min],
      $fallback,
    },
    {
      $field: 'lAsymmetric',
      lAsymmetric: [u64Min],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'lAsymmetric',
      lAsymmetric: [u64Min, 256n],
      $fallback,
    },
    {
      $field: 'lAsymmetric',
      lAsymmetric: [u64Min, 256n],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'lAsymmetric',
      lAsymmetric: [u64Min, 256n, u64Max],
      $fallback,
    },
    {
      $field: 'lAsymmetric',
      lAsymmetric: [u64Min, 256n, u64Max],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'lAsymmetric',
      lAsymmetric: u64TestValues,
      $fallback,
    },
    {
      $field: 'lAsymmetric',
      lAsymmetric: u64TestValues,
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'mAsymmetric',
      mAsymmetric: [],
      $fallback,
    },
    {
      $field: 'mAsymmetric',
      mAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'mAsymmetric',
      mAsymmetric: [s64Min],
      $fallback,
    },
    {
      $field: 'mAsymmetric',
      mAsymmetric: [s64Min],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'mAsymmetric',
      mAsymmetric: [s64Min, 0n],
      $fallback,
    },
    {
      $field: 'mAsymmetric',
      mAsymmetric: [s64Min, 0n],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'mAsymmetric',
      mAsymmetric: [s64Min, 0n, s64Max],
      $fallback,
    },
    {
      $field: 'mAsymmetric',
      mAsymmetric: [s64Min, 0n, s64Max],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'mAsymmetric',
      mAsymmetric: s64TestValues,
      $fallback,
    },
    {
      $field: 'mAsymmetric',
      mAsymmetric: s64TestValues,
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'nAsymmetric',
      nAsymmetric: [],
      $fallback,
    },
    {
      $field: 'nAsymmetric',
      nAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'nAsymmetric',
      nAsymmetric: [false],
      $fallback,
    },
    {
      $field: 'nAsymmetric',
      nAsymmetric: [false],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'nAsymmetric',
      nAsymmetric: [false, true],
      $fallback,
    },
    {
      $field: 'nAsymmetric',
      nAsymmetric: [false, true],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'nAsymmetric',
      nAsymmetric: [false, true, false],
      $fallback,
    },
    {
      $field: 'nAsymmetric',
      nAsymmetric: [false, true, false],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'oAsymmetric',
      oAsymmetric: [],
      $fallback,
    },
    {
      $field: 'oAsymmetric',
      oAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'oAsymmetric',
      oAsymmetric: [new Uint8Array([]).buffer],
      $fallback,
    },
    {
      $field: 'oAsymmetric',
      oAsymmetric: [new Uint8Array([]).buffer],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'oAsymmetric',
      oAsymmetric: [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
      ],
      $fallback,
    },
    {
      $field: 'oAsymmetric',
      oAsymmetric: [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
      ],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'oAsymmetric',
      oAsymmetric: [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
      $fallback,
    },
    {
      $field: 'oAsymmetric',
      oAsymmetric: [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'pAsymmetric',
      pAsymmetric: [],
      $fallback,
    },
    {
      $field: 'pAsymmetric',
      pAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'pAsymmetric',
      pAsymmetric: [''],
      $fallback,
    },
    {
      $field: 'pAsymmetric',
      pAsymmetric: [''],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'pAsymmetric',
      pAsymmetric: ['', '=8 bytes'],
      $fallback,
    },
    {
      $field: 'pAsymmetric',
      pAsymmetric: ['', '=8 bytes'],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'pAsymmetric',
      pAsymmetric: ['', '=8 bytes', 'Hello, 幸福!'],
      $fallback,
    },
    {
      $field: 'pAsymmetric',
      pAsymmetric: ['', '=8 bytes', 'Hello, 幸福!'],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'qAsymmetric',
      qAsymmetric: [],
      $fallback,
    },
    {
      $field: 'qAsymmetric',
      qAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'qAsymmetric',
      qAsymmetric: [{}],
      $fallback,
    },
    {
      $field: 'qAsymmetric',
      qAsymmetric: [{}],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'qAsymmetric',
      qAsymmetric: [{}, {}],
      $fallback,
    },
    {
      $field: 'qAsymmetric',
      qAsymmetric: [{}, {}],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'qAsymmetric',
      qAsymmetric: [{}, {}, {}],
      $fallback,
    },
    {
      $field: 'qAsymmetric',
      qAsymmetric: [{}, {}, {}],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'rAsymmetric',
      rAsymmetric: [],
      $fallback,
    },
    {
      $field: 'rAsymmetric',
      rAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'rAsymmetric',
      rAsymmetric: [{}],
      $fallback,
    },
    {
      $field: 'rAsymmetric',
      rAsymmetric: [{}],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'rAsymmetric',
      rAsymmetric: [{}, {}],
      $fallback,
    },
    {
      $field: 'rAsymmetric',
      rAsymmetric: [{}, {}],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'rAsymmetric',
      rAsymmetric: [{}, {}, {}],
      $fallback,
    },
    {
      $field: 'rAsymmetric',
      rAsymmetric: [{}, {}, {}],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'sAsymmetric',
      sAsymmetric: [],
      $fallback,
    },
    {
      $field: 'sAsymmetric',
      sAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'sAsymmetric',
      sAsymmetric: [[]],
      $fallback,
    },
    {
      $field: 'sAsymmetric',
      sAsymmetric: [[]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'sAsymmetric',
      sAsymmetric: [[null]],
      $fallback,
    },
    {
      $field: 'sAsymmetric',
      sAsymmetric: [[null]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'sAsymmetric',
      sAsymmetric: [[], [], []],
      $fallback,
    },
    {
      $field: 'sAsymmetric',
      sAsymmetric: [[], [], []],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'sAsymmetric',
      sAsymmetric: [[null, null, null]],
      $fallback,
    },
    {
      $field: 'sAsymmetric',
      sAsymmetric: [[null, null, null]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'sAsymmetric',
      sAsymmetric: [[], [null], [null, null], [null, null, null]],
      $fallback,
    },
    {
      $field: 'sAsymmetric',
      sAsymmetric: [[], [null], [null, null], [null, null, null]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'tAsymmetric',
      tAsymmetric: [],
      $fallback,
    },
    {
      $field: 'tAsymmetric',
      tAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'tAsymmetric',
      tAsymmetric: [[]],
      $fallback,
    },
    {
      $field: 'tAsymmetric',
      tAsymmetric: [[]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'tAsymmetric',
      tAsymmetric: [[0.0]],
      $fallback,
    },
    {
      $field: 'tAsymmetric',
      tAsymmetric: [[0.0]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'tAsymmetric',
      tAsymmetric: [[], [], []],
      $fallback,
    },
    {
      $field: 'tAsymmetric',
      tAsymmetric: [[], [], []],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'tAsymmetric',
      tAsymmetric: [f64TestValues],
      $fallback,
    },
    {
      $field: 'tAsymmetric',
      tAsymmetric: [f64TestValues],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'tAsymmetric',
      tAsymmetric: [[], [0.0], [0.0, Math.PI], [0.0, Math.PI, Number.EPSILON]],
      $fallback,
    },
    {
      $field: 'tAsymmetric',
      tAsymmetric: [[], [0.0], [0.0, Math.PI], [0.0, Math.PI, Number.EPSILON]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'uAsymmetric',
      uAsymmetric: [],
      $fallback,
    },
    {
      $field: 'uAsymmetric',
      uAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'uAsymmetric',
      uAsymmetric: [[]],
      $fallback,
    },
    {
      $field: 'uAsymmetric',
      uAsymmetric: [[]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'uAsymmetric',
      uAsymmetric: [[u64Min]],
      $fallback,
    },
    {
      $field: 'uAsymmetric',
      uAsymmetric: [[u64Min]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'uAsymmetric',
      uAsymmetric: [[], [], []],
      $fallback,
    },
    {
      $field: 'uAsymmetric',
      uAsymmetric: [[], [], []],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'uAsymmetric',
      uAsymmetric: [u64TestValues],
      $fallback,
    },
    {
      $field: 'uAsymmetric',
      uAsymmetric: [u64TestValues],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'uAsymmetric',
      uAsymmetric: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max]],
      $fallback,
    },
    {
      $field: 'uAsymmetric',
      uAsymmetric: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'vAsymmetric',
      vAsymmetric: [],
      $fallback,
    },
    {
      $field: 'vAsymmetric',
      vAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'vAsymmetric',
      vAsymmetric: [[]],
      $fallback,
    },
    {
      $field: 'vAsymmetric',
      vAsymmetric: [[]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'vAsymmetric',
      vAsymmetric: [[s64Min]],
      $fallback,
    },
    {
      $field: 'vAsymmetric',
      vAsymmetric: [[s64Min]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'vAsymmetric',
      vAsymmetric: [[], [], []],
      $fallback,
    },
    {
      $field: 'vAsymmetric',
      vAsymmetric: [[], [], []],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'vAsymmetric',
      vAsymmetric: [s64TestValues],
      $fallback,
    },
    {
      $field: 'vAsymmetric',
      vAsymmetric: [s64TestValues],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'vAsymmetric',
      vAsymmetric: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max]],
      $fallback,
    },
    {
      $field: 'vAsymmetric',
      vAsymmetric: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'wAsymmetric',
      wAsymmetric: [],
      $fallback,
    },
    {
      $field: 'wAsymmetric',
      wAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'wAsymmetric',
      wAsymmetric: [[]],
      $fallback,
    },
    {
      $field: 'wAsymmetric',
      wAsymmetric: [[]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'wAsymmetric',
      wAsymmetric: [[false]],
      $fallback,
    },
    {
      $field: 'wAsymmetric',
      wAsymmetric: [[false]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'wAsymmetric',
      wAsymmetric: [[], [], []],
      $fallback,
    },
    {
      $field: 'wAsymmetric',
      wAsymmetric: [[], [], []],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'wAsymmetric',
      wAsymmetric: [[false, true, false]],
      $fallback,
    },
    {
      $field: 'wAsymmetric',
      wAsymmetric: [[false, true, false]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'wAsymmetric',
      wAsymmetric: [[], [false], [false, true], [false, true, false]],
      $fallback,
    },
    {
      $field: 'wAsymmetric',
      wAsymmetric: [[], [false], [false, true], [false, true, false]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'xAsymmetric',
      xAsymmetric: [],
      $fallback,
    },
    {
      $field: 'xAsymmetric',
      xAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'xAsymmetric',
      xAsymmetric: [[]],
      $fallback,
    },
    {
      $field: 'xAsymmetric',
      xAsymmetric: [[]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'xAsymmetric',
      xAsymmetric: [[new Uint8Array([]).buffer]],
      $fallback,
    },
    {
      $field: 'xAsymmetric',
      xAsymmetric: [[new Uint8Array([]).buffer]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'xAsymmetric',
      xAsymmetric: [[], [], []],
      $fallback,
    },
    {
      $field: 'xAsymmetric',
      xAsymmetric: [[], [], []],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'xAsymmetric',
      xAsymmetric: [
        [
          new Uint8Array([]).buffer,
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
        ],
      ],
      $fallback,
    },
    {
      $field: 'xAsymmetric',
      xAsymmetric: [
        [
          new Uint8Array([]).buffer,
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
        ],
      ],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'xAsymmetric',
      xAsymmetric: [
        [],
        [new Uint8Array([]).buffer],
        [new Uint8Array([]).buffer, new Uint8Array([0, 42, 255]).buffer],
        [
          new Uint8Array([]).buffer,
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
        ],
      ],
      $fallback,
    },
    {
      $field: 'xAsymmetric',
      xAsymmetric: [
        [],
        [new Uint8Array([]).buffer],
        [new Uint8Array([]).buffer, new Uint8Array([0, 42, 255]).buffer],
        [
          new Uint8Array([]).buffer,
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
        ],
      ],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'yAsymmetric',
      yAsymmetric: [],
      $fallback,
    },
    {
      $field: 'yAsymmetric',
      yAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'yAsymmetric',
      yAsymmetric: [[]],
      $fallback,
    },
    {
      $field: 'yAsymmetric',
      yAsymmetric: [[]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'yAsymmetric',
      yAsymmetric: [['']],
      $fallback,
    },
    {
      $field: 'yAsymmetric',
      yAsymmetric: [['']],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'yAsymmetric',
      yAsymmetric: [[], [], []],
      $fallback,
    },
    {
      $field: 'yAsymmetric',
      yAsymmetric: [[], [], []],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'yAsymmetric',
      yAsymmetric: [['', '=8 bytes', 'Hello, 幸福!']],
      $fallback,
    },
    {
      $field: 'yAsymmetric',
      yAsymmetric: [['', '=8 bytes', 'Hello, 幸福!']],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'yAsymmetric',
      yAsymmetric: [
        [],
        [''],
        ['', '=8 bytes'],
        ['', '=8 bytes', 'Hello, 幸福!'],
      ],
      $fallback,
    },
    {
      $field: 'yAsymmetric',
      yAsymmetric: [
        [],
        [''],
        ['', '=8 bytes'],
        ['', '=8 bytes', 'Hello, 幸福!'],
      ],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'zAsymmetric',
      zAsymmetric: [],
      $fallback,
    },
    {
      $field: 'zAsymmetric',
      zAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'zAsymmetric',
      zAsymmetric: [[]],
      $fallback,
    },
    {
      $field: 'zAsymmetric',
      zAsymmetric: [[]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'zAsymmetric',
      zAsymmetric: [[{}]],
      $fallback,
    },
    {
      $field: 'zAsymmetric',
      zAsymmetric: [[{}]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'zAsymmetric',
      zAsymmetric: [[], [], []],
      $fallback,
    },
    {
      $field: 'zAsymmetric',
      zAsymmetric: [[], [], []],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'zAsymmetric',
      zAsymmetric: [[{}, {}, {}]],
      $fallback,
    },
    {
      $field: 'zAsymmetric',
      zAsymmetric: [[{}, {}, {}]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'zAsymmetric',
      zAsymmetric: [[], [{}], [{}, {}], [{}, {}, {}]],
      $fallback,
    },
    {
      $field: 'zAsymmetric',
      zAsymmetric: [[], [{}], [{}, {}], [{}, {}, {}]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'aaAsymmetric',
      aaAsymmetric: [],
      $fallback,
    },
    {
      $field: 'aaAsymmetric',
      aaAsymmetric: [],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'aaAsymmetric',
      aaAsymmetric: [[]],
      $fallback,
    },
    {
      $field: 'aaAsymmetric',
      aaAsymmetric: [[]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'aaAsymmetric',
      aaAsymmetric: [[{}]],
      $fallback,
    },
    {
      $field: 'aaAsymmetric',
      aaAsymmetric: [[{}]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'aaAsymmetric',
      aaAsymmetric: [[], [], []],
      $fallback,
    },
    {
      $field: 'aaAsymmetric',
      aaAsymmetric: [[], [], []],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'aaAsymmetric',
      aaAsymmetric: [[{}, {}, {}]],
      $fallback,
    },
    {
      $field: 'aaAsymmetric',
      aaAsymmetric: [[{}, {}, {}]],
    },
  );

  assertMatch(
    Bar.size,
    Bar.serialize,
    Bar.deserialize,
    {
      $field: 'aaAsymmetric',
      aaAsymmetric: [[], [{}], [{}, {}], [{}, {}, {}]],
      $fallback,
    },
    {
      $field: 'aaAsymmetric',
      aaAsymmetric: [[], [{}], [{}, {}], [{}, {}, {}]],
    },
  );

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aOptional',
    aOptional: null,
    $fallback,
  });

  f64TestValues.forEach((bOptional) => {
    assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
      $field: 'bOptional',
      bOptional,
      $fallback,
    });
  });

  u64TestValues.forEach((cOptional) => {
    assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
      $field: 'cOptional',
      cOptional,
      $fallback,
    });
  });

  s64TestValues.forEach((dOptional) => {
    assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
      $field: 'dOptional',
      dOptional,
      $fallback,
    });
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'eOptional',
    eOptional: false,
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'eOptional',
    eOptional: true,
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'fOptional',
    fOptional: new Uint8Array([]).buffer,
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'fOptional',
    fOptional: new Uint8Array([0]).buffer,
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'fOptional',
    fOptional: new Uint8Array([0, 42]).buffer,
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'fOptional',
    fOptional: new Uint8Array([0, 42, 255]).buffer,
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'gOptional',
    gOptional: '',
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'gOptional',
    gOptional: '=8 bytes',
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'gOptional',
    gOptional: 'Hello, 幸福!',
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'hOptional',
    hOptional: {},
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'iOptional',
    iOptional: {},
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'jOptional',
    jOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'jOptional',
    jOptional: [null],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'jOptional',
    jOptional: [null, null],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'jOptional',
    jOptional: [null, null, null],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'kOptional',
    kOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'kOptional',
    kOptional: [0.0],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'kOptional',
    kOptional: [0.0, Math.PI],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'kOptional',
    kOptional: [0.0, Math.PI, Number.EPSILON],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'kOptional',
    kOptional: f64TestValues,
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'lOptional',
    lOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'lOptional',
    lOptional: [u64Min],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'lOptional',
    lOptional: [u64Min, 256n],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'lOptional',
    lOptional: [u64Min, 256n, u64Max],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'lOptional',
    lOptional: u64TestValues,
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'mOptional',
    mOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'mOptional',
    mOptional: [s64Min],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'mOptional',
    mOptional: [s64Min, 0n],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'mOptional',
    mOptional: [s64Min, 0n, s64Max],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'mOptional',
    mOptional: s64TestValues,
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'nOptional',
    nOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'nOptional',
    nOptional: [false],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'nOptional',
    nOptional: [false, true],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'nOptional',
    nOptional: [false, true, false],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'oOptional',
    oOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'oOptional',
    oOptional: [new Uint8Array([]).buffer],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'oOptional',
    oOptional: [new Uint8Array([]).buffer, new Uint8Array([0, 42, 255]).buffer],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'oOptional',
    oOptional: [
      new Uint8Array([]).buffer,
      new Uint8Array([0, 42, 255]).buffer,
      new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
    ],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'pOptional',
    pOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'pOptional',
    pOptional: [''],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'pOptional',
    pOptional: ['', '=8 bytes'],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'pOptional',
    pOptional: ['', '=8 bytes', 'Hello, 幸福!'],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'qOptional',
    qOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'qOptional',
    qOptional: [{}],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'qOptional',
    qOptional: [{}, {}],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'qOptional',
    qOptional: [{}, {}, {}],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'rOptional',
    rOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'rOptional',
    rOptional: [{}],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'rOptional',
    rOptional: [{}, {}],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'rOptional',
    rOptional: [{}, {}, {}],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'sOptional',
    sOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'sOptional',
    sOptional: [[]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'sOptional',
    sOptional: [[null]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'sOptional',
    sOptional: [[], [], []],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'sOptional',
    sOptional: [[null, null, null]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'sOptional',
    sOptional: [[], [null], [null, null], [null, null, null]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'tOptional',
    tOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'tOptional',
    tOptional: [[]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'tOptional',
    tOptional: [[0.0]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'tOptional',
    tOptional: [[], [], []],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'tOptional',
    tOptional: [f64TestValues],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'tOptional',
    tOptional: [[], [0.0], [0.0, Math.PI], [0.0, Math.PI, Number.EPSILON]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'uOptional',
    uOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'uOptional',
    uOptional: [[]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'uOptional',
    uOptional: [[u64Min]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'uOptional',
    uOptional: [[], [], []],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'uOptional',
    uOptional: [u64TestValues],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'uOptional',
    uOptional: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'vOptional',
    vOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'vOptional',
    vOptional: [[]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'vOptional',
    vOptional: [[s64Min]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'vOptional',
    vOptional: [[], [], []],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'vOptional',
    vOptional: [s64TestValues],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'vOptional',
    vOptional: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'wOptional',
    wOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'wOptional',
    wOptional: [[]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'wOptional',
    wOptional: [[false]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'wOptional',
    wOptional: [[], [], []],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'wOptional',
    wOptional: [[false, true, false]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'wOptional',
    wOptional: [[], [false], [false, true], [false, true, false]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'xOptional',
    xOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'xOptional',
    xOptional: [[]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'xOptional',
    xOptional: [[new Uint8Array([]).buffer]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'xOptional',
    xOptional: [[], [], []],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'xOptional',
    xOptional: [
      [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    ],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'xOptional',
    xOptional: [
      [],
      [new Uint8Array([]).buffer],
      [new Uint8Array([]).buffer, new Uint8Array([0, 42, 255]).buffer],
      [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    ],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'yOptional',
    yOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'yOptional',
    yOptional: [[]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'yOptional',
    yOptional: [['']],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'yOptional',
    yOptional: [[], [], []],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'yOptional',
    yOptional: [['', '=8 bytes', 'Hello, 幸福!']],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'yOptional',
    yOptional: [[], [''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, 幸福!']],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'zOptional',
    zOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'zOptional',
    zOptional: [[]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'zOptional',
    zOptional: [[{}]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'zOptional',
    zOptional: [[], [], []],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'zOptional',
    zOptional: [[{}, {}, {}]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'zOptional',
    zOptional: [[], [{}], [{}, {}], [{}, {}, {}]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aaOptional',
    aaOptional: [],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aaOptional',
    aaOptional: [[]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aaOptional',
    aaOptional: [[{}]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aaOptional',
    aaOptional: [[], [], []],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aaOptional',
    aaOptional: [[{}, {}, {}]],
    $fallback,
  });

  assertRoundTrip(Bar.size, Bar.serialize, Bar.deserialize, {
    $field: 'aaOptional',
    aaOptional: [[], [{}], [{}, {}], [{}, {}, {}]],
    $fallback,
  });
}

/* eslint-enable @typescript-eslint/no-magic-numbers -- Re-enable this rule. */
