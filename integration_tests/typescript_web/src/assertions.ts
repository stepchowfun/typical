import { isEqual } from 'lodash';
import { sha256 } from 'js-sha256';

// The "omnifile" records the bytes of every message serialized by the exported functions below.
// It's used to validate that the TypeScript code generator encodes data identically to other code
// generators.
const omnifileHash =
  'cfe2ae57b1bb6467c87f11d50829d0b8301566f645dabc19b38c861cd577b0a1';
const omnifileSize = 80_395;
const omnifileBuffer = new ArrayBuffer(omnifileSize);
const omnifileArray = new Uint8Array(omnifileBuffer);
let omnifileOffset = 0;

function deepStrictEqual<T, U>(x: T, y: U): void {
  if (!isEqual(x, y)) {
    throw new Error('Mismatch!');
  }
}

export function assertMatch<T, U>(
  size: (message: T) => number,
  serialize: (message: T) => ArrayBuffer,
  deserialize: (dataView: DataView) => U,
  actual: T,
  expected: U,
): void {
  /* eslint-disable no-console -- Allow logging for this function. */

  console.log('Message to be serialized:', actual);

  const actualSize = size(actual);
  console.log('Expected size of the serialized message:', actualSize);

  const arrayBuffer = serialize(actual);
  const dataView = new DataView(arrayBuffer);
  deepStrictEqual(arrayBuffer.byteLength, actualSize);
  console.log('Bytes from serialization:', arrayBuffer);
  console.log('Size of the serialized message:', arrayBuffer.byteLength);

  const typedArray = new Uint8Array(arrayBuffer);
  omnifileArray.set(typedArray, omnifileOffset);
  omnifileOffset += arrayBuffer.byteLength;

  const replica = deserialize(dataView);
  deepStrictEqual(replica, expected);
  console.log('Message deserialized from those bytes:', replica);

  /* eslint-enable no-console -- Re-enable this rule. */
}

export function assertRoundTrip<U, T extends U>(
  size: (message: T) => number,
  serialize: (message: T) => ArrayBuffer,
  deserialize: (dataView: DataView) => U,
  message: T,
): void {
  assertMatch(size, serialize, deserialize, message, message);
}

export function verifyOmnifile(): void {
  deepStrictEqual(omnifileOffset, omnifileBuffer.byteLength);
  deepStrictEqual(sha256(omnifileBuffer), omnifileHash);

  // eslint-disable-next-line no-console -- Allow us to log this confirmation message.
  console.log('Integration tests passed.');
}
