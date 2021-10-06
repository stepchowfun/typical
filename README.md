# Typical: language-neutral algebraic data types for data interchange

[![Build status](https://github.com/stepchowfun/typical/workflows/Continuous%20integration/badge.svg?branch=main)](https://github.com/stepchowfun/typical/actions?query=branch%3Amain)

*Typical* is a so-called "[interface definition language](https://en.wikipedia.org/wiki/Interface_description_language)", or IDL. You define types in a language-neutral way, then Typical generates code in various languages for serializing and deserializing data based on those types. This is useful for marshalling messages between services, storing data on disk, etc. Typical uses an efficient binary encoding that allows you to change the types over time with forward and backward compatibility as your requirements evolve.

The main difference between Typical and related toolchains like Protocol Buffers and Apache Thrift is that Typical has a more modern type system with ergonomic support for [algebraic data types](https://en.wikipedia.org/wiki/Algebraic_data_type). Typical has a unique solution to the classic problem of how to safely add and remove required fields in structs and the lesser-known problem of how to safely perform exhaustive pattern matching on sum types as cases are added and removed over time.

Typical's design was inspired by insights from a branch of mathematics called [category theory](https://en.wikipedia.org/wiki/Category_theory) (especially the duality of limits and colimits), but you don't need to know any category theory to make effective use of Typical.

## A simple example

Suppose you want to build an API for sending emails. You need to decide how requests and responses will be [serialized](https://en.wikipedia.org/wiki/Serialization). You could use a self-describing format like JSON or XML, but there are some downsides worth considering:

1. There's no easy way to ensure the client and server agree on the structure of the data, especially if they are written in different programming languages and cannot share code.
2. Text-based encodings like JSON and XML incur a performance penalty.

Instead, you can describe the structure of your data with Typical. Create a file called `send-email.t` with the request and response types:

```sh
# This is the request type for the email sending API.
struct request {
  to:   string = 0
  from: string = 1
  body: string = 2
}

# This is the response type for the email sending API.
choice response {
  success       = 0
  error: string = 1
}
```

Notice that each field has both a name and an integer index. The name is only for humans, and the index is used to identify fields in the binary encoding. You can freely rename fields without worrying about binary incompatibility.

A `struct`, such as our `request` type, describes messages containing a collection of fields. This kind of type is also known as a *product type*, and its messages are variously called *records*, *objects*, or *tuples*.

A `choice`, such as our `response` type, describes messages containing exactly one field from a set of possibilities. This kind of type is lesser known among programmers, and it's also known as a *sum type*, *variant*, *discriminated union*, *disjoint union*, *tagged union*, or *coproduct*. Note that the `success` field in `response` doesn't have an explicit type; thus, its type implicitly defaults to `unit`, a built-in type that carries no extra information.

More generally, types built from `struct`s and `choice`s are called *algebraic data types*.

Now that we've defined some types, we can use Typical to generate the code for serialization and deserialization. For example, you can generate Rust code with the following:

```sh
$ typical generate send-email.t --rust-out-file send_email.rs
```

The client and server can both use the generated code to serialize and deserialize messages, which ensures they will understand each other.

Note that Typical only does serialization and deserialization. It has nothing to do with service meshes, encryption, authentication, or authorization, but it can be used together with those technologies.

## Changing types safely

TODO

## Importing other files

You don't need to fit all your type definitions in one file. You can organize your types into separate files at your leisure, and then import files from other files. For example, suppose you want to define a structured `email_address` type your email API, rather than representing email addresses as strings. You could create a file called `email.t` next to your `send-email.t` file with the following contents:

```sh
struct address {
  local_part: string = 0
  domain: string = 0
}
```

Then you can import it in `send-email.t`:

```sh
import 'email.t'

struct request {
  to: email.address = 0
  from: email.address = 1
  body: string = 2
}

choice response {
  success = 0
  error: string = 1
}
```

If you generate the code for `send-email.t` with the same command as above, the generated code will now include the types from both `send-email.t` and `email.t`, as the latter is imported by the former.

Import paths are considered relative to the directory containing the file doing the importing. Typical has no notion of a "top-level" directory on which all paths are based.

A useful convention is to create a `main.t` file that simply imports all the other files, directly or indirectly. Then it's clear which file to use for code generation. Alternatively, in a large organization, you might have a separate top-level file per project that imports only the types needed by that project. However, these are merely conventions, and Typical has no intrinsic notion of "project".

If you import two files with the same name from different directories, you will need to disambiguate usages of those files. Suppose, for example, you attempted the following:

```sh
import 'apis/email.t'
import 'helpers/email.t'

struct employee {
  name: string = 0
  email: email.address = 1 # Uh oh! Which file is this type from?
}
```

Fortunately, Typical will tell you about this problem and ask you to clarify what you mean. You can do so as follows:

```sh
import 'apis/email.t' as email_api
import 'helpers/email.t' as email_helpers

struct employee {
  name: string = 0
  email: email_helpers.address = 1
}
```

## Type system reference

### User-defined types

Every user-defined type is either a `struct` or a `choice`, and they have the same abstract syntax: a list of fields. A field consists of an optional cardinality, a human-readable name, an optional type, and an index. Here's are some examples of user-defined types with various fields:

```sh
struct server {
  hostname: string = 0
  unstable address: ip_address = 1
  optional owner: email.address = 2
}

choice ip_address {
  unknown: 0
  v4: ip.v4 = 1
  v6: ip.v6 = 2
}
```

The cardinality, if present, is either `optional` or `unstable`. The absence of a cardinality indicates that the field is required.

The name is a human-readable identifier for the field. It's used to refer to the field in code, but it's never encoded on the wire and can be safely renamed at will. The size of the name does not affect the size of the encoded messages.

The type, if present, is either a built-in type, the name of a user-defined type in the same file, or the name of an import and the name of a type from the file corresponding to that import. If the type is missing, it defaults to `unit`. This can be used to create traditional [enumerated types](https://en.wikipedia.org/wiki/Enumerated_type):

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
  - You could define your own Boolean type as a `choice` with two fields, and it would use the exact same space on the wire. However, the built-in `bool` type is often more convenient to use, since it corresponds to the native Boolean type of the relevant programming language in the generated code.
- `bytes` is the type of binary blobs with no further structure.
- `string` is the type of Unicode strings.
- Arrays (e.g., `[u64]`) are the types of sequences of some other type. Any type may be used for the elements, including nested arrays (e.g., `[[string]]`).

# Conventions

Typical does not require any particular naming convention for the names of types, fields, files, etc. However, it is valuable to establish a convention for consistency. The following are recommended:

- All identifiers should be `snake_case`.
- All files should be in `hyphen-case.t`.

Note that Typical generates code that uses the most popular naming convention for the relevant programming language, regardless of what convention is used for the type definitions. For example, a `struct` named `email_address` will be called `EmailAddress` in the generated code if the programming language is Rust, since that is the most popular convention for Rust.

## Binary encoding

The following sections describe Typical's efficient binary encoding.

### Built-in types

- `unit` takes 0 bytes to encode.
- `f64` is encoded in the little-endian double-precision floating-point format defined by IEEE 754 (*binary64*/*double*). Thus, it takes 8 bytes to encode.
- `u64` is encoded in a variable-length integer format with bijective numeration. It takes 1-9 bytes to encode, depending on the value. See below for details.
- `s64` is first ZigZag-converted to a 64-bit integer, which is then encoded in the same way as a `u64`. It takes 1-9 bytes to encode, depending on the magnitude of the value. See below for details.
- `bool` is first converted into an integer with `0` representing `false` and `1` representing `true`. The value is then encoded in the same way as a `u64`. It takes 1 byte to encode.
- `bytes` is encoded verbatim, with zero additional space overhead.
- `string` encoded as UTF-8.
- Arrays (e.g., `[u64]`) are in encoded in one of three ways:
  - Arrays of `unit` are represented by the number of elements encoded the same way as a `u64`. Since the elements themselves take 0 bytes to encode, there's no way to infer the number of elements from the size of the message. Thus, it must be encoded explicitly.
  - Arrays of `f64`, `u64`, `s64`, or `bool` are represented as the contiguous arrangement of the respective encodings of the elements. The number of elements is not explicitly encoded, since it is implied by the length of the message.
  - Arrays of any other type (`bytes`, `string`, nested arrays, or nested messages) are encoded as the contiguous arrangement of (*size*, *element*) pairs, where *size* is the number of bytes of the encoded *element* and is encoded in the same way as a `u64`. The *element* is encoded according to its type.

#### How `u64` is encoded

TODO

#### How `s64` is encoded

TODO

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
