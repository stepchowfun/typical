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

export function assertMatch<T, U>(
  size: (message: T) => number,
  serialize: (message: T) => ArrayBuffer,
  deserialize: (dataView: DataView) => U,
  actual: T,
  expected: U,
): void {
  console.log('Message to be serialized:', actual);

  const actualSize = size(actual);
  console.log('Expected size of the serialized message:', actualSize);

  const arrayBuffer = serialize(actual);
  const dataView = new DataView(arrayBuffer);
  deepStrictEqual(arrayBuffer.byteLength, actualSize);
  console.log('Bytes from serialization:', arrayBuffer);
  console.log('Size of the serialized message:', arrayBuffer.byteLength);

  writeFileSync(omnifilePath, Buffer.from(arrayBuffer), { flag: 'as' });

  const replica = deserialize(dataView);
  deepStrictEqual(replica, expected);
  console.log('Message deserialized from those bytes:', replica);
}

export function assertRoundTrip<U, T extends U>(
  size: (message: T) => number,
  serialize: (message: T) => ArrayBuffer,
  deserialize: (dataView: DataView) => U,
  message: T,
): void {
  assertMatch(size, serialize, deserialize, message, message);
}
