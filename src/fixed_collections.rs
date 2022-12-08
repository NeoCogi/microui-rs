//
// Copyright 2022-Present (c) Raja Lehtihet & Wael El Oraiby
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
// may be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//
use core::ops::{IndexMut, Index, AddAssign};
use core::str::Chars;
use core::cmp::Ordering;

pub trait IVec<T: Default + Copy> {
    fn push(&mut self, t: T) -> (&mut T, usize);
    fn pop(&mut self);

    fn top(&self) -> Option<&T>;

    fn top_mut(&mut self) -> Option<&mut T>;

    fn capacity(&self) -> usize;
    fn len(&self) -> usize;
    fn clear(&mut self);

    fn as_slice(&self) -> &[T];
    fn as_slice_mut(&mut self) -> &mut [T];

    fn get(&self, index: usize) -> &T;
    fn get_mut(&mut self, index: usize) -> &mut T;

    fn append(&mut self, other: &[T]) {
        for e in other {
            self.push(e.clone());
        }
    }
}

pub fn quick_sort_by<T, F: Fn(&T, &T) -> Ordering>(arr: &mut [T], f: F) {
    let len = arr.len();
    if len == 0 {
        return;
    }
    _quick_sort(arr, 0, (len - 1) as isize, &f);
}

fn _quick_sort<T, F: Fn(&T, &T) -> Ordering>(arr: &mut [T], low: isize, high: isize, f: &F) {
    if low < high {
        let p = partition(arr, low, high, f);
        _quick_sort(arr, low, p - 1, f);
        _quick_sort(arr, p + 1, high, f);
    }
}

fn partition<T, F: Fn(&T, &T) -> Ordering>(arr: &mut [T], low: isize, high: isize, f: &F) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while f(&arr[store_index as usize], &arr[pivot]).is_lt() {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && f(&arr[last_index as usize], &arr[pivot]).is_gt() {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}

#[derive(Clone)]
pub struct FixedVec<T: Default + Copy, const N: usize> {
    idx: usize,
    items: [T; N],
}

impl<T: Default + Copy, const N: usize> IVec<T> for FixedVec<T, N> {
    fn push(&mut self, t: T) -> (&mut T, usize) {
        assert!(self.idx < N - 1);
        self.items[self.idx] = t;
        self.idx += 1;
        (&mut self.items[self.idx - 1], self.idx - 1)
    }

    fn pop(&mut self) {
        assert!(self.idx > 0);
        self.idx -= 1;
        self.items[self.idx] = T::default();
    }

    fn top(&self) -> Option<&T> {
        if self.idx != 0 {
            Some(&self.items[self.idx - 1])
        } else {
            None
        }
    }

    fn top_mut(&mut self) -> Option<&mut T> {
        if self.idx != 0 {
            Some(&mut self.items[self.idx - 1])
        } else {
            None
        }
    }

    fn capacity(&self) -> usize {
        N
    }
    fn len(&self) -> usize {
        self.idx
    }
    fn clear(&mut self) {
        for i in 0..self.idx {
            self.items[i] = T::default();
        }
        self.idx = 0;
    }

    fn as_slice(&self) -> &[T] {
        &self.items[0..self.idx]
    }

    fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.items[0..self.idx]
    }

    fn get(&self, index: usize) -> &T {
        assert!((index as usize) < self.idx);
        assert!((index as usize) < N);
        &self.items[index as usize]
    }

    fn get_mut(&mut self, index: usize) -> &mut T {
        assert!((index as usize) < self.idx);
        assert!((index as usize) < N);
        &mut self.items[index as usize]
    }
}

impl<T: Default + Copy, const N: usize> Index<usize> for FixedVec<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index as usize)
    }
}

impl<T: Default + Copy, const N: usize> IndexMut<usize> for FixedVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index as usize)
    }
}

impl<T: Default + Copy, const N: usize> Default for FixedVec<T, N> {
    fn default() -> Self {
        Self { idx: 0, items: [T::default(); N] }
    }
}

impl<T: Default + Copy> Index<usize> for dyn IVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index as usize)
    }
}

impl<T: Default + Copy> IndexMut<usize> for dyn IVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index as usize)
    }
}

impl<const N: usize> core::fmt::Write for FixedVec<char, N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            if self.idx < N - 1 {
                self.push(c);
            } else {
                return Err(core::fmt::Error);
            }
        }
        Ok(())
    }
}

