import { Comprehensive, Degenerate } from '../generated/types';
import { assertMatch, assertRoundTrip } from './assertions';

assertMatch(
  Comprehensive.Types.Bar.size,
  Comprehensive.Types.Bar.serialize,
  Comprehensive.Types.Bar.deserialize,
  { field: 'aRequired' },
  { field: 'aRequired' },
);

// eslint-disable-next-line no-console
console.log();

assertRoundTrip(
  Degenerate.Types.EmptyStruct.size,
  Degenerate.Types.EmptyStruct.serialize,
  Degenerate.Types.EmptyStruct.deserialize,
  {},
);
