/*
 * @file            src/mrc/mod.rs
 * @description     
 * @author          TrollMii <trollmii@proton.me>
 * @createTime      2026-09-30 18:37:13
 * @lastModified    2026-09-30 19:21:49
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

*/

#[cfg(test)]
mod tests;

use std::{
    cell::UnsafeCell,
    ops::Deref,
    ptr::NonNull,
};

struct MrcInner<T> {
    refs: usize,
    value: UnsafeCell<T>,
}

/// A single-threaded, mutable reference-counting pointer.
///
/// `Mrc` provides shared ownership of a value through a non-atomic reference
/// counter. Cloning an `Mrc` is therefore very cheap: it only increments the
/// reference counter and copies the pointer.
///
/// Unlike [`std::sync::Arc`], `Mrc` is not thread-safe and must not be shared
/// between threads. The reference counter uses non-atomic operations, which
/// avoids the synchronization overhead required by atomic reference counting.
///
/// `Mrc` is intended for single-threaded runtimes where shared ownership is
/// useful but the overhead of atomic reference counting is unnecessary.
///
/// The contained value can be accessed through [`Mrc::get`] and replaced
/// through [`Mrc::set`].
///
/// # Performance
///
/// Cloning an `Mrc` is an O(1) operation consisting of a non-atomic reference
/// count increment and a pointer copy. Dropping an `Mrc` decrements the
/// reference count and deallocates the value when the last reference is
/// dropped.
///
/// # Thread Safety
///
/// `Mrc` is intentionally not thread-safe. It must not be sent to or shared
/// between threads.
///
/// For thread-safe shared ownership, use [`std::sync::Arc`] instead.
pub struct Mrc<T> {
    ptr: NonNull<MrcInner<T>>,
}

impl<T> Mrc<T> {
    /// Creates a new `Mrc` containing `value`.
    ///
    /// The newly created `Mrc` is the only owner of the value, so its strong
    /// reference count is initially `1`.
    ///
    /// Creating an `Mrc` allocates memory for the reference-counted value.
    #[inline(always)]
    pub fn new(value: T) -> Self {
        let inner = Box::new(MrcInner {
            refs: 1,
            value: UnsafeCell::new(value),
        });

        Self {
            ptr: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
        }
    }

    #[inline(always)]
    unsafe fn inner(&self) -> &MrcInner<T> {
        unsafe { self.ptr.as_ref() }
    }

    /// Returns a reference to the contained value.
    ///
    /// The returned reference points directly to the value stored inside the
    /// `Mrc`; the value itself is not copied.
    ///
    /// The reference remains valid as long as the underlying `MrcInner`
    /// allocation remains alive.
    #[inline(always)]
    pub fn get(&self) -> &T {
        unsafe { &*self.inner().value.get() }
    }

    /// Replaces the contained value.
    ///
    /// The value is replaced in place without changing the reference count
    /// or allocating a new `Mrc`.
    ///
    /// Any existing references to the contained value must not be used in a
    /// way that violates Rust's reference aliasing rules while the value is
    /// being replaced.
    #[inline(always)]
    pub fn set(&self, value: T) {
        unsafe {
            *self.inner().value.get() = value;
        }
    }

    /// Returns the number of strong references to the contained value.
    ///
    /// Every [`Mrc::clone`] increases this count by one, and dropping an
    /// `Mrc` decreases it by one.
    ///
    /// A newly created `Mrc` always has a strong count of `1`.
    #[inline(always)]
    pub fn strong_count(&self) -> usize {
        unsafe { self.inner().refs }
    }
}

impl<T> Clone for Mrc<T> {
    /// Creates another `Mrc` pointing to the same value.
    ///
    /// Cloning does not clone the contained `T`. It only increments the
    /// non-atomic reference counter and copies the internal pointer.
    ///
    /// This operation is O(1).
    #[inline(always)]
    fn clone(&self) -> Self {
        unsafe {
            (*self.ptr.as_ptr()).refs += 1;
        }

        Self { ptr: self.ptr }
    }
}

impl<T> Deref for Mrc<T> {
    type Target = T;

    /// Dereferences the `Mrc` to the contained value.
    ///
    /// This is equivalent to calling [`Mrc::get`].
    #[inline(always)]
    fn deref(&self) -> &T {
        self.get()
    }
}

impl<T> Drop for Mrc<T> {
    /// Releases one strong reference to the contained value.
    ///
    /// When the last strong reference is dropped, the contained value and its
    /// allocation are dropped.
    ///
    /// Dropping an `Mrc` normally requires only a reference count decrement.
    /// Deallocation occurs only when the strong reference count reaches zero.
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            let inner = self.inner();
            let refs = inner.refs - 1;

            if refs == 0 {
                drop(Box::from_raw(self.ptr.as_ptr()));
            } else {
                (*self.ptr.as_ptr()).refs = refs;
            }
        }
    }
}

static_assertions::assert_not_impl_any!(Mrc<String>: Send, Sync);