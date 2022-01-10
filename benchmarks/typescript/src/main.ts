import { hrtime } from 'process';
import { Types } from '../generated/types';

const pathologicalIterations = 5_000;
const massiveStringSize = 500_000_000;

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

function benchmark<T, U, V extends { $size: number }>(
  atlas: (message: T) => V,
  serializeIntoDataView: (
    dataView: DataView,
    offset: number,
    message: T,
    atlas: V,
  ) => number,
  deserialize: (dataView: DataView) => U,
  message: T,
  iterations: number,
): void {
  const messageAtlas = atlas(message);
  const messageSize = messageAtlas.$size;
  const arrayBufferSize = messageSize * iterations;
  const arrayBuffer = new ArrayBuffer(arrayBufferSize);
  const serializationDataView = new DataView(arrayBuffer);

  console.log('Message size:', messageSize);

  const serializationInstant = hrtime();

  for (
    let offset = 0;
    offset < arrayBufferSize;
    offset = serializeIntoDataView(
      serializationDataView,
      offset,
      message,
      messageAtlas,
    )
  ) {
    // All the work happens in the "increment" step of the `for` loop.
  }

  const serializationDurationParts = hrtime(serializationInstant);
  const serializationDuration =
    serializationDurationParts[0] +
    serializationDurationParts[1] / 1_000_000_000;

  console.log(`Wrote ${arrayBufferSize} bytes.`);
  console.log(`Serialization duration: ${serializationDuration}s`);
  console.log(
    `Serialization rate: ${
      arrayBufferSize / serializationDuration
    } bytes/second`,
  );

  const deserializationInstant = hrtime();

  for (let offset = 0; offset < arrayBufferSize; offset += messageSize) {
    const dataView = new DataView(arrayBuffer, offset, messageSize);
    deserialize(dataView);
  }

  const deserializationDurationParts = hrtime(deserializationInstant);
  const deserializationDuration =
    deserializationDurationParts[0] +
    deserializationDurationParts[1] / 1_000_000_000;
  console.log(`Deserialization duration: ${deserializationDuration}s`);
  console.log(
    `Deserialization rate: ${
      arrayBufferSize / deserializationDuration
    } bytes/second`,
  );
}

console.log('Massive message test.');

benchmark(
  Types.Struct.atlas,
  Types.Struct.serializeIntoDataView,
  Types.Struct.deserialize,
  { x: 'a'.repeat(massiveStringSize) },
  1,
);

console.log();
console.log('Pathological message test.');

benchmark(
  Types.Message.atlas,
  Types.Message.serializeIntoDataView,
  Types.Message.deserialize,
  {
    a: null,
    b: Math.PI,
    c: u64Max,
    d: s64Max,
    e: true,
    f: new Uint8Array([0, 42, 255]).buffer,
    g: 'Hello, World!',
    h: { x: 'Hello, World!' },
    i: { $field: 'x' as const, x: 'Hello, World!' },
    j: [null, null, null],
    k: f64TestValues,
    l: u64TestValues,
    m: s64TestValues,
    n: [false, true, false],
    o: [
      new Uint8Array([]).buffer,
      new Uint8Array([0, 42, 255]).buffer,
      new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
    ],
    p: ['', '=8 bytes', 'Hello, World!'],
    q: [{ x: 'Hello, World!' }, { x: 'Hello, World!' }, { x: 'Hello, World!' }],
    r: [
      { $field: 'x' as const, x: 'Hello, World!' },
      { $field: 'x' as const, x: 'Hello, World!' },
      { $field: 'x' as const, x: 'Hello, World!' },
    ],
    s: [[], [null], [null, null], [null, null, null]],
    t: [
      [],
      [0.0],
      [0.0, Math.PI],
      [0.0, Math.PI, Number.EPSILON],
      f64TestValues,
    ],
    u: [[], [u64Min], [u64Min, 256n], [u64Min, 256n, u64Max], u64TestValues],
    v: [[], [s64Min], [s64Min, 0n], [s64Min, 0n, s64Max], s64TestValues],
    w: [[], [false], [false, true], [false, true, false]],
    x: [
      [],
      [new Uint8Array([]).buffer],
      [new Uint8Array([]).buffer, new Uint8Array([0, 42, 255]).buffer],
      [
        new Uint8Array([]).buffer,
        new Uint8Array([0, 42, 255]).buffer,
        new Uint8Array([7, 6, 5, 4, 3, 2, 1, 0]).buffer,
      ],
    ],
    y: [[''], ['', '=8 bytes'], ['', '=8 bytes', 'Hello, World!']],
    z: [
      [],
      [{ x: 'Hello, World!' }],
      [{ x: 'Hello, World!' }, { x: 'Hello, World!' }],
      [{ x: 'Hello, World!' }, { x: 'Hello, World!' }, { x: 'Hello, World!' }],
    ],
    aa: [
      [],
      [{ $field: 'x' as const, x: 'Hello, World!' }],
      [
        { $field: 'x' as const, x: 'Hello, World!' },
        { $field: 'x' as const, x: 'Hello, World!' },
      ],
      [
        { $field: 'x' as const, x: 'Hello, World!' },
        { $field: 'x' as const, x: 'Hello, World!' },
        { $field: 'x' as const, x: 'Hello, World!' },
      ],
    ],
  },
  pathologicalIterations,
);
