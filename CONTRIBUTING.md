# Contributing

Thank you for your interest in contributing! You can contribute by filing [issues](https://github.com/stepchowfun/typical/issues) and submitting [pull requests](https://github.com/stepchowfun/typical/pulls). Please observe our [code of conduct](https://github.com/stepchowfun/typical/blob/main/CODE_OF_CONDUCT.md).

If you submit a pull request, please ensure your change passes the [GitHub Actions](https://github.com/stepchowfun/typical/actions) CI checks. This will be apparent from the required status check(s) in the pull request.

## Rust style guide

We're fortunate to have good tooling around enforcing a consistent style throughout the codebase. If you have [Toast](https://github.com/stepchowfun/toast), you can run the various lint checks by running `toast lint`. Otherwise, you can rely on our CI to do it for you. Here, we make note of a few conventions which are not yet enforced automatically. Please adhere to these conventions when possible, and provide appropriate justification for deviations from this guide. If you notice any style violations which appear unintentional, we invite you to bring them to our attention.

### Comments

**Rule:** Comments should be written in American English.

**Rule:** Comments should always be capitalized unless they start with a code-like expression (see below).

**Rule:** Comments which are sentences should be punctuated appropriately. For example:

```rust
// The following logic implements beta reduction.
```

**Rule:** Comments which are not sentences should not have a trailing period. For example:

```rust
// Already normalized
```

**Rule:** Code-like expressions, such as variable names, should be surrounded by backticks. For example:

```rust
// `source_range` is a half-open interval, closed on the left and open on the right.
```

### Trailing commas

The linter enforces that items in multi-line sequences (e.g., function arguments and macro arguments) have trailing commas.

**Rule:** Macros should be written to accept trailing commas as follows:

```rust
macro_rules! my_macro {
    ($foo:expr, $bar:expr, $baz:expr $(,)?) => {{
        ...
    }};
}
```

## The code generation unit tests

Each of the code generators has a unit test which contains a large string of generated code produced from the schemas in `integration_tests/types`. This generated code generally doesn't adhere to the 100-column line length limit enforced by the lint CI check. The following Ruby script can be used to wrap the lines for inclusion in the unit test such that they conform to the line length limit:

```ruby
#!/usr/bin/env ruby

MAX_COLUMNS = 100
GENERATED_CODE_PATH = 'types.rs'

File.read(GENERATED_CODE_PATH).each_line do |line|
  line.gsub!('\\', '\\\\\\\\')
  line.gsub!('"', '\\"')

  indentation = line[/\A */] + '    '

  while line
    if line.size <= MAX_COLUMNS
      puts(line)
      break
    end

    split_index = MAX_COLUMNS - 1

    while line[split_index] == ' '
      split_index -= 1

      if split_index == 0
        STDERR.puts('Unable to format the file.')
        exit(1)
      end
    end

    trailing_word_prefix = line[0...split_index][/\b(\w+|\W+)\Z/]
    if trailing_word_prefix && trailing_word_prefix.size != split_index
      split_index -= trailing_word_prefix.size
    end

    if split_index > 1 && line[split_index - 1] != ' '
      space_rindex = line.rindex(' ', split_index - 2)
      if space_rindex && space_rindex + 1 > indentation.size + 4
          split_index = space_rindex + 1
      end
    end

    puts(line[0...split_index] + '\\')

    line = indentation + line[split_index..]
  end
end
```

## Guidelines for generated code

Generally speaking, we aim to have generated code follow the same standards as handwritten code, except when doing so would add significant complexity to the code generator. Below are some additional language-specific considerations.

### Rust

Typical is designed to be invoked by a [Cargo build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html). See the [example project](https://github.com/stepchowfun/typical/tree/main/examples/rust) for how to set that up. The user is expected to create a dedicated source file which locally disables lint checks for that file and then includes the generated code as follows:

```rust
#![allow(clippy::all, clippy::pedantic, clippy::nursery, warnings)]

include!(concat!(env!("OUT_DIR"), "/types.rs"));
```

Note that the Rust integration test [disables specific checks](https://github.com/stepchowfun/typical/blob/main/integration_tests/rust/src/types.rs) rather than all of them to help us keep track of which checks we are violating.

### TypeScript

To ensure it will pass formatting checks now and in the future, the generated code should disable [Prettier](https://prettier.io/) for all top-level constructs as follows:

```typescript
// prettier-ignore
```

To ensure it will pass lint checks now and in the future, the generated code should disable [ESLint](https://eslint.org/) at the file level as follows:

```typescript
// eslint-disable
```
