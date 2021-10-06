# Typical: language-neutral algebraic data types for data interchange

[![Build status](https://github.com/stepchowfun/typical/workflows/Continuous%20integration/badge.svg?branch=main)](https://github.com/stepchowfun/typical/actions?query=branch%3Amain)

*Typical* is a so-called "[interface definition language](https://en.wikipedia.org/wiki/Interface_description_language)", or IDL. You define types in a language-neutral way, then Typical generates code in various languages for serializing and deserializing data based on those types. This is useful for marshalling messages between services, storing structured data on disk, etc. Typical uses an efficient binary encoding that still allows you to change the types over time with forward and backward compatibility as your requirements evolve.

The main difference between Typical and related toolchains like Protocol Buffers and Apache Thrift is that Typical has a more modern type system based on [algebraic data types](https://en.wikipedia.org/wiki/Algebraic_data_type), enabling a safer programming style with non-nullable types. Typical has a [novel solution](#changing-types-safely) to the classic problem of how to safely add and remove required fields in structs and the lesser-known dual problem of how to safely perform exhaustive pattern matching on sum types as cases are added and removed over time.

Typical's design was inspired by insights from a branch of mathematics called [category theory](https://en.wikipedia.org/wiki/Category_theory), especially the duality of limits and colimits and the notions of covariance and contravariance. Happily, you don't need to know about any of that to use Typical.

**Currently supported languages:**

- Rust

## Introduction

Suppose you want to build an API for sending emails. You need to decide how requests and responses will be [serialized](https://en.wikipedia.org/wiki/Serialization) for transport. You could use a self-describing format like JSON or XML, but there are some downsides worth considering:

1. It can be difficult to ensure the client and server agree on the shape of the data, especially if they are written in different programming languages and cannot share code.
2. Text-based formats like JSON and XML are generally less efficient to serialize and deserialize than binary formats, both in time and space.

Instead, you can describe the shape of your data with Typical. Create a *schema file* called `email_api.t` with the request and response types for a simple email sending endpoint:

```sh
# This is the request type for the email sending endpoint.
struct send_email_request {
  to:   string = 0
  from: string = 1
  body: string = 2
}

# This is the response type for the email sending endpoint.
choice send_email_response {
  success       = 0
  error: string = 1
}
```

A `struct`, such as our `send_email_request` type, describes messages containing a fixed set of fields. A `choice`, such as our `send_email_response` type, describes messages containing exactly one field from a set of possibilities. Types built from `struct`s and `choice`s are called *algebraic data types*, due to their correspondence to an idea from category theory called *initial algebras*. You don't need to know about initial algebras to use Typical.

Each field in a `struct` or a `choice` has both a name (e.g., `body`) and an integer index (e.g., `2`). The name is only for humans, and only the index is used to identify fields in the binary encoding. You can freely rename fields without worrying about binary incompatibility.

Fields also have a type. Note that the `success` field in `send_email_response` doesn't have an explicit type; that means its type implicitly defaults to `unit`, a built-in type that carries no information.

Now that we've defined some types, we can use Typical to generate the code for serialization and deserialization. For example, you can generate Rust code with the following:

```sh
$ typical generate email_api.t --rust-out-file email_api.rs
```

The client and server can use the generated code to serialize and deserialize messages, which ensures they will understand each other.

Note that Typical only does serialization and deserialization. It has nothing to do with service meshes, encryption, authentication, or authorization, but it can be used together with those technologies.

## Changing types safely

TODO

All told, Typical's solution to this classic problem can be seen as an application of the [robustness principle](https://en.wikipedia.org/wiki/Robustness_principle) to algebraic data types.

## Importing schemas from other schemas

You don't need to fit all your type definitions in one schema file. You can organize your types into separate schema files at your leisure, and then import schemas from other schemas. For example, suppose you want to define a structured `email_address` type your email API, rather than representing email addresses as strings. You could create a schema called `email_util.t` next to your `email_api.t` schema with the following contents:

```sh
struct address {
  local_part: string = 0
  domain: string = 0
}
```

Then you can import it in `email_api.t`:

```sh
import 'email_util.t'

struct send_email_request {
  to:   email_util.address = 0
  from: email_util.address = 1
  body: string = 2
}

choice send_email_response {
  success = 0
  error: string = 1
}
```

If you generate the code for `email_api.t` with the same command as above, the generated code will now include the types from both `email_api.t` and `email_util.t`, as the latter is imported by the former.

Import paths are considered relative to the directory containing the schema doing the importing. Typical has no notion of a "top-level" directory on which all paths are based.

A useful convention is to create a `main.t` schema that simply imports all the other schemas, directly or indirectly. Then it's clear which schema to use for code generation. Alternatively, in a large organization, you might have a separate top-level schema per project that imports only the types needed by that project. However, these are merely conventions, and Typical has no intrinsic notion of "project".

If you import two schemas with the same name from different directories, you will need to disambiguate usages of those schemas. Suppose, for example, you attempted the following:

```sh
import 'apis/email.t'
import 'util/email.t'

struct employee {
  name:  string = 0
  email: email.address = 1 # Uh oh! Which schema is this type from?
}
```

Fortunately, Typical will tell you about this problem and ask you to clarify what you mean. You can do so as follows:

```sh
import 'apis/email.t' as email_api
import 'util/email.t' as email_util

struct employee {
  name:  string = 0
  email: email_util.address = 1
}
```

## Type system reference

### User-defined types

Every user-defined type is either a `struct` or a `choice`, and they have the same abstract syntax: a name and a list of fields. A field consists of an optional cardinality, a human-readable name, an optional type, and an index. Here's are some examples of user-defined types with various fields:

```sh
struct server {
  hostname:         string        = 0
  unstable address: ip_address    = 1
  optional owner:   email.address = 2
}

choice ip_address {
  unknown   = 0
  v4: ip.v4 = 1
  v6: ip.v6 = 2
}
```

The cardinality, if present, is either `optional` or `unstable`. The absence of a cardinality indicates that the field is required.

The name is a human-readable identifier for the field. It's used to refer to the field in code, but it's never encoded on the wire and can be safely renamed at will. The size of the name does not affect the size of the encoded messages.

The type, if present, is either a built-in type, the name of a user-defined type in the same schema, or the name of an import and the name of a type from the schema corresponding to that import. If the type is missing, it defaults to `unit`. This can be used to create traditional [enumerated types](https://en.wikipedia.org/wiki/Enumerated_type):

```sh
choice weekday {
  monday = 0
  tuesday = 1
  wednesday = 2
  thursday = 3
  friday = 4
}
```

The index is a non-negative integer which is required to be unique within the type. The indices aren't required to be consecutive or in any particular order.

### Built-in types

The following built-in types are supported:

- `unit` is a type which holds no information. It's mainly used for the fields of `choice`s which represent enumerated types.
- `f64` the type of double-precision floating-point numbers as defined by IEEE 754 (*binary64*/*double*).
- `u64` is the type of unsigned 64-bit integers.
- `s64` is the type of signed 64-bit integers.
- `bool` is the type of Booleans.
  - You could define your own Boolean type as a `choice` with two fields, and it would use the exact same space on the wire. However, the built-in `bool` type is often more convenient to use, since it corresponds to the native Boolean type of the programming language targeted by the generated code.
- `bytes` is the type of binary blobs with no further structure.
- `string` is the type of Unicode strings.
- Arrays (e.g., `[u64]`) are the types of sequences of some other type. Any type may be used for the elements, including nested arrays (e.g., `[[string]]`).

### Naming conventions

Typical does not require any particular naming convention for the names of types, fields, schemas, etc. However, it is valuable to establish a convention for consistency. The following simple convention is recommended:

> All identifiers and schema names should be in `lower_snake_case`.

Note that Typical generates code that uses the most popular naming convention for the target programming language, regardless of what convention is used for the type definitions. For example, a `struct` named `email_address` will be called `EmailAddress` in the generated code if the target language is Rust, since that is the most popular convention for Rust.

An identifier must start with a letter or an underscore (`_`), and every subsequent character must be a letter, an underscore, or a digit. If you want to use a keyword (e.g., `choice`) as an identifier, you can do so by prefixing it with a `$` (e.g., `$choice`).

## Binary encoding

The following sections describe how Typical serializes data.

### Built-in types

- `unit` takes 0 bytes to encode.
- `f64` is encoded in the little-endian double-precision floating-point format defined by IEEE 754 (*binary64*/*double*). Thus, it takes 8 bytes to encode.
- `u64` is encoded in a variable-length integer format with bijective numeration. It takes 1-9 bytes to encode, depending on the value. See below for details.
- `s64` is first converted into an unsigned "ZigZag" representation, which is then encoded in the same way as a `u64`. It takes 1-9 bytes to encode, depending on the magnitude of the value. See below for details.
- `bool` is first converted into an integer with `0` representing `false` and `1` representing `true`. The value is then encoded in the same way as a `u64`. It takes 1 byte to encode.
- `bytes` is encoded verbatim, with zero additional space overhead.
- `string` encoded as UTF-8.
- Arrays (e.g., `[u64]`) are in encoded in one of three ways:
  - Arrays of `unit` are represented by the number of elements encoded the same way as a `u64`. Since the elements themselves take 0 bytes to encode, there's no way to infer the number of elements from the size of the message. Thus, it's encoded explicitly.
  - Arrays of `f64`, `u64`, `s64`, or `bool` are represented as the contiguous arrangement of the respective encodings of the elements. The number of elements is not explicitly encoded, since it is implied by the length of the message.
  - Arrays of any other type (`bytes`, `string`, nested arrays, or nested messages) are encoded as the contiguous arrangement of (*size*, *element*) pairs, where *size* is the number of bytes of the encoded *element* and is encoded in the same way as a `u64`. The *element* is encoded according to its type.

#### `u64` encoding in depth

Typical encodes `u64` using a variable-length encoding that allows smaller integers to use fewer bytes. With the distributions that occur in practice, most integers end up consuming only a single byte.

The encoding is as follows. Let `n` be the integer to be encoded. If `n` is less than `2^7 = 128`, it can fit into a single byte:

```
xxxx xxx1
```

If `n` is at least `2^7 = 128` but less than `2^7 + 2^14 = 16,512`, subtract `128` so the result fits into 14 bits, and encode it as follows:

```
xxxx xx10 xxxx xxxx
```

The encoding is little-endian, so the last byte contains the most significant bits.

If `n` is at least `2^7 + 2^14 = 16,512` but less than `2^7 + 2^14 + 2^21 = 2,113,664`, subtract `16,512` so the result fits into 21 bits, and encode it as follows:

```
xxxx x100 xxxx xxxx xxxx xxxx
```

And so on. Using this scheme, the largest 64-bit integer takes 9 bytes, compared to 8 for the native encoding. Thus, the encoding has a single byte of overhead in the worst case, but for most integers encountered in practice it saves 7 bytes.

The encoding is similar to the "base 128 varints" used by [Protocol Buffers](https://developers.google.com/protocol-buffers/docs/encoding#varints) and [Thrift's compact protocol](https://github.com/apache/thrift/blob/master/doc/specs/thrift-compact-protocol.md). However, Typical makes two changes to this encoding:

1. Typical moves all the continuation bits to the first byte. This allows the number of bytes in an encoded integer to be determined entirely from its first byte in a single instruction on modern processors (e.g., `BSF` or `TZCNT`). This is more efficient than checking each byte for a continuation bit separately.
2. Typical's encoding uses a technique called [bijective numeration](https://en.wikipedia.org/wiki/Bijective_numeration), which uses fewer bytes in some cases and never uses more bytes than the aforementioned base 128 varint encoding. For example, the number `16,511` uses two bytes in Typical's encoding, but 3 bytes without bijective numeration. However, the space savings is small and comes with a small runtime performance penalty, so whether this is an improvement depends on how much you value time versus space.

#### `s64` encoding in depth

Typical converts an `s64` into an unsigned "ZigZag" representation, and then encodes the result in the same way as a `u64`. The ZigZag representation converts signed integers with small magnitudes into unsigned integers with small magnitudes, and signed integers with large magnitudes into unsigned integers with large magnitudes. This allows integers with small magnitudes to be encoded using fewer bytes, thanks to the variable-width encoding used for `u64`.

Specifically, the ZigZag representation of a [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) 64-bit integer `n` is `(n >> 63) ^ (n << 1)`, where `>>` is an [arithmetic shift](https://en.wikipedia.org/wiki/Arithmetic_shift). The inverse operation is `(n >> 1) ^ -(n & 1)`, where `>>` is a [logical shift](https://en.wikipedia.org/wiki/Logical_shift).

To give you a sense of how it works, the ZigZag representations of the numbers (`0`, `-1`, `1`, `-2`, `2`) are (`0`, `1`, `2`, `3`, `4`), respectively.

The conversion of signed integers to their ZigZag representations before their subsequent encoding as variable-width integers is also used by [Protocol Buffers](https://developers.google.com/protocol-buffers/docs/encoding#signed_integers) and [Thrift's *compact protocol*](https://github.com/apache/thrift/blob/master/doc/specs/thrift-compact-protocol.md#integer-encoding).

### User-defined `struct`s

A `struct` is encoded as the contiguous arrangement of (*header*, *value*) pairs, where the *value* is encoded according to its type and the *header* is encoded as two contiguous parts:

  - The first part of the *header* is an 64-bit *tag*, which is encoded in the same was as a `u64`. The meaning of the *tag* is as follows:
    - The two least significant bits of the *tag* (not its encoding) are called the *size indicator* and indicate how to compute the size of the *value*:
      - `00`: The size of the *value* is 0 bytes.
      - `01`: The size of the *value* is 8 bytes.
      - `10`: The size of the *value* is given by the second part of the *header* (below).
      - `11`: The *value* is encoded as a `u64` (i.e., it's a `u64`, `s64`, or `bool`), and its size can be determined from its first byte.
    - The remaining 62 bits of the *tag* (not its encoding) represent the index of the *tag* as an unsigned integer.
  - The second part of the *header* is the size of the *value* encoded in the same was as a `u64`. It is only present if the *size indicator* is `10`.

A `struct` must follow these rules:

- Encoding rules:
  - Optional fields may be missing, but required and unstable fields must be present.
- Decoding rules:
  - Unknown fields are ignored.
  - If there are multiple fields with the same index, the first is used and the rest are ignored.
  - Optional and unstable fields may be missing, but required fields must be present.

### User-defined `choice`s

A `choice` is encoded in the same way as a struct, but with different rules:

- Encoding rules:
  - At least one required field must be present.
- Decoding rules:
  - The first field recognized by the receiver is used.
  - At least one required or unstable field must be present.

## Usage

Once Typical is [installed](#installation), you can use it to generate code for a schema called `main.t` with the following:

```sh
$ typical generate main.t --rust-out-file main.rs
```

You can change the `--rust-out-file` flag as appropriate to select the programming language.

Here are the supported command-line options:

```
USAGE:
    typical <SUBCOMMAND>

OPTIONS:
    -h, --help
            Prints help information

    -v, --version
            Prints version information


SUBCOMMANDS:
    generate
            Generate code for a schema and its transitive dependencies

    help
            Prints this message or the help of the given subcommand(s)
```

In particular, the `generate` subcommand has the following options:

```
USAGE:
    typical generate [OPTIONS] <SCHEMA_PATH>

FLAGS:
    -h, --help    Prints help information

OPTIONS:
        --rust-out-file <PATH>    Sets the path of the Rust file to emit

ARGS:
    <SCHEMA_PATH>    Sets the path of the schema
```

## Installation instructions

### Installation on macOS or Linux (x86-64)

If you're running macOS or Linux on an x86-64 CPU, you can install Typical with this command:

```sh
curl https://raw.githubusercontent.com/stepchowfun/typical/main/install.sh -LSfs | sh
```

The same command can be used again to update to the latest version.

The installation script supports the following optional environment variables:

- `VERSION=x.y.z` (defaults to the latest version)
- `PREFIX=/path/to/install` (defaults to `/usr/local/bin`)

For example, the following will install Typical into the working directory:

```sh
curl https://raw.githubusercontent.com/stepchowfun/typical/main/install.sh -LSfs | PREFIX=. sh
```

If you prefer not to use this installation method, you can download the binary from the [releases page](https://github.com/stepchowfun/typical/releases), make it executable (e.g., with `chmod`), and place it in some directory in your [`PATH`](https://en.wikipedia.org/wiki/PATH_\(variable\)) (e.g., `/usr/local/bin`).

### Installation on Windows (x86-64)

If you're running Windows on an x86-64 CPU, download the latest binary from the [releases page](https://github.com/stepchowfun/typical/releases) and rename it to `typical` (or `typical.exe` if you have file extensions visible). Create a directory called `Typical` in your `%PROGRAMFILES%` directory (e.g., `C:\Program Files\Typical`), and place the renamed binary in there. Then, in the "Advanced" tab of the "System Properties" section of Control Panel, click on "Environment Variables..." and add the full path to the new `Typical` directory to the `PATH` variable under "System variables". Note that the `Program Files` directory might have a different name if Windows is configured for language other than English.

To update to an existing installation, simply replace the existing binary.

### Installation with Cargo

If you have [Cargo](https://doc.rust-lang.org/cargo/), you can install Typical as follows:

```sh
cargo install typical
```

You can run that command with `--force` to update an existing installation.
