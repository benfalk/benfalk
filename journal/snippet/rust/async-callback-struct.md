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

This `BoxFuture` can be read as a dynamically allocated future on the
stack, it must be be _pinned_ to prevent references from moving, and
has a reported lifetime.  With this understanding we can now look back
at the `Cb<E>` type.  This is a dynamically allocated function pointer
which receives some owned type `E` and returns a `BoxFuture`.

The `'static` lifetime is assigned to the `BoxFuture` because it owns it's
own data and therefore can live the entire life of the application.  The
type of `()` simply means the future returns no output.

### The Implementation

{{#aa (snippet) ../../../crates/async_cb/src/lib.rs#mod?name=impls}}

This `Callback<E>` can seem a little scary; however, I think after we
break down the code it will seem more approachable.  This `new` function
has two declared types of `F` and `Ret`.  The `F` type must be a function
which returns a type `Ret` and the function's lifetime must be `'static`.
The `Ret` which the function returns has to implement a `Future` that
has no output, is `Send` safe, and has a `'static` lifetime.

The `Ret` type of this signature is very important, it is what allows
us to create our final callback.  This future return is exactly what
can be boxed into our `Cb<E>` from before.  Inside of the method body
a lambda is boxed up to create the `BoxFuture` when called.  With that
we now have a callback ready.

### The Test

{{#aa (snippet) ../../../crates/async_cb/src/lib.rs#mod?name=test}}

[futures]: https://crates.io/crates/futures
