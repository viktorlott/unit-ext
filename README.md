
# Fluent Interface for Unit type

This is a tiny utility crate for Rust developers who enjoy expressive, fluent-style code.

It extends the unit type with ergonomic methods for returning common wrapper values like
`Ok`, `Err`, `Some`, `None`, and `Default`. These helpers are particularly useful when you want
to inject side effects (like logging or metrics) while preserving functional control flow.

```rust,ignore
Some(10)
    .filter(|n| n > &5)// Return None if there's no match
    .map(|n| n * 2)//                          |
    .or_else(|| println!("Value too small").ret_none());


//            Return value after println
Some(10)//                          |
    .map(|v| println!("Some({v})").ret(v + 10))
    .or_else(|| println!("Default value").ret_default());
//                                           |
//       Return T::default() if there's no match


Some([1, 2, 3, 4])
    .map(|mut arr| 
        arr.clone_from_slice(&[4, 3, 2, 1]).ret(arr));
//                                           |
//                  Mutate arr, then return arr

"10".parse::<u8>().map_or_else(
//            Log error, then return None
//                          |
    |e| eprintln!("{e}").ret_none(),
    |v| println!("Got here").ret_some(v.add(10)),
);//                            |
//         Call side-effect, then return Some(value)
```

## Discarding Values Explicitly

Rust warns you when values are unused--which is good! But when discarding a value is
intentional, you should say so clearly:

```rust
use unit_ext::RetExt;

#[must_use]
fn noisy(x: i32) { println!("{x}"); }

(0..3).for_each(|n| noisy(n).discard_ret());
```

This basically turns `T` -> `()`, while `UnitExt` turns `()` -> `T`. So these two traits go hand in
hand.

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

// ------------------- VS --------------------

let value: Option<_> = if cfg!(target_pointer_width = 64) {
	Init::new().map_or_else(
		|e| { eprintln!("error: {e}"); None },
		|v| v.build_factory().into()
	)
} else {
	eprintln!("not supported");
	None
};
```

#### 2. `match` on `Result` with discard + default

```rust
use unit_ext::*;
use std::collections::HashMap;

fn do_it(mut value: Result<HashMap<String, String>, &str>) -> Option<()> {
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
| `().ret_ok(v)`          | `Ok(v)`                      |                                            |
| `().ret_err(e)`         | `Err(e)`                     |                                            |
| `().ret_none::<T>()`    | `None::<T>`                  | type hint retained                         |
| `().ret_some(v)`        | `Some(v)`                    | `v.into()` under the hood                  |
| `().ret_default::<T>()` | `T::default()`               |                                            |
| `val.discard_self()`    | `let _ = val; ()`            | explicit, greppable side-effect marker     |
| `func().discard_ret()`  | alias; same as `discard_self`| keeps the **`ret_` rhythm**                |


## Why?

Sometimes you want to return something like `Ok(value)` or `None` *after* performing a side-effect,
but writing it out every time feels too verbose. With `unit_ext`, you write:

```rust
use unit_ext::UnitExt;

let _ = ().ret_ok::<_, bool>(123);    // -> Result<i32, bool>
let _ = ().ret_some("hi");            // -> Option<&str>
let _ = ().ret_default::<Vec<u8>>();  // -> Vec<u8>
let _ = ().ret_none::<u8>();          // -> Option<u8>::None
```

Behind the scenes, `()` is a zero-sized type (ZST), so these helpers don't add overhead--they
just make intent clear and code cleaner.

#### Naming Rationale: Why `ret_*`?

The method names in this crate follow a `ret_*` prefix pattern--short for "return".
This was chosen for clarity, greppability, and to clearly communicate the intent: you're returning
a value from `()` in a fluent, readable way.

#### Why not `to_*`, `as_*`, `then_*`, or `into_*`?

These were considered, and here's why they were avoided:

- **`to_*`** - Suggests a transformation of `self`, which isn't accurate for unit `()`
- **`as_*`** - Implies a reinterpretation of the value, not construction
- **`then_*`** - Often signals sequencing or chaining based on a result (`.then()` in futures, etc.)
- **`into_*`** - Suggests a conversion *from* `self`, which isn't happening here

By contrast:

- **`ret_*`** - Clearly signals that we're *returning* a value in a fluent way
from a no-value context, like a post-logging statement or a side-effect expression.