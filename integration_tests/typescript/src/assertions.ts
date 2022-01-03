import { deepStrictEqual } from 'assert';

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

  // eslint-disable-next-line no-console
  console.log('Bytes from serialization:', arrayBuffer);

  // eslint-disable-next-line no-console
  console.log('Size of the serialized value:', arrayBuffer.byteLength);

  const [numBytesRead, replica] = deserialize(dataView, 0);

  // eslint-disable-next-line no-console
  console.log('Value deserialized from those bytes:', replica);

  deepStrictEqual(numBytesRead, numBytesWritten);
  deepStrictEqual(replica, expected);
}

export function assertRoundTrip<T, U extends T>(
  size: (value: T) => number,
  serialize: (dataView: DataView, offset: number, value: T) => number,
  deserialize: (dataView: DataView, offset: number) => [number, U],
  value: T,
): void {
  assertMatch(size, serialize, deserialize, value, value);
}
