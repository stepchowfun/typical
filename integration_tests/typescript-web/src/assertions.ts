import { isEqual } from 'lodash';
import { sha256 } from 'js-sha256';

// The "omnifile" records the bytes of every message serialized by the exported functions below.
// It's used to validate that the TypeScript code generator encodes data identically to other code
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
  atlas: (message: T) => V,
  serializeUnsafe: (
    dataView: DataView,
    offset: number,
    message: T,
    atlas: V,
  ) => number,
  serialize: (message: T) => ArrayBuffer,
  deserialize: (dataView: DataView) => U,
  actual: T,
  expected: U,
): void {
  console.log('Message to be serialized:', actual);

  const actualAtlas = atlas(actual);
  const actualSize = actualAtlas.$size;
  console.log('Expected size of the serialized message:', actualSize);

  const arrayBuffer = new ArrayBuffer(actualSize);
  const dataView = new DataView(arrayBuffer);
  const numBytesWritten = serializeUnsafe(dataView, 0, actual, actualAtlas);
  deepStrictEqual(numBytesWritten, actualSize);
  deepStrictEqual(arrayBuffer.byteLength, numBytesWritten);
  console.log('Bytes from serialization:', arrayBuffer);
  console.log('Size of the serialized message:', arrayBuffer.byteLength);

  const typedArray = new Uint8Array(arrayBuffer);
  omnifileArray.set(typedArray, omnifileOffset);
  omnifileOffset += arrayBuffer.byteLength;

  const replica = deserialize(dataView);
  deepStrictEqual(replica, expected);
  console.log('Message deserialized from those bytes:', replica);

  const arrayBufferReplica = serialize(actual);
  const dataViewReplica = new DataView(arrayBufferReplica);
  deepStrictEqual(dataViewReplica.byteLength, dataView.byteLength);
  for (let i = 0; i < dataViewReplica.byteLength; i += 1) {
    deepStrictEqual(dataViewReplica.getUint8(i), dataView.getUint8(i));
  }
}

export function assertRoundTrip<U, T extends U, V extends { $size: number }>(
  atlas: (message: T) => V,
  serializeUnsafe: (
    dataView: DataView,
    offset: number,
    message: T,
    atlas: V,
  ) => number,
  serialize: (message: T) => ArrayBuffer,
  deserialize: (dataView: DataView) => U,
  message: T,
): void {
  assertMatch(atlas, serializeUnsafe, serialize, deserialize, message, message);
}

export function verifyOmnifile(): void {
  deepStrictEqual(omnifileOffset, omnifileBuffer.byteLength);
  deepStrictEqual(sha256(omnifileBuffer), omnifileHash);

  console.log('Integration tests passed.');
}
