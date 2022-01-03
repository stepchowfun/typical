import { CircularDependency } from '../generated/types';
import { assertRoundTrip } from './assertions';

export default function run(): void {
  assertRoundTrip(
    CircularDependency.Types.StructFromAbove.size,
    CircularDependency.Types.StructFromAbove.serialize,
    CircularDependency.Types.StructFromAbove.deserialize,
    {},
  );

  // eslint-disable-next-line no-console
  console.log();

  assertRoundTrip(
    CircularDependency.Dependency.Types.StructFromBelow.size,
    CircularDependency.Dependency.Types.StructFromBelow.serialize,
    CircularDependency.Dependency.Types.StructFromBelow.deserialize,
    { x: {} },
  );
}
