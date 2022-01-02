import checkOk from './round-trip';
import { Comprehensive } from '../generated/types';

checkOk(
  Comprehensive.Bar.Bar.size,
  Comprehensive.Bar.Bar.serialize,
  Comprehensive.Bar.Bar.deserialize,
  { field: 'aRequired' },
);

checkOk(
  Comprehensive.Main.EmptyStruct.size,
  Comprehensive.Main.EmptyStruct.serialize,
  Comprehensive.Main.EmptyStruct.deserialize,
  {},
);
