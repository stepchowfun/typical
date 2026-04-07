import { StructFromBelow } from '../generated/circular_dependency/dependency/types';
import { StructFromAbove } from '../generated/circular_dependency/types';
import { assertRoundTrip } from './assertions';

export default function run(): void {
  assertRoundTrip(
    StructFromAbove.size,
    StructFromAbove.serialize,
    StructFromAbove.deserialize,
    {
      field: 'field',
      size: 'size',
      elements: 'elements',
      fallback: 'fallback',
    },
  );

  // eslint-disable-next-line no-console -- Allow us to separate the test groups with a line break.
  console.log();

  assertRoundTrip(
    StructFromBelow.size,
    StructFromBelow.serialize,
    StructFromBelow.deserialize,
    {
      x: {
        field: 'field',
        size: 'size',
        elements: 'elements',
        fallback: 'fallback',
      },
    },
  );
}
