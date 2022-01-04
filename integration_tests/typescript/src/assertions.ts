import { deepStrictEqual } from 'assert';
import { unlinkSync, writeFileSync } from 'fs';

// The "omnifile" records the bytes of every value serialized by the exported functions below. It's
// used to validate that the TypeScript code generator encodes data identically to other code
// generators.
const omnifilePath = '/tmp/omnifile-typescript';

try {
  unlinkSync(omnifilePath);
} catch (_) {
  // Attempting to delete the file will fail if the file doesn't exist. This is harmless.
}

export function assertMatch<T, U>(
  size: (value: T) => number,
  serialize: (dataView: DataView, offset: number, value: T) => number,
  deserialize: (dataView: DataView, offset: number) => [number, U],
  actual: T,
  expected: U,
): void {
  // eslint-disable-next-line no-console
  console.log('Value to be serialized:', actual);

  const actualSize = size(actual);
  // eslint-disable-next-line no-console
  console.log('Expected size of the serialized value:', actualSize);

  const arrayBuffer = new ArrayBuffer(actualSize);
  const dataView = new DataView(arrayBuffer);

  const numBytesWritten = serialize(dataView, 0, actual);
  deepStrictEqual(numBytesWritten, actualSize);
  deepStrictEqual(arrayBuffer.byteLength, numBytesWritten);

  // eslint-disable-next-line no-console
  console.log('Bytes from serialization:', arrayBuffer);

  writeFileSync(omnifilePath, new Uint8Array(arrayBuffer), { flag: 'as' });

  // eslint-disable-next-line no-console
  console.log('Size of the serialized value:', arrayBuffer.byteLength);

  const [numBytesRead, replica] = deserialize(dataView, 0);

  // eslint-disable-next-line no-console
  console.log('Value deserialized from those bytes:', replica);

  deepStrictEqual(numBytesRead, numBytesWritten);
  deepStrictEqual(replica, expected);
}

export function assertRoundTrip<U, T extends U>(
  size: (value: T) => number,
  serialize: (dataView: DataView, offset: number, value: T) => number,
  deserialize: (dataView: DataView, offset: number) => [number, U],
  value: T,
): void {
  assertMatch(size, serialize, deserialize, value, value);
}
