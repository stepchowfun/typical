import { Comprehensive } from '../generated/types';

const bar: Comprehensive.Bar.BarOut = { field: 'aRequired' };

// eslint-disable-next-line no-console
console.log(bar);

// eslint-disable-next-line no-console
console.log(Comprehensive.Bar.Bar.size(bar));
