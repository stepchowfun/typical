/* eslint-disable @typescript-eslint/no-magic-numbers -- Allowed just for this file. */

import { Comprehensive } from '../generated/types';
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
  assertRoundTrip(
    Comprehensive.Types.Foo.size,
    Comprehensive.Types.Foo.serialize,
    Comprehensive.Types.Foo.deserialize,
    {
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
    },
  );

  // eslint-disable-next-line no-console -- Allow us to separate the test groups with a line break.
  console.log();

  assertRoundTrip(
    Comprehensive.Types.Foo.size,
    Comprehensive.Types.Foo.serialize,
    Comprehensive.Types.Foo.deserialize,
    {
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
    },
  );

  // eslint-disable-next-line no-console -- Allow us to separate the test groups with a line break.
  console.log();

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aRequired',
      aRequired: null,
    },
    {
      $field: 'aRequired',
      aRequired: null,
    },
  );

  f64TestValues.forEach((bRequired) => {
    assertMatch(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        $field: 'bRequired',
        bRequired,
      },
      {
        $field: 'bRequired',
        bRequired,
      },
    );
  });

  u64TestValues.forEach((cRequired) => {
    assertMatch(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        $field: 'cRequired',
        cRequired,
      },
      {
        $field: 'cRequired',
        cRequired,
      },
    );
  });

  s64TestValues.forEach((dRequired) => {
    assertMatch(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        $field: 'dRequired',
        dRequired,
      },
      {
        $field: 'dRequired',
        dRequired,
      },
    );
  });

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'eRequired',
      eRequired: false,
    },
    {
      $field: 'eRequired',
      eRequired: false,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'eRequired',
      eRequired: true,
    },
    {
      $field: 'eRequired',
      eRequired: true,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'fRequired',
      fRequired: new Uint8Array([]).buffer,
    },
    {
      $field: 'fRequired',
      fRequired: new Uint8Array([]).buffer,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'fRequired',
      fRequired: new Uint8Array([0]).buffer,
    },
    {
      $field: 'fRequired',
      fRequired: new Uint8Array([0]).buffer,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'fRequired',
      fRequired: new Uint8Array([0, 42]).buffer,
    },
    {
      $field: 'fRequired',
      fRequired: new Uint8Array([0, 42]).buffer,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'fRequired',
      fRequired: new Uint8Array([0, 42, 255]).buffer,
    },
    {
      $field: 'fRequired',
      fRequired: new Uint8Array([0, 42, 255]).buffer,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'gRequired',
      gRequired: '',
    },
    {
      $field: 'gRequired',
      gRequired: '',
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'gRequired',
      gRequired: '=8 bytes',
    },
    {
      $field: 'gRequired',
      gRequired: '=8 bytes',
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'gRequired',
      gRequired: 'Hello, 幸福!',
    },
    {
      $field: 'gRequired',
      gRequired: 'Hello, 幸福!',
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'hRequired',
      hRequired: {},
    },
    {
      $field: 'hRequired',
      hRequired: {},
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'iRequired',
      iRequired: {},
    },
    {
      $field: 'iRequired',
      iRequired: {},
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'jRequired',
      jRequired: [],
    },
    {
      $field: 'jRequired',
      jRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'jRequired',
      jRequired: [null],
    },
    {
      $field: 'jRequired',
      jRequired: [null],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'jRequired',
      jRequired: [null, null],
    },
    {
      $field: 'jRequired',
      jRequired: [null, null],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'jRequired',
      jRequired: [null, null, null],
    },
    {
      $field: 'jRequired',
      jRequired: [null, null, null],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'kRequired',
      kRequired: [],
    },
    {
      $field: 'kRequired',
      kRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'kRequired',
      kRequired: [0.0],
    },
    {
      $field: 'kRequired',
      kRequired: [0.0],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'kRequired',
      kRequired: [0.0, Math.PI],
    },
    {
      $field: 'kRequired',
      kRequired: [0.0, Math.PI],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'kRequired',
      kRequired: [0.0, Math.PI, Number.EPSILON],
    },
    {
      $field: 'kRequired',
      kRequired: [0.0, Math.PI, Number.EPSILON],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'kRequired',
      kRequired: f64TestValues,
    },
    {
      $field: 'kRequired',
      kRequired: f64TestValues,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'lRequired',
      lRequired: [],
    },
    {
      $field: 'lRequired',
      lRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'lRequired',
      lRequired: [u64Min],
    },
    {
      $field: 'lRequired',
      lRequired: [u64Min],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'lRequired',
      lRequired: [u64Min, 256n],
    },
    {
      $field: 'lRequired',
      lRequired: [u64Min, 256n],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'lRequired',
      lRequired: [u64Min, 256n, u64Max],
    },
    {
      $field: 'lRequired',
      lRequired: [u64Min, 256n, u64Max],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'lRequired',
      lRequired: u64TestValues,
    },
    {
      $field: 'lRequired',
      lRequired: u64TestValues,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'mRequired',
      mRequired: [],
    },
    {
      $field: 'mRequired',
      mRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'mRequired',
      mRequired: [s64Min],
    },
    {
      $field: 'mRequired',
      mRequired: [s64Min],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'mRequired',
      mRequired: [s64Min, 0n],
    },
    {
      $field: 'mRequired',
      mRequired: [s64Min, 0n],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'mRequired',
      mRequired: [s64Min, 0n, s64Max],
    },
    {
      $field: 'mRequired',
      mRequired: [s64Min, 0n, s64Max],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'mRequired',
      mRequired: s64TestValues,
    },
    {
      $field: 'mRequired',
      mRequired: s64TestValues,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'nRequired',
      nRequired: [],
    },
    {
      $field: 'nRequired',
      nRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'nRequired',
      nRequired: [false],
    },
    {
      $field: 'nRequired',
      nRequired: [false],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'nRequired',
      nRequired: [false, true],
    },
    {
      $field: 'nRequired',
      nRequired: [false, true],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'nRequired',
      nRequired: [false, true, false],
    },
    {
      $field: 'nRequired',
      nRequired: [false, true, false],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'oRequired',
      oRequired: [],
    },
    {
      $field: 'oRequired',
      oRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'oRequired',
      oRequired: [new Uint8Array([]).buffer],
    },
    {
      $field: 'oRequired',
      oRequired: [new Uint8Array([]).buffer],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'oRequired',
      oRequired: [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
      ],
    },
    {
      $field: 'oRequired',
      oRequired: [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
      ],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'oRequired',
      oRequired: [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    },
    {
      $field: 'oRequired',
      oRequired: [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'pRequired',
      pRequired: [],
    },
    {
      $field: 'pRequired',
      pRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'pRequired',
      pRequired: [''],
    },
    {
      $field: 'pRequired',
      pRequired: [''],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'pRequired',
      pRequired: ['', '=8 bytes'],
    },
    {
      $field: 'pRequired',
      pRequired: ['', '=8 bytes'],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'pRequired',
      pRequired: ['', '=8 bytes', 'Hello, 幸福!'],
    },
    {
      $field: 'pRequired',
      pRequired: ['', '=8 bytes', 'Hello, 幸福!'],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'qRequired',
      qRequired: [],
    },
    {
      $field: 'qRequired',
      qRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'qRequired',
      qRequired: [{}],
    },
    {
      $field: 'qRequired',
      qRequired: [{}],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'qRequired',
      qRequired: [{}, {}],
    },
    {
      $field: 'qRequired',
      qRequired: [{}, {}],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'qRequired',
      qRequired: [{}, {}, {}],
    },
    {
      $field: 'qRequired',
      qRequired: [{}, {}, {}],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'rRequired',
      rRequired: [],
    },
    {
      $field: 'rRequired',
      rRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'rRequired',
      rRequired: [{}],
    },
    {
      $field: 'rRequired',
      rRequired: [{}],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'rRequired',
      rRequired: [{}, {}],
    },
    {
      $field: 'rRequired',
      rRequired: [{}, {}],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'rRequired',
      rRequired: [{}, {}, {}],
    },
    {
      $field: 'rRequired',
      rRequired: [{}, {}, {}],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'sRequired',
      sRequired: [],
    },
    {
      $field: 'sRequired',
      sRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'sRequired',
      sRequired: [[]],
    },
    {
      $field: 'sRequired',
      sRequired: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'sRequired',
      sRequired: [[null]],
    },
    {
      $field: 'sRequired',
      sRequired: [[null]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'sRequired',
      sRequired: [[], [], []],
    },
    {
      $field: 'sRequired',
      sRequired: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'sRequired',
      sRequired: [[null, null, null]],
    },
    {
      $field: 'sRequired',
      sRequired: [[null, null, null]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'sRequired',
      sRequired: [[], [null], [null, null], [null, null, null]],
    },
    {
      $field: 'sRequired',
      sRequired: [[], [null], [null, null], [null, null, null]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'tRequired',
      tRequired: [],
    },
    {
      $field: 'tRequired',
      tRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'tRequired',
      tRequired: [[]],
    },
    {
      $field: 'tRequired',
      tRequired: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'tRequired',
      tRequired: [[0.0]],
    },
    {
      $field: 'tRequired',
      tRequired: [[0.0]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'tRequired',
      tRequired: [[], [], []],
    },
    {
      $field: 'tRequired',
      tRequired: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'tRequired',
      tRequired: [f64TestValues],
    },
    {
      $field: 'tRequired',
      tRequired: [f64TestValues],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'tRequired',
      tRequired: [[], [0.0], [0.0, Math.PI], [0.0, Math.PI, Number.EPSILON]],
    },
    {
      $field: 'tRequired',
      tRequired: [[], [0.0], [0.0, Math.PI], [0.0, Math.PI, Number.EPSILON]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'uRequired',
      uRequired: [],
    },
    {
      $field: 'uRequired',
      uRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'uRequired',
      uRequired: [[]],
    },
    {
      $field: 'uRequired',
      uRequired: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'uRequired',
      uRequired: [[u64Min]],
    },
    {
      $field: 'uRequired',
      uRequired: [[u64Min]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'uRequired',
      uRequired: [[], [], []],
    },
    {
      $field: 'uRequired',
      uRequired: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'uRequired',
      uRequired: [u64TestValues],
    },
    {
      $field: 'uRequired',
      uRequired: [u64TestValues],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'uRequired',
      uRequired: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max]],
    },
    {
      $field: 'uRequired',
      uRequired: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'vRequired',
      vRequired: [],
    },
    {
      $field: 'vRequired',
      vRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'vRequired',
      vRequired: [[]],
    },
    {
      $field: 'vRequired',
      vRequired: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'vRequired',
      vRequired: [[s64Min]],
    },
    {
      $field: 'vRequired',
      vRequired: [[s64Min]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'vRequired',
      vRequired: [[], [], []],
    },
    {
      $field: 'vRequired',
      vRequired: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'vRequired',
      vRequired: [s64TestValues],
    },
    {
      $field: 'vRequired',
      vRequired: [s64TestValues],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'vRequired',
      vRequired: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max]],
    },
    {
      $field: 'vRequired',
      vRequired: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'wRequired',
      wRequired: [],
    },
    {
      $field: 'wRequired',
      wRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'wRequired',
      wRequired: [[]],
    },
    {
      $field: 'wRequired',
      wRequired: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'wRequired',
      wRequired: [[false]],
    },
    {
      $field: 'wRequired',
      wRequired: [[false]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'wRequired',
      wRequired: [[], [], []],
    },
    {
      $field: 'wRequired',
      wRequired: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'wRequired',
      wRequired: [[false, true, false]],
    },
    {
      $field: 'wRequired',
      wRequired: [[false, true, false]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'wRequired',
      wRequired: [[], [false], [false, true], [false, true, false]],
    },
    {
      $field: 'wRequired',
      wRequired: [[], [false], [false, true], [false, true, false]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'xRequired',
      xRequired: [],
    },
    {
      $field: 'xRequired',
      xRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'xRequired',
      xRequired: [[]],
    },
    {
      $field: 'xRequired',
      xRequired: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'xRequired',
      xRequired: [[new Uint8Array([]).buffer]],
    },
    {
      $field: 'xRequired',
      xRequired: [[new Uint8Array([]).buffer]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'xRequired',
      xRequired: [[], [], []],
    },
    {
      $field: 'xRequired',
      xRequired: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'xRequired',
      xRequired: [
        [
          new Uint8Array([]).buffer,
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
        ],
      ],
    },
    {
      $field: 'xRequired',
      xRequired: [
        [
          new Uint8Array([]).buffer,
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
        ],
      ],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
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
    },
    {
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
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'yRequired',
      yRequired: [],
    },
    {
      $field: 'yRequired',
      yRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'yRequired',
      yRequired: [[]],
    },
    {
      $field: 'yRequired',
      yRequired: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'yRequired',
      yRequired: [['']],
    },
    {
      $field: 'yRequired',
      yRequired: [['']],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'yRequired',
      yRequired: [[], [], []],
    },
    {
      $field: 'yRequired',
      yRequired: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'yRequired',
      yRequired: [['', '=8 bytes', 'Hello, 幸福!']],
    },
    {
      $field: 'yRequired',
      yRequired: [['', '=8 bytes', 'Hello, 幸福!']],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'yRequired',
      yRequired: [[], [''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, 幸福!']],
    },
    {
      $field: 'yRequired',
      yRequired: [[], [''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, 幸福!']],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'zRequired',
      zRequired: [],
    },
    {
      $field: 'zRequired',
      zRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'zRequired',
      zRequired: [[]],
    },
    {
      $field: 'zRequired',
      zRequired: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'zRequired',
      zRequired: [[{}]],
    },
    {
      $field: 'zRequired',
      zRequired: [[{}]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'zRequired',
      zRequired: [[], [], []],
    },
    {
      $field: 'zRequired',
      zRequired: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'zRequired',
      zRequired: [[{}, {}, {}]],
    },
    {
      $field: 'zRequired',
      zRequired: [[{}, {}, {}]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'zRequired',
      zRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
    },
    {
      $field: 'zRequired',
      zRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aaRequired',
      aaRequired: [],
    },
    {
      $field: 'aaRequired',
      aaRequired: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aaRequired',
      aaRequired: [[]],
    },
    {
      $field: 'aaRequired',
      aaRequired: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aaRequired',
      aaRequired: [[{}]],
    },
    {
      $field: 'aaRequired',
      aaRequired: [[{}]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aaRequired',
      aaRequired: [[], [], []],
    },
    {
      $field: 'aaRequired',
      aaRequired: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aaRequired',
      aaRequired: [[{}, {}, {}]],
    },
    {
      $field: 'aaRequired',
      aaRequired: [[{}, {}, {}]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aaRequired',
      aaRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
    },
    {
      $field: 'aaRequired',
      aaRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
    },
  );

  const $fallback: { $field: 'aRequired'; aRequired: null } = {
    $field: 'aRequired',
    aRequired: null,
  };

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
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
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
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
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
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
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
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

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aOptional',
      aOptional: null,
      $fallback,
    },
    {
      $field: 'aOptional',
      aOptional: null,
      $fallback,
    },
  );

  f64TestValues.forEach((bOptional) => {
    assertMatch(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        $field: 'bOptional',
        bOptional,
        $fallback,
      },
      {
        $field: 'bOptional',
        bOptional,
        $fallback,
      },
    );
  });

  u64TestValues.forEach((cOptional) => {
    assertMatch(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        $field: 'cOptional',
        cOptional,
        $fallback,
      },
      {
        $field: 'cOptional',
        cOptional,
        $fallback,
      },
    );
  });

  s64TestValues.forEach((dOptional) => {
    assertMatch(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        $field: 'dOptional',
        dOptional,
        $fallback,
      },
      {
        $field: 'dOptional',
        dOptional,
        $fallback,
      },
    );
  });

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'eOptional',
      eOptional: false,
      $fallback,
    },
    {
      $field: 'eOptional',
      eOptional: false,
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'eOptional',
      eOptional: true,
      $fallback,
    },
    {
      $field: 'eOptional',
      eOptional: true,
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'fOptional',
      fOptional: new Uint8Array([]).buffer,
      $fallback,
    },
    {
      $field: 'fOptional',
      fOptional: new Uint8Array([]).buffer,
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'fOptional',
      fOptional: new Uint8Array([0]).buffer,
      $fallback,
    },
    {
      $field: 'fOptional',
      fOptional: new Uint8Array([0]).buffer,
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'fOptional',
      fOptional: new Uint8Array([0, 42]).buffer,
      $fallback,
    },
    {
      $field: 'fOptional',
      fOptional: new Uint8Array([0, 42]).buffer,
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'fOptional',
      fOptional: new Uint8Array([0, 42, 255]).buffer,
      $fallback,
    },
    {
      $field: 'fOptional',
      fOptional: new Uint8Array([0, 42, 255]).buffer,
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'gOptional',
      gOptional: '',
      $fallback,
    },
    {
      $field: 'gOptional',
      gOptional: '',
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'gOptional',
      gOptional: '=8 bytes',
      $fallback,
    },
    {
      $field: 'gOptional',
      gOptional: '=8 bytes',
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'gOptional',
      gOptional: 'Hello, 幸福!',
      $fallback,
    },
    {
      $field: 'gOptional',
      gOptional: 'Hello, 幸福!',
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'hOptional',
      hOptional: {},
      $fallback,
    },
    {
      $field: 'hOptional',
      hOptional: {},
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'iOptional',
      iOptional: {},
      $fallback,
    },
    {
      $field: 'iOptional',
      iOptional: {},
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'jOptional',
      jOptional: [],
      $fallback,
    },
    {
      $field: 'jOptional',
      jOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'jOptional',
      jOptional: [null],
      $fallback,
    },
    {
      $field: 'jOptional',
      jOptional: [null],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'jOptional',
      jOptional: [null, null],
      $fallback,
    },
    {
      $field: 'jOptional',
      jOptional: [null, null],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'jOptional',
      jOptional: [null, null, null],
      $fallback,
    },
    {
      $field: 'jOptional',
      jOptional: [null, null, null],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'kOptional',
      kOptional: [],
      $fallback,
    },
    {
      $field: 'kOptional',
      kOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'kOptional',
      kOptional: [0.0],
      $fallback,
    },
    {
      $field: 'kOptional',
      kOptional: [0.0],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'kOptional',
      kOptional: [0.0, Math.PI],
      $fallback,
    },
    {
      $field: 'kOptional',
      kOptional: [0.0, Math.PI],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'kOptional',
      kOptional: [0.0, Math.PI, Number.EPSILON],
      $fallback,
    },
    {
      $field: 'kOptional',
      kOptional: [0.0, Math.PI, Number.EPSILON],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'kOptional',
      kOptional: f64TestValues,
      $fallback,
    },
    {
      $field: 'kOptional',
      kOptional: f64TestValues,
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'lOptional',
      lOptional: [],
      $fallback,
    },
    {
      $field: 'lOptional',
      lOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'lOptional',
      lOptional: [u64Min],
      $fallback,
    },
    {
      $field: 'lOptional',
      lOptional: [u64Min],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'lOptional',
      lOptional: [u64Min, 256n],
      $fallback,
    },
    {
      $field: 'lOptional',
      lOptional: [u64Min, 256n],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'lOptional',
      lOptional: [u64Min, 256n, u64Max],
      $fallback,
    },
    {
      $field: 'lOptional',
      lOptional: [u64Min, 256n, u64Max],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'lOptional',
      lOptional: u64TestValues,
      $fallback,
    },
    {
      $field: 'lOptional',
      lOptional: u64TestValues,
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'mOptional',
      mOptional: [],
      $fallback,
    },
    {
      $field: 'mOptional',
      mOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'mOptional',
      mOptional: [s64Min],
      $fallback,
    },
    {
      $field: 'mOptional',
      mOptional: [s64Min],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'mOptional',
      mOptional: [s64Min, 0n],
      $fallback,
    },
    {
      $field: 'mOptional',
      mOptional: [s64Min, 0n],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'mOptional',
      mOptional: [s64Min, 0n, s64Max],
      $fallback,
    },
    {
      $field: 'mOptional',
      mOptional: [s64Min, 0n, s64Max],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'mOptional',
      mOptional: s64TestValues,
      $fallback,
    },
    {
      $field: 'mOptional',
      mOptional: s64TestValues,
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'nOptional',
      nOptional: [],
      $fallback,
    },
    {
      $field: 'nOptional',
      nOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'nOptional',
      nOptional: [false],
      $fallback,
    },
    {
      $field: 'nOptional',
      nOptional: [false],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'nOptional',
      nOptional: [false, true],
      $fallback,
    },
    {
      $field: 'nOptional',
      nOptional: [false, true],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'nOptional',
      nOptional: [false, true, false],
      $fallback,
    },
    {
      $field: 'nOptional',
      nOptional: [false, true, false],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'oOptional',
      oOptional: [],
      $fallback,
    },
    {
      $field: 'oOptional',
      oOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'oOptional',
      oOptional: [new Uint8Array([]).buffer],
      $fallback,
    },
    {
      $field: 'oOptional',
      oOptional: [new Uint8Array([]).buffer],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'oOptional',
      oOptional: [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
      ],
      $fallback,
    },
    {
      $field: 'oOptional',
      oOptional: [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
      ],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'oOptional',
      oOptional: [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
      $fallback,
    },
    {
      $field: 'oOptional',
      oOptional: [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'pOptional',
      pOptional: [],
      $fallback,
    },
    {
      $field: 'pOptional',
      pOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'pOptional',
      pOptional: [''],
      $fallback,
    },
    {
      $field: 'pOptional',
      pOptional: [''],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'pOptional',
      pOptional: ['', '=8 bytes'],
      $fallback,
    },
    {
      $field: 'pOptional',
      pOptional: ['', '=8 bytes'],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'pOptional',
      pOptional: ['', '=8 bytes', 'Hello, 幸福!'],
      $fallback,
    },
    {
      $field: 'pOptional',
      pOptional: ['', '=8 bytes', 'Hello, 幸福!'],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'qOptional',
      qOptional: [],
      $fallback,
    },
    {
      $field: 'qOptional',
      qOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'qOptional',
      qOptional: [{}],
      $fallback,
    },
    {
      $field: 'qOptional',
      qOptional: [{}],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'qOptional',
      qOptional: [{}, {}],
      $fallback,
    },
    {
      $field: 'qOptional',
      qOptional: [{}, {}],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'qOptional',
      qOptional: [{}, {}, {}],
      $fallback,
    },
    {
      $field: 'qOptional',
      qOptional: [{}, {}, {}],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'rOptional',
      rOptional: [],
      $fallback,
    },
    {
      $field: 'rOptional',
      rOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'rOptional',
      rOptional: [{}],
      $fallback,
    },
    {
      $field: 'rOptional',
      rOptional: [{}],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'rOptional',
      rOptional: [{}, {}],
      $fallback,
    },
    {
      $field: 'rOptional',
      rOptional: [{}, {}],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'rOptional',
      rOptional: [{}, {}, {}],
      $fallback,
    },
    {
      $field: 'rOptional',
      rOptional: [{}, {}, {}],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'sOptional',
      sOptional: [],
      $fallback,
    },
    {
      $field: 'sOptional',
      sOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'sOptional',
      sOptional: [[]],
      $fallback,
    },
    {
      $field: 'sOptional',
      sOptional: [[]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'sOptional',
      sOptional: [[null]],
      $fallback,
    },
    {
      $field: 'sOptional',
      sOptional: [[null]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'sOptional',
      sOptional: [[], [], []],
      $fallback,
    },
    {
      $field: 'sOptional',
      sOptional: [[], [], []],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'sOptional',
      sOptional: [[null, null, null]],
      $fallback,
    },
    {
      $field: 'sOptional',
      sOptional: [[null, null, null]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'sOptional',
      sOptional: [[], [null], [null, null], [null, null, null]],
      $fallback,
    },
    {
      $field: 'sOptional',
      sOptional: [[], [null], [null, null], [null, null, null]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'tOptional',
      tOptional: [],
      $fallback,
    },
    {
      $field: 'tOptional',
      tOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'tOptional',
      tOptional: [[]],
      $fallback,
    },
    {
      $field: 'tOptional',
      tOptional: [[]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'tOptional',
      tOptional: [[0.0]],
      $fallback,
    },
    {
      $field: 'tOptional',
      tOptional: [[0.0]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'tOptional',
      tOptional: [[], [], []],
      $fallback,
    },
    {
      $field: 'tOptional',
      tOptional: [[], [], []],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'tOptional',
      tOptional: [f64TestValues],
      $fallback,
    },
    {
      $field: 'tOptional',
      tOptional: [f64TestValues],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'tOptional',
      tOptional: [[], [0.0], [0.0, Math.PI], [0.0, Math.PI, Number.EPSILON]],
      $fallback,
    },
    {
      $field: 'tOptional',
      tOptional: [[], [0.0], [0.0, Math.PI], [0.0, Math.PI, Number.EPSILON]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'uOptional',
      uOptional: [],
      $fallback,
    },
    {
      $field: 'uOptional',
      uOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'uOptional',
      uOptional: [[]],
      $fallback,
    },
    {
      $field: 'uOptional',
      uOptional: [[]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'uOptional',
      uOptional: [[u64Min]],
      $fallback,
    },
    {
      $field: 'uOptional',
      uOptional: [[u64Min]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'uOptional',
      uOptional: [[], [], []],
      $fallback,
    },
    {
      $field: 'uOptional',
      uOptional: [[], [], []],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'uOptional',
      uOptional: [u64TestValues],
      $fallback,
    },
    {
      $field: 'uOptional',
      uOptional: [u64TestValues],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'uOptional',
      uOptional: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max]],
      $fallback,
    },
    {
      $field: 'uOptional',
      uOptional: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'vOptional',
      vOptional: [],
      $fallback,
    },
    {
      $field: 'vOptional',
      vOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'vOptional',
      vOptional: [[]],
      $fallback,
    },
    {
      $field: 'vOptional',
      vOptional: [[]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'vOptional',
      vOptional: [[s64Min]],
      $fallback,
    },
    {
      $field: 'vOptional',
      vOptional: [[s64Min]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'vOptional',
      vOptional: [[], [], []],
      $fallback,
    },
    {
      $field: 'vOptional',
      vOptional: [[], [], []],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'vOptional',
      vOptional: [s64TestValues],
      $fallback,
    },
    {
      $field: 'vOptional',
      vOptional: [s64TestValues],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'vOptional',
      vOptional: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max]],
      $fallback,
    },
    {
      $field: 'vOptional',
      vOptional: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'wOptional',
      wOptional: [],
      $fallback,
    },
    {
      $field: 'wOptional',
      wOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'wOptional',
      wOptional: [[]],
      $fallback,
    },
    {
      $field: 'wOptional',
      wOptional: [[]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'wOptional',
      wOptional: [[false]],
      $fallback,
    },
    {
      $field: 'wOptional',
      wOptional: [[false]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'wOptional',
      wOptional: [[], [], []],
      $fallback,
    },
    {
      $field: 'wOptional',
      wOptional: [[], [], []],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'wOptional',
      wOptional: [[false, true, false]],
      $fallback,
    },
    {
      $field: 'wOptional',
      wOptional: [[false, true, false]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'wOptional',
      wOptional: [[], [false], [false, true], [false, true, false]],
      $fallback,
    },
    {
      $field: 'wOptional',
      wOptional: [[], [false], [false, true], [false, true, false]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'xOptional',
      xOptional: [],
      $fallback,
    },
    {
      $field: 'xOptional',
      xOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'xOptional',
      xOptional: [[]],
      $fallback,
    },
    {
      $field: 'xOptional',
      xOptional: [[]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'xOptional',
      xOptional: [[new Uint8Array([]).buffer]],
      $fallback,
    },
    {
      $field: 'xOptional',
      xOptional: [[new Uint8Array([]).buffer]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'xOptional',
      xOptional: [[], [], []],
      $fallback,
    },
    {
      $field: 'xOptional',
      xOptional: [[], [], []],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'xOptional',
      xOptional: [
        [
          new Uint8Array([]).buffer,
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
        ],
      ],
      $fallback,
    },
    {
      $field: 'xOptional',
      xOptional: [
        [
          new Uint8Array([]).buffer,
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
        ],
      ],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
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
    },
    {
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
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'yOptional',
      yOptional: [],
      $fallback,
    },
    {
      $field: 'yOptional',
      yOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'yOptional',
      yOptional: [[]],
      $fallback,
    },
    {
      $field: 'yOptional',
      yOptional: [[]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'yOptional',
      yOptional: [['']],
      $fallback,
    },
    {
      $field: 'yOptional',
      yOptional: [['']],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'yOptional',
      yOptional: [[], [], []],
      $fallback,
    },
    {
      $field: 'yOptional',
      yOptional: [[], [], []],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'yOptional',
      yOptional: [['', '=8 bytes', 'Hello, 幸福!']],
      $fallback,
    },
    {
      $field: 'yOptional',
      yOptional: [['', '=8 bytes', 'Hello, 幸福!']],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'yOptional',
      yOptional: [[], [''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, 幸福!']],
      $fallback,
    },
    {
      $field: 'yOptional',
      yOptional: [[], [''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, 幸福!']],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'zOptional',
      zOptional: [],
      $fallback,
    },
    {
      $field: 'zOptional',
      zOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'zOptional',
      zOptional: [[]],
      $fallback,
    },
    {
      $field: 'zOptional',
      zOptional: [[]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'zOptional',
      zOptional: [[{}]],
      $fallback,
    },
    {
      $field: 'zOptional',
      zOptional: [[{}]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'zOptional',
      zOptional: [[], [], []],
      $fallback,
    },
    {
      $field: 'zOptional',
      zOptional: [[], [], []],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'zOptional',
      zOptional: [[{}, {}, {}]],
      $fallback,
    },
    {
      $field: 'zOptional',
      zOptional: [[{}, {}, {}]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'zOptional',
      zOptional: [[], [{}], [{}, {}], [{}, {}, {}]],
      $fallback,
    },
    {
      $field: 'zOptional',
      zOptional: [[], [{}], [{}, {}], [{}, {}, {}]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aaOptional',
      aaOptional: [],
      $fallback,
    },
    {
      $field: 'aaOptional',
      aaOptional: [],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aaOptional',
      aaOptional: [[]],
      $fallback,
    },
    {
      $field: 'aaOptional',
      aaOptional: [[]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aaOptional',
      aaOptional: [[{}]],
      $fallback,
    },
    {
      $field: 'aaOptional',
      aaOptional: [[{}]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aaOptional',
      aaOptional: [[], [], []],
      $fallback,
    },
    {
      $field: 'aaOptional',
      aaOptional: [[], [], []],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aaOptional',
      aaOptional: [[{}, {}, {}]],
      $fallback,
    },
    {
      $field: 'aaOptional',
      aaOptional: [[{}, {}, {}]],
      $fallback,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      $field: 'aaOptional',
      aaOptional: [[], [{}], [{}, {}], [{}, {}, {}]],
      $fallback,
    },
    {
      $field: 'aaOptional',
      aaOptional: [[], [{}], [{}, {}], [{}, {}, {}]],
      $fallback,
    },
  );
}

/* eslint-enable @typescript-eslint/no-magic-numbers -- Re-enable this rule. */
