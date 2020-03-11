# Atomize

Elixir style atoms for Rust

From Elixir:
An atom is a constant whose value is its own name.
Some other languages call these symbols.
They are often useful to enumerate over distinct values.

## Creating an Atom

```rs
use atomize::{a, Atom};

fn main() {
    // `a!(apple)` will always create the same value
    let apple: Atom = a!(apple);

    assert_eq!(apple, a!(apple));
}
```

## Atom Equality

Atoms are compared in O(1) time.
In fact, they compile to simple u64 and so
are compared in a single x64 operation

```rs
assert_eq!(a!(orange), a!(orange));
assert_ne!(a!(orange), a!(apple));
```

## Mixing

Atoms can also be mixed

```rs
let apple_and_orange = a!(apple) + a!(orange);

assert_eq!(apple_and_orange, a!(orange) + a!(apple));
```
