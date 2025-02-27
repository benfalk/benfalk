---
CREATED_AT: 2025-02-27T11:22:22.316533836+00:00
TOPIC: snippet
lang: rust
title: async callback struct
---

## Async Callback Struct

### The Structure

{{#aa (snippet) ../../../crates/async_cb/src/lib.rs#mod?name=structure}}

First I create a `Cb<E>` type alias using `BoxFuture` from the [futures]
crate.  Before explaining this `Cb<E>` more first it helps to make sure
we understand exactly _what_ this `BoxFuture` actually is.  It turns out
that it is also a type alias:

```rust
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
```

This `BoxFuture` can be read as a dynamically allocated future allocated on
the stack, it must be be _pinned_ to prevent reference from moving, and
it has a reported lifetime.  With this understanding we can now look back
at the `Cb<E>` type.  This can be understood as a dynamically allocated
function pointer which receives some type `E` and returns a `BoxFuture`.

The `'static` lifetime is assigned to the `BoxFuture` because it owns it's
own data and therefore can live the entire life of the application.  The
type of `()` simply means the future returns no output.

Now that we have a better understanding of the `BoxFuture` it is time to
unpack the custom `Cb<E>` type alias.  It is a `Box` wrapping a function
pointer that takes ownership of type `E` and returns a freshly allocated
`BoxFuture` which we described before hand.

### The Implementations

{{#aa (snippet) ../../../crates/async_cb/src/lib.rs#mod?name=impls}}

### The Test

{{#aa (snippet) ../../../crates/async_cb/src/lib.rs#mod?name=test}}

[futures]: https://crates.io/crates/futures
