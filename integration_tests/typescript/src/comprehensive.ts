import { CircularDependency } from '../generated/types';
import { assertRoundTrip } from './assertions';

export default function run(): void {
  assertRoundTrip(
    CircularDependency.Types.StructFromAbove.size,
    CircularDependency.Types.StructFromAbove.serialize,
    CircularDependency.Types.StructFromAbove.deserialize,
    {},
  );
}
