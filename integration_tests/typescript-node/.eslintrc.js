module.exports = {
  root: true,
  env: {
    es2021: true,
  },
  extends: [
    'eslint:all',
    'airbnb-base',

    // We don't need to extend `plugin:@typescript-eslint/recommended-requiring-type-checking` as
    // well, since this configuration (unlike the `recommended` one) already includes it.
    'plugin:@typescript-eslint/all',

    'plugin:eslint-comments/recommended',
    'plugin:import/recommended',
    'plugin:import/typescript',
    'prettier',
  ],
  parserOptions: {
    project: 'tsconfig.json',
  },
  plugins: ['@typescript-eslint'],
  rules: {
    // Modify this rule to accommodate the fact that React component names are PascalCase and to
    // skip the rule for members that require quotes (e.g., '&:focus, &:hover' in JSS).
    '@typescript-eslint/naming-convention': [
      'error',
      {
        selector: 'default',
        format: ['camelCase'],
      },
      {
        selector: 'variable',
        format: ['camelCase', 'UPPER_CASE', 'PascalCase'],
      },
      {
        selector: 'memberLike',
        format: null,
        modifiers: ['requiresQuotes'],
      },
      {
        selector: 'typeLike',
        format: ['PascalCase'],
      },
    ],

    // This rule is annoying and sometimes difficult to satisfy. The `ReadonlyDeep` type from
    // `type-fest` sometimes causes this rule to overflow the call stack during linting. See:
    // https://github.com/typescript-eslint/typescript-eslint/issues/4476
    '@typescript-eslint/prefer-readonly-parameter-types': 'off',

    // This allows us to define algebraic data types.
    '@typescript-eslint/no-type-alias': 'off',

    // These built-in rules need to be disabled since they have TypeScript-aware versions in
    // `plugin:@typescript-eslint/all`. The set of rules was obtained by searching the
    // `https://github.com/typescript-eslint/typescript-eslint` repository for the string
    // "note you must disable the base rule as it can report incorrect errors".
    'brace-style': 'off',
    'comma-dangle': 'off',
    'comma-spacing': 'off',
    'default-param-last': 'off',
    'dot-notation': 'off',
    'func-call-spacing': 'off',
    indent: 'off',
    'init-declarations': 'off',
    'keyword-spacing': 'off',
    'lines-between-class-members': 'off',
    'no-array-constructor': 'off',
    'no-dupe-class-members': 'off',
    'no-duplicate-imports': 'off',
    'no-empty-function': 'off',
    'no-empty-function': 'off',
    'no-extra-parens': 'off',
    'no-extra-semi': 'off',
    'no-implied-eval': 'off',
    'no-invalid-this': 'off',
    'no-loop-func': 'off',
    'no-loss-of-precision': 'off',
    'no-magic-numbers': 'off',
    'no-redeclare': 'off',
    'no-restricted-imports': 'off',
    'no-shadow': 'off',
    'no-throw-literal': 'off',
    'no-unused-expressions': 'off',
    'no-unused-vars': 'off',
    'no-use-before-define': 'off',
    'no-useless-constructor': 'off',
    'object-curly-spacing': 'off',
    'padding-line-between-statements': 'off',
    'padding-line-between-statements': 'off',
    quotes: 'off',
    'require-await': 'off',
    'return-await': 'off',
    semi: 'off',
    'space-before-function-paren': 'off',

    // Report unnecessary `eslint-disable` directives.
    'eslint-comments/no-unused-disable': 'error',

    // Require uses of escape hatches to be justified with an explanation.
    'eslint-comments/require-description': 'error',

    // Forbid file extensions in imports.
    'import/extensions': ['error', 'never'],
  },
};