pub trait IString {
    fn push(&mut self, c: char);
    fn pop(&mut self);
    fn as_str(&self) -> &str;
    fn chars(&self) -> Chars<'_>;
    fn char_count(&self) -> usize;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn as_u8_slice(&self) -> &[u8];
    fn capacity(&self) -> usize;
    fn add_str(&mut self, s: &str);

    fn append_real(&mut self, fmt: &str, v: f32) {
        assert!(self.capacity() - self.len() >= 32);
        unsafe {
            let mut fmt_ascii = [0u8; 32];
            let mut i = 0;
            for c in fmt.chars() {
                fmt_ascii[i] = c as u8;
                i += 1;
            }
            fmt_ascii[i] = 0;

            let mut number_ascii = [0u8; 32];
            let number_ptr = number_ascii.as_mut_ptr();
            libc::sprintf(number_ptr as *mut libc::c_char, fmt_ascii.as_ptr() as *const libc::c_char, v as f64);
            for c in 0..number_ascii.len() {
                if number_ascii[c] != 0 {
                    self.push(number_ascii[c] as char);
                } else {
                    break;
                }
            }
        }
    }

    fn append_int(&mut self, fmt: &str, v: i32) {
        assert!(self.capacity() - self.len() >= 32);
        unsafe {
            let mut fmt_ascii = [0u8; 32];
            let mut i = 0;
            for c in fmt.chars() {
                fmt_ascii[i] = c as u8;
                i += 1;
            }
            fmt_ascii[i] = 0;

            let mut number_ascii = [0u8; 32];
            let number_ptr = number_ascii.as_mut_ptr();
            libc::sprintf(number_ptr as *mut libc::c_char, fmt_ascii.as_ptr() as *const libc::c_char, v);
            for c in 0..number_ascii.len() {
                if number_ascii[c] != 0 {
                    self.push(number_ascii[c] as char);
                } else {
                    break;
                }
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct FixedString<const N: usize> {
    char_count: usize,
    vec: FixedVec<u8, N>,
}

impl<const N: usize> FixedString<N> {
    pub fn new() -> Self {
        Self { char_count: 0, vec: FixedVec::default() }
    }
}

impl<const N: usize> IString for FixedString<N> {
    fn push(&mut self, c: char) {
        let mut encoding = [0; 4];
        let bytes = c.encode_utf8(&mut encoding).bytes();
        for b in bytes {
            self.vec.push(b);
        }
        self.char_count += 1;
    }

    fn pop(&mut self) {
        let ch = self.chars().rev().next().unwrap();

        let bc = ch.len_utf8();
        for _ in 0..bc {
            self.vec.pop();
        }
        self.char_count -= 1;
    }

    fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.vec.as_slice()) }
    }

    fn chars(&self) -> Chars<'_> {
        self.as_str().chars()
    }

    fn char_count(&self) -> usize {
        self.char_count
    }

    fn clear(&mut self) {
        self.char_count = 0;
        self.vec.clear();
    }

    fn len(&self) -> usize {
        self.vec.len()
    }

    fn as_u8_slice(&self) -> &[u8] {
        self.vec.as_slice()
    }

    fn capacity(&self) -> usize {
        N
    }

    fn add_str(&mut self, s: &str) {
        for c in s.chars() {
            self.push(c)
        }
    }
}

impl<const N: usize> AddAssign<&str> for FixedString<N> {
    fn add_assign(&mut self, rhs: &str) {
        for c in rhs.chars() {
            self.push(c);
        }
    }
}

impl AddAssign<&str> for dyn IString {
    fn add_assign(&mut self, rhs: &str) {
        for c in rhs.chars() {
            self.push(c);
        }
    }
}

// impl<const N: usize> core::fmt::Write for FixedString<N> {
//     fn write_str(&mut self, s: &str) -> core::fmt::Result {
//         for c in s.chars() {
//             if self.vec.len() < N - 1 {
//                 self.push(c);
//             } else {
//                 return Err(core::fmt::Error);
//             }
//         }
//         Ok(())
//     }
// }

impl<const N: usize> Index<usize> for FixedString<N> {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        self.vec.get(index as usize)
    }
}

impl<const N: usize> Index<core::ops::Range<usize>> for FixedString<N> {
    type Output = str;
    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        unsafe { core::str::from_utf8_unchecked(&self.vec.as_slice()[index.start..index.end]) }
    }
}
