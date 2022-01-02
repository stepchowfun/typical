export default function checkOk<T, U>(
  size: (value: T) => number,
  serialize: (dataView: DataView, offset: number, value: T) => number,
  deserialize: (dataView: DataView, offset: number) => [number, U],
  value: T,
): void {
  // eslint-disable-next-line no-console
  console.log('Value:', value);

  const valueSize = size(value);
  // eslint-disable-next-line no-console
  console.log('Predicted size:', valueSize);

  const arrayBuffer = new ArrayBuffer(valueSize);
  const dataView = new DataView(arrayBuffer);

  // eslint-disable-next-line no-console
  console.log('Actual size:', serialize(dataView, 0, value));

  // eslint-disable-next-line no-console
  console.log('Serialized bytes:', arrayBuffer);

  const [bytesRead, clone] = deserialize(dataView, 0);

  // eslint-disable-next-line no-console
  console.log('Bytes read:', bytesRead);

  // eslint-disable-next-line no-console
  console.log('Deserialized value:', clone);
}
