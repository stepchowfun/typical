import { CircularDependency } from '../generated/types';
import { assertRoundTrip } from './assertions';

export default function run(): void {
  assertRoundTrip(
    CircularDependency.Types.StructFromAbove.atlas,
    CircularDependency.Types.StructFromAbove.serializeIntoDataView,
    CircularDependency.Types.StructFromAbove.serialize,
    CircularDependency.Types.StructFromAbove.deserialize,
    {},
  );

  console.log();

  assertRoundTrip(
    CircularDependency.Dependency.Types.StructFromBelow.atlas,
    CircularDependency.Dependency.Types.StructFromBelow.serializeIntoDataView,
    CircularDependency.Dependency.Types.StructFromBelow.serialize,
    CircularDependency.Dependency.Types.StructFromBelow.deserialize,
    { x: {} },
  );
}
