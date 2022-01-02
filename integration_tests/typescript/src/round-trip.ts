import { deepStrictEqual } from 'assert';

export function checkMatch<T, U>(
  size: (value: T) => number,
  serialize: (dataView: DataView, offset: number, value: T) => number,
  deserialize: (dataView: DataView, offset: number) => [number, U],
  value: T,
): void {
  // eslint-disable-next-line no-console
  console.log('Value to be serialized:', value);

  const valueSize = size(value);
  // eslint-disable-next-line no-console
  console.log('Expected size of the serialized value:', valueSize);

  const arrayBuffer = new ArrayBuffer(valueSize);
  const dataView = new DataView(arrayBuffer);

  serialize(dataView, 0, value);

  // eslint-disable-next-line no-console
  console.log('Bytes from serialization:', arrayBuffer);

  // eslint-disable-next-line no-console
  console.log('Size of the serialized value:', arrayBuffer.byteLength);

  const clone = deserialize(dataView, 0)[1];

  // eslint-disable-next-line no-console
  console.log('Value deserialized from those bytes:', clone);

  deepStrictEqual(clone, value);
}

export function checkOk<T, U>(
  size: (value: T) => number,
  serialize: (dataView: DataView, offset: number, value: T) => number,
  deserialize: (dataView: DataView, offset: number) => [number, U],
  value: T,
): void {
  // eslint-disable-next-line no-console
  console.log('Value to be serialized:', value);

  const valueSize = size(value);
  // eslint-disable-next-line no-console
  console.log('Expected size of the serialized value:', valueSize);

  const arrayBuffer = new ArrayBuffer(valueSize);
  const dataView = new DataView(arrayBuffer);

  serialize(dataView, 0, value);

  // eslint-disable-next-line no-console
  console.log('Bytes from serialization:', arrayBuffer);

  // eslint-disable-next-line no-console
  console.log('Size of the serialized value:', arrayBuffer.byteLength);

  const clone = deserialize(dataView, 0)[1];

  // eslint-disable-next-line no-console
  console.log('Value deserialized from those bytes:', clone);
}
