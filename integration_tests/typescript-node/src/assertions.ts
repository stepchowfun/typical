import { deepStrictEqual } from 'assert';
import { unlinkSync, writeFileSync } from 'fs';

// The "omnifile" records the bytes of every message serialized by the exported functions below.
// It's used to validate that the TypeScript code generator encodes data identically to other code
// generators.
const omnifilePath = '/tmp/omnifile-typescript';

try {
  unlinkSync(omnifilePath);
} catch (_) {
  // Attempting to delete the file will fail if the file doesn't exist. This is harmless.
}

export function assertMatch<T, U, V extends { $size: number }>(
  atlas: (message: T) => V,
  serializeIntoDataView: (
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
  const numBytesWritten = serializeIntoDataView(
    dataView,
    0,
    actual,
    actualAtlas,
  );
  deepStrictEqual(numBytesWritten, actualSize);
  deepStrictEqual(arrayBuffer.byteLength, numBytesWritten);
  console.log('Bytes from serialization:', arrayBuffer);
  console.log('Size of the serialized message:', arrayBuffer.byteLength);

  writeFileSync(omnifilePath, new Uint8Array(arrayBuffer), { flag: 'as' });

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
  serializeIntoDataView: (
    dataView: DataView,
    offset: number,
    message: T,
    atlas: V,
  ) => number,
  serialize: (message: T) => ArrayBuffer,
  deserialize: (dataView: DataView) => U,
  message: T,
): void {
  assertMatch(
    atlas,
    serializeIntoDataView,
    serialize,
    deserialize,
    message,
    message,
  );
}
