import { Comprehensive } from '../generated/types';

const bar: Comprehensive.Bar.BarOut = { field: 'aRequired' };
const barSize1 = Comprehensive.Bar.Bar.size(bar);
const arrayBuffer = new ArrayBuffer(barSize1);
const dataView = new DataView(arrayBuffer);
const barSize2 = Comprehensive.Bar.Bar.serialize(dataView, 0, bar);

// eslint-disable-next-line no-console
console.log(bar);

// eslint-disable-next-line no-console
console.log(barSize1);

// eslint-disable-next-line no-console
console.log(barSize2);

// eslint-disable-next-line no-console
console.log(arrayBuffer);
