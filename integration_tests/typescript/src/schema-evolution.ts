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
      optionalToAsymmetric: 'optionalToAsymmetric',
      optionalToOptional: 'optionalToOptional',
      optionalToNonexistent: 'optionalToNonexistent',
    },
    {
      requiredToRequired: 'required_to_required',
      requiredToAsymmetric: 'required_to_asymmetric',
      requiredToOptional: 'required_to_optional',
      asymmetricToRequired: 'asymmetric_to_required',
      asymmetricToAsymmetric: 'asymmetric_to_asymmetric',
      asymmetricToOptional: 'asymmetric_to_optional',
      optionalToRequired: 'optional_to_required',
      optionalToAsymmetric: 'optionalToAsymmetric',
      optionalToOptional: 'optionalToOptional',
      nonexistentToAsymmetric: undefined,
      nonexistentToOptional: undefined,
    },
  );

  // eslint-disable-next-line no-console
  console.log();

  const fallback: { field: 'requiredToRequired'; value: string } = {
    field: 'requiredToRequired',
    value: 'requiredToRequired',
  };

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      field: 'requiredToRequired',
      value: 'requiredToRequired',
    },
    {
      field: 'requiredToRequired',
      value: 'requiredToRequired',
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      field: 'requiredToAsymmetric',
      value: 'requiredToAsymmetric',
    },
    {
      field: 'requiredToAsymmetric',
      value: 'requiredToAsymmetric',
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      field: 'asymmetricToRequired',
      value: 'asymmetricToRequired',
      fallback,
    },
    {
      field: 'asymmetricToRequired',
      value: 'asymmetricToRequired',
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      field: 'asymmetricToAsymmetric',
      value: 'asymmetricToAsymmetric',
      fallback,
    },
    {
      field: 'asymmetricToAsymmetric',
      value: 'asymmetricToAsymmetric',
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      field: 'asymmetricToOptional',
      value: 'asymmetricToOptional',
      fallback,
    },
    {
      field: 'asymmetricToOptional',
      value: 'asymmetricToOptional',
      fallback,
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      field: 'asymmetricToNonexistent',
      value: 'asymmetricToNonexistent',
      fallback,
    },
    fallback,
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      field: 'optionalToRequired',
      value: 'optionalToRequired',
      fallback,
    },
    {
      field: 'optionalToRequired',
      value: 'optionalToRequired',
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      field: 'optionalToAsymmetric',
      value: 'optionalToAsymmetric',
      fallback,
    },
    {
      field: 'optionalToAsymmetric',
      value: 'optionalToAsymmetric',
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      field: 'optionalToOptional',
      value: 'optionalToOptional',
      fallback,
    },
    {
      field: 'optionalToOptional',
      value: 'optionalToOptional',
      fallback,
    },
  );

  assertMatch(
    SchemaEvolution.Before.ExampleChoice.size,
    SchemaEvolution.Before.ExampleChoice.serialize,
    SchemaEvolution.After.ExampleChoice.deserialize,
    {
      field: 'optionalToNonexistent',
      value: 'optionalToNonexistent',
      fallback,
    },
    fallback,
  );

  assertMatch(
    SchemaEvolution.Types.SingletonStruct.size,
    SchemaEvolution.Types.SingletonStruct.serialize,
    SchemaEvolution.Types.SingletonChoice.deserialize,
    {
      x: 'foo',
    },
    {
      field: 'x',
      value: 'foo',
    },
  );

  assertMatch(
    SchemaEvolution.Types.SingletonChoice.size,
    SchemaEvolution.Types.SingletonChoice.serialize,
    SchemaEvolution.Types.SingletonStruct.deserialize,
    {
      field: 'x',
      value: 'foo',
    },
    {
      x: 'foo',
    },
  );
}
