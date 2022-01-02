import { Comprehensive } from '../generated/types';
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
  Comprehensive.Main.EmptyStruct.size,
  Comprehensive.Main.EmptyStruct.serialize,
  Comprehensive.Main.EmptyStruct.deserialize,
  {},
);
