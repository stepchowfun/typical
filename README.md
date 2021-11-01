# Typical: algebraic data types for data interchange

[![Build status](https://github.com/stepchowfun/typical/workflows/Continuous%20integration/badge.svg?branch=main)](https://github.com/stepchowfun/typical/actions?query=branch%3Amain)

*Typical* helps you [serialize](https://en.wikipedia.org/wiki/Serialization) data in a language-independent fashion. You define data types in a file called a *schema*, then Typical generates the corresponding serialization and deserialization code for various languages. The generated code can be used for marshalling messages between services, storing structured data on disk, etc. Typical uses a compact binary encoding which supports forward and backward compatibility between different versions of your schema to accommodate evolving requirements.

The main difference between Typical and related toolchains like Protocol Buffers and Apache Thrift is that Typical has a more modern type system based on [algebraic data types](https://en.wikipedia.org/wiki/Algebraic_data_type), emphasizing a safer programming style with non-nullable types and pattern matching. You'll feel at home if you have experience with languages which embrace that style, such as Rust, Swift, Kotlin, Haskell, etc. Typical offers a new solution (["asymmetric" fields](#introducing-asymmetric-fields)) to the classic problem of how to safely add and remove required fields in structs as well as the lesser-known dual problem of how to safely add and remove cases in sum types while supporting exhaustive pattern matching.

In short, Typical offers two important features that are conventionally thought to be at odds: (1) uncompromising type safety and (2) binary compatibility between schema versions.

**Supported languages:**

- Rust
- *Coming soon:* TypeScript

## Introduction

Suppose you want to build an API for sending emails, and you need to decide how requests and responses will be serialized for transport. You could use a self-describing format like JSON or XML, but you may want better type safety and performance. *Typical* has a great story to tell about those things.

### Write a schema

You can start by creating a schema file called `email_api.t` with the relevant types for your email API:

```perl
struct SendEmailRequest {
    to: String = 0
    subject: String = 1
    body: String = 2
}

choice SendEmailResponse {
    success = 0
    error: String = 1
}
```

A `struct`, such as our `SendEmailRequest` type, describes messages containing a fixed set of fields (in this case, `to`, `subject`, and `body`). A `choice`, such as our `SendEmailResponse` type, describes messages containing exactly one field from a fixed set of possibilities (in this case, `success` and `error`). `struct`s and `choice`s are called *algebraic data types* due to their correspondence to ideas from category theory called *products* and *sums*, respectively, but you don't need to know anything about that to use Typical.

Each field in a `struct` or a `choice` has both a name (e.g., `body`) and an integer index (e.g., `2`). The name is just for humans, as only the index is used to identify fields in the binary encoding. You can freely rename fields without worrying about binary incompatibility.

Each field also has a type. If the type is missing, as it is for the `success` field above, then it defaults to a built-in type called `Unit`.

### Generate code for serialization and deserialization

Now that we've defined some types, we can use Typical to generate the code for serialization and deserialization. For example, you can generate Rust code with the following:

```sh
typical generate email_api.t --rust-out email_api.rs
```

Refer to the [example Rust project](https://github.com/stepchowfun/typical/tree/main/examples/rust) for how to automate this with a [Cargo build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

The client and server can then use the generated code to serialize and deserialize messages for mutual communication. If the client and server are written in different languages, you can generate code for each language.

Note that Typical only does serialization and deserialization. It has nothing to do with networking, encryption, authentication, or authorization, but it can be used together with those technologies.

### Serialize and deserialize messages

With the code generated in the previous section, a program could construct a message and serialize it to a file (for example) as follows:

```rust
let request = SendEmailRequestOut {
    to: "typical@example.com".to_owned(),
    subject: "I love Typical!".to_owned(),
    body: "It makes serialization easy and safe.".to_owned(),
};

let mut file = BufWriter::new(File::create("/tmp/request")?);
request.serialize(&mut file)?;
```

Another program, possibly written in a different language, could read the message from disk (for example) and deserialize it as follows:

```rust
let mut file = BufReader::new(File::open("/tmp/request")?);
let request = SendEmailRequestIn::deserialize(&mut file)?;

println!("to: {}", request.to);
println!("subject: {}", request.subject);
println!("body: {}", request.body);
```

The full code for this example can be found [here](https://github.com/stepchowfun/typical/tree/main/examples/rust/src/main.rs).

We'll see in the next section why our `SendEmailRequest` type turned into `SendEmailRequestOut` and `SendEmailRequestIn`.

## Required, `optional`, and `asymmetric` fields

Fields are required by default. This is an unusual design decision, since required fields are often thought to cause trouble for backward and forward compatibility between schema versions. Let's explore this topic in detail and see how Typical deals with it.

### The trouble with adding and removing required fields directly

Experience has taught us that it can be difficult to introduce a required field to a type that is already being used. For example, suppose your new email API is up and running, and you want to add a new `from` field to the request type:

```perl
struct SendEmailRequest {
    to: String = 0
    from: String = 3 # A new required field
    subject: String = 1
    body: String = 2
}
```

The only safe way to roll out this change (as written) is to finish updating all clients before beginning to update any servers. Otherwise, a client still running the old code might send a request to an updated server, which promptly rejects the request because it lacks the new field.

That kind of attentive rollout may not be feasible. You may not be in control of the order in which clients and servers are updated. Or, the clients and servers might be updated together, but not atomically. The client and the server might even be part of the same replicated service, so it wouldn't be possible to update one before the other no matter how careful you are.

Removing a required field can present analogous difficulties. Suppose, despite the aforementioned challenges, you were able to successfully introduce `from` as a required field. Now, an unrelated issue is forcing you to roll it back. That's just as dangerous as adding it was in the first place: if a client gets updated before a server, that client may then send the server a message without the `from` field, which the server will reject since it still expects that field to be present.

### The trouble with promoting `optional` fields to required and vice versa

A somewhat safer way to introduce a required field is to first introduce it as `optional`, and later promote it to required. For example, you can safely introduce this change:

```perl
struct SendEmailRequest {
    to: String = 0
    optional from: String = 3 # A new optional field
    subject: String = 1
    body: String = 2
}
```

You would then update clients to set the new field. Once you're confident that the new field is always being set, you can promote it to required.

The trouble is that, as long as the field is `optional`, you can't rely on the type system to ensure the new field is always being set. Even if you're confident you've updated the client code appropriately, a collaborator might not be aware of your efforts and might introduce a new violation of your policy before you have the chance to promote the field to required.

You can run into analogous trouble when demoting a required field to `optional`. Once the field has been demoted, clients might stop setting the field before the servers can handle its absence, unless you can be sure the servers are updated promptly enough.

### The trouble with making every field `optional`

Due to the trouble associated with required fields, the conventional wisdom is simply to never use them; all fields should be declared `optional`. For example:

```perl
struct SendEmailRequest {
    optional to: String = 0
    optional subject: String = 1
    optional body: String = 2
}
```

However, this advice ignores the reality that some things really are *semantically required*, even if they aren't declared required in the schema. An API cannot be expected to work if it doesn't have the data it needs. Having semantically required fields declared as `optional` places extra burden on both writers and readers: writers cannot rely on the type system to prevent them from accidentally forgetting to set the fields, and readers must address the case of the fields being missing to satisfy the type checker even though those fields are always supposed to be set.

### Introducing: `asymmetric` fields

Typical offers an intermediate state between `optional` and required: `asymmetric`. An `asymmetric` field in a `struct` is considered required for the writer, but `optional` for the reader. Unlike `optional` fields, an `asymmetric` field can be safely promoted to required and vice versa.

Let's make that more concrete with our email API example. Instead of directly introducing the `from` field as required, we first introduce it as `asymmetric`:

```perl
struct SendEmailRequest {
    to: String = 0
    asymmetric from: String = 3 # A new asymmetric field
    subject: String = 1
    body: String = 2
}
```

Let's take a look at the generated code for this schema; we'll choose Rust for this example. The generated code has two flavors of our `SendEmailRequest` type, one for serialization and another for deserialization:

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
    // Serialization code omitted.
}

impl Deserialize for SendEmailRequestIn {
    // Deserialization code omitted.
}
```

We can see the effect of `from` being an `asymmetric` field: its type is `String` in `SendEmailRequestOut`, but its type is `Option<String>` in `SendEmailRequestIn`. That means clients (which use `SendEmailRequestOut`) are now required to set the new field, but servers (which use `SendEmailRequestIn`) aren't yet allowed to rely on it. Once this change has been rolled out (at least to clients), we can safely promote the field to required in a subsequent change.

It works in reverse too. Suppose we now want to remove a required field. It may be unsafe to delete the field directly, since then clients might stop setting it before servers can handle its absence. But we can demote it to `asymmetric`, which forces servers to consider it `optional` and handle its potential absence, even though clients are still required to set it. Once that change has been rolled out (at least to servers), we can confidently delete the field (or demote it to `optional`), as the servers no longer rely on it.

In some situations, a field might stay in the `asymmetric` state for months, say, if you're waiting for a sufficient fraction of your users to update your mobile app. Typical can help immensely in those situations by preventing new code which uses the field inappropriately from being introduced during that period.

### What about `choice`s?

Our discussion so far has been framed around `struct`s, since they are more familiar to most programmers. However, the same kind of consideration must be given to `choice`s.

The code generated for `choice`s supports case analysis, so clients can take different actions depending on which field was set. Happily, this is done in a way that ensures you've handled all the cases. This is called *exhaustive pattern matching*, and it's a great feature to help you write correct code. But that extra rigor can be a double-edged sword: readers will fail to deserialize a `choice` if they don't recognize the field that was set.

That means it's unsafe, in general, to add or remove required fields to a `choice`—just like with `struct`s. If you add a required field, updated writers may start setting it before non-updated readers know how to handle it. Conversely, if you remove a required field, updated readers will no longer be able to handle it even though non-updated writers may still be setting it.

Not to worry—`choice`s can have `optional` and `asymmetric` fields, just like `struct`s!

An `optional` field of a `choice` must be paired with a fallback field, which is used as a backup in case the reader doesn't recognize or doesn't want to handle the original field. So readers aren't required to handle `optional` fields; hence, *optional*. Note that the fallback itself might be `optional`, in which case the fallback must have a fallback, etc. Eventually, the fallback chain ends with a required field. Readers will scan the fallback chain for the first field they recognize.

*Note:* An `optional` field in a `choice` is not simply a field with a nullable type. The word "optional" here means that readers can ignore it and use a fallback instead, not that its payload might be missing.

An `asymmetric` field must also be paired with a fallback, but the fallback chain is not made available to readers; they must be able to handle the `asymmetric` field directly. Thus, `asymmetric` fields in `choice`s behave like `optional` fields for writers and like required fields for readers—the opposite of their behavior in `struct`s.

As with `struct`s, an `asymmetric` field in a `choice` can be safely promoted to required and vice versa.

Consider a more elaborate version of our API response type:

```perl
choice SendEmailResponse {
    success = 0
    error: String = 1
    optional authentication_error: String = 2 # A more specific type of error for curious clients
    asymmetric please_try_again = 3 # To be promoted to required in the future
}
```

Let's inspect the generated code. As with `struct`s, we end up with separate types for serialization and deserialization:

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
    // Serialization code omitted.
}

impl Deserialize for SendEmailResponseIn {
    // Deserialization code omitted.
}
```

The required cases (`Success` and `Error`) are as you would expect in both types.

The `optional` case, `AuthenticationError`, has a `String` for the error message and a second payload for the fallback. A writer might set the less specific `Error` case as the fallback. Readers can use the fallback if they don't wish to handle the optional case, and readers which don't even know about the optional case will use the fallback automatically.

The `asymmetric` case, `PleaseTryAgain`, also requires writers to provide a fallback. However, readers don't get to use it. This is a safe intermediate state to use before changing the field to required (which will stop requiring writers to provide a fallback) or changing the field from required to `optional` or nonexistent (which will stop readers from having to handle it).

### What about default values?

Typical has no notion of a "default" value for each type. This means, for example, if a reader sees the value `0` for a field, it can be confident that this value was explicitly set by a writer, and that the writer didn't just accidentally forget to set it. Zeroes, empty strings, empty arrays, and so on aren't special in any way.

## Summary of what kinds of schema changes are safe

Any schema can be safely migrated to any other schema through a series of backward and forward compatible changes. Here are the rules for what is allowed in a single change:

- You can safely rename and reorder fields, as long as you don't change their indices.
- You can safely add and remove `optional` and `asymmetric` fields.
- You can safely convert any fields to `asymmetric` and vice versa.
- You can safely convert a `struct` with exactly one field, which must be required, into a `choice` with just that field and vice versa.
- No other changes are guaranteed to be safe.

In mathematical terms, these rules define a homogeneous compatibility [relation](https://en.wikipedia.org/wiki/Binary_relation) over schemas which is reflexive (every schema is compatible with itself) and symmetric (forward compatibility and backward compatibility imply each other), but not transitive (two individually safe schema changes are not necessarily safe as a single change).

## Schema reference

A schema contains only two kinds of things: imports and user-defined types. The order of those things doesn't matter. Whitespace doesn't matter either.

### Imports

You don't need to fit all your type definitions in one schema file. You can organize your types into separate schema files at your leisure, and then import schemas from other schemas. For example, suppose you have a schema called `email_util.t` with the following contents:

```perl
struct Address {
    local_part: String = 0
    domain: String = 1
}
```

Then you can import it from another file, say `email_api.t`:

```perl
import 'email_util.t'

struct SendEmailRequest {
    to: email_util.Address = 0
    subject: String = 1
    body: String = 2
}
```

The generated code for `email_api.t` will now include the types from both `email_api.t` and `email_util.t`, as the latter is imported by the former.

Import paths are considered relative to the directory containing the schema doing the importing. Typical has no notion of a "top-level" directory on which all paths are based.

A useful convention is to create a `main.t` schema that simply imports all the other schemas, directly or indirectly. Then it's clear which schema to use for code generation. Alternatively, in a large organization, you might have a separate top-level schema per project that imports only the types needed by that project. However, these are merely conventions, and Typical has no intrinsic notion of "project".

If you import two schemas with the same name from different directories, you'll need to disambiguate usages of those schemas. Suppose, for example, you attempted the following:

```perl
import 'apis/email.t'
import 'util/email.t'

struct Employee {
    name: String = 0
    email: email.Address = 1 # Uh oh! Which schema is this type from?
}
```

Fortunately, Typical will tell you about this problem and ask you to clarify what you mean. You can do so with import aliases as follows:

```perl
import 'apis/email.t' as email_api
import 'util/email.t' as email_util

struct Employee {
    name: String = 0
    email: email_util.Address = 1
}
```


### User-defined types

Every user-defined type is either a `struct` or a `choice`, and they have the same abstract syntax: a name, an optional list of indices of deleted fields, and a list of fields. Here's are some examples of user-defined types:

```perl
import 'apis/email.t'
import 'net/ip.t'

choice DeviceIpAddress {
    static_v4: ip.V4Address = 0
    static_v6: ip.V6Address = 1
    dynamic = 2
}

struct Device {
    hostname: String = 0
    asymmetric ip_address: DeviceIpAddress = 1
    optional owner: email.Address = 2
}
```

#### Fields

A field consists of an optional rule, a human-readable name, an optional type, and an index.

The rule, if present, is either `optional` or `asymmetric`. The absence of a rule indicates that the field is required.

The name is a human-readable identifier for the field. It's used to refer to the field in code, but it's never encoded on the wire and can be safely renamed at will. The size of the name doesn't affect the size of the encoded messages, so be as descriptive as you want.

The type, if present, is either a built-in type (e.g., `String`), the name of a user-defined type in the same schema (e.g., `Server`), or the name of an import and the name of a type from the schema corresponding to that import (e.g., `email.Address`). If the type is missing, it defaults to `Unit`. This can be used to create traditional [enumerated types](https://en.wikipedia.org/wiki/Enumerated_type):

```perl
choice Weekday {
    monday = 0
    tuesday = 1
    wednesday = 2
    thursday = 3
    friday = 4
}
```

The index is a non-negative integer which is required to be unique within the type. The indices aren't required to be consecutive or in any particular order, but starting with consecutive indices is a good convention.

#### Deleted fields

If you delete a field, you must be careful not to reuse that field's index for any new fields as long as there are messages still containing the deleted field. Otherwise, the old field would be decoded as the new field, which is likely to result in deserialization errors and is almost certainly not what you want. To avoid this, you can reserve the indices of deleted fields to prevent them from being reused. For example, if we delete the `ip_address` and `owner` fields from the `Device` `struct` above, we can reserve their indices as follows:

```perl
struct Device {
    deleted 1 2

    hostname: String = 0
}
```

Typical will then prevent us from introducing new fields with those indices.

### Built-in types

The following built-in types are supported:

- `Unit` is a type which holds no information. It's mainly used for the fields of `choice`s which represent enumerated types.
- `F64` is the type of double-precision floating-point numbers as defined by IEEE 754.
- `U64` is the type of integers in the range [`0, 2^64`).
- `S64` is the type of integers in the range [`-2^63, 2^63`).
- `Bool` is the type of Booleans.
  - You could define your own Boolean type as a `choice` with two fields, and it would use the exact same space on the wire. However, the built-in `Bool` type is often more convenient to use, since it corresponds to the native Boolean type of the programming language targeted by the generated code.
- `Bytes` is the type of binary blobs with no further structure.
- `String` is the type of Unicode strings.
- Arrays (e.g., `[U64]`) are the types of sequences of some other type. Any type may be used for the elements, including nested arrays (e.g., `[[String]]`).

### Comments

Comments can be used to add helpful context to your schemas. A comment begins with a `#` and continues to the end of the line, as with Python, Ruby, Perl, etc.

### Identifiers

An identifier (the name of a type, field, or import) must start with a letter or an underscore (`_`), and every subsequent character must be a letter, an underscore, or a digit. If you want to use a keyword (e.g., `choice`) as an identifier, you can do so by prefixing it with a `$` (e.g., `$choice`).

## Schema style guide

Typical doesn't require any particular naming convention or formatting style. However, it's valuable to establish conventions for consistency. We recommend being consistent with the examples given in this guide. For example:

- Use `UpperCamelCase` for the names of types.
- Use `lower_snake_case` for the names of everything else: fields, import aliases, and schema files.
- Indent fields with 4 spaces.

Note that Typical generates code that uses the most popular naming convention for the target programming language, regardless of what convention is used for the type definitions. For example, a `struct` named `email_address` will be called `EmailAddressOut`/`EmailAddressIn` in the code generated for Rust, since idiomatic Rust uses `UpperCamelCase` for the names of user-defined types.

## Security

The generated deserialization code is designed to be safe from malicious inputs in the sense that it protects against unsafe memory accesses like buffer over-reading, buffer overflowing, and arbitrary code execution.

There is currently no way to configure resource limits, so it's good practice to reject implausibly large messages before attempting to deserialize them. It's also good practice to apply techniques like [rate limiting](https://en.wikipedia.org/wiki/Rate_limiting) to mitigate [denial-of-service attacks](https://en.wikipedia.org/wiki/Denial-of-service_attack).

In general, you can expect the size of a deserialized message in memory to be within the same order of magnitude as the size of the corresponding serialized message on the wire. However, there is one exception: for values of type `[Unit]` (array of units), only the number of elements is encoded, since the `Unit` values themselves take up zero bytes on the wire. That means an attacker can force the deserialization logic to reconstruct arbitrarily large arrays of units given only the number of elements (cf. [billion laughs attack](https://en.wikipedia.org/wiki/Billion_laughs_attack)). For this reason, we strongly recommend avoiding the use of `[Unit]` in your schema if you intend to consume untrusted inputs. This isn't a major loss, however, since that type is generally useless anyway. It's only supported for the uniformity of the type system; the array [type constructor](https://en.wikipedia.org/wiki/Type_constructor) accepts any type for its argument, even if some combinations have no practical purpose.

## Binary encoding

The following sections describe how Typical serializes your data.

### Variable-width integers

Many situations require Typical to serialize integer values, e.g., for encoding field indices, buffer sizes, and user-provided integer data. Where appropriate, Typical uses a variable-width encoding that allows smaller integers to use fewer bytes. With the distributions that occur in practice, most integers end up consuming only a single byte.

#### Unsigned variable-width integers

For unsigned integers, the valid range is [`0`, `2^64`).

Let `n` be the integer to be encoded. The encoding scheme described below is [little-endian](https://en.wikipedia.org/wiki/Endianness), so the last byte contains the most significant bits.

- If `0 <= n < 128`:
  - Embed the 7 bits of `n` in 1 byte as follows: `xxxxxxx1`.
- If `128 <= n < 16,512`
  - Embed the 14 bits of `n - 128` in 2 bytes as follows: `xxxxxx10 xxxxxxxx`.
- If `16,512 <= n < 2,113,664`
  - Embed the 21 bits of `n - 16,512` in 3 bytes as follows: `xxxxx100 xxxxxxxx xxxxxxxx`.
- If `2,113,664 <= n < 270,549,120`
  - Embed the 28 bits of `n - 2,113,664` in 4 bytes as follows: `xxxx1000 xxxxxxxx xxxxxxxx xxxxxxxx`.
- If `270,549,120 <= n < 34,630,287,488`
  - Embed the 35 bits of `n - 270,549,120` in 5 bytes as follows: `xxx10000 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx`.
- If `34,630,287,488 <= n < 4,432,676,798,592`
  - Embed the 42 bits of `n - 34,630,287,488` in 6 bytes as follows: `xx100000 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx`.
- If `4,432,676,798,592 <= n < 567,382,630,219,904`
  - Embed the 49 bits of `n - 4,432,676,798,592` in 7 bytes as follows: `x1000000 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx`.
- If `567,382,630,219,904 <= n < 72,624,976,668,147,840`
  - Embed the 56 bits of `n - 567,382,630,219,904` in 8 bytes as follows: `10000000 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx`.
- If `72,624,976,668,147,840 <= n < 18,446,744,073,709,551,616`
  - Embed the 64 bits of `n - 72,624,976,668,147,840` in 9 bytes as follows: `00000000 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx`.

The number of trailing zeros in the first byte indicates how many subsequent bytes there are. This allows the number of bytes in an encoded integer to be efficiently determined with a single instruction (e.g., `BSF` or `TZCNT`) on most modern processors.

This variable-width integer encoding is similar to the "base 128 varints" used by [Protocol Buffers](https://developers.google.com/protocol-buffers/docs/encoding#varints) and [Thrift's *compact protocol*](https://github.com/apache/thrift/blob/master/doc/specs/thrift-compact-protocol.md). However, Typical's encoding differs in two ways:

1. For time efficiency reasons, Typical moves all the continuation bits to the first byte. This allows the continuation bits to be counted with a single CPU instruction on most processors, as mentioned above.
2. For space efficiency reasons, Typical uses a technique called [bijective numeration](https://en.wikipedia.org/wiki/Bijective_numeration), which uses fewer bytes in some cases and never uses more bytes than the aforementioned base 128 varint encoding. For example, the number `16,511` uses 2 bytes in Typical's encoding, but 3 bytes in the encoding used by Protocol Buffers and Thrift's *compact protocol*.

#### Unsigned variable-width integers

For signed integers, the valid range is [`-2^63`, `2^63`).

Typical converts signed integers into an unsigned "ZigZag" representation, and then encodes the unsigned result as described above. The ZigZag representation converts signed integers with small magnitudes into unsigned integers with small magnitudes, and signed integers with large magnitudes into unsigned integers with large magnitudes. This allows signed integers with small magnitudes to be encoded using fewer bytes.

Specifically, the ZigZag representation of a [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) 64-bit integer `n` is `(n >> 63) ^ (n << 1)`, where `>>` is an [arithmetic shift](https://en.wikipedia.org/wiki/Arithmetic_shift). The inverse operation is `(n >> 1) ^ -(n & 1)`, where `>>` is a [logical shift](https://en.wikipedia.org/wiki/Logical_shift).

To give you a sense of how it works, the ZigZag representations of the numbers (`0`, `-1`, `1`, `-2`, `2`) are (`0`, `1`, `2`, `3`, `4`), respectively.

### User-defined `struct`s

A `struct` is encoded as the contiguous arrangement of (*header*, *value*) pairs, one pair per field, where the value is encoded according to its type and the header is encoded as one or two contiguous parts:

  - The first part of the header is an unsigned integer *tag*, which is encoded as a variable-width integer. The meaning of the tag is as follows:
    - The two least significant bits of the tag (not its variable-width encoding) are called the *size mode* and indicate how to compute the size of the value:
      - `0`: The size of the value is 0 bytes.
      - `1`: The size of the value is 8 bytes.
      - `2`: The size of the value is given by the second part of the header (below).
      - `3`: The value is encoded as a variable-width integer, so its size can be determined from its first byte.
    - The remaining bits of the tag (not its variable-width encoding) represent the index of the field as an unsigned integer.
  - The second part of the header is the size of the value encoded as a variable-width integer. It's only present if the size mode is `2`.

For fields of type `Unit`, `F64`, `U64`, `S64`, or `Bool` for which the index is less than 32, the header is encoded as a single byte.

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

For a simple enumerated type (such as `Weekday` above), a field with an index less than 32 takes up a single byte.

### Built-in types

- `Unit` takes 0 bytes to encode.
- `F64` is normally encoded in the little-endian double-precision floating-point format defined by IEEE 754. Thus, it normally takes 8 bytes to encode. However, for field values (rather than, say, elements of an array), [positive zero](https://en.wikipedia.org/wiki/Signed_zero) is encoded as 0 bytes.
- `U64` is normally encoded as a variable-width integer. Thus, it normally takes 1-9 bytes to encode, depending on the value. However, for field values (rather than, say, elements of an array), `0` is encoded as 0 bytes, and values equal to or greater than `567,382,630,219,904` are encoded as fixed-width 8-byte little-endian integers.
- `S64` is normally first converted into the unsigned ZigZag representation, which is then encoded as a variable-width integer. Thus, it normally takes 1-9 bytes to encode, depending on the magnitude of the value. However, for field values (rather than, say, elements of an array), `0` is encoded as 0 bytes, and values for which the ZigZag representation is equal to or greater than `567,382,630,219,904` are encoded as fixed-width 8-byte [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) little-endian integers (verbatim, not with a ZigZag representation).
- `Bool` is first converted into an integer with `0` representing `false` and `1` representing `true`. The value is then encoded in the same way as a `U64`, including the special behavior in the case of field values if applicable. Thus, `false` takes 0 bytes to encode, and `true` takes 1 byte.
- `Bytes` is encoded verbatim.
- `String` is encoded as UTF-8.
- Arrays (e.g., `[U64]`) are encoded in one of three ways:
  - Arrays of `Unit` are represented by the number of elements encoded the same way as a `U64`, including the special behavior in the case of field values if applicable. Since the elements (of type `Unit`) take 0 bytes to encode, there's no way to infer the number of elements from the size of the buffer. Thus, it's encoded explicitly.
  - Arrays of `F64`, `U64`, `S64`, or `Bool` are represented as the contiguous arrangement of the respective encodings of the elements. The number of elements is not explicitly encoded.
  - Arrays of any other type (`Bytes`, `String`, nested arrays, or nested messages) are encoded as the contiguous arrangement of (*size*, *element*) pairs, where *size* is the number of bytes of the encoded *element* and is encoded as a variable-width integer. The *element* is encoded according to its type. The number of elements is not explicitly encoded.

Notice that several types can take advantage of a more compact representation when they are used for the values of fields. For example, a variable-width integer takes 1-9 bytes to encode, but an integer field takes 0-8 bytes to encode, not including the field header. This may seem impossible—the resolution to this paradox is that the extra information comes from the size mode in the field header.

## Usage

Once Typical is [installed](#installation-instructions), you can use it to generate code for a schema called `main.t` with the following:

```sh
typical generate main.t --rust-out types.rs --typescript-out types.ts
```

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
        --rust-out <PATH>          Sets the path of the Rust file to emit
        --typescript-out <PATH>    Sets the path of the TypeScript file to emit

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

If you're running Windows on an x86-64 CPU, download the latest binary from the [releases page](https://github.com/stepchowfun/typical/releases) and rename it to `typical` (or `typical.exe` if you have file extensions visible). Create a directory called `Typical` in your `%PROGRAMFILES%` directory (e.g., `C:\Program Files\Typical`), and place the renamed binary in there. Then, in the "Advanced" tab of the "System Properties" section of Control Panel, click on "Environment Variables..." and add the full path to the new `Typical` directory to the `PATH` variable under "System variables". Note that the `Program Files` directory might have a different name if Windows is configured for a language other than English.

To update to an existing installation, simply replace the existing binary.

### Installation with Cargo

If you have [Cargo](https://doc.rust-lang.org/cargo/), you can install Typical as follows:

```sh
cargo install typical
```

You can run that command with `--force` to update an existing installation.
