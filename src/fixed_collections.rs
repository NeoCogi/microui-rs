use core::ops::{IndexMut, Index};

pub trait IVec<T: Default + Copy> {
    fn push(&mut self, t: T) -> (&mut T, usize);
    fn pop(&mut self);

    fn top(&self) -> Option<&T>;

    fn top_mut(&mut self) -> Option<&mut T>;

    fn capacity(&self) -> usize;
    fn len(&self) -> usize;
    fn reset(&mut self);

    fn to_slice(&self) -> &[T];
    fn to_slice_mut(&mut self) -> &mut [T];

    fn get(&self, index: usize) -> &T;
    fn get_mut(&mut self, index: usize) -> &mut T;

    fn append(&mut self, other: &[T]) {
        for e in other {
            self.push(e.clone());
        }
    }
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
    fn reset(&mut self) {
        for i in 0..self.idx {
            self.items[i] = T::default();
        }
        self.idx = 0;
    }

    fn to_slice(&self) -> &[T] {
        &self.items[0..self.idx]
    }

    fn to_slice_mut(&mut self) -> &mut [T] {
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

#[derive(Clone)]
pub struct FixedString<const N: usize> {
    len: usize,
    vec: FixedVec<char, N>,
}