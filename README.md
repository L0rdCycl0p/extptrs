<!--
 * @file            README.md
 * @description     
 * @author          TrollMii <trollmii@proton.me>
 * @createTime      2026-09-30 19:05:38
 * @lastModified    2026-09-30 19:05:56
 * Copyright ©L0rdCycl0p
                    ExtPtrs (Extended Pointers)
Copyright (C) 2026 L0rdCycl0p

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

-->

# extptrs

**Extended Pointer Types for Rust.**

`extptrs` provides specialized pointer and smart-pointer types for low-level Rust code where the standard pointer types do not provide the desired ownership, performance, or aliasing semantics.

The crate is designed for systems programming, runtimes, schedulers, allocators, and other performance-sensitive code.

## Features

* Lightweight pointer wrappers
* Low-overhead reference counting
* Non-atomic alternatives to thread-safe smart pointers
* Explicit unsafe APIs where Rust's normal ownership model is intentionally bypassed
* `#[inline(always)]` on performance-critical operations
* Minimal runtime overhead
* No synchronization unless explicitly required

## Mrc

`Mrc<T>` is a single-threaded reference-counted pointer.

It provides shared ownership using a **non-atomic** reference counter. This makes cloning considerably cheaper than atomic reference counting when thread safety is not required.

```rust
use extptrs::Mrc;

let value = Mrc::new(42);

let other = value.clone();

assert_eq!(*value, 42);
assert_eq!(*other, 42);
```

Cloning an `Mrc` is an O(1) operation:

1. Increment the non-atomic reference counter.
2. Copy the pointer.

No atomic operations or locks are involved.

### Mutable access

`Mrc` can provide mutable access through an explicit unsafe API:

```rust
let value = Mrc::new(42);

unsafe {
    *value.get_mut_unchecked() = 100;
}

assert_eq!(*value, 100);
```

This is unsafe because `Mrc` allows multiple owners of the same value. The caller is responsible for ensuring that the returned mutable reference does not violate Rust's aliasing rules.

## Thread Safety

`Mrc` is intentionally **not thread-safe**.

It uses non-atomic reference counting and is intended for single-threaded environments such as:

* runtimes
* schedulers
* game engines
* interpreters
* embedded systems
* low-level infrastructure

If shared ownership between threads is required, use `std::sync::Arc` instead.

## Why extptrs?

Rust's standard library provides excellent general-purpose pointer types, but systems software sometimes needs different trade-offs.

For example:

| Type     | Reference Count | Thread Safe | Clone Cost |
| -------- | --------------- | ----------- | ---------- |
| `Box<T>` | None            | N/A         | Move       |
| `Rc<T>`  | Non-atomic      | No          | Low        |
| `Arc<T>` | Atomic          | Yes         | Higher     |
| `Mrc<T>` | Non-atomic      | No          | Low        |

`extptrs` is intended for pointer types where the semantics are deliberately specialized for a particular low-level use case.

## Performance

The crate prioritizes low overhead over general-purpose convenience.

Performance-sensitive operations are designed to compile down to the smallest practical sequence of instructions. The crate avoids atomic synchronization when the type does not require thread safety.

However, **always benchmark your actual workload**. A lower-level primitive is not automatically faster in every situation.

## Safety

`extptrs` contains unsafe code by design.

Unsafe operations are kept explicit where they bypass Rust's normal ownership and borrowing guarantees. Users of unsafe APIs are responsible for maintaining the documented safety invariants.

The crate does not attempt to make inherently unsafe aliasing patterns safe merely by wrapping them in a pointer type.

## Status

`extptrs` is primarily intended as a low-level building block.

The API may evolve as additional pointer types are added and their performance characteristics are evaluated.

## License

Licensed under the GNU General Public License v3.0 or later (`GPL-3.0-or-later`).

See [LICENSE](LICENSE) for the full license text.
