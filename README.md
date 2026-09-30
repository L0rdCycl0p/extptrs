<!--
 * @file            README.md
 * @description     Extended Pointer Types for Rust
 * @author          TrollMii <trollmii@proton.me>
 * @createTime      2026-09-30 19:05:38
 * @lastModified    2026-09-30 19:21:49
 * Copyright ©L0rdCycl0p
 *
 * ExtPtrs (Extended Pointers)
 *
 * Copyright (C) 2026 L0rdCycl0p
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
-->

# extptrs

**Extended Pointer Types for Rust.**

`extptrs` provides specialized pointer and smart-pointer types for low-level Rust code where the standard pointer types do not provide the desired ownership, performance, or aliasing semantics.

The crate is designed for systems programming, runtimes, schedulers, allocators, and other performance-sensitive code.

## Features

* Lightweight pointer wrappers
* Low-overhead reference counting
* Non-atomic alternatives to thread-safe smart pointers
* Shared mutable storage for specialized single-threaded use cases
* Explicit unsafe internals where Rust's normal ownership model is intentionally bypassed
* `#[inline(always)]` on performance-critical operations
* Minimal runtime overhead
* No synchronization unless explicitly required

## Mrc

`Mrc<T>` is a single-threaded, non-atomic reference-counted pointer.

It provides shared ownership through a plain `usize` reference counter instead of atomic reference counting. This makes cloning very cheap when thread safety is not required.

```rust
use extptrs::Mrc;

let value = Mrc::new(42);

let other = value.clone();

assert_eq!(*value, 42);
assert_eq!(*other, 42);
```

Cloning an `Mrc` is an O(1) operation:

1. Increment the non-atomic reference counter.
2. Copy the internal pointer.

No atomic operations or locks are involved.

### Mutable shared storage

Unlike `Rc<T>` and `Arc<T>`, `Mrc<T>` is designed to allow the value stored inside the shared allocation to be replaced.

```rust
use extptrs::Mrc;

let value = Mrc::new(String::from("Hello"));

value.set(String::from("World"));

assert_eq!(value.get(), "World");
```

`set()` replaces the contained value without changing the reference count or allocating a new `Mrc`.

The `Mrc` allocation itself remains the same:

```text
Mrc<T>
 ├── reference counter
 └── value
       ↓
      "Hello"

        set("World")

Mrc<T>
 ├── reference counter
 └── value
       ↓
      "World"
```

This is intended for runtimes and other systems where multiple owners need to refer to the same mutable storage.

### Accessing the value

`get()` returns a reference to the value stored inside the `Mrc`.

```rust
let value = Mrc::new(42);

let reference = value.get();

assert_eq!(*reference, 42);
```

The value is not copied.

`Deref` is also implemented, so the value can be accessed directly:

```rust
let value = Mrc::new(42);

assert_eq!(*value, 42);
```

## Thread Safety

`Mrc` is intentionally **not thread-safe**.

It uses non-atomic reference counting and is intended exclusively for single-threaded environments such as:

* runtimes
* schedulers
* game engines
* interpreters
* embedded systems
* low-level infrastructure

`Mrc` does not provide synchronization between threads and must not be used as a replacement for `Arc<T>` in multi-threaded code.

If shared ownership between threads is required, use `std::sync::Arc` instead.

## Why extptrs?

Rust's standard library provides excellent general-purpose pointer types, but systems software sometimes needs different trade-offs.

For example:

| Type     | Reference Count | Thread Safe    | Clone Cost | Mutable Shared Storage |
| -------- | --------------- | -------------- | ---------- | ---------------------- |
| `Box<T>` | None            | Depends on `T` | Move       | No                     |
| `Rc<T>`  | Non-atomic      | No             | Low        | No                     |
| `Arc<T>` | Atomic          | Yes            | Higher     | No                     |
| `Mrc<T>` | Non-atomic      | No             | Low        | Yes                    |

`extptrs` is intended for pointer types where the semantics are deliberately specialized for a particular low-level use case.

## Performance

The crate prioritizes low overhead over general-purpose convenience.

`Mrc<T>` avoids atomic reference counting entirely. Its reference counter is a normal `usize`, making `clone()` and `drop()` lightweight operations in a single-threaded environment.

Performance-sensitive operations are marked with `#[inline(always)]` where appropriate.

However, **always benchmark your actual workload**. A lower-level primitive is not automatically faster in every situation.

## Safety

`extptrs` contains unsafe code by design.

The internal implementation uses low-level primitives such as `UnsafeCell`, `NonNull`, and raw allocation management to provide its specialized pointer semantics.

`Mrc<T>` intentionally provides mutable shared storage that does not follow Rust's normal immutable-reference model.

In particular, `Mrc::get()` returns a reference to the stored value while `Mrc::set()` can replace that value through the shared `Mrc`.

This is an intentional property of `Mrc` and is designed for controlled single-threaded environments.

Users should understand the aliasing and lifetime implications of keeping references obtained from `get()` while modifying the stored value through `set()`.

`Mrc` is **not** intended to make arbitrary shared mutable aliasing safe under Rust's normal reference guarantees. It is a specialized low-level primitive whose semantics must be understood by its users.

## Status

`extptrs` is primarily intended as a low-level building block.

The API may evolve as additional pointer types are added and their performance characteristics are evaluated.

## License

Licensed under the GNU General Public License v3.0 or later (`GPL-3.0-or-later`).

See [LICENSE](LICENSE) for the full license text.
