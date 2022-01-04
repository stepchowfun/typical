import { isEqual } from 'lodash';
import { sha256 } from 'js-sha256';

// The "omnifile" records the bytes of every value serialized by the exported functions below. It's
// used to validate that the TypeScript code generator encodes data identically to other code
// generators.
const omnifileHash =
  '15fbfc488b07fba7e273903c3198a983fe8a131c4d692ce80aa0c149194db83f';
const omnifileBuffer = new ArrayBuffer(80_305);
const omnifileArray = new Uint8Array(omnifileBuffer);
let omnifileOffset = 0;

// This is a version of Node.js's `assert.deepStrictEqual` function that works in browsers.
function deepStrictEqual<T, U>(x: T, y: U): void {
  if (!isEqual(x, y)) {
    throw new Error('Mismatch!');
  }
}

export function assertMatch<T, U, V extends { $size: number }>(
  atlas: (value: T) => V,
  serialize: (dataView: DataView, offset: number, value: T, atlas: V) => number,
  deserialize: (dataView: DataView) => U,
  actual: T,
  expected: U,
): void {
  console.log('Value to be serialized:', actual);

  const actualAtlas = atlas(actual);
  const actualSize = actualAtlas.$size;
  console.log('Expected size of the serialized value:', actualSize);

  const arrayBuffer = new ArrayBuffer(actualSize);
  const dataView = new DataView(arrayBuffer);

  const numBytesWritten = serialize(dataView, 0, actual, actualAtlas);
  deepStrictEqual(numBytesWritten, actualSize);
  deepStrictEqual(arrayBuffer.byteLength, numBytesWritten);

  console.log('Bytes from serialization:', arrayBuffer);
  console.log('Size of the serialized value:', arrayBuffer.byteLength);

  const typedArray = new Uint8Array(arrayBuffer);
  omnifileArray.set(typedArray, omnifileOffset);
  omnifileOffset += arrayBuffer.byteLength;

  const replica = deserialize(dataView);

  console.log('Value deserialized from those bytes:', replica);

  deepStrictEqual(replica, expected);
}

export function assertRoundTrip<U, T extends U, V extends { $size: number }>(
  atlas: (value: T) => V,
  serialize: (dataView: DataView, offset: number, value: T, atlas: V) => number,
  deserialize: (dataView: DataView) => U,
  value: T,
): void {
  assertMatch(atlas, serialize, deserialize, value, value);
}

export function verifyOmnifile(): void {
  deepStrictEqual(omnifileOffset, omnifileBuffer.byteLength);
  deepStrictEqual(sha256(omnifileBuffer), omnifileHash);

  console.log('Integration tests passed.');
}
