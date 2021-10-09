# Typical: algebraic data types for data interchange

[![Build status](https://github.com/stepchowfun/typical/workflows/Continuous%20integration/badge.svg?branch=main)](https://github.com/stepchowfun/typical/actions?query=branch%3Amain)

*Typical* helps you serialize data in a language-independent fashion. You define data types in a file called a *schema*, then Typical uses that schema to generate the corresponding serialization and deserialization code for various languages. The generated code can be used for marshalling messages between services, storing structured data on disk, etc. Typical uses a compact binary encoding which supports forward and backward compatibility between different versions of your schema to accommodate evolving requirements.

The main difference between Typical and related toolchains like Protocol Buffers and Apache Thrift is that Typical has a more modern type system based on [algebraic data types](https://en.wikipedia.org/wiki/Algebraic_data_type), emphasizing a safer programming style with non-nullable types and pattern matching. You'll feel at home if you have experience with languages which embrace that style, such as Rust, Swift, Kotlin, Haskell, etc. Typical proposes a [new solution](#required-optional-and-asymmetric-fields) to the classic problem of how to safely add and remove required fields in structs and the lesser-known dual problem of how to safely perform exhaustive pattern matching on sum types as cases are added and removed over time.

**Supported languages:**

- Rust
- *Coming soon:* TypeScript

## Introduction

Suppose you want to build an API for sending emails, and you need to decide how requests and responses will be [serialized](https://en.wikipedia.org/wiki/Serialization) for transport. You could use a self-describing format like JSON or XML, but you may prefer to have better type safety and performance. *Typical* has a great story to tell about those things.

### Write a schema

You can start by creating a schema file called `email_api.t` with the relevant types for your email API:

```perl
struct send_email_request {
    to: string = 0
    subject: string = 1
    body: string = 2
}

choice send_email_response {
    success = 0
    error: string = 1
}
```

A `struct`, such as our `send_email_request` type, describes messages containing a fixed set of fields (in this case, `to`, `subject`, and `body`). A `choice`, such as our `send_email_response` type, describes messages containing exactly one field from a fixed set of possibilities (in this case, `success` and `error`). `struct`s and `choice`s are called *algebraic data types* due to their correspondence to ideas from category theory called *products* and *sums*, respectively, but you don't need to know anything about that to use Typical.

Each field in a `struct` or a `choice` has both a name (e.g., `subject`) and an integer index (e.g., `1`). The name is just for humans, as only the index is used to identify fields in the binary encoding. You can freely rename fields without worrying about binary incompatibility.

Each field also has a type, either explicitly or implicitly. If the type is missing, as it is for the `success` field above, then it implicitly defaults to a built-in type called `unit`.

### Generate code for serialization and deserialization

Now that we've defined some types, we can use Typical to generate the code for serialization and deserialization. For example, you can generate Rust code with the following:

```sh
$ typical generate email_api.t --rust-out-file email_api.rs
```

The client and server can then use the generated code to serialize and deserialize messages for mutual communication. If the client and server are written in different languages, you can generate code for each language.

Note that Typical only does serialization and deserialization. It has nothing to do with service meshes, encryption, authentication, or authorization, but it can be used together with those technologies.

### Serialize and deserialize messages

A program could then construct a request and serialize it to a file:

```rust
let request = SendEmailRequestOut {
    to: "typical@example.com".to_owned(),
    subject: "I love Typical!".to_owned(),
    body: "It makes serialization easy and safe.".to_owned(),
};

let mut file = BufWriter::new(File::create("/tmp/request")?);
request.serialize(&mut file)
```

A different program, possibly written in a different language, could read the request from disk and deserialize it:

```rust
let mut file = BufReader::new(File::open("/tmp/request")?);
let request = SendEmailRequestIn::deserialize(&mut file)?;

println!("to: {}", request.to);
println!("subject: {}", request.subject);
println!("body: {}", request.body);
```

The full code for the example can be found [here](https://github.com/stepchowfun/typical/tree/main/example).

We'll see in the next section why our `send_email_request` type turned into `SendEmailRequestOut` and `SendEmailRequestIn`.

## Required, `optional`, and `asymmetric` fields

Fields are required by default. This is an unusual design decision, since required fields are often thought to cause trouble for backward and forward compatibility between schema versions. Let's explore this topic in detail and see how Typical deals with it.

### The trouble with required fields

Experience has taught us that it can be difficult to introduce a required field to a type that is already being used. For example, suppose your new email API is up and running, and you want to add a new `from` field to the request type:

```perl
struct send_email_request {
    to: string = 0
    from: string = 3 # A new required field
    subject: string = 1
    body: string = 2
}
```

The only safe way to roll out this change (as written) is to finish updating all clients before beginning to update any servers. Otherwise, a client still running the old code might send a request to an updated server, which promptly rejects the request because it lacks the new field.

That kind of rollout may not be feasible. You may not be in control of the order in which clients and servers are updated. Or, the clients and servers might be updated together, but not atomically. The client and the server might even be part of the same replicated service, so it wouldn't be possible to update one before the other no matter how careful you are.

Removing a required field can present analogous difficulties. Suppose, despite the aforementioned challenges, you were able to successfully introduce `from` as a required field. Now, an unrelated issue is forcing you to roll it back. That's just as dangerous as adding it was in the first place: if a client gets updated before a server, that client may then send the server a message without the `from` field, which the server will reject since it still expects that field to be present.

### The trouble with `optional` fields

Due to the trouble associated with required fields, the conventional wisdom is simply to never use them; all fields should be `optional`.

However, this advice ignores the reality that some things really are *semantically required*, even if they aren't declared as required in the schema. An API cannot be expected to work if it doesn't have the data it needs. Having semantically required fields declared as `optional` places extra burden on both writers and readers: writers cannot rely on the type system to prevent them from accidentally forgetting to set the field, and readers must handle the case of the field being missing to satisfy the type checker even though that field is always supposed to be set.

For those of us who haven't given up on the idea of required fields, the standard process for introducing one is to transition between three phases: (1) introduce the field as `optional`, (2) update all the writers to set the new field, and finally (3) promote it to required. Unfortunately, you can't rely on the type system to ensure you've done phase (2) completely. That phase can be nontrivial in a large system, and you may forget to set the field somewhere.

To remove a required field, the standard process is to transition through two phases: (1) demote it to `optional`, but ensure that writers are still setting it, and (2) start allowing the field to be unset or delete the field entirely. Here, phase (1) is the troublesome one, since the type system no longer guarantees that the field is still being set by writers during that time.

### Introducing: `asymmetric` fields

Typical offers an intermediate state between `optional` and required: `asymmetric`. An `asymmetric` field in a `struct` is considered required for the writer, but `optional` for the reader. This state allows you to safely introduce and remove required fields.

Let's make that more concrete with our email API example. Instead of directly introducing the `from` field as required, we first introduce it as `asymmetric`:

```perl
struct send_email_request {
    to: string = 0
    asymmetric from: string = 3 # A new asymmetric field
    subject: string = 1
    body: string = 2
}
```

Let's take a look at the generated code for this schema. In Rust, for example, we actually end up with two different types, one for serialization and another for deserialization:

```rust
pub struct SendEmailRequestOut {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
}

pub struct SendEmailRequestIn {
    pub to: String,
    pub from: Option<String>,
    pub subject: String,
    pub body: String,
}

impl Serialize for SendEmailRequestOut {
    // Implementation omitted.
}

impl Deserialize for SendEmailRequestIn {
    // Implementation omitted.
}
```

Typical also generates code (not shown above) for converting `SendEmailRequestOut` into `SendEmailRequestIn`, which is logically equivalent to serialization followed by deserialization, but faster.

We can see the effect of `from` being an `asymmetric` field: its type is `String` in `SendEmailRequestOut`, but its type is `Option<String>` in `SendEmailRequestIn`. Our clients use the former to construct requests, and our servers will decode them into the latter.

Once this schema change has been rolled out, clients are setting the new field, but servers are not yet relying on it. We need to go through this intermediate phase before we can safely promote the field to required. **This notion of `asymmetric` fields is what makes Typical special.**

It works in reverse too. Suppose we now want to remove the field. It could be unsafe to delete the field directly, since then clients might stop setting it before servers can handle its absence. But we can demote it to `asymmetric`, which forces servers to consider it `optional` and handle its potential absence while clients are still required to set it. Once that change has rolled out, we can confidently delete the field (or demote it to `optional`), as the servers no longer require it.

For some kinds of changes, a field might stay in the `asymmetric` state for months, say, if you are waiting for users to update your mobile app. Typical helps immensely in that situation.

### What about `choice`s?

Our discussion so far has been framed around `struct`s, since they are more familiar to most programmers. However, the same kind of consideration must be given to `choice`s.

The code generated for `choice`s supports case analysis, so clients can take different actions depending on which field was set. Happily, the generated code ensures you've handled all the cases when you use it. This is called *exhaustive pattern matching*, and it's a great feature to help you write correct code. But that extra rigor can be a double-edged sword: readers will fail to deserialize a `choice` if the field is not recognized.

That means it's unsafe, in general, to add or remove required fields—just like with `struct`s. If you add a required field, writers might start using it before readers can understand it. Conversely, if you remove a required field, readers may no longer be able to handle it while writers are still using it.

Not to worry—`choice`s support `optional` and `asymmetric` fields too!

An `optional` field of a `choice` must be paired with a fallback field, which is used as a backup in case the reader doesn't recognize the original field. So readers are not required to handle optional fields; hence, *optional*. Note that the fallback itself might be `optional`, in which case the fallback must have a fallback, etc. Eventually, the fallback chain ends with a required field. Readers will scan the fallback chain for the first field they recognize.

An `asymmetric` field must also be paired with a fallback, but the fallback chain is not made available to readers: they must be able to handle the `asymmetric` field directly. Messages without any fallbacks can be deserialized, since readers do not use them. In summary, `asymmetric` fields in `choice`s behave like optional fields for writers and like required fields for readers—the opposite of their behavior in `struct`s.

That may sound useless, but it's exactly what's needed to safely introduce or remove required fields from `choice`s.

To see what the generated code looks like for `optional` and `asymmetric` fields, consider a more elaborate version of our API response type:

```perl
choice send_email_response {
    success = 0
    error: string = 1
    optional authentication_error: string = 2 # A specific type of error
    asymmetric please_try_again = 3
}
```

As with `struct`s, the generated code for a `choice` has separate types for serialization and deserialization:

```rust
pub enum SendEmailResponseOut {
    Success,
    Error(String),
    AuthenticationError(String, Box<SendEmailResponseOut>),
    PleaseTryAgain(Box<SendEmailResponseOut>),
}

pub enum SendEmailResponseIn {
    Success,
    Error(String),
    AuthenticationError(String, Box<SendEmailResponseIn>),
    PleaseTryAgain,
}

impl Serialize for SendEmailResponseOut {
    // Implementation omitted.
}

impl Deserialize for SendEmailResponseIn {
    // Implementation omitted.
}
```

Typical also generates code (not shown above) for converting `SendEmailResponseOut` into `SendEmailResponseIn`, which is logically equivalent to serialization followed by deserialization, but faster.

The required cases (`Success` and `Error`) are as you would expect in both types.

The `optional` case, `AuthenticationError`, has a `String` for the error message and a second payload for the fallback field. Readers can use the fallback if they don't wish to handle this case, and readers which don't even know about this case will use the fallback automatically.

The `asymmetric` case, `PleaseTryAgain`, also requires writers to provide a fallback. However, readers don't get to use it. This is a safe intermediate state to use before changing the field to required (which will stop requiring writers to provide a fallback) or changing the field from required to something else (which will stop readers from having to handle it).

### Summary

Non-nullable types and exhaustive pattern matching are important safety features of modern type systems, but they are not well-supported by most data interchange formats. Typical, on the other hand, embraces them.

The rules are as follows:

- You can safely rename and reorder fields, as long as you don't change their indices.
- You can safely add and remove `optional` and `asymmetric` fields.
- You can safely convert `optional` fields to `asymmetric` and vice versa.
- You can safely convert `asymmetric` fields to required and vice versa.
- You can safely convert a `struct` with exactly one field, which must be required, into a `choice` with just that field and vice versa.
- No other changes are guaranteed to be safe. In particular, it may be unsafe to add or remove required fields, unless you can carefully manage the order in which writers and readers are updated.

All told, the idea of `asymmetric` fields can be understood as an application of the [robustness principle](https://en.wikipedia.org/wiki/Robustness_principle) to algebraic data types.

## Schema style guide

Typical doesn't require any particular naming convention or formatting style. However, it's valuable to establish conventions for consistency. We recommend being consistent with the examples given in this guide. For example:

- Use `lower_snake_case` for the names of everything: types, fields, etc.
- Indent fields with 4 spaces.

Note that Typical generates code that uses the most popular naming convention for the target programming language, regardless of what convention is used for the type definitions. For example, a `struct` named `email_address` will be called `EmailAddress` (or `EmailAddressOut`/`EmailAddressIn`) in the generated code if the target language is Rust, since idiomatic Rust uses `UpperCamelCase` for the names of user-defined types.

## Schema reference

A schema contains only two kinds of things: imports and user-defined types. The order of those things doesn't matter. Whitespace doesn't matter either.

### Imports

You don't need to fit all your type definitions in one schema file. You can organize your types into separate schema files at your leisure, and then import schemas from other schemas. For example, suppose you have a schema called `email_util.t` with the following contents:

```perl
struct address {
    local_part: string = 0
    domain: string = 1
}
```

Then you can import it from another file, say `email_api.t`:

```perl
import 'email_util.t'

struct send_email_request {
    to: email_util.address = 0
    subject: string = 1
    body: string = 2
}
```

The generated code for `email_api.t` will now include the types from both `email_api.t` and `email_util.t`, as the latter is imported by the former.

Import paths are considered relative to the directory containing the schema doing the importing. Typical has no notion of a "top-level" directory on which all paths are based.

A useful convention is to create a `main.t` schema that simply imports all the other schemas, directly or indirectly. Then it's clear which schema to use for code generation. Alternatively, in a large organization, you might have a separate top-level schema per project that imports only the types needed by that project. However, these are merely conventions, and Typical has no intrinsic notion of "project".

If you import two schemas with the same name from different directories, you'll need to disambiguate usages of those schemas. Suppose, for example, you attempted the following:

```perl
import 'apis/email.t'
import 'util/email.t'

struct employee {
    name: string = 0
    email: email.address = 1 # Uh oh! Which schema is this type from?
}
```

Fortunately, Typical will tell you about this problem and ask you to clarify what you mean. You can do so as follows:

```perl
import 'apis/email.t' as email_api
import 'util/email.t' as email_util

struct employee {
    name: string = 0
    email: email_util.address = 1
}
```


### User-defined types

Every user-defined type is either a `struct` or a `choice`, and they have the same abstract syntax: a name and a list of fields. A field consists of an optional rule, a human-readable name, an optional type, and an index. Here's are some examples of user-defined types with various fields:

```perl
import 'apis/email.t'
import 'net/ip.t'

choice device_ip_address {
    static_v4: ip.v4_address = 0
    static_v6: ip.v6_address = 1
    dynamic = 2
}

struct device {
    hostname: string = 0
    asymmetric ip_address: device_ip_address = 1
    optional owner: email.address = 2
}
```

The rule, if present, is either `optional` or `asymmetric`. The absence of a rule indicates that the field is required.

The name is a human-readable identifier for the field. It's used to refer to the field in code, but it's never encoded on the wire and can be safely renamed at will. The size of the name doesn't affect the size of the encoded messages, so be as descriptive as you want.

The type, if present, is either a built-in type (e.g., `string`), the name of a user-defined type in the same schema (e.g., `server`), or the name of an import and the name of a type from the schema corresponding to that import (e.g., `email.address`). If the type is missing, it defaults to `unit`. This can be used to create traditional [enumerated types](https://en.wikipedia.org/wiki/Enumerated_type):

```perl
choice weekday {
    monday = 0
    tuesday = 1
    wednesday = 2
    thursday = 3
    friday = 4
}
```

The index is a non-negative integer which is required to be unique within the type. The indices aren't required to be consecutive or in any particular order, but starting with consecutive indices is a good convention.

### Built-in types

The following built-in types are supported:

- `unit` is a type which holds no information. It's mainly used for the fields of `choice`s which represent enumerated types.
- `f64` is the type of double-precision floating-point numbers as defined by IEEE 754.
- `u64` is the type of unsigned 64-bit integers.
- `s64` is the type of signed 64-bit integers.
- `bool` is the type of Booleans.
  - You could define your own Boolean type as a `choice` with two fields, and it would use the exact same space on the wire. However, the built-in `bool` type is often more convenient to use, since it corresponds to the native Boolean type of the programming language targeted by the generated code.
- `bytes` is the type of binary blobs with no further structure.
- `string` is the type of Unicode strings.
- Arrays (e.g., `[u64]`) are the types of sequences of some other type. Any type may be used for the elements, including nested arrays (e.g., `[[string]]`).

### Comments

Comments can be used to add helpful context to your schemas. A comment begins with a `#` and continues to the end of the line, as with Python, Ruby, Perl, etc.

### Identifiers

An identifier (the name of a type, field, or import) must start with a letter or an underscore (`_`), and every subsequent character must be a letter, an underscore, or a digit. If you want to use a keyword (e.g., `choice`) as an identifier, you can do so by prefixing it with a `$` (e.g., `$choice`).

## Binary encoding

The following sections describe how Typical serializes your data.

Note that the generated deserialization code is designed to be safe from malicious inputs, in that it protects against unsafe memory accesses like buffer over-reading and overflowing, denial-of-service attacks like [billion laughs](https://en.wikipedia.org/wiki/Billion_laughs_attack), and arbitrary code execution.

### Built-in types

- `unit` takes 0 bytes to encode.
- `f64` is encoded in the little-endian double-precision floating-point format defined by IEEE 754. Thus, it takes 8 bytes to encode.
- `u64` is encoded in a variable-width integer format with bijective numeration. It takes 1-9 bytes to encode, depending on the value. See below for details.
- `s64` is first converted into an unsigned "ZigZag" representation, which is then encoded in the same way as a `u64`. It takes 1-9 bytes to encode, depending on the magnitude of the value. See below for details.
- `bool` is first converted into an integer with `0` representing `false` and `1` representing `true`. The value is then encoded in the same way as a `u64`. It takes 1 byte to encode.
- `bytes` is encoded verbatim, with zero additional space overhead.
- `string` encoded as UTF-8.
- Arrays (e.g., `[u64]`) are encoded in one of three ways:
  - Arrays of `unit` are represented by the number of elements encoded the same way as a `u64`. Since the elements themselves take 0 bytes to encode, there's no way to infer the number of elements from the size of the message. Thus, it's encoded explicitly.
  - Arrays of `f64`, `u64`, `s64`, or `bool` are represented as the contiguous arrangement of the respective encodings of the elements. The number of elements is not explicitly encoded, since it's implied by the length of the message.
  - Arrays of any other type (`bytes`, `string`, nested arrays, or nested messages) are encoded as the contiguous arrangement of (*size*, *element*) pairs, where *size* is the number of bytes of the encoded *element* and is encoded in the same way as a `u64`. The *element* is encoded according to its type.

#### `u64` encoding in depth

Typical encodes `u64` using a variable-width encoding that allows smaller integers to use fewer bytes. With the distributions that occur in practice, most integers end up consuming only a single byte.

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

And so on. Notice that the number of trailing zeros in the first byte indicates how many subsequent bytes there are.

Using this encoding, the largest 64-bit integer takes 9 bytes, compared to 8 for the native encoding. Thus, the encoding has a single byte of overhead in the worst case, but for most integers encountered in practice it saves 7 bytes. This is such a good trade-off most of the time that Typical doesn't even offer fixed-width integer types. However, if you really need to store fixed-width integers, you can always encode them as `bytes` at the expense of some type safety.

The encoding is similar to the "base 128 varints" used by [Protocol Buffers](https://developers.google.com/protocol-buffers/docs/encoding#varints) and [Thrift's *compact protocol*](https://github.com/apache/thrift/blob/master/doc/specs/thrift-compact-protocol.md). However, Typical's encoding differs in two ways:

1. Typical moves all the continuation bits to the first byte. This allows the number of bytes in an encoded integer to be determined entirely from its first byte in a single instruction on modern processors (e.g., `BSF` or `TZCNT`). This is more efficient than checking each byte for a continuation bit separately.
2. Typical's encoding uses a technique called [bijective numeration](https://en.wikipedia.org/wiki/Bijective_numeration), which uses fewer bytes in some cases and never uses more bytes than the aforementioned base 128 varint encoding. For example, the number `16,511` uses two bytes in Typical's encoding, but 3 bytes in the encoding used by Protocol Buffers and Thrift's *compact protocol*. However, the space savings is small and comes with a small runtime performance penalty, so whether this is an improvement depends on how much you value time versus space.

#### `s64` encoding in depth

Typical converts an `s64` into an unsigned "ZigZag" representation, and then encodes the result in the same way as a `u64`. The ZigZag representation converts signed integers with small magnitudes into unsigned integers with small magnitudes, and signed integers with large magnitudes into unsigned integers with large magnitudes. This allows integers with small magnitudes to be encoded using fewer bytes, thanks to the variable-width encoding used for `u64`.

Specifically, the ZigZag representation of a [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) 64-bit integer `n` is `(n >> 63) ^ (n << 1)`, where `>>` is an [arithmetic shift](https://en.wikipedia.org/wiki/Arithmetic_shift). The inverse operation is `(n >> 1) ^ -(n & 1)`, where `>>` is a [logical shift](https://en.wikipedia.org/wiki/Logical_shift).

To give you a sense of how it works, the ZigZag representations of the numbers (`0`, `-1`, `1`, `-2`, `2`) are (`0`, `1`, `2`, `3`, `4`), respectively.

The conversion of signed integers to their ZigZag representations before their subsequent encoding as variable-width integers is also used by [Protocol Buffers](https://developers.google.com/protocol-buffers/docs/encoding#signed_integers) and [Thrift's *compact protocol*](https://github.com/apache/thrift/blob/master/doc/specs/thrift-compact-protocol.md#integer-encoding).

### User-defined `struct`s

A `struct` is encoded as the contiguous arrangement of (*header*, *value*) pairs, one pair per field, where the *value* is encoded according to its type and the *header* is encoded as two contiguous parts:

  - The first part of the *header* is a 64-bit *tag*, which is encoded in the same was as a `u64` (i.e., as a variable-width integer). The meaning of the *tag* is as follows:
    - The two least significant bits of the *tag* (not its encoding) are called the *size indicator* and indicate how to compute the size of the *value*:
      - `00`: The size of the *value* is 0 bytes.
      - `01`: The size of the *value* is 8 bytes.
      - `10`: The size of the *value* is given by the second part of the *header* (below).
      - `11`: The *value* is encoded as a `u64` (i.e., it's a `u64`, `s64`, or `bool`), and its size can be determined from its first byte.
    - The remaining 62 bits of the *tag* (not its encoding) represent the index of the field as an unsigned integer.
  - The second part of the *header* is the size of the *value* encoded in the same was as a `u64`. It's only present if the *size indicator* is `10`.

For fields of type `unit`, `f64`, `u64`, `s64`, or `bool` for which the index is less than 32, the *header* is encoded as a single byte.

A `struct` must follow these rules:

- Encoding rules:
  - Optional fields may be missing, but required and `asymmetric` fields must be present.
- Decoding rules:
  - Unrecognized fields are ignored.
  - All required fields must be present, whereas `optional` and `asymmetric` fields may be missing.

### User-defined `choice`s

A `choice` is encoded in the same way as a `struct`, but with different rules:

- Encoding rules:
  - At least one required field must be present.
- Decoding rules:
  - The first field recognized by the receiver is used.
  - At least one required or `asymmetric` field must be present.

For a simple enumerated type (such as `weekday` above), the encoding of a field with an index less than 32 takes up a single byte.

## Usage

Once Typical is [installed](#installation-instructions), you can use it to generate code for a schema called `main.t` with the following:

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
