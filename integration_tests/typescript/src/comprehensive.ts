import { Comprehensive } from '../generated/types';
import { assertMatch, assertRoundTrip } from './assertions';

const u64Min = 0n;
const u64Max = 18_446_744_073_709_551_615n;
const s64Min = -9_223_372_036_854_775_808n;
const s64Max = 9_223_372_036_854_775_807n;

const f64TestValues: number[] = [
  0.0,
  Math.PI,
  Number.EPSILON,
  Number.POSITIVE_INFINITY,
  Number.MAX_VALUE,
  -Number.MAX_VALUE,
  Number.MIN_VALUE,
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
      gRequired: 'Hello, World!',
      hRequired: {},
      iRequired: {},
      jRequired: [null, null, null],
      kRequired: f64TestValues,
      lRequired: u64TestValues,
      mRequired: s64TestValues,
      nRequired: [false, true, false],
      oRequired: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
        new Uint8Array([2, 44, 253]).buffer,
      ],
      pRequired: ['', '=8 bytes', 'Hello, World!'],
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
        [new Uint8Array([0, 42, 255]).buffer],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
        ],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
      yRequired: [[''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, World!']],
      zRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
      aaRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
      aAsymmetric: null,
      bAsymmetric: Math.PI,
      cAsymmetric: u64Max,
      dAsymmetric: s64Max,
      eAsymmetric: true,
      fAsymmetric: new Uint8Array([0, 42, 255]).buffer,
      gAsymmetric: 'Hello, World!',
      hAsymmetric: {},
      iAsymmetric: {},
      jAsymmetric: [null, null, null],
      kAsymmetric: f64TestValues,
      lAsymmetric: u64TestValues,
      mAsymmetric: s64TestValues,
      nAsymmetric: [false, true, false],
      oAsymmetric: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
        new Uint8Array([2, 44, 253]).buffer,
      ],
      pAsymmetric: ['', '=8 bytes', 'Hello, World!'],
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
        [new Uint8Array([0, 42, 255]).buffer],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
        ],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
      yAsymmetric: [[''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, World!']],
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

  // eslint-disable-next-line no-console
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
      gRequired: 'Hello, World!',
      hRequired: {},
      iRequired: {},
      jRequired: [null, null, null],
      kRequired: f64TestValues,
      lRequired: u64TestValues,
      mRequired: s64TestValues,
      nRequired: [false, true, false],
      oRequired: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
        new Uint8Array([2, 44, 253]).buffer,
      ],
      pRequired: ['', '=8 bytes', 'Hello, World!'],
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
        [new Uint8Array([0, 42, 255]).buffer],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
        ],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
      yRequired: [[''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, World!']],
      zRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
      aaRequired: [[], [{}], [{}, {}], [{}, {}, {}]],
      aAsymmetric: null,
      bAsymmetric: Math.PI,
      cAsymmetric: u64Max,
      dAsymmetric: s64Max,
      eAsymmetric: true,
      fAsymmetric: new Uint8Array([0, 42, 255]).buffer,
      gAsymmetric: 'Hello, World!',
      hAsymmetric: {},
      iAsymmetric: {},
      jAsymmetric: [null, null, null],
      kAsymmetric: f64TestValues,
      lAsymmetric: u64TestValues,
      mAsymmetric: s64TestValues,
      nAsymmetric: [false, true, false],
      oAsymmetric: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
        new Uint8Array([2, 44, 253]).buffer,
      ],
      pAsymmetric: ['', '=8 bytes', 'Hello, World!'],
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
        [new Uint8Array([0, 42, 255]).buffer],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
        ],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
      yAsymmetric: [[''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, World!']],
      zAsymmetric: [[], [{}], [{}, {}], [{}, {}, {}]],
      aaAsymmetric: [[], [{}], [{}, {}], [{}, {}, {}]],
      aOptional: null,
      bOptional: Math.PI,
      cOptional: u64Max,
      dOptional: s64Max,
      eOptional: true,
      fOptional: new Uint8Array([0, 42, 255]).buffer,
      gOptional: 'Hello, World!',
      hOptional: {},
      iOptional: {},
      jOptional: [null, null, null],
      kOptional: f64TestValues,
      lOptional: u64TestValues,
      mOptional: s64TestValues,
      nOptional: [false, true, false],
      oOptional: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
        new Uint8Array([2, 44, 253]).buffer,
      ],
      pOptional: ['', '=8 bytes', 'Hello, World!'],
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
        [new Uint8Array([0, 42, 255]).buffer],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
        ],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
      yOptional: [[''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, World!']],
      zOptional: [[], [{}], [{}, {}], [{}, {}, {}]],
      aaOptional: [[], [{}], [{}, {}], [{}, {}, {}]],
    },
  );

  // eslint-disable-next-line no-console
  console.log();

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aRequired',
    },
  );

  f64TestValues.forEach((value) => {
    assertRoundTrip(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        field: 'bRequired',
        value,
      },
    );
  });

  u64TestValues.forEach((value) => {
    assertRoundTrip(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        field: 'cRequired',
        value,
      },
    );
  });

  s64TestValues.forEach((value) => {
    assertRoundTrip(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        field: 'dRequired',
        value,
      },
    );
  });

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'eRequired',
      value: false,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'eRequired',
      value: true,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'fRequired',
      value: new Uint8Array([]).buffer,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'fRequired',
      value: new Uint8Array([0]).buffer,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'fRequired',
      value: new Uint8Array([0, 42]).buffer,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'fRequired',
      value: new Uint8Array([0, 42, 255]).buffer,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'gRequired',
      value: '',
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'gRequired',
      value: '=8 bytes',
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'gRequired',
      value: 'Hello, World!',
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'hRequired',
      value: {},
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'iRequired',
      value: {},
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'jRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'jRequired',
      value: [null],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'jRequired',
      value: [null, null],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'jRequired',
      value: [null, null, null],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kRequired',
      value: [0.0],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kRequired',
      value: [0.0, Math.PI],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kRequired',
      value: [0.0, Math.PI, Number.EPSILON],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kRequired',
      value: f64TestValues,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lRequired',
      value: [u64Min],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lRequired',
      value: [u64Min, 256n],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lRequired',
      value: [u64Min, 256n, u64Max],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lRequired',
      value: u64TestValues,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mRequired',
      value: [s64Min],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mRequired',
      value: [s64Min, 0n],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mRequired',
      value: [s64Min, 0n, s64Max],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mRequired',
      value: s64TestValues,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'nRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'nRequired',
      value: [false],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'nRequired',
      value: [false, true],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'nRequired',
      value: [false, true, false],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'oRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'oRequired',
      value: [new Uint8Array([0, 42, 255]).buffer],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'oRequired',
      value: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
      ],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'oRequired',
      value: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
        new Uint8Array([2, 44, 253]).buffer,
      ],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'pRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'pRequired',
      value: [''],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'pRequired',
      value: ['', '=8 bytes'],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'pRequired',
      value: ['', '=8 bytes', 'Hello, World!'],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'qRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'qRequired',
      value: [{}],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'qRequired',
      value: [{}, {}],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'qRequired',
      value: [{}, {}, {}],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'rRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'rRequired',
      value: [{}],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'rRequired',
      value: [{}, {}],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'rRequired',
      value: [{}, {}, {}],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sRequired',
      value: [[]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sRequired',
      value: [[null]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sRequired',
      value: [[], [], []],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sRequired',
      value: [[null, null, null]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sRequired',
      value: [[], [null], [null, null], [null, null, null]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tRequired',
      value: [[]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tRequired',
      value: [[0.0]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tRequired',
      value: [[], [], []],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tRequired',
      value: [f64TestValues],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tRequired',
      value: [[], [0.0], [0.0, Math.PI], [0.0, Math.PI, Number.EPSILON]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uRequired',
      value: [[]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uRequired',
      value: [[u64Min]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uRequired',
      value: [[], [], []],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uRequired',
      value: [u64TestValues],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uRequired',
      value: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vRequired',
      value: [[]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vRequired',
      value: [[s64Min]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vRequired',
      value: [[], [], []],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vRequired',
      value: [s64TestValues],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vRequired',
      value: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wRequired',
      value: [[]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wRequired',
      value: [[false]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wRequired',
      value: [[], [], []],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wRequired',
      value: [[false, true, false]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wRequired',
      value: [[], [false], [false, true], [false, true, false]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xRequired',
      value: [[]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xRequired',
      value: [[new Uint8Array([0, 42, 255]).buffer]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xRequired',
      value: [[], [], []],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xRequired',
      value: [
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xRequired',
      value: [
        [new Uint8Array([0, 42, 255]).buffer],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
        ],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yRequired',
      value: [[]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yRequired',
      value: [['']],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yRequired',
      value: [[], [], []],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yRequired',
      value: [['', '=8 bytes', 'Hello, World!']],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yRequired',
      value: [[], [''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, World!']],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zRequired',
      value: [[]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zRequired',
      value: [[{}]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zRequired',
      value: [[], [], []],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zRequired',
      value: [[{}, {}, {}]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zRequired',
      value: [[], [{}], [{}, {}], [{}, {}, {}]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaRequired',
      value: [],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaRequired',
      value: [[]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaRequired',
      value: [[{}]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaRequired',
      value: [[], [], []],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaRequired',
      value: [[{}, {}, {}]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaRequired',
      value: [[], [{}], [{}, {}], [{}, {}, {}]],
    },
  );

  const fallback: { field: 'aRequired' } = { field: 'aRequired' };

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aAsymmetric',
      fallback,
    },
    {
      field: 'aAsymmetric',
    },
  );

  f64TestValues.forEach((value) => {
    assertMatch(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        field: 'bAsymmetric',
        value,
        fallback,
      },
      {
        field: 'bAsymmetric',
        value,
      },
    );
  });

  u64TestValues.forEach((value) => {
    assertMatch(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        field: 'cAsymmetric',
        value,
        fallback,
      },
      {
        field: 'cAsymmetric',
        value,
      },
    );
  });

  s64TestValues.forEach((value) => {
    assertMatch(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        field: 'dAsymmetric',
        value,
        fallback,
      },
      {
        field: 'dAsymmetric',
        value,
      },
    );
  });

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'eAsymmetric',
      value: false,
      fallback,
    },
    {
      field: 'eAsymmetric',
      value: false,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'eAsymmetric',
      value: true,
      fallback,
    },
    {
      field: 'eAsymmetric',
      value: true,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'fAsymmetric',
      value: new Uint8Array([]).buffer,
      fallback,
    },
    {
      field: 'fAsymmetric',
      value: new Uint8Array([]).buffer,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'fAsymmetric',
      value: new Uint8Array([0]).buffer,
      fallback,
    },
    {
      field: 'fAsymmetric',
      value: new Uint8Array([0]).buffer,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'fAsymmetric',
      value: new Uint8Array([0, 42]).buffer,
      fallback,
    },
    {
      field: 'fAsymmetric',
      value: new Uint8Array([0, 42]).buffer,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'fAsymmetric',
      value: new Uint8Array([0, 42, 255]).buffer,
      fallback,
    },
    {
      field: 'fAsymmetric',
      value: new Uint8Array([0, 42, 255]).buffer,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'gAsymmetric',
      value: '',
      fallback,
    },
    {
      field: 'gAsymmetric',
      value: '',
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'gAsymmetric',
      value: '=8 bytes',
      fallback,
    },
    {
      field: 'gAsymmetric',
      value: '=8 bytes',
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'gAsymmetric',
      value: 'Hello, World!',
      fallback,
    },
    {
      field: 'gAsymmetric',
      value: 'Hello, World!',
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'hAsymmetric',
      value: {},
      fallback,
    },
    {
      field: 'hAsymmetric',
      value: {},
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'iAsymmetric',
      value: {},
      fallback,
    },
    {
      field: 'iAsymmetric',
      value: {},
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'jAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'jAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'jAsymmetric',
      value: [null],
      fallback,
    },
    {
      field: 'jAsymmetric',
      value: [null],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'jAsymmetric',
      value: [null, null],
      fallback,
    },
    {
      field: 'jAsymmetric',
      value: [null, null],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'jAsymmetric',
      value: [null, null, null],
      fallback,
    },
    {
      field: 'jAsymmetric',
      value: [null, null, null],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'kAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kAsymmetric',
      value: [0.0],
      fallback,
    },
    {
      field: 'kAsymmetric',
      value: [0.0],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kAsymmetric',
      value: [0.0, Math.PI],
      fallback,
    },
    {
      field: 'kAsymmetric',
      value: [0.0, Math.PI],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kAsymmetric',
      value: [0.0, Math.PI, Number.EPSILON],
      fallback,
    },
    {
      field: 'kAsymmetric',
      value: [0.0, Math.PI, Number.EPSILON],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kAsymmetric',
      value: f64TestValues,
      fallback,
    },
    {
      field: 'kAsymmetric',
      value: f64TestValues,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'lAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lAsymmetric',
      value: [u64Min],
      fallback,
    },
    {
      field: 'lAsymmetric',
      value: [u64Min],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lAsymmetric',
      value: [u64Min, 256n],
      fallback,
    },
    {
      field: 'lAsymmetric',
      value: [u64Min, 256n],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lAsymmetric',
      value: [u64Min, 256n, u64Max],
      fallback,
    },
    {
      field: 'lAsymmetric',
      value: [u64Min, 256n, u64Max],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lAsymmetric',
      value: u64TestValues,
      fallback,
    },
    {
      field: 'lAsymmetric',
      value: u64TestValues,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'mAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mAsymmetric',
      value: [s64Min],
      fallback,
    },
    {
      field: 'mAsymmetric',
      value: [s64Min],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mAsymmetric',
      value: [s64Min, 0n],
      fallback,
    },
    {
      field: 'mAsymmetric',
      value: [s64Min, 0n],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mAsymmetric',
      value: [s64Min, 0n, s64Max],
      fallback,
    },
    {
      field: 'mAsymmetric',
      value: [s64Min, 0n, s64Max],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mAsymmetric',
      value: s64TestValues,
      fallback,
    },
    {
      field: 'mAsymmetric',
      value: s64TestValues,
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'nAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'nAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'nAsymmetric',
      value: [false],
      fallback,
    },
    {
      field: 'nAsymmetric',
      value: [false],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'nAsymmetric',
      value: [false, true],
      fallback,
    },
    {
      field: 'nAsymmetric',
      value: [false, true],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'nAsymmetric',
      value: [false, true, false],
      fallback,
    },
    {
      field: 'nAsymmetric',
      value: [false, true, false],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'oAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'oAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'oAsymmetric',
      value: [new Uint8Array([0, 42, 255]).buffer],
      fallback,
    },
    {
      field: 'oAsymmetric',
      value: [new Uint8Array([0, 42, 255]).buffer],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'oAsymmetric',
      value: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
      ],
      fallback,
    },
    {
      field: 'oAsymmetric',
      value: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
      ],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'oAsymmetric',
      value: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
        new Uint8Array([2, 44, 253]).buffer,
      ],
      fallback,
    },
    {
      field: 'oAsymmetric',
      value: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
        new Uint8Array([2, 44, 253]).buffer,
      ],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'pAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'pAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'pAsymmetric',
      value: [''],
      fallback,
    },
    {
      field: 'pAsymmetric',
      value: [''],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'pAsymmetric',
      value: ['', '=8 bytes'],
      fallback,
    },
    {
      field: 'pAsymmetric',
      value: ['', '=8 bytes'],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'pAsymmetric',
      value: ['', '=8 bytes', 'Hello, World!'],
      fallback,
    },
    {
      field: 'pAsymmetric',
      value: ['', '=8 bytes', 'Hello, World!'],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'qAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'qAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'qAsymmetric',
      value: [{}],
      fallback,
    },
    {
      field: 'qAsymmetric',
      value: [{}],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'qAsymmetric',
      value: [{}, {}],
      fallback,
    },
    {
      field: 'qAsymmetric',
      value: [{}, {}],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'qAsymmetric',
      value: [{}, {}, {}],
      fallback,
    },
    {
      field: 'qAsymmetric',
      value: [{}, {}, {}],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'rAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'rAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'rAsymmetric',
      value: [{}],
      fallback,
    },
    {
      field: 'rAsymmetric',
      value: [{}],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'rAsymmetric',
      value: [{}, {}],
      fallback,
    },
    {
      field: 'rAsymmetric',
      value: [{}, {}],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'rAsymmetric',
      value: [{}, {}, {}],
      fallback,
    },
    {
      field: 'rAsymmetric',
      value: [{}, {}, {}],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'sAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sAsymmetric',
      value: [[]],
      fallback,
    },
    {
      field: 'sAsymmetric',
      value: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sAsymmetric',
      value: [[null]],
      fallback,
    },
    {
      field: 'sAsymmetric',
      value: [[null]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sAsymmetric',
      value: [[], [], []],
      fallback,
    },
    {
      field: 'sAsymmetric',
      value: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sAsymmetric',
      value: [[null, null, null]],
      fallback,
    },
    {
      field: 'sAsymmetric',
      value: [[null, null, null]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sAsymmetric',
      value: [[], [null], [null, null], [null, null, null]],
      fallback,
    },
    {
      field: 'sAsymmetric',
      value: [[], [null], [null, null], [null, null, null]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'tAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tAsymmetric',
      value: [[]],
      fallback,
    },
    {
      field: 'tAsymmetric',
      value: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tAsymmetric',
      value: [[0.0]],
      fallback,
    },
    {
      field: 'tAsymmetric',
      value: [[0.0]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tAsymmetric',
      value: [[], [], []],
      fallback,
    },
    {
      field: 'tAsymmetric',
      value: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tAsymmetric',
      value: [f64TestValues],
      fallback,
    },
    {
      field: 'tAsymmetric',
      value: [f64TestValues],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tAsymmetric',
      value: [[], [0.0], [0.0, Math.PI], [0.0, Math.PI, Number.EPSILON]],
      fallback,
    },
    {
      field: 'tAsymmetric',
      value: [[], [0.0], [0.0, Math.PI], [0.0, Math.PI, Number.EPSILON]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'uAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uAsymmetric',
      value: [[]],
      fallback,
    },
    {
      field: 'uAsymmetric',
      value: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uAsymmetric',
      value: [[u64Min]],
      fallback,
    },
    {
      field: 'uAsymmetric',
      value: [[u64Min]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uAsymmetric',
      value: [[], [], []],
      fallback,
    },
    {
      field: 'uAsymmetric',
      value: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uAsymmetric',
      value: [u64TestValues],
      fallback,
    },
    {
      field: 'uAsymmetric',
      value: [u64TestValues],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uAsymmetric',
      value: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max]],
      fallback,
    },
    {
      field: 'uAsymmetric',
      value: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'vAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vAsymmetric',
      value: [[]],
      fallback,
    },
    {
      field: 'vAsymmetric',
      value: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vAsymmetric',
      value: [[s64Min]],
      fallback,
    },
    {
      field: 'vAsymmetric',
      value: [[s64Min]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vAsymmetric',
      value: [[], [], []],
      fallback,
    },
    {
      field: 'vAsymmetric',
      value: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vAsymmetric',
      value: [s64TestValues],
      fallback,
    },
    {
      field: 'vAsymmetric',
      value: [s64TestValues],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vAsymmetric',
      value: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max]],
      fallback,
    },
    {
      field: 'vAsymmetric',
      value: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'wAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wAsymmetric',
      value: [[]],
      fallback,
    },
    {
      field: 'wAsymmetric',
      value: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wAsymmetric',
      value: [[false]],
      fallback,
    },
    {
      field: 'wAsymmetric',
      value: [[false]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wAsymmetric',
      value: [[], [], []],
      fallback,
    },
    {
      field: 'wAsymmetric',
      value: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wAsymmetric',
      value: [[false, true, false]],
      fallback,
    },
    {
      field: 'wAsymmetric',
      value: [[false, true, false]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wAsymmetric',
      value: [[], [false], [false, true], [false, true, false]],
      fallback,
    },
    {
      field: 'wAsymmetric',
      value: [[], [false], [false, true], [false, true, false]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'xAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xAsymmetric',
      value: [[]],
      fallback,
    },
    {
      field: 'xAsymmetric',
      value: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xAsymmetric',
      value: [[new Uint8Array([0, 42, 255]).buffer]],
      fallback,
    },
    {
      field: 'xAsymmetric',
      value: [[new Uint8Array([0, 42, 255]).buffer]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xAsymmetric',
      value: [[], [], []],
      fallback,
    },
    {
      field: 'xAsymmetric',
      value: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xAsymmetric',
      value: [
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
      fallback,
    },
    {
      field: 'xAsymmetric',
      value: [
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xAsymmetric',
      value: [
        [new Uint8Array([0, 42, 255]).buffer],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
        ],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
      fallback,
    },
    {
      field: 'xAsymmetric',
      value: [
        [new Uint8Array([0, 42, 255]).buffer],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
        ],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'yAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yAsymmetric',
      value: [[]],
      fallback,
    },
    {
      field: 'yAsymmetric',
      value: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yAsymmetric',
      value: [['']],
      fallback,
    },
    {
      field: 'yAsymmetric',
      value: [['']],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yAsymmetric',
      value: [[], [], []],
      fallback,
    },
    {
      field: 'yAsymmetric',
      value: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yAsymmetric',
      value: [['', '=8 bytes', 'Hello, World!']],
      fallback,
    },
    {
      field: 'yAsymmetric',
      value: [['', '=8 bytes', 'Hello, World!']],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yAsymmetric',
      value: [[], [''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, World!']],
      fallback,
    },
    {
      field: 'yAsymmetric',
      value: [[], [''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, World!']],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'zAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zAsymmetric',
      value: [[]],
      fallback,
    },
    {
      field: 'zAsymmetric',
      value: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zAsymmetric',
      value: [[{}]],
      fallback,
    },
    {
      field: 'zAsymmetric',
      value: [[{}]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zAsymmetric',
      value: [[], [], []],
      fallback,
    },
    {
      field: 'zAsymmetric',
      value: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zAsymmetric',
      value: [[{}, {}, {}]],
      fallback,
    },
    {
      field: 'zAsymmetric',
      value: [[{}, {}, {}]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zAsymmetric',
      value: [[], [{}], [{}, {}], [{}, {}, {}]],
      fallback,
    },
    {
      field: 'zAsymmetric',
      value: [[], [{}], [{}, {}], [{}, {}, {}]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaAsymmetric',
      value: [],
      fallback,
    },
    {
      field: 'aaAsymmetric',
      value: [],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaAsymmetric',
      value: [[]],
      fallback,
    },
    {
      field: 'aaAsymmetric',
      value: [[]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaAsymmetric',
      value: [[{}]],
      fallback,
    },
    {
      field: 'aaAsymmetric',
      value: [[{}]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaAsymmetric',
      value: [[], [], []],
      fallback,
    },
    {
      field: 'aaAsymmetric',
      value: [[], [], []],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaAsymmetric',
      value: [[{}, {}, {}]],
      fallback,
    },
    {
      field: 'aaAsymmetric',
      value: [[{}, {}, {}]],
    },
  );

  assertMatch(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaAsymmetric',
      value: [[], [{}], [{}, {}], [{}, {}, {}]],
      fallback,
    },
    {
      field: 'aaAsymmetric',
      value: [[], [{}], [{}, {}], [{}, {}, {}]],
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aOptional',
      fallback,
    },
  );

  f64TestValues.forEach((value) => {
    assertRoundTrip(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        field: 'bOptional',
        value,
        fallback,
      },
    );
  });

  u64TestValues.forEach((value) => {
    assertRoundTrip(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        field: 'cOptional',
        value,
        fallback,
      },
    );
  });

  s64TestValues.forEach((value) => {
    assertRoundTrip(
      Comprehensive.Types.Bar.size,
      Comprehensive.Types.Bar.serialize,
      Comprehensive.Types.Bar.deserialize,
      {
        field: 'dOptional',
        value,
        fallback,
      },
    );
  });

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'eOptional',
      value: false,
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'eOptional',
      value: true,
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'fOptional',
      value: new Uint8Array([]).buffer,
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'fOptional',
      value: new Uint8Array([0]).buffer,
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'fOptional',
      value: new Uint8Array([0, 42]).buffer,
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'fOptional',
      value: new Uint8Array([0, 42, 255]).buffer,
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'gOptional',
      value: '',
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'gOptional',
      value: '=8 bytes',
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'gOptional',
      value: 'Hello, World!',
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'hOptional',
      value: {},
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'iOptional',
      value: {},
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'jOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'jOptional',
      value: [null],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'jOptional',
      value: [null, null],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'jOptional',
      value: [null, null, null],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kOptional',
      value: [0.0],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kOptional',
      value: [0.0, Math.PI],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kOptional',
      value: [0.0, Math.PI, Number.EPSILON],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'kOptional',
      value: f64TestValues,
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lOptional',
      value: [u64Min],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lOptional',
      value: [u64Min, 256n],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lOptional',
      value: [u64Min, 256n, u64Max],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'lOptional',
      value: u64TestValues,
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mOptional',
      value: [s64Min],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mOptional',
      value: [s64Min, 0n],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mOptional',
      value: [s64Min, 0n, s64Max],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'mOptional',
      value: s64TestValues,
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'nOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'nOptional',
      value: [false],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'nOptional',
      value: [false, true],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'nOptional',
      value: [false, true, false],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'oOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'oOptional',
      value: [new Uint8Array([0, 42, 255]).buffer],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'oOptional',
      value: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
      ],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'oOptional',
      value: [
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([1, 43, 254]).buffer,
        new Uint8Array([2, 44, 253]).buffer,
      ],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'pOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'pOptional',
      value: [''],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'pOptional',
      value: ['', '=8 bytes'],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'pOptional',
      value: ['', '=8 bytes', 'Hello, World!'],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'qOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'qOptional',
      value: [{}],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'qOptional',
      value: [{}, {}],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'qOptional',
      value: [{}, {}, {}],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'rOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'rOptional',
      value: [{}],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'rOptional',
      value: [{}, {}],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'rOptional',
      value: [{}, {}, {}],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sOptional',
      value: [[]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sOptional',
      value: [[null]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sOptional',
      value: [[], [], []],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sOptional',
      value: [[null, null, null]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'sOptional',
      value: [[], [null], [null, null], [null, null, null]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tOptional',
      value: [[]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tOptional',
      value: [[0.0]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tOptional',
      value: [[], [], []],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tOptional',
      value: [f64TestValues],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'tOptional',
      value: [[], [0.0], [0.0, Math.PI], [0.0, Math.PI, Number.EPSILON]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uOptional',
      value: [[]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uOptional',
      value: [[u64Min]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uOptional',
      value: [[], [], []],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uOptional',
      value: [u64TestValues],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'uOptional',
      value: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vOptional',
      value: [[]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vOptional',
      value: [[s64Min]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vOptional',
      value: [[], [], []],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vOptional',
      value: [s64TestValues],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'vOptional',
      value: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wOptional',
      value: [[]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wOptional',
      value: [[false]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wOptional',
      value: [[], [], []],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wOptional',
      value: [[false, true, false]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'wOptional',
      value: [[], [false], [false, true], [false, true, false]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xOptional',
      value: [[]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xOptional',
      value: [[new Uint8Array([0, 42, 255]).buffer]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xOptional',
      value: [[], [], []],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xOptional',
      value: [
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'xOptional',
      value: [
        [new Uint8Array([0, 42, 255]).buffer],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
        ],
        [
          new Uint8Array([0, 42, 255]).buffer,
          new Uint8Array([1, 43, 254]).buffer,
          new Uint8Array([2, 44, 253]).buffer,
        ],
      ],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yOptional',
      value: [[]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yOptional',
      value: [['']],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yOptional',
      value: [[], [], []],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yOptional',
      value: [['', '=8 bytes', 'Hello, World!']],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'yOptional',
      value: [[], [''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, World!']],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zOptional',
      value: [[]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zOptional',
      value: [[{}]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zOptional',
      value: [[], [], []],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zOptional',
      value: [[{}, {}, {}]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'zOptional',
      value: [[], [{}], [{}, {}], [{}, {}, {}]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaOptional',
      value: [],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaOptional',
      value: [[]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaOptional',
      value: [[{}]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaOptional',
      value: [[], [], []],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaOptional',
      value: [[{}, {}, {}]],
      fallback,
    },
  );

  assertRoundTrip(
    Comprehensive.Types.Bar.size,
    Comprehensive.Types.Bar.serialize,
    Comprehensive.Types.Bar.deserialize,
    {
      field: 'aaOptional',
      value: [[], [{}], [{}, {}], [{}, {}, {}]],
      fallback,
    },
  );
}
