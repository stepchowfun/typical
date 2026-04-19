import {
  type EmptyChoiceIn,
  type EmptyChoiceOut,
  EmptyStruct,
  type EmptyStructIn,
  type EmptyStructOut,
} from '../generated/degenerate/types';
import { assertRoundTrip } from './assertions';

// eslint-disable-next-line @typescript-eslint/no-unused-vars -- We only care that it type checks.
function initialIn<T>(x: EmptyChoiceIn): T {
  return x;
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars -- We only care that it type checks.
function initialOut<T>(x: EmptyChoiceOut): T {
  return x;
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars -- We only care that it type checks.
function terminalIn<T>(x: T): EmptyStructIn {
  return {};
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars -- We only care that it type checks.
function terminalOut<T>(x: T): EmptyStructOut {
  return {};
}

export default function run(): void {
  assertRoundTrip(
    EmptyStruct.size,
    EmptyStruct.serialize,
    EmptyStruct.deserialize,
    {},
  );
}
