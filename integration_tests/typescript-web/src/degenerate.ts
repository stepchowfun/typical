import { Degenerate } from '../generated/types';
import { assertRoundTrip } from './assertions';

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function initialIn<T>(x: Degenerate.Types.EmptyChoiceIn): T {
  return x;
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function initialOut<T>(x: Degenerate.Types.EmptyChoiceOut): T {
  return x;
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function terminalIn<T>(x: T): Degenerate.Types.EmptyStructIn {
  return {};
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function terminalOut<T>(x: T): Degenerate.Types.EmptyStructOut {
  return {};
}

export default function run(): void {
  assertRoundTrip(
    Degenerate.Types.EmptyStruct.atlas,
    Degenerate.Types.EmptyStruct.serialize,
    Degenerate.Types.EmptyStruct.deserialize,
    {},
  );
}
