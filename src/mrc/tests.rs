/*
 * @file            src/mrc/tests.rs
 * @description     
 * @author          TrollMii <trollmii@proton.me>
 * @createTime      2026-09-30 19:16:28
 * @lastModified    2026-09-30 19:18:27
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

use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn new_contains_value() {
    let value = Mrc::new(42);

    assert_eq!(*value.get(), 42);
}

#[test]
fn deref_returns_value() {
    let value = Mrc::new(123);

    assert_eq!(*value, 123);
    assert_eq!(*value.get(), 123);
}

#[test]
fn initial_strong_count_is_one() {
    let value = Mrc::new(42);

    assert_eq!(value.strong_count(), 1);
}

#[test]
fn clone_increments_strong_count() {
    let value = Mrc::new(42);
    let clone = value.clone();

    assert_eq!(value.strong_count(), 2);
    assert_eq!(clone.strong_count(), 2);
}

#[test]
fn dropping_clone_decrements_strong_count() {
    let value = Mrc::new(42);

    {
        let clone = value.clone();
        assert_eq!(value.strong_count(), 2);

        drop(clone);
    }

    assert_eq!(value.strong_count(), 1);
}

#[test]
fn clones_share_the_same_value() {
    let value = Mrc::new(42);
    let clone = value.clone();

    assert_eq!(*value, *clone);
}

#[test]
fn set_changes_shared_value() {
    let value = Mrc::new(42);
    let clone = value.clone();

    value.set(100);

    assert_eq!(*value, 100);
    assert_eq!(*clone, 100);
}

#[test]
fn set_drops_old_value() {
    struct DropCounter<'a> {
        counter: &'a AtomicUsize,
    }

    impl Drop for DropCounter<'_> {
        fn drop(&mut self) {
            self.counter.fetch_add(1, Ordering::Relaxed);
        }
    }

    let counter = AtomicUsize::new(0);

    let value = Mrc::new(DropCounter { counter: &counter });

    value.set(DropCounter { counter: &counter });

    assert_eq!(counter.load(Ordering::Relaxed), 1);
}

#[test]
fn last_reference_drops_value() {
    struct DropCounter<'a> {
        counter: &'a AtomicUsize,
    }

    impl Drop for DropCounter<'_> {
        fn drop(&mut self) {
            self.counter.fetch_add(1, Ordering::Relaxed);
        }
    }

    let counter = AtomicUsize::new(0);

    {
        let value = Mrc::new(DropCounter { counter: &counter });

        let clone = value.clone();

        assert_eq!(counter.load(Ordering::Relaxed), 0);

        drop(clone);

        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    assert_eq!(counter.load(Ordering::Relaxed), 1);
}

#[test]
fn many_clones_drop_correctly() {
    let value = Mrc::new(42);

    let clones: Vec<_> = (0..1000).map(|_| value.clone()).collect();

    assert_eq!(value.strong_count(), 1001);

    for clone in &clones {
        assert_eq!(**clone, 42);
    }

    assert_eq!(value.strong_count(), 1001);

    drop(clones);

    assert_eq!(value.strong_count(), 1);
}
