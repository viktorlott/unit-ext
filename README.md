
# Fluent Interface for Unit type

<div align="center">
    <a href="https://github.com/viktorlott/unit-ext">
        <img alt="Github" src="https://img.shields.io/github/languages/code-size/viktorlott/unit-ext?style=flat-square&logo=github" height="20">
    </a>
    <a href="https://crates.io/crates/unit-ext">
        <img alt="crates.io" src="https://img.shields.io/crates/v/unit-ext.svg?style=flat-square&logo=rust" height="20">
    </a>
    <a href="https://docs.rs/unit-ext/latest">
        <img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-unit--ext-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">
    </a>
</div>


This is a tiny utility crate for Rust developers who enjoy expressive,
fluent-style code.

It extends the unit type with ergonomic methods for returning common
wrapper values like `Ok`, `Err`, `Some`, `None`, and `Default`. These
helpers are particularly useful when you want to inject side effects
(like logging or metrics) while preserving functional control flow.

```rust,ignore
use unit_ext::UnitExt;

maybe_some_or_none(10)
    .filter(|n| n > &5)// Return None if there's no match
    .map(|n| n * 2)//                          |
    .or_else(|| println!("Value too small").ret_none());


//                         Return value after println
maybe_some_or_none(10)//            |
    .map(|v| println!("Some({v})").ret(v + 10))
    .or_else(|| println!("Default value").ret_default());
//                                           |
//             Return T::default() if there's no match


maybe_some_or_none([1, 2, 3, 4])
    .map(|mut arr|
        arr.clone_from_slice(&[4, 3, 2, 1]).ret(arr));
//                                           |
//                        Mutate arr, then return arr


text.parse::<u8>().map_or_else(
//              Log error, then return None
//                          |
    |e| eprintln!("{e}").ret_none(),
    |v| println!("Got here").ret_some(v.add(10)),
);//                            |
//          Call side-effect, then return Some(value)
```

## Discarding Values Explicitly

While `UnitExt` helps go from `()` → `T`, there are times you want the
reverse: to *explicitly discard* a value and return `()`. This is
especially useful when calling functions purely for their side
effects.

Rust will warn you when values are unused--which is great. But when
ignoring a return value is intentional, it's better to make that clear
in code.

That's where `RetExt` comes in:

```rust
use unit_ext::RetExt;
// We must use the returned value of noisy
//         /
#[must_use]
fn noisy(x: i32) -> i32 { println!("{x}"); x }

(0..3).for_each(|n| noisy(n).discard_ret());
//                               |
//        We intentionally discard the return value
```

This turns `T -> ()` in a way that's:

- **Explicit** – makes intent obvious 
- **Greppable** – easy to search for side-effect-only code
- **Consistent** – matches the `ret_*` method rhythm

Where `UnitExt` turns `()` into values, `RetExt` discards values back
into `()`.

These two traits work together: one for building values from a
no-value context, and one for discarding values in side-effect
contexts—-making your fluent code clearer and more intentional.

---
## Examples

#### 1. `map_or_else` with side-effect logging  

```rust,ignore
use unit_ext::UnitExt;

let value: Option<_> = if cfg!(target_pointer_width = 64) {
    Init::new().map_or_else(
        |e| eprintln!("error: {e}").ret_none(),
        |v| v.build_factory().into()
    )
} else {
    eprintln!("not supported").ret_none()
};
```

#### 2. `match` on `Result` with discard + default

```rust
use unit_ext::*;
use std::collections::HashMap;

type Storage = HashMap<String, String>;

fn do_it(mut value: Result<Storage, &str>) -> Option<()> {
    match value {
        Ok(mut v)  => v.remove("key").discard_ret().into(),
        Err(e) => eprintln!("error: {e}").ret_none(),
    }
}
```

---
## Quick reference
| helper                  | expands to                   | notes                                      |
|-------------------------|------------------------------|--------------------------------------------|
| `().ret(v)`             | `v`                          | returns v                                  |
| `().ret_ok(v)`          | `Ok(v)`                      | -                                          |
| `().ret_err(e)`         | `Err(e)`                     | -                                          |
| `().ret_none::<T>()`    | `None::<T>`                  | type hint retained                         |
| `().ret_some(v)`        | `Some(v)`                    | `v.into()` under the hood                  |
| `().ret_default::<T>()` | `T::default()`               | -                                          |
| `val.discard_self()`    | `let _ = val; ()`            | explicit, greppable side-effect marker     |
| `func().discard_ret()`  | alias; same as `discard_self`| keeps the **`ret_` rhythm**                |


## Why?

Sometimes you want to return something like `Ok(value)` or `None`
*after* performing a side-effect, but writing it out every time feels
too verbose. With `unit_ext`, you write:

```rust
use unit_ext::UnitExt;

let _ = ().ret_ok::<_, bool>(123);    // -> Result<i32, bool>
let _ = ().ret_some("hi");            // -> Option<&str>
let _ = ().ret_default::<Vec<u8>>();  // -> Vec<u8>
let _ = ().ret_none::<u8>();          // -> Option<u8>::None
```

Behind the scenes, `()` is a zero-sized type (ZST), so these helpers
don't add overhead--they just make intent clear and code cleaner.

## Oh but you can just do:
```rust,ignore
|e| { eprintln!("error: {e}"); None },
```
But that only works cleanly if you're okay with unformatted code. As
soon as you run rustfmt, it'll expand into a multi-line block--which
may not be what you want. You can suppress formatting with
`#[rustfmt::skip]`, but doing that every time gets tedious fast.

There are also crates like tap that explore similar ideas around
inserting side effects into fluent chains. But the goal of this crate
is to make those patterns feel more intentional, expressive, and
consistent, especially in codebases that value clean formatting and
readability.

#### Naming Rationale: Why `ret_*`?

The method names in this crate follow a `ret_*` prefix pattern--short
for "return". This was chosen for clarity, greppability, and to
clearly communicate the intent: you're returning a value from `()` in
a fluent, readable way.

#### Why not `to_*`, `as_*`, `then_*`, or `into_*`?

These were considered, and here's why they were avoided:

- **`to_*`** - Suggests a transformation of `self`, which isn't
  accurate for unit `()`
- **`as_*`** - Implies a reinterpretation of the value, not
  construction
- **`then_*`** - Often signals sequencing or chaining based on a
  result (`.then()` in futures, etc.)
- **`into_*`** - Suggests a conversion *from* `self`, which isn't
  happening here

By contrast:

- **`ret_*`** - Clearly signals that we're *returning* a value in a
  fluent way from a no-value context, like a post-logging statement or
  a side-effect expression.