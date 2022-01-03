import { Comprehensive, Degenerate } from '../generated/types';
import { checkOk, checkMatch } from './round-trip';

checkOk(
  Comprehensive.Bar.Bar.size,
  Comprehensive.Bar.Bar.serialize,
  Comprehensive.Bar.Bar.deserialize,
  { field: 'aRequired' },
);

// eslint-disable-next-line no-console
console.log();

checkMatch(
  Degenerate.Types.EmptyStruct.size,
  Degenerate.Types.EmptyStruct.serialize,
  Degenerate.Types.EmptyStruct.deserialize,
  {},
);
