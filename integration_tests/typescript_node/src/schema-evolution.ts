import {
  ExampleChoice as AfterExampleChoice,
  type ExampleChoiceIn as AfterExampleChoiceIn,
  ExampleStruct as AfterExampleStruct,
} from '../generated/schema_evolution/after';
import {
  ExampleChoice as BeforeExampleChoice,
  type ExampleChoiceOut as BeforeExampleChoiceOut,
  ExampleStruct as BeforeExampleStruct,
} from '../generated/schema_evolution/before';
import {
  SingletonChoice,
  SingletonStruct,
} from '../generated/schema_evolution/types';
import { assertMatch } from './assertions';

function choiceTestCases(
  fallbackBefore: BeforeExampleChoiceOut,
  fallbackAfter: AfterExampleChoiceIn,
): [BeforeExampleChoiceOut, AfterExampleChoiceIn][] {
  return [
    [
      {
        requiredToRequired: 'required_to_required',
      },
      {
        $field: 'requiredToRequired',
        requiredToRequired: 'required_to_required',
      },
    ],
    [
      {
        requiredToAsymmetric: 'required_to_asymmetric',
      },
      {
        $field: 'requiredToAsymmetric',
        requiredToAsymmetric: 'required_to_asymmetric',
      },
    ],
    [
      {
        asymmetricToRequired: 'asymmetric_to_required',
        $fallback: fallbackBefore,
      },
      {
        $field: 'asymmetricToRequired',
        asymmetricToRequired: 'asymmetric_to_required',
      },
    ],
    [
      {
        asymmetricToAsymmetric: 'asymmetric_to_asymmetric',
        $fallback: fallbackBefore,
      },
      {
        $field: 'asymmetricToAsymmetric',
        asymmetricToAsymmetric: 'asymmetric_to_asymmetric',
      },
    ],
    [
      {
        asymmetricToOptional: 'asymmetric_to_optional',
        $fallback: fallbackBefore,
      },
      {
        $field: 'asymmetricToOptional',
        asymmetricToOptional: 'asymmetric_to_optional',
        $fallback: fallbackAfter,
      },
    ],
    [
      {
        asymmetricToNonexistent: 'asymmetric_to_nonexistent',
        $fallback: fallbackBefore,
      },
      fallbackAfter,
    ],
    [
      {
        optionalToRequired: 'optional_to_required',
        $fallback: fallbackBefore,
      },
      {
        $field: 'optionalToRequired',
        optionalToRequired: 'optional_to_required',
      },
    ],
    [
      {
        optionalToAsymmetric: 'optional_to_asymmetric',
        $fallback: fallbackBefore,
      },
      {
        $field: 'optionalToAsymmetric',
        optionalToAsymmetric: 'optional_to_asymmetric',
      },
    ],
    [
      {
        optionalToOptional: 'optional_to_optional',
        $fallback: fallbackBefore,
      },
      {
        $field: 'optionalToOptional',
        optionalToOptional: 'optional_to_optional',
        $fallback: fallbackAfter,
      },
    ],
    [
      {
        optionalToNonexistent: 'optional_to_nonexistent',
        $fallback: fallbackBefore,
      },
      fallbackAfter,
    ],
  ];
}

export default function run(): void {
  assertMatch(
    BeforeExampleStruct.size,
    BeforeExampleStruct.serialize,
    AfterExampleStruct.deserialize,
    {
      requiredToRequired: 'required_to_required',
      requiredToAsymmetric: 'required_to_asymmetric',
      requiredToOptional: 'required_to_optional',
      requiredToNonexistent: 'required_to_nonexistent',
      asymmetricToRequired: 'asymmetric_to_required',
      asymmetricToAsymmetric: 'asymmetric_to_asymmetric',
      asymmetricToOptional: 'asymmetric_to_optional',
      asymmetricToNonexistent: 'asymmetric_to_nonexistent',
      optionalToRequired: 'optional_to_required',
      optionalToAsymmetric: undefined,
      optionalToOptional: undefined,
      optionalToNonexistent: undefined,
    },
    {
      requiredToRequired: 'required_to_required',
      requiredToAsymmetric: 'required_to_asymmetric',
      requiredToOptional: 'required_to_optional',
      asymmetricToRequired: 'asymmetric_to_required',
      asymmetricToAsymmetric: 'asymmetric_to_asymmetric',
      asymmetricToOptional: 'asymmetric_to_optional',
      optionalToRequired: 'optional_to_required',
      optionalToAsymmetric: undefined,
      optionalToOptional: undefined,
      nonexistentToAsymmetric: undefined,
      nonexistentToOptional: undefined,
    },
  );

  assertMatch(
    BeforeExampleStruct.size,
    BeforeExampleStruct.serialize,
    AfterExampleStruct.deserialize,
    {
      requiredToRequired: 'required_to_required',
      requiredToAsymmetric: 'required_to_asymmetric',
      requiredToOptional: 'required_to_optional',
      requiredToNonexistent: 'required_to_nonexistent',
      asymmetricToRequired: 'asymmetric_to_required',
      asymmetricToAsymmetric: 'asymmetric_to_asymmetric',
      asymmetricToOptional: 'asymmetric_to_optional',
      asymmetricToNonexistent: 'asymmetric_to_nonexistent',
      optionalToRequired: 'optional_to_required',
      optionalToAsymmetric: 'optional_to_asymmetric',
      optionalToOptional: 'optional_to_optional',
      optionalToNonexistent: 'optional_to_nonexistent',
    },
    {
      requiredToRequired: 'required_to_required',
      requiredToAsymmetric: 'required_to_asymmetric',
      requiredToOptional: 'required_to_optional',
      asymmetricToRequired: 'asymmetric_to_required',
      asymmetricToAsymmetric: 'asymmetric_to_asymmetric',
      asymmetricToOptional: 'asymmetric_to_optional',
      optionalToRequired: 'optional_to_required',
      optionalToAsymmetric: 'optional_to_asymmetric',
      optionalToOptional: 'optional_to_optional',
      nonexistentToAsymmetric: undefined,
      nonexistentToOptional: undefined,
    },
  );

  console.log();

  const secondFallbacks = choiceTestCases(
    {
      requiredToRequired: 'required_to_required',
    },
    {
      $field: 'requiredToRequired',
      requiredToRequired: 'required_to_required',
    },
  );

  for (let i = 0; i < secondFallbacks.length; i += 1) {
    const firstFallbacks = choiceTestCases(...secondFallbacks[i]);

    for (let j = 0; j < firstFallbacks.length; j += 1) {
      const tests = choiceTestCases(...firstFallbacks[j]);

      for (let k = 0; k < tests.length; k += 1) {
        const [before, after] = tests[k];

        assertMatch(
          BeforeExampleChoice.size,
          BeforeExampleChoice.serialize,
          AfterExampleChoice.deserialize,
          before,
          after,
        );
      }
    }
  }

  console.log();

  assertMatch(
    SingletonStruct.size,
    SingletonStruct.serialize,
    SingletonChoice.deserialize,
    {
      x: 'foo',
    },
    {
      $field: 'x',
      x: 'foo',
    },
  );

  assertMatch(
    SingletonChoice.size,
    SingletonChoice.serialize,
    SingletonStruct.deserialize,
    {
      x: 'foo',
    },
    {
      x: 'foo',
    },
  );
}
