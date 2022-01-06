import { SchemaEvolution } from '../generated/types';
import { assertMatch } from './assertions';

export default function run(): void {
  assertMatch(
    SchemaEvolution.Before.ExampleStruct.size,
    SchemaEvolution.Before.ExampleStruct.serialize,
    SchemaEvolution.After.ExampleStruct.deserialize,
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
    SchemaEvolution.Before.ExampleStruct.size,
    SchemaEvolution.Before.ExampleStruct.serialize,
    SchemaEvolution.After.ExampleStruct.deserialize,
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

  const $fallback: {
    $field: 'requiredToRequired';
    requiredToRequired: string;
  } = {
    $field: 'requiredToRequired',
    requiredToRequired: 'required_to_required',
  };

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      $field: 'requiredToRequired',
      requiredToRequired: 'required_to_required',
    },
    {
      $field: 'requiredToRequired',
      requiredToRequired: 'required_to_required',
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      $field: 'requiredToAsymmetric',
      requiredToAsymmetric: 'required_to_asymmetric',
    },
    {
      $field: 'requiredToAsymmetric',
      requiredToAsymmetric: 'required_to_asymmetric',
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      $field: 'asymmetricToRequired',
      asymmetricToRequired: 'asymmetric_to_required',
      $fallback,
    },
    {
      $field: 'asymmetricToRequired',
      asymmetricToRequired: 'asymmetric_to_required',
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      $field: 'asymmetricToAsymmetric',
      asymmetricToAsymmetric: 'asymmetric_to_asymmetric',
      $fallback,
    },
    {
      $field: 'asymmetricToAsymmetric',
      asymmetricToAsymmetric: 'asymmetric_to_asymmetric',
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      $field: 'asymmetricToOptional',
      asymmetricToOptional: 'asymmetric_to_optional',
      $fallback,
    },
    {
      $field: 'asymmetricToOptional',
      asymmetricToOptional: 'asymmetric_to_optional',
      $fallback,
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      $field: 'asymmetricToNonexistent',
      asymmetricToNonexistent: 'asymmetric_to_nonexistent',
      $fallback,
    },
    $fallback,
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      $field: 'optionalToRequired',
      optionalToRequired: 'optional_to_required',
      $fallback,
    },
    {
      $field: 'optionalToRequired',
      optionalToRequired: 'optional_to_required',
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      $field: 'optionalToAsymmetric',
      optionalToAsymmetric: 'optional_to_asymmetric',
      $fallback,
    },
    {
      $field: 'optionalToAsymmetric',
      optionalToAsymmetric: 'optional_to_asymmetric',
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      $field: 'optionalToOptional',
      optionalToOptional: 'optional_to_optional',
      $fallback,
    },
    {
      $field: 'optionalToOptional',
      optionalToOptional: 'optional_to_optional',
      $fallback,
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      $field: 'optionalToNonexistent',
      optionalToNonexistent: 'optional_to_nonexistent',
      $fallback,
    },
    $fallback,
  );

  assertMatch(
    SchemaEvolution.Types.SingletonStruct.size,
    SchemaEvolution.Types.SingletonStruct.serialize,
    SchemaEvolution.Types.SingletonChoice.deserialize,
    {
      x: 'foo',
    },
    {
      $field: 'x',
      x: 'foo',
    },
  );

  assertMatch(
    SchemaEvolution.Types.SingletonChoice.size,
    SchemaEvolution.Types.SingletonChoice.serialize,
    SchemaEvolution.Types.SingletonStruct.deserialize,
    {
      $field: 'x',
      x: 'foo',
    },
    {
      x: 'foo',
    },
  );
}
